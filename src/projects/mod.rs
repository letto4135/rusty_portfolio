use rocket_dyn_templates::{context, Template};
use serde::{Deserialize, Serialize};

use crate::models::project::Project;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectShort {
    pub title: String,
    pub link: String,
    pub short_desc: String,
}

impl ProjectShort {
    pub fn new(title: String, link: String, short_desc: String) -> ProjectShort {
        ProjectShort {
            title,
            link,
            short_desc,
        }
    }
}

#[get("/projects")]
pub fn projects_page() -> Template {
    Template::render(
        "projects",
        context! {
            title: "My Projects",
            projects: vec![
            ProjectShort::new(
                "Liberty Street Church".to_string(),
                "/project/ls_church".to_string(),
                "Church site that I built in Python/Django".to_string(),
            ),
            ProjectShort::new(
                "Cairo Outdoors".to_string(),
                "/project/cairo_outdoors".to_string(),
                "A small site for a local business that I frequent.".to_string(),
            ),
            ProjectShort::new(
                "Rippling Waters Camp".to_string(),
                "/project/rippling_waters_camp".to_string(),
                "Built for a campground in WV.".to_string(),
            ),
            ProjectShort::new(
                "Student Data Warehouse".to_string(),
                "/project/student_data_warehouse".to_string(),
                "A project completed through an internship at WVUP.".to_string(),
            ),
            ProjectShort::new(
                "FreeCell".to_string(),
                "/project/freecell".to_string(),
                "FreeCell built with Java/GWT.".to_string(),
            ),
            ProjectShort::new(
                "TimeSheet Appliction".to_string(),
                "/project/timesheet".to_string(),
                "TimeSheet built in C# .net core for a CS class.".to_string(),
            )
        ],
        },
    )
}

#[get("/cairo_outdoors")]
pub fn cairo_outdoors() -> Template {
    let base = "/static/cairo_outdoors/".to_owned();
    Template::render("project", Project::new(
        "Cairo Outdoors".to_string(),
        "Cairo Outdoors".to_string(),
        "A website for Cairo Outdoors written as a static website in React, deployed on Firebase"
            .to_string(),
        vec![
            base.clone() + "mainPage.png",
            base.clone() + "climbing.png",
            base.clone() + "biking.png",
        ],
        Some("https://cairooutdoors.com".to_string()),
    ))
}

#[get("/freecell")]
pub fn freecell() -> Template {
    let base = "/static/freecell/".to_owned();
    Template::render(
        "project",
        Project::new(
            "FreeCell".to_string(),
            "FreeCell".to_string(),
            "A Java/GWT application for FreeCell. Written for Data Structures, our
            assignment was to create FreeCell with no additional requirements. I took it upon
            myself to learn GWT in order to do a web version."
                .to_string(),
            vec![
                base.clone() + "InitialGame.PNG",
                base.clone() + "gameInProgress.PNG",
                base.clone() + "gameWon.PNG",
            ],
            Some("https://freecell.mister-life.com".to_string()),
        ),
    )
}

#[get("/ls_church")]
pub fn ls_church() -> Template {
    let base = "/static/ls_church/".to_owned();
    Template::render("project", Project::new(
        "Liberty Street Church".to_string(),
        "Liberty Street Church".to_string(),
        "A website for Liberty Street Church written in Python using Django, deployed on fly.io.
        The pictures are hosted on Cloudinary. All of the websites text is configured via the Django admin."
            .to_string(),
        vec![
            base.clone() + "main.png",
            base.clone() + "bottom.png",
            base.clone() + "emailjs.png",
            base.clone() + "django.png",
        ],
        Some("https://libertystreetchurch.com".to_string()),
    ))
}

#[get("/rippling_waters_camp")]
pub fn rippling_waters_camp() -> Template {
    let base = "/static/rippling_waters/".to_owned();
    Template::render(
        "project",
        Project::new(
            "Rippling Waters Camp".to_string(),
            "Rippling Waters Camp".to_string(),
            "A static website for the Rippling Waters Camp built in React, deployed on Firebase.
            The camps old website was being shut down by the service provider and the person that
            built it no longer wanted to maintain it so I built them a new one."
                .to_string(),
            vec![
                base.clone() + "mainPage.png",
                base.clone() + "contact.png",
                base.clone() + "facilities.png",
                base.clone() + "facilityDetail.png",
            ],
            Some("https://ripplingwaterscamp.com".to_string()),
        ),
    )
}

#[get("/timesheet")]
pub fn timesheet() -> Template {
    let base = "/static/timesheet/".to_owned();
    Template::render(
        "project",
        Project::new(
            "TimeSheet".to_string(),
            "TimeSheet".to_string(),
            "A C# .net core application for TimeSheet. Written for a class based on
            .net core the app has features like adding employees, pay rates, hours worked,
            timesheet approval, creation of divisons to categorize employees, etc."
                .to_string(),
            vec![
                base.clone() + "seetimesheet-1.png",
                base.clone() + "createdivision2-2.png",
                base.clone() + "unapprovedtime-2.png",
                base.clone() + "updateemployee2-1.png",
                base.clone() + "payroll.png",
            ],
            None,
        ),
    )
}

#[get("/student_data_warehouse")]
pub fn warehouse() -> Template {
    let base = "/static/warehouse/".to_owned();
    Template::render("project", Project::new(
        "Student Data Warehouse".to_string(),
        "Student Data Warehouse".to_string(),
        "A data warehouse application written in C# .net core for West Virginia University at Parkersburg.
        Allows the professors and administration to view several metrics on GPA, full time enrollment,
        graduation statistics, and more. The application is deployed locally to the WVUP servers and not
        available to the public.".to_string(),
        vec![
            base.clone() + "DemographicData.jpg",
            base.clone() + "GpaData.jpg",
            base.clone() + "GraduationData.jpg",
            base.clone() + "WithdrawalData.jpg",
            base.clone() + "DegreeChange.jpg",
            base.clone() + "EnrollmentData.jpg",
        ],
        None
    ))
}
