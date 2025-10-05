import { AfterContentInit, ChangeDetectorRef, Component } from '@angular/core';
import { PostService } from '../../services/post.service';
import moment from 'moment';

@Component({
    selector: 'app-blog',
    standalone: true,
    imports: [],
    templateUrl: './blog.page.component.html',
    styleUrl: './blog.page.component.scss'
})
export class BlogPageComponent implements AfterContentInit {
    get posts() {
        return this.postService.posts ?? [];
    }

    constructor(private postService: PostService, private changeDetectorRef: ChangeDetectorRef) { }

    ngAfterContentInit() {
        this.updatePosts();
    }

    updatePosts() {
        this.postService.getPosts().then((_) => {
            console.info("Updated posts");
            // Note that we do not care about the posts here, as we access them directly through the getter.
            // We only want to trigger change detection when posts have finished loading.
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
