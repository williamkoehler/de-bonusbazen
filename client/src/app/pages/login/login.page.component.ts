import { ChangeDetectorRef, Component, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { Router } from '@angular/router';

import { AccountService } from '../../services/account.service';
import * as errors from '../../services/errors';
import { SpinnerComponent } from "../../components/spinner/spinner.component";

const NAME_REGEX = /^[a-zA-Z0-9_]{3,20}$/;

const NICE_MESSAGES = [
    "That is taking longer than expected, but we're on it!",
    "Good things come to those who wait! Still working on it...",
    "Just a moment more, we're logging you in.",
    "Why is the login taking so long? We don't even know...",
    "Thanks for your patience! We're almost there.",
    "Sometimes the server needs a moment to think...",
    "Maybe grab a coffee a GEWIS while we log you in?",
    "Maybe it's your internet connection?",
    "What kind of shitty connection are you using?",
    "Ben jij en appel?",
    "Almost there! Just a few more seconds...",
    "Mijn oma is een olifant!",
]

interface Error {
    message: string;
    flaggedFields?: string[];
}

@Component({
    selector: 'app-login-page',
    imports: [
        FormsModule,
        SpinnerComponent
    ],
    templateUrl: './login.page.component.html',
    styleUrl: './login.page.component.scss'
})
export class LoginPageComponent implements OnInit {
    name: string = '';
    password: string = '';


    loading: boolean = false;
    message?: string;
    error?: Error;

    constructor(private accountService: AccountService, private router: Router, private changeDetectorRef: ChangeDetectorRef) { }

    ngOnInit() {
        if (this.accountService.isLoggedIn) {
            this.router.navigate(['/']);
        }
    }

    showNiceMessage(delay: number = 1500) {
        setTimeout(() => {
            if (this.loading) {
                this.message = NICE_MESSAGES[Math.floor(Math.random() * NICE_MESSAGES.length)];
                this.changeDetectorRef.markForCheck();
                this.showNiceMessage(8000);
            }
        }, delay);
    }

    async onLogInClick() {
        try {
            this.loading = true;
            this.error = undefined;
            this.changeDetectorRef.markForCheck();

            this.showNiceMessage();

            // Clean up inputs
            this.name = this.name.trim();

            // Validate inputs
            {
                const lines = [];
                const flaggedFields = [];

                if (this.name === '') {
                    lines.push("A valid name is required.");
                    flaggedFields.push("name");
                }
                else if (!NAME_REGEX.test(this.name)) {
                    lines.push("You could at least provide a valid name.");
                    flaggedFields.push("name");
                }

                if (this.password === '') {
                    lines.push("Why would you login without a password?");
                    flaggedFields.push("password");
                }

                if (flaggedFields.length > 0) {
                    this.error = {
                        message: lines.join("\n"),
                        flaggedFields: flaggedFields,
                    }
                    this.changeDetectorRef.detectChanges();
                    return;
                }
            }

            try {
                await this.accountService.login(this.name, this.password);

                this.router.navigate(['/']);
            }
            catch (err) {
                const lines = [];
                const flaggedFields = [];

                if (err instanceof errors.UnverifiedError) {
                    lines.push("Your account has not yet been verified.");
                    flaggedFields.push("name");
                }
                else if (err instanceof errors.UnauthenticatedError) {
                    lines.push("Name or password are not valid.");
                    flaggedFields.push("name");
                    flaggedFields.push("password");
                }
                else if (err instanceof errors.JwtGenerationFailedError) {
                    lines.push("A server error occuring while generating a user token.");
                    flaggedFields.push("email");
                }
                else
                    lines.push('An unexpected error occurred. Please try again later...');

                this.error = {
                    message: lines.join("\n"),
                    flaggedFields: flaggedFields,
                }
            }
        }
        finally {
            this.loading = false;
            this.message = undefined;
            this.changeDetectorRef.markForCheck();
        }
    }
}
