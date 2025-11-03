import { ChangeDetectorRef, Component, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { AccountService } from '../../services/account.service';
import { Router } from '@angular/router';
import { SpinnerComponent } from "../../components/spinner/spinner.component";
import * as errors from '../../services/errors';

const NAME_REGEX = /^(?:\p{L}|[_]){3,20}$/u;
const NICKNAME_REGEX = /^(?:\p{L}|[ _]){3,30}$/u;
const EMAIL_REGEX = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;

const NICE_MESSAGES = [
    "Welcome aboard! We are preparing your account.",
    "I know it's taking a bit longer, but good things come to those who wait!",
    "Just a moment more, we're setting things up for you.",
    "Great things are worth waiting for. Your account is almost ready!",
    "Thanks for your patience! We're putting the finishing touches on your account.",
    "Hang tight! We're making sure everything is perfect for you.",
    "Almost there! Your account is being created with care.",
    "Ben jij en appel?",
    "Almost there! Just a few more seconds...",
    "Mijn oma is een olifant!",
]

interface Error {
    message: string;
    flaggedFields?: string[];
}

@Component({
    selector: 'app-register-page',
    imports: [
        FormsModule,
        SpinnerComponent
    ],
    templateUrl: './register.page.component.html',
    styleUrl: './register.page.component.scss'
})
export class RegisterPageComponent implements OnInit {
    name: string = '';
    nickname: string = '';
    email: string = '';
    password: string = '';

    loading: boolean = false;
    message?: string;
    error?: Error;

    constructor(private accountService: AccountService, private router: Router, private changeDetectorRef: ChangeDetectorRef) { }

    ngOnInit() {
        if (this.accountService.isLoggedIn) {
            this.router.navigate(['/login']);
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

    async onRegisterClick() {
        try {
            this.loading = true;
            this.error = undefined;
            this.changeDetectorRef.markForCheck();

            this.showNiceMessage();

            // Clean up inputs
            this.name = this.name.trim();
            this.nickname = this.nickname.trim();
            this.email = this.email.trim();

            // Validate inputs
            {
                const lines = [];
                const flaggedFields = [];

                if (this.name === '') {
                    lines.push("A valid name is required.");
                    flaggedFields.push("name");
                }
                else if (!NAME_REGEX.test(this.name)) {
                    lines.push("A valid name can only contain letters, spaces and underscores.");
                    flaggedFields.push("name");
                }

                if (this.nickname === '') {
                    lines.push("A valid nickname is required.");
                    flaggedFields.push("nickname");
                }
                else if (!NICKNAME_REGEX.test(this.nickname)) {
                    lines.push("A valid nickname can only contain letters, spaces and underscores.");
                    flaggedFields.push("nickname");
                }

                if (this.email === '') {
                    lines.push("An valid E-Mail is required.");
                    flaggedFields.push("email");
                }
                else if (!EMAIL_REGEX.test(this.email)) {
                    lines.push("E-Mail is not valid.");
                    flaggedFields.push("email");
                }

                if (this.password === '') {
                    lines.push("A valid password is required.");
                    flaggedFields.push("password");
                }
                else if (this.password.length < 6) {
                    lines.push("Password must be at least 6 characters long.");
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
                await this.accountService.register(this.name, this.nickname, this.email, this.password);

                this.router.navigate(['/']);
            }
            catch (err) {
                const lines = [];
                const flaggedFields = [];

                if (err instanceof errors.NameIsTakenError) {
                    lines.push("Name is already taken.");
                    flaggedFields.push("name");
                }
                else if (err instanceof errors.InvalidNameError) {
                    lines.push("Your name does not follow the server's guidelines.");
                    flaggedFields.push("name");
                }
                else if (err instanceof errors.InvalidNicknameError) {
                    lines.push("Your nickname does not follow the server's guidelines.");
                    flaggedFields.push("email");
                }
                else if (err instanceof errors.EmailIsTakenError) {
                    lines.push("E-Mail address is already taken.");
                    flaggedFields.push("email");
                }
                else if (err instanceof errors.InvalidEmailError) {
                    lines.push("Your E-Mail address is is not valid.");
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
