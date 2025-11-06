import { EventEmitter, Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Product, RawProduct } from './models/ah';

@Injectable({
    providedIn: 'root'
})
export class AhService {
    _productsMostBonus: Product[] = [];
    _lastPage: number = 0;
    _lastGetProductsMostBonus: number | undefined;
    _productsMostBonusPromise?: Promise<[Product[], number]>;

    public onProductsChanged: EventEmitter<Product[]> = new EventEmitter();

    public get productsMostBonus(): Product[] | undefined {
        return this._productsMostBonus;
    }

    constructor(private httpClient: HttpClient) { }

    public reset() {
        this._productsMostBonus = [];
        this._lastPage = 0;
        this._lastGetProductsMostBonus = undefined;
        this._productsMostBonusPromise = undefined;
    }

    public getPosts(lastPage: number = 0): Promise<[Product[], number]> {
        const productsMostBonusPromise = this._productsMostBonusPromise;
        this._productsMostBonusPromise = (async () => {
            if (productsMostBonusPromise)
                await productsMostBonusPromise;

            if (lastPage <= 0)
                lastPage = 1;

            if (this._lastGetProductsMostBonus && (Date.now() - this._lastGetProductsMostBonus) > (60 * 60000)) // 60 minute cache
                this.reset();

            const firstPage = this._lastPage;

            const tasks = [];
            for (let i = firstPage; i < lastPage; i++) {
                const page = i;
                tasks.push(new Promise<Product[]>((resolve, _reject) => {
                    this.httpClient.get<RawProduct[]>(`/api/ah/products/most_bonus?page=${page}`).subscribe(rawPosts => {
                        resolve(rawPosts.map(rawPost => Product.fromRaw(rawPost)));
                    });
                }));
            }

            for (const products of await Promise.all<Product[]>(tasks)) {
                this._productsMostBonus.push(...products);
            }

            this._lastPage = lastPage;
            this._lastGetProductsMostBonus = Date.now();
            this._productsMostBonusPromise = undefined;

            this.onProductsChanged.emit(this._productsMostBonus);
            return [this._productsMostBonus, lastPage];
        })();

        return this._productsMostBonusPromise;
    }
}
