use crate::credentials::GitlabCredentials;
use reqwest::blocking::Response;

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
        
        return self.process_project_response(resp);
    }  

    pub fn get_groups(&self) -> Result<Vec<Group>, String> {
        let url = &format!(
            "{}groups?private_token={}&simple=true",
            BASE_URL,
            self.credentials.user_token
        );
        let resp = reqwest::blocking::get(url).unwrap();
        let status = resp.status();

        if !status.is_success() {
            return Err(self.get_error_msg(status));
        }

        let text_response = &resp.text().unwrap();
        let json = json::parse(text_response).unwrap();

        let mut result = vec![];
        if let json::JsonValue::Array(list) = json {
            result = list.iter().map(|group| {
                let id =  format!("{}", group["id"]).parse::<u64>().unwrap();
                let name = format!("{}", group["full_name"]);
                Group { id, name }
            }).collect();
        }

        return Ok(result);
    }

    pub fn get_group_projects(&self, group_id: &str) -> Result<Vec<Project>, ErrorMessage> {
        let url = &format!(
            "{}groups/{}/projects?private_token={}&simple=true",
            BASE_URL,
            group_id,
            self.credentials.user_token
        );
        let resp = reqwest::blocking::get(url).unwrap();

        return self.process_project_response(resp);
    }

    pub fn post_project(&self, name: &str) -> Result<String, ErrorMessage> {
        let url = &format!(
            "{}projects?private_token={}",
            BASE_URL,
            self.credentials.user_token
        );
        let mut body = std::collections::HashMap::new();
        body.insert("name", name);

        let client = reqwest::blocking::Client::new();
        let resp = client.post(url)
            .json(&body)
            .send()
            .expect("error trying to create project");
        if resp.status().is_success() {
            let json = json::parse(&resp.text().unwrap()).unwrap();
            Ok(format!("{}", json["id"]))
        } else {
            Err(self.get_error_msg(resp.status()))
        }
    }

    fn process_project_response(&self, resp: Response) -> Result<Vec<Project>, ErrorMessage> {
        let status = resp.status();

        if !status.is_success() {
            return Err(self.get_error_msg(status));
        }

        let text_response = &resp.text().unwrap();
        let json = json::parse(text_response).unwrap();

        let mut result = vec![];
        if let json::JsonValue::Array(list) = json {
            result = list.iter().map(|project| {
                let id = format!("{}", project["id"]).parse::<u64>().unwrap();
                let name = format!("{}", project["name"]);
                Project { id, name }
            }).collect();
        }

        return Ok(result);
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
