import { AfterContentInit, ChangeDetectorRef, Component, ElementRef, ViewChild } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ActivatedRoute, Router } from '@angular/router';
import { PostService } from '../../../services/post.service';
import moment from 'moment';
import { Post } from '../../../services/models/post';
import { SpinnerComponent } from "../../../components/spinner/spinner.component";

@Component({
    selector: 'app-post-preview',
    imports: [
        FormsModule,
        SpinnerComponent
    ],
    templateUrl: './post-preview.page.component.html',
    styleUrl: './post-preview.page.component.scss'
})
export class PostPreviewPageComponent implements AfterContentInit {
    id?: number;
    post?: Post;

    saveLoading: boolean = false;
    deleteLoading: boolean = false;

    get loading() {
        return this.saveLoading || this.deleteLoading;
    }

    constructor(private router: Router, private route: ActivatedRoute, private postService: PostService, private changeDetectorRef: ChangeDetectorRef) {
        const rawId = this.route.snapshot.paramMap.get('id');
        if (rawId) this.id = Number(rawId);
    }

    async ngAfterContentInit() {
        if (this.id) {
            this.post = await this.postService.getPost(this.id);

            this.changeDetectorRef.markForCheck();
        }
        else {
            console.error('No post ID specified, navigating back to blog page.');
            this.router.navigate(['/blog']);
        }
    }

    formatDate(date: Date | undefined): string {
        if (date)
            return moment(date).format('MMMM Do YYYY, h:mm:ss a');
        else
            return 'No Date Specified';
    }
}
