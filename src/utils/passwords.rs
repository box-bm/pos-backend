use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, Error, Params, PasswordHash, PasswordHasher, PasswordVerifier,
};

use crate::config::env::get_enviroment_variable;

fn get_argon<'a>(secret: &'a str) -> Result<Argon2<'a>, Error> {
    let params = Params::default();

    Argon2::<'a>::new_with_secret(
        secret.as_bytes(),
        argon2::Algorithm::Argon2i,
        argon2::Version::V0x10,
        params,
    )
}

pub fn hash_password<'a>(password: &'a str) -> Result<String, String> {
    let secret = get_enviroment_variable("PASSWORD_HASHER".to_string());
    let salt = SaltString::generate(&mut OsRng);

    let argon = get_argon(&secret).unwrap();

    match argon.hash_password(password.as_bytes(), &salt) {
        Ok(hash) => Ok(hash.to_string()),
        Err(err) => Err(err.to_string()),
    }
}

pub fn verify_password<'a>(password: &'a str, db_password: &'a str) -> Result<bool, String> {
    let secret = get_enviroment_variable("PASSWORD_HASHER".to_string());
    let argon = get_argon(&secret).unwrap();

    let parsed_hash = PasswordHash::new(db_password).unwrap();

    match argon.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(true),
        Err(err) => Err(err.to_string()),
    }
}
