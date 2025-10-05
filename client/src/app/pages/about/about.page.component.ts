import { AfterContentInit, ChangeDetectorRef, Component } from '@angular/core';
import { UserService } from '../../services/user.service';
import { User, UserRights } from '../../services/models/user';

@Component({
    selector: 'app-about',
    standalone: true,
    imports: [],
    templateUrl: './about.page.component.html',
    styleUrl: './about.page.component.scss'
})
export class AboutPageComponent implements AfterContentInit {
    members: User[] = []

    constructor(private userService: UserService, private changeDetectorRef: ChangeDetectorRef) { }

    ngAfterContentInit() {
        this.userService.getUsers().then(users => {
            this.members = users.filter(user => user.rights >= UserRights.Member);
            this.changeDetectorRef.detectChanges();
        })
    }
}