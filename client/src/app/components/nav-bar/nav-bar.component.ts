import { Component } from '@angular/core';
import { NavigationEnd, Router, RouterLink } from "@angular/router";
import { AccountService } from '../../services/account.service';

@Component({
    selector: 'app-nav-bar',
    imports: [RouterLink],
    templateUrl: './nav-bar.component.html',
    styleUrl: './nav-bar.component.scss'
})
export class NavBarComponent {
    menuOpen: boolean = false;

    get isLoggedIn(): boolean {
        return this.authenticationService.isLoggedIn;
    }

    get nickname(): string | undefined {
        return this.authenticationService.nickname;
    }

    constructor(private router: Router, private authenticationService: AccountService) {
        router.events.subscribe((event) => {
            // Close menu on route change
            if (event instanceof NavigationEnd)
                this.menuOpen = false;
        });
    }
}
