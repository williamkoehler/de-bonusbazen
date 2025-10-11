import { ChangeDetectorRef, Component } from '@angular/core';
import { AhService } from '../../../services/ah.service';
import moment from 'moment';

@Component({
    selector: 'app-ah-most-bonus-page',
    imports: [],
    templateUrl: './ah.most-bonus.page.component.html',
    styleUrl: './ah.most-bonus.page.component.scss'
})
export class AhMostBonusPageComponent {
    get products() {
        return this.ahService.productsMostBonus ?? [];
    }

    constructor(private ahService: AhService, private changeDetectorRef: ChangeDetectorRef) { }

    ngAfterContentInit() {
        this.updatePosts();
    }

    updatePosts() {
        this.ahService.getPosts().then((_) => {
            console.info("Updated products");
            // Note that we do not care about the products here, as we access them directly through the getter.
            // We only want to trigger change detection when products have finished loading.
            this.changeDetectorRef.detectChanges();
        });
    }

    formatDate(date: Date | undefined): string {
        if (date)
            return moment(date).format('MMMM Do YYYY, h:mm:ss a');
        else
            return 'No Date Specified';
    }
}
