import { AfterContentInit, Component } from '@angular/core';
import { UserService } from '../../services/user.service';

@Component({
    selector: 'app-home',
    standalone: true,
    imports: [],
    templateUrl: './home.page.component.html',
    styleUrl: './home.page.component.scss'
})
export class HomePageComponent implements AfterContentInit {
    constructor(private userService: UserService) { }

    ngAfterContentInit() {
        this.userService.getUsers().then(users => {
            console.log(users);
        })
    }
}
