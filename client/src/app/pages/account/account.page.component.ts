import { ChangeDetectorRef, Component, OnInit } from '@angular/core';
import { Router } from '@angular/router';

import { AccountService } from '../../services/account.service';
import { User } from '../../services/models/user';
import { UserService } from '../../services/user.service';
import { FormsModule } from '@angular/forms';
import { PendingState } from '../../misc';

@Component({
    selector: 'app-account-page',
    imports: [
        FormsModule
    ],
    templateUrl: './account.page.component.html',
    styleUrl: './account.page.component.scss'
})
export class AccountPageComponent implements OnInit {
    user?: User;

    _profilePictureFile?: File;
    _profilePictureUrl?: string;
    _name?: string;
    _nickname?: string;
    _email?: string;

    updatePendingState: PendingState = PendingState.None;

    password?: string;
    password2?: string;
    updatePasswordPendingState: PendingState = PendingState.None;

    get hasProfilePicture() {
        return (this.user?.hasProfilePicture ?? false) || this._profilePictureUrl != undefined;
    }
    get profilePictureUrl() {
        if (this._profilePictureUrl)
            return this._profilePictureUrl;
        else
            return `api/users/${this.id}/profile_picture`;
    }
    get hadProfilePictureChanged() {
        return this._profilePictureUrl != undefined;
    }

    get id() {
        return this.accountService.id!;
    }

    get nickname() {
        return this._nickname ?? this.accountService.nickname ?? "";
    }
    set nickname(value: string) {
        this._nickname = value;
    }
    get hasNicknameChanged() {
        return this.nickname != this.accountService.nickname;
    }

    get email() {
        return this._email ?? this.accountService.email ?? "";
    }
    set email(value: string) {
        this._email = value;
    }
    get hasEmailChanged() {
        return this.email != this.accountService.email;
    }

    constructor(private accountService: AccountService, private userService: UserService, private router: Router, private changeDetectorRef: ChangeDetectorRef) { }

    ngOnInit() {
        if (!this.accountService.isLoggedIn) {
            this.router.navigate(['/login']);
        }

        this.userService.getUserById(this.accountService.id!).then(user => {
            this.user = user;
            this.changeDetectorRef.detectChanges();
        });
    }

    onFileSelected(event: Event) {
        const input = event.target as HTMLInputElement;
        if (input.files && input.files[0]) {
            this._profilePictureFile = input.files[0];

            console.info(this._profilePictureFile);

            var reader = new FileReader();

            reader.readAsDataURL(this._profilePictureFile);

            reader.onload = (event) => {
                this._profilePictureUrl = event.target!.result as string;
                console.info(this._profilePictureUrl);
                this.changeDetectorRef.detectChanges();
            }
        }
    }

    async onUpdateClick() {
        if (this.updatePendingState != PendingState.Pending) {
            try {
                this.updatePendingState = PendingState.Pending;

                if (this._profilePictureFile)
                    await this.accountService.updateProfilePicture(this._profilePictureFile);

                await this.accountService.update(this._nickname, this._email, undefined);

                setTimeout(() => {
                    this.updatePendingState = PendingState.None;
                    this.changeDetectorRef.detectChanges();
                }, 1000);
            } catch {
                this.updatePendingState = PendingState.Error;
                this.changeDetectorRef.detectChanges();
            }
        }
    }

    async onUpdatePasswordClick() {
        if (this.updatePasswordPendingState != PendingState.Pending) {
            try {
                this.updatePasswordPendingState = PendingState.Pending;

                if (!this.password)
                    throw new Error("Password is empty");
                if (this.password != this.password2)
                    throw new Error("Passwords do not match");

                await this.accountService.update(undefined, undefined, this.password);

                setTimeout(() => {
                    this.updatePasswordPendingState = PendingState.None;
                    this.changeDetectorRef.detectChanges();
                }, 1000);
            } catch {
                this.updatePasswordPendingState = PendingState.Error;
                this.changeDetectorRef.detectChanges();
            }
        }
    }

    onLogOutClick() {
        this.accountService.logout();
        this.router.navigate(['/']);
    }
}
