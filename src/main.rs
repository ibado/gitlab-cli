use clap::{App, /*Arg,*/ SubCommand};

fn main(){
    let matches = App::new("gitlab-cli")
        .version("0.1")
        .author("Ignacio Bado")
        .about("Allow you to manage your github projects without living the terminal")
        .subcommand(
            SubCommand::with_name("status")
                .about("print the gitlab status")
        )
        .get_matches();
    if matches.is_present("status") {
        println!("status is not yet implemented..");
    }
}
