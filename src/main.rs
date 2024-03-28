pub mod api;

use rocket::launch;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(crate::api::hello_world::stage())
}
