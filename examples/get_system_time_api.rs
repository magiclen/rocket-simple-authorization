#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::request::Request;

use rocket_simple_authorization::{authorizer, SimpleAuthorization};

use chrono::prelude::*;
use short_crypt::ShortCrypt;

const KEY: &str = "magickey";

// 1. Implement any struct you want for authorization.
pub struct Auth {
    auth_data: String,
}

impl Auth {
    pub fn as_str(&self) -> &str {
        self.auth_data.as_str()
    }
}

// 2. Implement `SimpleAuthorization` for the auth struct.
#[async_trait]
impl<'r> SimpleAuthorization<'r> for Auth {
    async fn authorizing(request: &'r Request<'_>, authorization: Option<&'r str>) -> Option<Self> {
        let sc = request.rocket().state::<ShortCrypt>().unwrap();

        match authorization {
            Some(authorization) => {
                match sc.decrypt_url_component(authorization) {
                    Ok(result) => {
                        match String::from_utf8(result) {
                            Ok(user_name) => {
                                Some(Auth {
                                    auth_data: user_name,
                                })
                            }
                            Err(_) => None,
                        }
                    }
                    Err(_) => None,
                }
            }
            None => None,
        }
    }
}

// 3. Make the auth struct be an authorizer(cacheable).
authorizer!(ref Auth);

// 4. Use the auth struct as a request guard.
#[get("/time")]
fn system_time(auth: &Auth) -> Option<String> {
    // 5. Handle the auth struct.
    match auth.as_str() {
        "magiclen.org" => (),
        _ => return None,
    }

    let utc: DateTime<Utc> = Utc::now();

    Some(utc.format("%Y-%m-%d-%H-%M-%S").to_string())
}

#[get("/time", rank = 2)]
fn system_time_401() -> Status {
    Status::Unauthorized
}

#[launch]
fn rocket() -> _ {
    rocket::build().manage(ShortCrypt::new(KEY)).mount("/", routes![system_time, system_time_401])
}
