use argon2::Argon2;
use serde::{Deserialize, Serialize};

pub fn generate_hash(password: &str) -> String {
    let argon2 = Argon2::default();

    let salt =
        argon2::password_hash::SaltString::generate(&mut argon2::password_hash::rand_core::OsRng);
    argon2::PasswordHash::generate(argon2, password, salt.as_salt())
        .unwrap()
        .to_string()
}

pub fn verify_hash(hash: &str, password: &str) -> bool {
    let hash = argon2::PasswordHash::new(hash).unwrap();

    let argon2 = Argon2::default();
    hash.verify_password(&[&argon2], password).is_ok()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub id: i32,
    pub rights: crate::users::model::Rights,
    pub exp: usize,
}

pub fn generate_jwt(
    user: &super::model::User,
    expiry_time: u64,
    secret: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();

    let exp = now + expiry_time;

    let claims = JwtClaims {
        id: user.id(),
        rights: user.rights(),
        exp: exp as usize,
    };

    Ok(jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(secret.as_ref()),
    )?)
}

pub fn verify_jwt(token: &str, secret: &str) -> Result<JwtClaims, Box<dyn std::error::Error>> {
    Ok(jsonwebtoken::decode::<JwtClaims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()),
        &jsonwebtoken::Validation::default(),
    )?
    .claims)
}
