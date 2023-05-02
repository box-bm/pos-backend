extern crate diesel;
use crate::{
    schema::users,
    utils::passwords::{hash_password, verify_password},
};
use chrono::NaiveDateTime;
use diesel::{
    associations::HasTable, ExpressionMethods, Insertable, PgConnection, QueryDsl, QueryResult,
    Queryable, RunQueryDsl,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub password: String,
    #[serde(rename = "createdAt")]
    pub created_at: NaiveDateTime,
    #[serde(rename = "updatedAt")]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct RegisterUserHandler {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserSchema {
    pub email: String,
    pub password: String,
}

impl User {
    pub fn login<'a>(
        conn: &mut PgConnection,
        user_credentials: &LoginUserSchema,
    ) -> Result<User, String> {
        use self::users::dsl::*;

        match users::table().get_result::<User>(conn) {
            Ok(user_data) => {
                match verify_password(&user_credentials.password, &user_data.password) {
                    Ok(_) => Ok(user_data),
                    Err(err) => Err(err.to_string()),
                }
            }
            Err(err) => Err(err.to_string()),
        }
    }

    pub fn find_email_exist<'a>(
        conn: &mut PgConnection,
        user_email: &'a str,
    ) -> Result<User, diesel::result::Error> {
        use self::users::dsl::*;

        users.filter(email.eq(&user_email)).first(conn)
    }

    pub fn register_new_user<'a>(
        conn: &mut PgConnection,
        user: &RegisterUserHandler,
    ) -> Result<QueryResult<usize>, String> {
        let user_exists = Self::find_email_exist(conn, &user.email);

        let hashed_password = match hash_password(&user.password) {
            Ok(result) => result,
            Err(_err) => "".to_string(),
        };

        if user_exists.ok().is_none() && !hashed_password.is_empty() {
            let new = NewUser {
                name: &user.name,
                email: &user.email,
                password: &hashed_password,
            };

            return Ok(diesel::insert_into(users::table).values(&new).execute(conn));
        }
        Err("Email already taken".to_string())
    }
}
