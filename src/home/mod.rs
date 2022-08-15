use rocket_dyn_templates::{context, Template};

use crate::projects::projects_vec;

#[get("/")]
pub fn index_page() -> Template {
    Template::render(
        "index",
        context! {
            title: "Mister-Life",
            message: "New site who dis.",
            projects: projects_vec(),
        },
    )
}
