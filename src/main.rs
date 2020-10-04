use clap::{App, Arg};
//use clap_generate::{generate, generators::Bash};

const BASIC_URL: &str = "https://gitlab.com/api/v4/";

fn main() {
    let app = App::new("glab")
        .version("0.1")
        .author("Ignacio Bado")
        .about("Allow you to manage your gitlab projects without leaving the terminal")
        .subcommand(
            App::new("status")
                .about("print the gitlab status")
        )
        .subcommand(
            App::new("login").about("Interactively set Gitlab credentials")
        )
        .subcommand(
            App::new("project")
                .about("create or manage existing projects")
                .subcommand(
                    App::new("create")
                        .about("Create a new project with the given name")
                        .arg(
                            Arg::new("name")
                                .long("name")
                                .index(1)
                                .required(true)
                        )
                )
                .subcommand(
                    App::new("list").about("List the existing projects")
                )
        );

    //generate::<Bash, _>(&mut app, "glab", &mut std::io::stdout());

    let matches = app.get_matches();

    if matches.is_present("status") {
        println!("status is not yet implemented..");
    }
    if matches.is_present("login") {
        println!("\tlogin is not implemented yet");
    }
    if let Some(m) = matches.subcommand_matches("project") {
        if let Some(m) = m.subcommand_matches("create") {
            if m.is_present("name") {
                let name = m.value_of("name").unwrap();
                create_project(&name);
            }
        } else if let Some(_) = m.subcommand_matches("list") {
            list_projects()
        }
    }
}

fn list_projects() {
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

fn create_project(name: &str) {
    println!("creating project with name: {}...", name);
}

struct GitlabCredentials {
    user_name: String,
    user_token: String,
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