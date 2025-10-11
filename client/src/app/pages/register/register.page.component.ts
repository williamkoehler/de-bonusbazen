import { Component, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { AccountService } from '../../services/account.service';
import { Router } from '@angular/router';

const NAME_REGEX = /^[a-zA-Z0-9_]{3,20}$/;
const NICKNAME_REGEX = /^.{3,30}$/;
const EMAIL_REGEX = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;

@Component({
    selector: 'app-register-page',
    imports: [
        FormsModule
    ],
    templateUrl: './register.page.component.html',
    styleUrl: './register.page.component.scss'
})
export class RegisterPageComponent implements OnInit {
    name: string = '';
    nickname: string = '';
    email: string = '';
    password: string = '';

    error: string = '';

    constructor(private accountService: AccountService, private router: Router) { }

    ngOnInit() {
        if (this.accountService.isLoggedIn) {
            this.router.navigate(['/login']);
        }
    }

    async onRegisterClick() {
        // Clean up inputs
        this.name = this.name.trim();
        this.nickname = this.nickname.trim();
        this.email = this.email.trim();

        // Validate inputs
        {
            let missing = [];
            let invalid = [];

            if (this.name === '')
                missing.push("Name");
            else if (!NAME_REGEX.test(this.name))
                invalid.push("Name");

            if (this.nickname === '')
                missing.push("Nickname");
            else if (!NICKNAME_REGEX.test(this.nickname))
                invalid.push("Nickname");

            if (this.email === '')
                missing.push("E-Mail");
            else if (!EMAIL_REGEX.test(this.email))
                invalid.push("E-Mail");

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

        await this.accountService.register(this.name, this.nickname, this.email, this.password);

        this.router.navigate(['/']);
    }
}
