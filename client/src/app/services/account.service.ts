import { HttpClient, HttpErrorResponse, HttpStatusCode } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { RawLoginRequestBody, RawLoginResponseBody, RawRegisterRequestBody } from './models/account';
import { InternalServiceError, InvalidNameOrPasswordServiceError } from './errors';
import store from 'store2';

interface AccountInfo {
    token: string;
    id: number;
    name: string;
    nickname: string;
    rights: string;
}

@Injectable({
    providedIn: 'root'
})
export class AccountService {
    _info?: AccountInfo;

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

    get rights(): string | undefined {
        return this._info?.rights;
    }

    constructor(private httpClient: HttpClient) {
        this.readStore();
    }

    readStore() {
        const accountStore = store.namespace('account');

        const token = accountStore('token');
        if (typeof token !== 'string')
            return;

        this._info = {
            token: token,
            id: accountStore('id'),
            name: accountStore('name'),
            nickname: accountStore('nickname'),
            rights: accountStore('rights'),
        }
    }

    writeStore() {
        const accountStore = store.namespace('account');

        if (this._info) {
            accountStore('token', this._info.token);
            accountStore('id', this._info.id);
            accountStore('name', this._info.name);
            accountStore('nickname', this._info.nickname);
            accountStore('rights', this._info.rights);
        }
        else {
            accountStore.clear();
        }
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
                        rights: body.rights
                    };

                    this.writeStore();

                    resolve();
                },
                error: (err: HttpErrorResponse) => {
                    switch (err.status) {
                        case HttpStatusCode.Unauthorized:
                            reject(new InvalidNameOrPasswordServiceError());
                            break;
                        default:
                            reject(new InternalServiceError(err.message));
                            break;
                    }
                }
            });
        });
    }

    logout() {
        this._info = undefined;
        this.writeStore();
    }

    async register(name: string, nickname: string, email: string, password: string) {
        return new Promise<void>((resolve, reject) => {
            const requestBody: RawRegisterRequestBody = {
                name: name,
                nickname: nickname,
                email: email,
                password: password,
            }

            this.httpClient.post('/api/register', requestBody).subscribe({
                next: _ => {
                    resolve();
                },
                error: err => {
                    reject(err);
                }
            });
        });
    }
}