import { EventEmitter, Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { RawUser, User } from './models/user';

@Injectable({
    providedIn: 'root'
})
export class UserService {
    _users: User[] = [];
    _userIds: Map<number, number> = new Map();
    _lastGetUsers?: number;
    _usersPromise?: Promise<User[]>;

    public onUsersChanged: EventEmitter<User[]> = new EventEmitter();

    public get users(): User[] {
        return this._users;
    }

    constructor(private httpClient: HttpClient) { }

    public async getUserById(id: number): Promise<User> {
        const userIndex = this._userIds.get(id);
        if (userIndex !== undefined) {
            const user = this._users[userIndex];
            if (user.secondaryDataLoaded) return user;
        }

        // Fetch secondary user data
        return new Promise((resolve, reject) => {
            this.httpClient.get<RawUser>(`/api/users/${id}`).subscribe({
                next: rawUser => {
                    const user = new User(rawUser);

                    const index = this._userIds.get(user.id);
                    if (index)
                        this._users[index] = user;
                    else {
                        this._userIds.set(user.id, this._users.length);
                        this._users.push(user);
                    }

                    resolve(user);
                    this.onUsersChanged.emit(this._users);
                },
                error: err => reject(err)
            });
        });

    }

    public getUsers(): Promise<User[]> {
        if (!this._lastGetUsers || (Date.now() - this._lastGetUsers) > 60000) { // 1 minute cache
            this._users = [];
            this._userIds = new Map();

            if (!this._usersPromise) {
                this._usersPromise = new Promise((resolve, _reject) => {
                    this.httpClient.get<RawUser[]>('/api/users').subscribe({
                        next: rawUsers => {
                            for (const rawUser of rawUsers) {
                                const user = new User(rawUser);

                                const index = this._userIds.get(user.id);
                                if (index)
                                    this._users[index] = user;
                                else {
                                    this._userIds.set(user.id, this._users.length);
                                    this._users.push(user);
                                }
                            }

                            this._lastGetUsers = Date.now();
                            this._usersPromise = undefined;

                            resolve(this._users);
                            this.onUsersChanged.emit(this._users);
                        }
                    });
                });
            }

            return this._usersPromise;
        }
        else
            return Promise.resolve(this._users);
    }
}
