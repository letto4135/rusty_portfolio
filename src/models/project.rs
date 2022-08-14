use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub title: String,
    pub name: String,
    pub description: String,
    pub link: Option<String>,
}

impl Project {
    pub fn new(title: String, name: String, description: String, link: Option<String>) -> Project {
        Project {
            title,
            name,
            description,
            link,
        }
    }

    pub fn default() -> Project {
        Project::new(
            "Project Title".to_string(),
            "Project Name".to_string(),
            "Project Description".to_string(),
            Some("https://www.example.com/photo.png".to_string()),
        )
    }
}
