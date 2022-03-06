mod credentials;
mod config;
mod gitlab;

use crate::credentials::{GitlabCredentials, write_credentials};
use crate::gitlab::GitlabRepo;

pub fn list_projects() {
    let projects = gitlab_repo().get_projects();

    match projects {
        Ok(project_list) => {
            println!("projects:");
            for i in project_list {
                println!("\t{} - Id({})", i.name, i.id);
            }
        },
        Err(message) => println!("{}", message)
    }
}

pub fn list_groups() {
    let groups = gitlab_repo().get_groups();

    match groups {
        Ok(group_list) => {
            println!("groups:");
            for i in group_list {
                println!("\t{} - Id({})", i.name, i.id);
            }
        },
        Err(message) => println!("{}", message)
    }
}

pub fn create_project(name: &str) {
    println!("creating project with name: {}...", name);

    match gitlab_repo().post_project(name) {
        Ok(id) => println!("\tproject created successfully with Id {}", id),
        Err(msg) => println!("\terror trying to create project: {}", msg)
    }
}

pub fn login() -> Result<(), std::io::Error> {
    println!("\tGitlab user: ");
    let mut gitlab_user = String::new();
    std::io::stdin().read_line(&mut gitlab_user)?;
    let user = gitlab_user.replace("\n", "");

    println!("\tGitlab token: ");
    let mut gitlab_token = String::new();
    std::io::stdin().read_line(&mut gitlab_token)?;
    let token = String::from(gitlab_token.replace("\n", ""));

    write_credentials(GitlabCredentials {user_name: user, user_token: token})?;

    println!("\tLogin successfully!");
    Ok(())
}

fn gitlab_repo() -> GitlabRepo {
    let credentials = get_credentianls();
    return GitlabRepo::new(credentials);
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
