export class InternalServiceError extends Error {
    constructor(message: string) {
        super(message);
    }
}

export class InvalidNameOrPasswordServiceError extends Error {
    constructor() {
        super();
    }
}