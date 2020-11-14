
const BASIC_URL: &str = "https://gitlab.com/api/v4/";

struct GitlabCredentials {
    pub user_name: String,
    pub user_token: String,
}

impl GitlabCredentials {
    pub fn get() -> Self {
        let user_name = Self::get_env_var("GITLAB_USER");
        let user_token = Self::get_env_var("GITLAB_TOKEN");

        GitlabCredentials {
            user_name,
            user_token,
        }
    }

    fn get_env_var(name: &str) -> String {
        std::env::var(name).unwrap_or_else(|_err| {
            eprintln!("\tGITLAB_USER and/or GITLAB_TOKEN env variables are not available\n");
            let gitlab_token_url = "https://gitlab.com/profile/personal_access_tokens";
            eprintln!("\tTo generate a gitlab token go to: {}", gitlab_token_url);
            std::process::exit(1);
        })
    }
}

pub fn list_projects() {
    let credentials = GitlabCredentials::get();
    let url = &format!(
        "{}users/{}/projects?private_token={}",
        BASIC_URL,
        credentials.user_name,
        credentials.user_token
    );
    let resp = reqwest::blocking::get(url)
        .unwrap()
        .text()
        .unwrap();
    let json = json::parse(&resp).unwrap();
    let len = json.len() - 1;
    println!("projects:");
    for i in 0..len {
        println!("\t{}", json[i]["name"]);
    }
}

pub fn create_project(name: &str) {
    println!("creating project with name: {}...", name);

    let credentials = GitlabCredentials::get();
    let url = &format!(
        "{}projects?private_token={}",
        BASIC_URL,
        credentials.user_token
    );
    let mut body = std::collections::HashMap::new();
    body.insert("name", name);

    let client = reqwest::blocking::Client::new();
    let resp = client.post(url)
        .json(&body)
        .send()
        .expect("error trying to create project");
    if resp.status().is_success() {
        println!("project created successfully");
    } else {
        println!("error {} trying to create project", resp.status().as_u16());
    }
}