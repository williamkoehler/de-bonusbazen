import { EventEmitter, Injectable } from '@angular/core';
import { Post, RawPost } from './models/post';
import { HttpClient, HttpErrorResponse } from '@angular/common/http';
import { handleError } from './errors';
import { AccountService } from './account.service';

@Injectable({
    providedIn: 'root'
})
export class PostService {
    _posts: Post[] = [];
    _postsIds: Map<number, number> = new Map();
    _lastGetPosts?: number;
    _postsPromise?: Promise<Post[]>;

    public onPostsChanged: EventEmitter<Post[]> = new EventEmitter();
    public onPostChanged: EventEmitter<Post> = new EventEmitter();

    public get posts(): Post[] | undefined {
        return this._posts;
    }

    constructor(private httpClient: HttpClient, private accountService: AccountService) { }

    public getPosts(): Promise<Post[]> {
        if (!this._lastGetPosts || (Date.now() - this._lastGetPosts) > (5 * 60000)) { // 5 minute cache
            this._posts = [];
            this._postsIds = new Map();

            if (!this._postsPromise) {
                this._postsPromise = new Promise((resolve, _reject) => {
                    this.httpClient.get<RawPost[]>('/api/posts',
                        { headers: this.accountService.httpHeaders }).subscribe(rawPosts => {
                            for (const rawPost of rawPosts) {
                                const post = Post.fromRaw(rawPost);

                                const index = this._postsIds.get(post.id);
                                if (index !== undefined)
                                    this._posts[index] = post;
                                else {
                                    this._postsIds.set(post.id, this._posts.length);
                                    this._posts.push(post);
                                }
                            }

                            this._lastGetPosts = Date.now();
                            this._postsPromise = undefined;

                            resolve(this._posts);
                            this.onPostsChanged.emit(this._posts);
                        });
                });
            }

            return this._postsPromise;
        }
        else
            return Promise.resolve(this._posts);
    }

    public getPost(id: number): Promise<Post> {
        return new Promise((resolve, reject) => {
            this.httpClient.get<RawPost>(`/api/posts/${id}`,
                { headers: this.accountService.httpHeaders }).subscribe({
                    next: rawPost => {
                        const post = Post.fromRaw(rawPost);

                        const index = this._postsIds.get(post.id);
                        if (index !== undefined)
                            this._posts[index] = post;
                        else {
                            this._postsIds.set(post.id, this._posts.length);
                            this._posts.push(post);
                        }

                        resolve(post);
                        this.onPostsChanged.emit(this._posts);
                    },
                    error: (err: HttpErrorResponse) => {
                        reject(handleError(err));
                    }
                });
        });
    }

    public async postPost(post2: Post): Promise<Post> {
        return new Promise((resolve, reject) => {
            this.httpClient.post<RawPost>('/api/posts', post2.toRaw(),
                { headers: this.accountService.httpHeaders }).subscribe({
                    next: rawPost => {
                        const post = Post.fromRaw(rawPost);

                        // Update body as it is not sent back from the server
                        post.body = post2.body;

                        const index = this._postsIds.get(post.id);
                        if (index !== undefined)
                            this._posts[index] = post;
                        else {
                            this._postsIds.set(post.id, this._posts.length);
                            this._posts.push(post);
                        }

                        resolve(post);
                        this.onPostsChanged.emit(this._posts);
                    },
                    error: (err: HttpErrorResponse) => {
                        reject(handleError(err));
                    }
                });
        });
    }

    public async updatePost(post: Post, includeVisibility: boolean = false, includeTitle: boolean = false, includeBody: boolean = false): Promise<void> {
        return new Promise((resolve, reject) => {
            this.httpClient.patch(`/api/posts/${post.id}`, post.toRaw(includeVisibility, includeTitle, includeBody),
                { headers: this.accountService.httpHeaders }).subscribe({
                    next: _ => {
                        resolve();
                    },
                    error: (err: HttpErrorResponse) => {
                        reject(handleError(err));
                    }
                });
        });
    }

    public async deletePost(post: Post): Promise<void> {
        return new Promise((resolve, reject) => {
            this.httpClient.delete(`/api/posts/${post.id}`,
                { headers: this.accountService.httpHeaders }).subscribe({
                    next: _ => {
                        this._lastGetPosts = undefined;
                        resolve();
                    },
                    error: (err: HttpErrorResponse) => {
                        reject(handleError(err));
                    }
                });
        });
    }
}
