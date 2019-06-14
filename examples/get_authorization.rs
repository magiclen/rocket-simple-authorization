#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_simple_authorization;

use rocket::request::Request;

use rocket_simple_authorization::SimpleAuthorization;

// 1. Implement any struct you want for authorization.
pub struct AuthKey<'a> {
    authorization: Option<&'a str>
}

impl<'a> AuthKey<'a> {
    pub fn as_str(&self) -> Option<&'a str> {
        self.authorization.clone()
    }
}

// 2. Implement `SimpleAuthorization<E>` for the auth struct. The default `<E>` is `<&'a str>`.
impl<'a, 'r> SimpleAuthorization<'a, 'r> for AuthKey<'a> {
    fn has_authority(_request: &'a Request<'r>, key: Option<&'a str>) -> Option<Option<&'a str>> {
        Some(key)
    }

    fn create_auth(authorization: Option<&'a str>) -> AuthKey {
        AuthKey {
            authorization
        }
    }
}

// 3. Make the auth struct be an authorizer.
authorizer!(AuthKey<'a>);

// 4. Use the auth struct as a request guard.
#[get("/")]
fn authorization(auth_key: AuthKey) -> &str {
    // 5. Handle the auth struct.
    auth_key.as_str().unwrap_or("")
}

fn main() {
    rocket::ignite().mount("/", routes![authorization]).launch();
}