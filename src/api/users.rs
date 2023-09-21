use super::{Api, Error};
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub data: UserData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserData {
    pub email: String,
    pub username: String,
}

pub struct UsersApi<'a>(pub &'a Api);

impl<'a> UsersApi<'a> {
    pub async fn me(&'a self) -> Result<Option<User>, Error> {
        let UsersApi(api) = self;
        api.execute(Method::GET, "/users/me", None).await
    }
}
