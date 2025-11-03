export interface RawReCaptchaResponseBody {
    site_key: string;
}

export interface RawLoginRequestBody {
    name: string;
    password: string;
}

export interface RawLoginResponseBody {
    token: string;
    id: number;
    nickname?: string;
    email: string;
    rights: string;
}

export interface RawRegisterRequestBody {
    recaptcha: string;
    name: string;
    nickname?: string;
    email: string;
    password: string;
}

export interface RawRegisterErrorBody {
    reason: string;
}

export interface RawUpdateRequestBody {
    nickname?: string;
    email?: string;
    password?: string;
}