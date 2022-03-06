use crate::credentials::GitlabCredentials;

const BASE_URL: &str = "https://gitlab.com/api/v4/";

pub struct GitlabRepo {
    credentials: GitlabCredentials,
}

pub struct Project {
    pub id: u64,
    pub name: String,
}

pub struct Group {
    pub id: u64,
    pub name: String,
}

type ErrorMessage = String;

impl GitlabRepo {
    pub fn new(credentials: GitlabCredentials) -> Self {
        GitlabRepo { credentials } 
    }

    pub fn get_projects(&self) -> Result<Vec<Project>, ErrorMessage> {
        let url = &format!(
            "{}users/{}/projects?private_token={}&simple=true",
            BASE_URL,
            self.credentials.user_name,
            self.credentials.user_token
        );
        let resp = reqwest::blocking::get(url).unwrap();
        let status = resp.status();

        if status.is_success() {
            let text_response = &resp.text().unwrap();
            let json = json::parse(text_response).unwrap();
            match json {
                json::JsonValue::Array(list) => {
                    let result: Vec<_> = list.iter().map(|project| {
                        Project {
                            id: format!("{}", project["id"]).parse::<u64>().unwrap(),
                            name: format!("{}", project["name"])
                        }
                    }).collect();
                    Ok(result)
                },
                _ => Err(String::from("api error"))
            }
        } else {
            Err(self.get_error_msg(status))
        }
    }

    pub fn get_groups(&self) -> Result<Vec<Group>, String> {
        let url = &format!(
            "{}groups?private_token={}&simple=true",
            BASE_URL,
            self.credentials.user_token
        );
        let resp = reqwest::blocking::get(url).unwrap();
        let status = resp.status();
        if status.is_success() {
            let text_response = &resp.text().unwrap();
            let json = json::parse(text_response).unwrap();
            match json {
                json::JsonValue::Array(list) => {
                    let result: Vec<_> = list.iter().map(|group| {
                        Group {
                            id: format!("{}", group["id"]).parse::<u64>().unwrap(),
                            name: format!("{}", group["full_name"])
                        }
                    }).collect();
                    Ok(result)
                },
                _ => Err(String::from("api error"))
            }
        } else {
            Err(self.get_error_msg(status))
        }
    }

    fn get_error_msg(&self, status: reqwest::StatusCode) -> String {
        let code = status.as_u16();
        let error = status.canonical_reason().unwrap();
        let mut error_message = format!("\t{}: {}", code, error);
        if code == 401 {
            error_message.push_str("\tYour token is not valid");
        }
        error_message
    }
}
