pub mod api;
pub mod database;

use database::Db;
use rocket::launch;
use rocket_db_pools::Database;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .attach(crate::api::hello_world::stage())
}
