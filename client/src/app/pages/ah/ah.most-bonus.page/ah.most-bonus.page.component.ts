import { ChangeDetectorRef, Component, HostListener } from '@angular/core';
import { AhService } from '../../../services/ah.service';
import { Product } from '../../../services/models/ah';
import moment from 'moment';

@Component({
    selector: 'app-ah-most-bonus-page',
    imports: [],
    templateUrl: './ah.most-bonus.page.component.html',
    styleUrl: './ah.most-bonus.page.component.scss'
})
export class AhMostBonusPageComponent {
    products: Product[] = [];
    lastPage = 0;

    constructor(private ahService: AhService, private changeDetectorRef: ChangeDetectorRef) { }

    ngAfterContentInit() {
        this.getNextPosts();
    }

    getNextPosts() {
        this.ahService.getPosts(this.lastPage + 1).then(([products, lastPage]) => {
            console.info("Successfully loaded next products");
            this.products = products;
            this.lastPage = lastPage;

            this.changeDetectorRef.markForCheck();
        });
    }

    @HostListener('window:scroll', [])
    scrollHandler() {
        const d = document.documentElement;
        if ((d.scrollTop + d.clientHeight) > (d.scrollHeight - d.clientHeight))
            this.getNextPosts();
    }

    formatDate(date: Date | undefined): string {
        if (date)
            return moment(date).format('MMMM Do YYYY, h:mm:ss a');
        else
            return 'No Date Specified';
    }
}
