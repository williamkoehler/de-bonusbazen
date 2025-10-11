export class Product {
    id: number;
    name: string
    image: string | undefined;
    price: number | undefined;
    price_before_bonus: number | undefined;

    constructor(id: number, name: string, image?: string, price?: number, price_before_bonus?: number) {
        this.id = id;
        this.name = name;
        this.image = image;
        this.price = price;
        this.price_before_bonus = price_before_bonus;
    }

    static fromRaw(rawProduct: RawProduct) {
        if (!rawProduct.id)
            throw new Error("AH Product id is required");

        return new Product(
            rawProduct.id,
            rawProduct.name ?? "[unnamed]",
            rawProduct.image,
            rawProduct.price,
            rawProduct.price_before_bonus
        );
    }
}

export interface RawProduct {
    id?: number;
    name?: string;
    image?: string;
    price?: number;
    price_before_bonus?: number;
}