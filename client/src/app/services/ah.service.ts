import { EventEmitter, Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Product, RawProduct } from './models/ah';

@Injectable({
    providedIn: 'root'
})
export class AhService {
    _productsMostBonus: Product[] = [];
    _lastGetProductsMostBonus: number | undefined;
    _productsMostBonusPromise?: Promise<Product[]>;

    public onProductsChanged: EventEmitter<Product[]> = new EventEmitter();

    public get productsMostBonus(): Product[] | undefined {
        return this._productsMostBonus;
    }

    constructor(private httpClient: HttpClient) { }

    public getPosts(): Promise<Product[]> {
        if (!this._lastGetProductsMostBonus || (Date.now() - this._lastGetProductsMostBonus) > (5 * 60000)) { // 5 minute cache
            this._productsMostBonus = [];

            if (!this._productsMostBonusPromise) {
                this._productsMostBonusPromise = new Promise((resolve, _reject) => {
                    this.httpClient.get<RawProduct[]>('/api/ah/products/most_bonus').subscribe(rawPosts => {
                        for (const rawPost of rawPosts) {
                            const post = Product.fromRaw(rawPost);
                            this._productsMostBonus.push(post);
                        }

                        this._lastGetProductsMostBonus = Date.now();
                        this._productsMostBonusPromise = undefined;

                        resolve(this._productsMostBonus);
                        this.onProductsChanged.emit(this._productsMostBonus);
                    });
                });
            }

            return this._productsMostBonusPromise;
        }
        else
            return Promise.resolve(this._productsMostBonus);
    }
}
