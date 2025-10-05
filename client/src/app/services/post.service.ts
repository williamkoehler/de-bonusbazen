import { EventEmitter, Injectable } from '@angular/core';
import { Post, RawPost } from './models/post';
import { HttpClient } from '@angular/common/http';

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

    constructor(private httpClient: HttpClient) { }

    public getPosts(): Promise<Post[]> {
        if (!this._lastGetPosts || (Date.now() - this._lastGetPosts) > (5*60000)) { // 5 minute cache
            this._posts = [];
            this._postsIds = new Map();

            if (!this._postsPromise) {
                this._postsPromise = new Promise((resolve, _reject) => {
                    this.httpClient.get<RawPost[]>('/api/posts').subscribe(rawPosts => {
                        for (const rawPost of rawPosts) {
                            const post = Post.fromRaw(rawPost);

                            const index = this._postsIds.get(post.id);
                            if (index)
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

    public async postPost(post: Post) {
        return new Promise((resolve, _reject) => {
            this.httpClient.post<RawPost>('/api/posts', post).subscribe(rawPost => {
                const post = Post.fromRaw(rawPost);

                const index = this._postsIds.get(post.id);
                if (index)
                    this._posts[index] = post;
                else {
                    this._postsIds.set(post.id, this._posts.length);
                    this._posts.push(post);
                }

                resolve(post);
            });
        });
    }
}
