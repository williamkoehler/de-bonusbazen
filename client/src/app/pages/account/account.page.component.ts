import { ChangeDetectorRef, Component, OnInit } from '@angular/core';
import { Router } from '@angular/router';

import { AccountService } from '../../services/account.service';
import { User } from '../../services/models/user';
import { UserService } from '../../services/user.service';

@Component({
    selector: 'app-account-page',
    imports: [],
    templateUrl: './account.page.component.html',
    styleUrl: './account.page.component.scss'
})
export class AccountPageComponent implements OnInit {
    user?: User;

    get hasProfilePicture() {
        return this.user?.hasProfilePicture ?? false;
    }

    get id() {
        return this.accountService.id!;
    }

    get name() {
        return this.accountService.name!;
    }

    get nickname() {
        return this.accountService.nickname;
    }

    get email() {
        return this.accountService.name;
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

    onLogOutClick() {
        this.accountService.logout();
        this.router.navigate(['/']);
    }
}
