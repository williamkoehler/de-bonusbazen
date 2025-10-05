export class User {
    id: number;
    name: string
    nickname: string | undefined;
    rights: UserRights;
    hasProfilePicture: boolean;

    secondaryDataLoaded: boolean = false;

    constructor(rawUser: RawUser) {
        if (!rawUser.id)
            throw new Error("User must have an id");

        this.id = rawUser.id;
        this.name = rawUser.name ?? "[unknown]";
        this.nickname = rawUser.nickname;
        switch (rawUser.rights) {
            case 'normal':
                this.rights = UserRights.Normal;
                break;
            case 'member':
                this.rights = UserRights.Member;
                break;
            case 'maintainer':
                this.rights = UserRights.Maintainer;
                break;
            case 'admin':
                this.rights = UserRights.Admin;
                break;
            default:
            case 'unauthenticated':
                this.rights = UserRights.Unauthenticated;
                break;
        }
        this.hasProfilePicture = rawUser.has_profile_picture ?? false;
    }
}

export enum UserRights {
    Unauthenticated = 0,
    Normal = 1,
    Member = 2,
    Maintainer = 3,
    Admin = 4,
}

export interface RawUser {
    id?: number;
    name?: string;
    nickname?: string;
    rights?: string;
    has_profile_picture?: boolean;
}