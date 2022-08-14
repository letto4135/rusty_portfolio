use rocket_dyn_templates::{context, Template};

#[get("/about")]
pub fn about_page() -> Template {
    Template::render(
        "about",
        context! {
            title: "About Me",
            about: "
                My name is Alex, I am pursuing my bachelors degree in software engineering.
                I have already completed my Associate of Applied Science in Computer Science,
                and my Associate of Applied Science in Computer & Information Technology.
                I worked in a CS learn and earn through my school where, over the course of
                eight months, I developed a Student Data Warehouse application.
                I also worked in a CIT learn and earn for one year where I assist in managing
                Active Directory, and several databases for a company of over 400 employees.
                I am currently working for realtor.com as a Software Engineer.
            ",
            languages: vec![
                "JavaScript (Intermediate - Advanced)",
                "Python (Intermediate - Advanced)",
                "Rust (Intermediate)",
                "C# (Intermediate)",
                "Java (Beginner - Intermediate)",
                "HTML/CSS (Intermediate)",
                "English (Native)",
                "Norwegian (Beginner)"
            ],
            frameworks: vec![
                "React", "Node/Express", "Django", ".net Core", "Actix", "Rocket"
            ],
            interests: vec! [
                "Programming", "Rock Climbing", "Traveling", "Learning"
            ],
        },
    )
}
