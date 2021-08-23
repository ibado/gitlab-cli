use std::io::{Write, BufRead};
use std::fs::OpenOptions;
use std::error::Error;

const BASIC_URL: &str = "https://gitlab.com/api/v4/";
const GITLAB_USER_KEY: &str = "GITLAB_USER";
const GITLAB_TOKEN_KEY: &str = "GITLAB_TOKEN";

struct GitlabCredentials {
    pub user_name: String,
    pub user_token: String,
}

impl GitlabCredentials {
    pub fn get() -> Result<Self, Box<dyn Error>> {
        load_config();
        let user_name = std::env::var(GITLAB_USER_KEY)?;
        let user_token = std::env::var(GITLAB_TOKEN_KEY)?;

        Result::Ok(
            GitlabCredentials {
                user_name,
                user_token,
            }
        )
    }
}

pub fn list_projects() {
    let credentials = get_credentianls();
    let url = &format!(
        "{}users/{}/projects?private_token={}",
        BASIC_URL,
        credentials.user_name,
        credentials.user_token
    );
    let resp = reqwest::blocking::get(url).unwrap();
    let status = resp.status();
    if status.is_success() {
        let text_response = &resp.text().unwrap();
        let json = json::parse(text_response).unwrap();
        let len = json.len() - 1;
        println!("projects:");
        for i in 0..len {
            println!("\t{}", json[i]["name"]);
        }
    } else {
        let code = status.as_u16();
        let error = status.canonical_reason().unwrap();
        println!("\t{}: {}", code, error);
        if code == 401 {
            println!("\tYour token is not valid");
        }
    }
}

pub fn create_project(name: &str) {
    println!("creating project with name: {}...", name);

    let credentials = get_credentianls();
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

pub fn login() -> Result<(), std::io::Error> {
    println!("\tGitlab user: ");
    let mut gitlab_user = String::new();
    std::io::stdin().read_line(&mut gitlab_user)?;
    let user = gitlab_user.replace("\n", "");
    write_config(GITLAB_USER_KEY.to_string(), user)?;

    println!("\tGitlab token: ");
    let mut gitlab_token = String::new();
    std::io::stdin().read_line(&mut gitlab_token)?;
    let token = String::from(gitlab_token.replace("\n", ""));
    write_config(GITLAB_TOKEN_KEY.to_string(), token)?;

    println!("\tLogin successfully!");
    Ok(())
}

fn get_credentianls() -> GitlabCredentials {
    match GitlabCredentials::get() {
        Ok(credentials) => credentials,
        Err(_) => {
            eprintln!("\tGITLAB_USER and/or GITLAB_TOKEN env variables are not set\n");
            println!("\tYou can use 'glab login' command or set them manually");
            let gitlab_token_url = "https://gitlab.com/profile/personal_access_tokens";
            println!("\tTo generate a gitlab token go to: {}", gitlab_token_url);
            std::process::exit(1);
        }
    }
}

fn load_config() {
    let file = OpenOptions::new()
        .read(true)
        .open(config_file_name());

    if file.is_err() { return (); }

    let lines = std::io::BufReader::new(file.unwrap()).lines();
    for line in lines {
        if let Ok(config) = line {
            let words: Vec<_> = config.split("=").collect();
            std::env::set_var(words[0], words[1])
        }
    }
}

fn write_config(key: String, value: String) -> Result<(), std::io::Error> {
    let file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(config_file_name())
        .unwrap();
    return writeln!(&file, "{}={}", key, value);
}

fn config_file_name() -> String {
    let home = std::env::var("HOME").unwrap();
    home + "/.glab"
}
