import { ChangeDetectorRef, Component, OnInit } from '@angular/core';
import { NavigationEnd, Router, RouterLink } from "@angular/router";
import { AccountService } from '../../services/account.service';

@Component({
    selector: 'app-nav-bar',
    imports: [RouterLink],
    templateUrl: './nav-bar.component.html',
    styleUrl: './nav-bar.component.scss'
})
export class NavBarComponent implements OnInit {
    menuOpen: boolean = false;

    get isLoggedIn(): boolean {
        return this.accountService.isLoggedIn;
    }

    get nickname(): string | undefined {
        return this.accountService.nickname;
    }

    constructor(private router: Router, private accountService: AccountService, private changeDetectorRef: ChangeDetectorRef) {
        router.events.subscribe((event) => {
            // Close menu on route change
            if (event instanceof NavigationEnd)
                this.menuOpen = false;
        });
    }

    ngOnInit(): void {
        this.accountService.onChanged.subscribe(() => this.changeDetectorRef.markForCheck());
    }
}
