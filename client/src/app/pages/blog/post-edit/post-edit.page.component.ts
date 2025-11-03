import { AfterContentInit, ChangeDetectorRef, Component, ElementRef, ViewChild } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ActivatedRoute, Router } from '@angular/router';
import { PostService } from '../../../services/post.service';
import moment from 'moment';
import { Post } from '../../../services/models/post';
import { SpinnerComponent } from "../../../components/spinner/spinner.component";

@Component({
    selector: 'app-post-edit',
    imports: [
        FormsModule,
        SpinnerComponent
    ],
    templateUrl: './post-edit.page.component.html',
    styleUrl: './post-edit.page.component.scss'
})
export class PostEditPageComponent implements AfterContentInit {
    id?: number;
    post?: Post;

    visiblity: 'hidden' | 'draft' | 'visible' = 'hidden';
    updatedVisiblity: boolean = false;

    title: string = '';
    updatedTitle: boolean = false;

    body: string = '';
    updatedBody: boolean = false;

    get updated() {
        return this.updatedVisiblity || this.updatedTitle || this.updatedBody;
    }

    get valid() {
        return this.title.trim().length > 0 && this.body.length > 0;
    }

    @ViewChild('bodyInput') bodyInput!: ElementRef<HTMLTextAreaElement>;

    saveLoading: boolean = false;
    deleteLoading: boolean = false;

    get loading() {
        return this.saveLoading || this.deleteLoading;
    }

    constructor(private router: Router, private route: ActivatedRoute, private postService: PostService, private changeDetectorRef: ChangeDetectorRef) {
        this.id = Number(this.route.snapshot.paramMap.get('id'));
    }

    async ngAfterContentInit() {
        if (this.id)
            this.post = await this.postService.getPost(this.id);
        else
            this.post = new Post('', '');

        this.visiblity = this.post.visibility;
        this.title = this.post.title;
        this.body = this.post.body ?? '';

        this.changeDetectorRef.detectChanges();


        setTimeout(() => this.onBodyInput(), 1);
    }

    onVisibilityClick(visibility: 'hidden' | 'draft' | 'visible') {
        this.visiblity = visibility;
        this.updatedVisiblity = (this.post?.visibility !== this.visiblity);
    }

    onBodyInput() {
        this.bodyInput.nativeElement.style.height = 'auto';
        this.bodyInput.nativeElement.style.height = Math.min(this.bodyInput.nativeElement.scrollHeight, 500) + 'px';
    }

    async onSaveClick() {
        try {
            this.saveLoading = true;
            this.changeDetectorRef.markForCheck();

            if (!this.post)
                return;

            if (this.updatedVisiblity) {
                this.post.visibility = this.visiblity;
            }

            if (this.updatedTitle) {
                this.post.title = this.title;
            }

            if (this.updatedBody) {
                this.post.body = this.body;
            }

            try {
                if (this.id)
                    await this.postService.updatePost(this.post, this.updatedVisiblity, this.updatedTitle, this.updatedBody);
                else {
                    this.post = await this.postService.postPost(this.post);
                    this.router.navigate(['/blog']);
                }

                this.updatedVisiblity = false;
                this.updatedTitle = false;
                this.updatedBody = false;
            }
            catch (err) {
                console.error('Failed to save/create post: ', err);
            }
        }
        finally {
            this.saveLoading = false;
            this.changeDetectorRef.markForCheck();
        }
    }

    async onDeleteClick() {
        try {
            this.deleteLoading = true;
            this.changeDetectorRef.markForCheck();

            if (!this.post)
                return;

            try {
                await this.postService.deletePost(this.post);

                this.router.navigate(['/blog']);
            }
            catch (err) {
                console.error('Failed to save/create post: ', err);
            }
        }
        finally {
            this.deleteLoading = false;
            this.changeDetectorRef.markForCheck();
        }
    }

    formatDate(date: Date | undefined): string {
        if (date)
            return moment(date).format('MMMM Do YYYY, h:mm:ss a');
        else
            return 'No Date Specified';
    }
}
