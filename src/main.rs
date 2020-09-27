use clap::{App, Arg, SubCommand};

fn main(){
    let matches = App::new("gitlab-cli")
        .version("0.1")
        .author("Ignacio Bado")
        .about("Allow you to manage your github projects without living the terminal")
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
        )
        .get_matches();

    if matches.is_present("status") {
        println!("status is not yet implemented..");
    }
    if let Some(m) = matches.subcommand_matches("project") {
        if let Some(m) = m.subcommand_matches("create") {
            if m.is_present("name") {
                let name = m.value_of("name").unwrap();
                create_project(&name)
            }
        }
    }
}

fn create_project(name: &str) {
    println!("creating project with name: {}...", name)
}
