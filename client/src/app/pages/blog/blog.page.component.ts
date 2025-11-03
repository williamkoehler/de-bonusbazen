import { AfterContentInit, ChangeDetectorRef, Component } from '@angular/core';
import { Router } from '@angular/router';
import { PostService } from '../../services/post.service';
import { Post } from '../../services/models/post';
import moment from 'moment';
import { AccountService } from '../../services/account.service';

@Component({
    selector: 'app-blog',
    imports: [],
    templateUrl: './blog.page.component.html',
    styleUrl: './blog.page.component.scss'
})
export class BlogPageComponent implements AfterContentInit {
    get posts() {
        return this.postService.posts ?? [];
    }

    constructor(
        private router: Router,
        protected accountService: AccountService,
        private postService: PostService,
        private changeDetectorRef: ChangeDetectorRef) { }

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

    onEditPostClick(post?: Post) {
        if (post)
            this.router.navigate(['/blog/edit', { id: post.id }]);
        else
            this.router.navigate(['/blog/edit']);
    }

    formatDate(date: Date | undefined): string {
        if (date)
            return moment(date).format('MMMM Do YYYY, h:mm:ss a');
        else
            return 'No Date Specified';
    }
}
