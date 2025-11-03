import { HttpClient, HttpErrorResponse, HttpStatusCode } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { RawLoginRequestBody, RawLoginResponseBody, RawReCaptchaResponseBody, RawRegisterRequestBody, RawUpdateRequestBody } from './models/account';
import { errorFromReason, handleError, InternalServiceError } from './errors';
import store from 'store2';

const RECAPTCHA_SCRIPT_ID = 'recaptcha-script';

declare const grecaptcha: any;

interface AccountInfo {
    token: string;
    id: number;
    name: string;
    nickname: string;
    email?: string;
    rights: string;
}

@Injectable({
    providedIn: 'root'
})
export class AccountService {
    _info?: AccountInfo;
    _recaptchaSiteKey?: string;

    get isLoggedIn(): boolean {
        return !!this._info;
    }

    get token(): string | undefined {
        return this._info?.token;
    }

    get id(): number | undefined {
        return this._info?.id;
    }

    get name(): string | undefined {
        return this._info?.name;
    }

    get nickname(): string | undefined {
        return this._info?.nickname;
    }

    get email(): string | undefined {
        return this._info?.email;
    }

    get rights(): string | undefined {
        return this._info?.rights;
    }

    get httpHeaders(): Record<string, string> {
        if (this.token !== undefined) {
            return {
                Authorization: `Bearer ${this.token}`
            }
        }
        else {
            return {};
        }
    }

    get hasAdminRights(): boolean {
        return this.rights === 'admin' || this.rights === 'member' || this.rights === 'maintainer';
    }

    constructor(private httpClient: HttpClient) {
        this.readStore();
    }

    private async readStore() {
        const accountStore = store.namespace('account');

        const token = accountStore('token');
        if (typeof token !== 'string')
            return;

        if (await this.check(token)) {
            this._info = {
                token: token,
                id: accountStore('id'),
                name: accountStore('name'),
                nickname: accountStore('nickname'),
                email: accountStore('email'),
                rights: accountStore('rights'),
            }
        }
        else {
            accountStore.remove('token')
        }
    }

    private writeStore() {
        const accountStore = store.namespace('account');

        if (this._info) {
            accountStore('token', this._info.token);
            accountStore('id', this._info.id);
            accountStore('name', this._info.name);
            accountStore('nickname', this._info.nickname);
            accountStore('email', this._info.email);
            accountStore('rights', this._info.rights);
        }
        else {
            accountStore.clear();
        }
    }

    async check(token?: string): Promise<boolean> {
        token ??= this.token;
        if (!token)
            return false;

        return await new Promise((resolve, reject) => {
            this.httpClient.get('/api/check', {
                headers: {
                    Authorization: `Bearer ${token}`
                }
            }).subscribe({
                next: _ => {
                    resolve(true);
                },
                error: (err: HttpErrorResponse) => {
                    if (err.status === HttpStatusCode.Unauthorized) {
                        resolve(false);
                    }
                    else
                        reject(err);
                }
            });
        });
    }

    async login(name: string, password: string) {
        return new Promise<void>((resolve, reject) => {
            const requestBody: RawLoginRequestBody = {
                name: name,
                password: password,
            }

            this.httpClient.post<RawLoginResponseBody>('/api/login', requestBody).subscribe({
                next: body => {
                    this._info = {
                        token: body.token,
                        id: body.id,
                        name: name,
                        nickname: body.nickname ?? name,
                        email: body.email,
                        rights: body.rights
                    };

                    this.writeStore();

                    resolve();
                },
                error: (err: HttpErrorResponse) => {
                    reject(handleError(err));
                }
            });
        });
    }

    logout() {
        this._info = undefined;
        this.writeStore();
    }

    private async prepare() {
        // ReCaptcha script already loaded
        if (document.getElementById(RECAPTCHA_SCRIPT_ID) && this._recaptchaSiteKey) {
            return;
        }

        const siteKeyPromise = new Promise<void>(async (resolve, reject) => {
            this.httpClient.get<RawReCaptchaResponseBody>('/api/recaptcha', {}).subscribe({
                next: body => {
                    this._recaptchaSiteKey = body.site_key;

                    resolve();
                },
                error: (err: HttpErrorResponse) => {
                    console.error('Failed to get reCAPTCHA site key: ', err);
                    reject(new InternalServiceError('Failed to get reCAPTCHA site key: ' + err.message));
                }
            });
        });

        await siteKeyPromise;

        const script = document.createElement('script');
        script.id = RECAPTCHA_SCRIPT_ID;
        script.src = `https://www.google.com/recaptcha/api.js?render=${this._recaptchaSiteKey}`;
        script.async = true;
        script.defer = true;

        const loadPromise = new Promise<void>(async (resolve, reject) => {
            script.onload = () => resolve();
            script.onerror = () => reject('Failed to load reCAPTCHA script');
        });

        document.head.appendChild(script);

        await loadPromise;
    }

    async register(name: string, nickname: string, email: string, password: string) {
        await this.prepare();

        const recaptcha = await new Promise<string>((resolve, reject) => {
            grecaptcha.ready(() => {
                grecaptcha.execute(this._recaptchaSiteKey, { action: 'submit' })
                    .then(async (token: any) => {
                        console.log("successfully loaded reCAPTCHA token: ", token);
                        resolve(token);
                    }).
                    catch((err: any) => {
                        console.log("failed to load reCAPTCHA token: ", err);
                        reject(new InternalServiceError('Failed to load reCAPTCHA token.'));
                    });
            });
        });

        return new Promise<void>((resolve, reject) => {
            const requestBody: RawRegisterRequestBody = {
                recaptcha: recaptcha,
                name: name,
                nickname: nickname,
                email: email,
                password: password,
            }

            this.httpClient.post('/api/register', requestBody).subscribe({
                next: _ => {
                    resolve();
                },
                error: (err: HttpErrorResponse) => {
                    reject(handleError(err));
                }
            });
        });
    }

    async update(nickname?: string, email?: string, password?: string) {
        if (this._info) {
            return new Promise<void>((resolve, reject) => {
                const requestBody: RawUpdateRequestBody = {
                    nickname: nickname,
                    email: email,
                    password: password,
                }

                this.httpClient.patch(
                    `/api/users/${this.id}`,
                    requestBody,
                    {
                        headers: {
                            Authorization: `Bearer ${this.token}`
                        }
                    }
                )
                    .subscribe({
                        next: _ => {
                            if (nickname)
                                this._info!.nickname = nickname;
                            if (email)
                                this._info!.email = email;

                            this.writeStore();

                            resolve();
                        },
                        error: err => {
                            reject(err);
                        }
                    });
            });
        }
    }

    async updateProfilePicture(file: File) {
        if (this._info) {
            return new Promise<void>((resolve, reject) => {
                var reader = new FileReader();

                reader.readAsArrayBuffer(file);

                reader.onload = (event) => {
                    const arrayBuffer = event.target!.result as ArrayBuffer;

                    this.httpClient.patch(
                        `/api/users/${this.id}/profile_picture`,
                        arrayBuffer,
                        {
                            headers: {
                                Authorization: `Bearer ${this.token}`
                            }
                        }
                    )
                        .subscribe({
                            next: _ => {
                                resolve();
                            },
                            error: err => {
                                reject(err);
                            }
                        });

                }
            });
        }
    }
}