import { Component, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { AccountService } from '../../services/account.service';
import { Router } from '@angular/router';

@Component({
    selector: 'app-register-page',
    standalone: true,
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

    error?: string;

    constructor(private accountService: AccountService, private router: Router) { }

    ngOnInit() {
        if (this.accountService.isLoggedIn) {
            this.router.navigate(['/login']);
        }
    }

    async onRegisterClick() {
        await this.accountService.register(this.name, this.nickname, this.email, this.password);

        this.router.navigate(['/']);
    }
}
