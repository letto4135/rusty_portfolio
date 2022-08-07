use rocket_dyn_templates::{context, Template};

#[get("/about")]
pub fn about_page() -> Template {
    Template::render(
        "index",
        context! {
            title: "Mister-Life",
            message: "Built with Rusts Rocket framework!"
        },
    )
}
