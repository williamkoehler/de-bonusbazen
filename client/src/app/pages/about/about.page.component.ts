import { AfterContentInit, ChangeDetectorRef, Component } from '@angular/core';
import { UserService } from '../../services/user.service';
import { User, UserRights } from '../../services/models/user';

const QUOTES: string[] = [
    "Among the best, it's the best.",
    "A true gem in the rough.",
    "Unparalleled from start to finish.",
    "Simply outstanding in every way.",
    "A cut above the rest.",
    "Remarkably impressive.",
    "An absolute pleasure to work with.",
    "Consistently exceeds expectations."
];

@Component({
    selector: 'app-about',
    imports: [],
    templateUrl: './about.page.component.html',
    styleUrl: './about.page.component.scss'
})
export class AboutPageComponent implements AfterContentInit {
    members: User[] = [];

    quote: string;

    constructor(private userService: UserService, private changeDetectorRef: ChangeDetectorRef) {
        this.quote = this.getRandomQuote();
    }

    getRandomQuote(): string {
        const now = new Date();
        const minutesSinceEpoch = Math.floor(now.getTime() / 60000); // Convert milliseconds to minutes
        const interval = Math.floor(minutesSinceEpoch / 10); // Change every 10 minutes
        const seed = interval.toString();

        // Generate random number based on seed
        let hash = 0;
        for (let i = 0; i < seed.length; i++) {
            hash = (hash << 5) - hash + seed.charCodeAt(i);
            hash |= 0;
        }

        const index = Math.abs(hash) % QUOTES.length;
        return QUOTES[index];
    }

    ngAfterContentInit() {
        this.userService.getUsers().then(users => {
            this.members = users.filter(user => user.rights >= UserRights.Member).sort((a, b) => a.name.localeCompare(b.name));
            this.changeDetectorRef.detectChanges();
        })
    }
}