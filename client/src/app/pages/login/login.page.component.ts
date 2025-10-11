import { Component, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { Router } from '@angular/router';

import { AccountService } from '../../services/account.service';
import { InvalidNameOrPasswordServiceError } from '../../services/errors';

@Component({
    selector: 'app-login-page',
    imports: [
        FormsModule
    ],
    templateUrl: './login.page.component.html',
    styleUrl: './login.page.component.scss'
})
export class LoginPageComponent implements OnInit {
    name: string = '';
    password: string = '';

    error?: string;

    constructor(private accountService: AccountService, private router: Router) { }

    ngOnInit() {
        if (this.accountService.isLoggedIn) {
            this.router.navigate(['/']);
        }
    }

    async onLogInClick() {
        if (this.name == '' || this.password == '') {
            this.error = "Fill in the name and password fields.";
            return;
        }

        try {
            await this.accountService.login(this.name, this.password);

            this.router.navigate(['/']);
        }
        catch (err) {
            if (err instanceof InvalidNameOrPasswordServiceError) {
                this.error = "Invalid name or password.";
            }
            else {
                this.error = "Internal error.";
            }
        }
    }
}
