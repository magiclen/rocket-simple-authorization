/// To let a struct which implements the `SimpleAuthorization` trait become an authorizer.
#[macro_export]
macro_rules! authorizer {
    ($name:ty) => {
        #[$crate::rocket::async_trait]
        impl<'r> $crate::rocket::request::FromRequest<'r> for $name {
            type Error = ();

            async fn from_request(
                request: &'r $crate::rocket::request::Request<'_>,
            ) -> $crate::rocket::request::Outcome<Self, Self::Error> {
                let key: Option<&str> = request.headers().get("authorization").next();

                match <$name as $crate::SimpleAuthorization>::authorizing(request, key).await {
                    Some(ins) => $crate::rocket::outcome::Outcome::Success(ins),
                    None => $crate::rocket::outcome::Outcome::Forward(
                        $crate::rocket::http::Status::Unauthorized,
                    ),
                }
            }
        }
    };
    (ref $name:ty) => {
        $crate::authorizer!($name);

        #[$crate::rocket::async_trait]
        impl<'r> $crate::rocket::request::FromRequest<'r> for &'r $name {
            type Error = ();

            async fn from_request(
                request: &'r $crate::rocket::request::Request<'_>,
            ) -> $crate::rocket::request::Outcome<Self, Self::Error> {
                async fn f(request: &$crate::rocket::request::Request<'_>) -> Option<$name> {
                    let key: Option<&str> = request.headers().get("authorization").next();

                    match <$name as $crate::SimpleAuthorization>::authorizing(request, key).await {
                        Some(ins) => Some(ins),
                        None => None,
                    }
                }

                let cache = request.local_cache_async(f(request)).await;

                match cache.as_ref() {
                    Some(cache) => $crate::rocket::outcome::Outcome::Success(cache),
                    None => $crate::rocket::outcome::Outcome::Forward(
                        $crate::rocket::http::Status::Unauthorized,
                    ),
                }
            }
        }
    };
}
