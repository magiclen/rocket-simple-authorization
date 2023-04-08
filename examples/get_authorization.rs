#[macro_use]
extern crate rocket;

use rocket::request::Request;
use rocket_simple_authorization::{authorizer, SimpleAuthorization};

// 1. Implement any struct you want for authorization.
pub struct AuthKey<'a> {
    authorization: Option<&'a str>,
}

impl<'a> AuthKey<'a> {
    pub fn as_str(&self) -> Option<&'a str> {
        self.authorization
    }
}

// 2. Implement `SimpleAuthorization` for the auth struct.
#[async_trait]
impl<'r> SimpleAuthorization<'r> for AuthKey<'r> {
    async fn authorizing(
        _request: &'r Request<'_>,
        authorization: Option<&'r str>,
    ) -> Option<Self> {
        Some(AuthKey {
            authorization,
        })
    }
}

// 3. Make the auth struct be an authorizer.
authorizer!(AuthKey<'r>);

// 4. Use the auth struct as a request guard.
#[get("/")]
fn authorization(auth_key: AuthKey) -> &str {
    // 5. Handle the auth struct.
    auth_key.as_str().unwrap_or("")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![authorization])
}
