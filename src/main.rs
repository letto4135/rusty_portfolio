#[macro_use]
extern crate rocket;

use rocket_dyn_templates::Template;

mod about;
mod home;

use about::about_page;
use home::index_page;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index_page, about_page])
}
