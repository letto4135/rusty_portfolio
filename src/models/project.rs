use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub title: String,
    pub name: String,
    pub description: String,
    pub pics: Vec<String>,
    pub link: Option<String>,
}

impl Project {
    pub fn new(
        title: String,
        name: String,
        description: String,
        pics: Vec<String>,
        link: Option<String>,
    ) -> Project {
        Project {
            title,
            name,
            description,
            pics,
            link,
        }
    }
}
