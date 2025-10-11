import { Component } from '@angular/core';
import { RouterLink } from "@angular/router";
import { AccountService } from '../../services/account.service';

@Component({
    selector: 'app-nav-bar',
    imports: [RouterLink],
    templateUrl: './nav-bar.component.html',
    styleUrl: './nav-bar.component.scss'
})
export class NavBarComponent {
    get isLoggedIn(): boolean {
        return this.authenticationService.isLoggedIn;
    }

    get nickname(): string | undefined {
        return this.authenticationService.nickname;
    }
    
    constructor(private authenticationService: AccountService) { }
}
