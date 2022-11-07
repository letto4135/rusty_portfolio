use rocket::Request;
use rocket_dyn_templates::{context, Template};

#[catch(404)]
pub fn not_found(req: &Request) -> Template {
    Template::render(
        "404",
        context! {title: "404", message: format!("Sorry, {} is not a valid path.", req.uri())},
    )
}
