use clap::{App, Arg};
//use clap_generate::{generate, generators::Bash};
use glab::{list_projects, create_project};

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