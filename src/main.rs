#[macro_use]
extern crate rocket;

mod about;
mod err_handling;
mod home;
mod models;
mod projects;

use about::about_page;
use home::index_page;
use projects::*;
use rocket::fs::FileServer;
use rocket::response::Redirect;
use rocket_dyn_templates::Template;

#[get("/favicon.ico")]
fn favicon() -> Redirect {
    Redirect::to(uri!(""))
}

#[launch]
fn rocket() -> _ {
    let figment = rocket::Config::figment()
        .merge(("port", 8000))
        .merge(("address", "0.0.0.0"));

    rocket::custom(figment)
        .attach(Template::fairing())
        .register("/", catchers![err_handling::not_found])
        .mount("/static", FileServer::from("static"))
        .mount("/", routes![index_page, about_page, projects_page, favicon])
        .mount(
            "/project",
            routes![
                cairo_outdoors,
                warehouse,
                ls_church,
                rippling_waters_camp,
                freecell,
                timesheet,
                pokedex,
            ],
        )
}
