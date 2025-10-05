export interface RawLoginRequestBody {
    name: string;
    password: string;
} 

export interface RawLoginResponseBody {
    token: string;
    id: number;
    nickname?: string;
    rights: string;
} 

export interface RawRegisterRequestBody {
    name: string;
    nickname?: string;
    email: string;
    password: string;
} 