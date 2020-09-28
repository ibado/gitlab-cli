use clap::{App, Arg, SubCommand};

const BASIC_URL: &str = "https://gitlab.com/api/v4/";

fn main() {
    let matches = App::new("glab")
        .version("0.1")
        .author("Ignacio Bado")
        .about("Allow you to manage your gitlab projects without leaving the terminal")
        .subcommand(
            SubCommand::with_name("status")
                .about("print the gitlab status")
        )
        .subcommand(
            SubCommand::with_name("project")
                .about("create or manage existing projects")
                .subcommand(
                    SubCommand::with_name("create")
                        .arg(
                            Arg::with_name("name")
                                .long("name")
                                .index(1)
                                .required(true)
                        )
                )
                .subcommand(
                    SubCommand::with_name("list")
                )
        )
        .get_matches();

    if matches.is_present("status") {
        println!("status is not yet implemented..");
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
    let user = std::env::var("GITLAB_USER")
        .expect("GITLAB_USER env variable is not available");
    let token = std::env::var("GITLAB_TOKEN")
        .expect("GITLAB_TOKEN env variable is not available");
    let url = &format!("{}users/{}/projects?private_token={}", BASIC_URL, user, token);
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