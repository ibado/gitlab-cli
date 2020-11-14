use clap::{App, Arg, ArgMatches};

const LOGIN: &str = "login";
const STATUS: &str = "status";
const PROJECT: &str = "project";
const CREATE: &str = "create";
const LIST: &str = "list";
const NAME: &str = "name";

fn main() {
    let matches = get_matches();
    if matches.is_present(STATUS) {
        println!("status is not yet implemented..");
    } else if matches.is_present(LOGIN) {
        println!("\tlogin is not implemented yet");
    } else if let Some(project) = matches.subcommand_matches(PROJECT) {
        if let Some(create) = project.subcommand_matches(CREATE) {
            if create.is_present(NAME) {
                let name = create.value_of(NAME).unwrap();
                glab::create_project(&name);
            }
        } else if let Some(_) = project.subcommand_matches(LIST) {
            glab::list_projects()
        }
    }
}

fn get_matches() -> ArgMatches {
    let app = App::new("glab")
        .version("0.1")
        .author("Ignacio Bado")
        .about("Allow you to manage your gitlab projects without leaving the terminal")
        .subcommand(App::new(STATUS)
            .about("print the gitlab status")
        )
        .subcommand(App::new(LOGIN)
            .about("Interactively set Gitlab credentials")
        )
        .subcommand(App::new(PROJECT)
            .about("create or manage existing projects")
            .subcommand(App::new(CREATE)
                .about("Create a new project with the given name")
                .arg(Arg::new(NAME)
                    .index(1)
                    .required(true)
                )
            )
            .subcommand(App::new(LIST)
                .about("List the existing projects")
            )
        );
    app.get_matches()
}