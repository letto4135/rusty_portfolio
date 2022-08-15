use rocket_dyn_templates::{context, Template};

#[get("/about")]
pub fn about_page() -> Template {
    Template::render(
        "about",
        context! {
            title: "About Me",
            about: "
                My name is Alex, I have my Associate of Applied Science in Computer Science,
                and Associate of Applied Science in Computer & Information Technology.
                I am pursuing my bachelors degree in software engineering and have 2 semesters left.
                I currently work for realtor.com as a Software Engineer. I have worked as a full stack
                intern for my college where I created the student data warehouse used by the school.
                I also worked as a CIT intern for one year where I assisted in managing Active Directory,
                and several databases for a company of over 400 employees.
                I have an extreme passion for technology and I am always looking to learn new things which
                is part of why this site is written in Rust/Rocket instead of JS/React.
            ",
            pics: vec![
                "/static/personal_pics/comp_sci.jpeg",
                "/static/personal_pics/comp_info_tech.jpeg",
            ],
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
