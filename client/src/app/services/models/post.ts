export class Post {
    id: number;
    createdAt: Date | undefined;
    updatedAt: Date | undefined;
    author: number | undefined;
    title: string
    body: string | undefined;

    // Metadata
    hasDetails: boolean = false;

    constructor(title: string, body?: string) {
        this.id = -1;
        this.title = title;
        this.body = body;
    }

    static fromRaw(rawUser: RawPost) {
        if (!rawUser.id)
            throw new Error("Post id is required");

        const post = new Post(rawUser.title ?? "[untitled]", rawUser.body);
        
        post.id = rawUser.id;
        post.createdAt = rawUser.created_at ? new Date(rawUser.created_at) : undefined;
        post.updatedAt = rawUser.updated_at ? new Date(rawUser.updated_at) : undefined;
        post.author = rawUser.author;

        return post;
    }
}

export interface RawPost {
    id?: number;
    created_at?: string;
    updated_at?: string;
    author?: number;
    title?: string;
    body?: string;
}