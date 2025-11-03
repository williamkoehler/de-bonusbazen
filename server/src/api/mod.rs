use axum::{Json, http::StatusCode};

pub mod auth;
pub mod middleware;

pub mod ah;
pub mod post;
pub mod user;

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorReason {
    // Authentication/authorization errors
    #[cfg(not(debug_assertions))]
    InvalidReCaptcha,

    Unverified,
    Unauthenticated,
    Unauthorized,
    
    InvalidName,
    NameIsTaken,
    InvalidNickname,
    InvalidEmail,
    EmailIsTaken,

    // Internal errors
    ReCaptchaVerificationFailed,
    JwtGenerationFailed,
    InternalError,
}

impl Into<(StatusCode, Json<ErrorBody>)> for ErrorReason {
    fn into(self) -> (StatusCode, Json<ErrorBody>) {
        let status_code = match self {
            #[cfg(not(debug_assertions))]
            ErrorReason::InvalidReCaptcha => StatusCode::IM_A_TEAPOT,
            ErrorReason::Unauthenticated | ErrorReason::Unverified => {
                StatusCode::UNAUTHORIZED
            }

            ErrorReason::Unauthorized => StatusCode::FORBIDDEN,

            ErrorReason::InvalidName
            | ErrorReason::NameIsTaken
            | ErrorReason::InvalidNickname
            | ErrorReason::InvalidEmail
            | ErrorReason::EmailIsTaken => StatusCode::BAD_REQUEST,

            // Internal errors
            ErrorReason::ReCaptchaVerificationFailed
            | ErrorReason::JwtGenerationFailed
            | ErrorReason::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status_code, Json(ErrorBody { reason: self }))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, serde::Serialize)]
pub struct ErrorBody {
    reason: ErrorReason,
}
