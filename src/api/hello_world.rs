use rocket::{fairing::AdHoc, get, routes};

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Hello World Routes", |rocket| async {
        rocket.mount("/hello-world", routes![hello_world])
    })
}

#[get("/")]
pub async fn hello_world() -> &'static str {
    "Hello, World"
}
