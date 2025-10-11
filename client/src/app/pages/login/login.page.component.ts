import { Component, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { Router } from '@angular/router';

import { AccountService } from '../../services/account.service';
import { InvalidNameOrPasswordServiceError } from '../../services/errors';

const NAME_REGEX = /^[a-zA-Z0-9_]{3,20}$/;

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

    error: string = '';

    constructor(private accountService: AccountService, private router: Router) { }

    ngOnInit() {
        if (this.accountService.isLoggedIn) {
            this.router.navigate(['/']);
        }
    }

    async onLogInClick() {
        // Clean up inputs
        this.name = this.name.trim();

        // Validate inputs
        {
            let missing = [];
            let invalid = [];

            if (this.name === '')
                missing.push("Name");
            else if (!NAME_REGEX.test(this.name))
                invalid.push("Name");

            if (this.password === '')
                missing.push("Password");

            if (missing.length > 0 || invalid.length > 0) {
                let errors = []

                if (missing.length > 0)
                    errors.push("Missing fields: " + missing.join(", "));

                if (invalid.length > 0)
                    errors.push("Invalid fields: " + invalid.join(", "));

                this.error = errors.join("\n");
                return;
            }
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
