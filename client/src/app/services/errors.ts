import { HttpErrorResponse, HttpStatusCode } from "@angular/common/http";

export class InternalServiceError extends Error {
    constructor(message: string) {
        super(message);
    }
}

export class InvalidRecaptchaError extends Error { constructor() { super(); } }

export class UnverifiedError extends Error { constructor() { super(); } }
export class UnauthenticatedError extends Error { constructor() { super(); } }
export class UnauthorizedError extends Error { constructor() { super(); } }

export class InvalidNameError extends Error { constructor() { super(); } }
export class NameIsTakenError extends Error { constructor() { super(); } }
export class InvalidNicknameError extends Error { constructor() { super(); } }
export class InvalidEmailError extends Error { constructor() { super(); } }
export class EmailIsTakenError extends Error { constructor() { super(); } }

export class ReCaptchaVerificationFailedError extends Error { constructor() { super(); } }
export class JwtGenerationFailedError extends Error { constructor() { super(); } }

export function errorFromReason(reason: string): Error {
    switch (reason) {
        case 'invalid_recaptcha': return new InvalidRecaptchaError();
        case 'unverified': return new UnverifiedError();
        case 'unauthenticated': return new UnauthenticatedError();
        case 'unauthorized': return new UnauthorizedError();
        case 'invalid_name': return new InvalidNameError();
        case 'name_is_taken': return new NameIsTakenError();
        case 'invalid_nickname': return new InvalidNicknameError();
        case 'invalid_email': return new InvalidEmailError();
        case 'email_is_taken': return new EmailIsTakenError();
        case 're_captcha_verification_failed': return new ReCaptchaVerificationFailedError();
        case 'jwt_generation_failed': return new JwtGenerationFailedError();
        default: return new InternalServiceError('An internal server error occurred.');
    }
}

export function handleError(err: HttpErrorResponse): Error {
    switch (err.status) {
        case HttpStatusCode.Unauthorized:
        case HttpStatusCode.Forbidden:
        case HttpStatusCode.BadRequest:
        case HttpStatusCode.ImATeapot:
        case HttpStatusCode.InternalServerError:
            return errorFromReason(err.error.reason);
            break;
        default:
            return new InternalServiceError('Internal server error: ' + err.message);
            break;
    }
}