use clap::Clap;

fn main() {

    let opts = Opts::parse();

    match opts.subcommand {
        SubCommand::Status => println!("status is not implemented yet.."),
        SubCommand::Login => {
            let result = glab::login();
            if result.is_err() {
                print!("Error: {}", result.err().unwrap().to_string());
            }
        },
        SubCommand::Project(cmd) => match cmd {
            ProjectCommand::Create(arg) => glab::create_project(&arg.name),
            ProjectCommand::List => glab::list_projects(),
        },
        SubCommand::Group(cmd) => match cmd {
            GroupCommand::List => glab::list_groups(),
        }
    };
}

#[derive(Clap)]
#[clap(version = "0.0.1", author = "Ignacio Bado")]
struct Opts {
    #[clap(subcommand)]
    subcommand: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Status,
    Login,
    Project(ProjectCommand),
    Group(GroupCommand),
}

#[derive(Clap)]
enum ProjectCommand {
    Create(CreateCmd),
    List,
}

#[derive(Clap)]
enum GroupCommand {
    List,
}

#[derive(Clap)]
struct CreateCmd {
    name: String,
}
