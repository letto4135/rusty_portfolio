use rocket_dyn_templates::{context, Template};

#[get("/")]
pub fn index_page() -> Template {
    Template::render(
        "index",
        context! {
            title: "Mister-Life",
            message: "WIP: New site who dis. Built with Rusts Rocket framework!"
        },
    )
}
