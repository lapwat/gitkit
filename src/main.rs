use clap::{Args, Parser, Subcommand};
use git2::Repository;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    #[clap(flatten)]
    global: Global,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Args)]
struct Global {
    #[arg(short, long, env)]
    user: String,

    #[arg(short, long, env, default_value = "~/gitkit")]
    directory: String
}

#[derive(Subcommand)]
enum Command {
    Add {
        repository: String,
    },
}

fn main() {
    let mut arguments = Arguments::parse();

    // expand ~ in workdir
    arguments.global.directory = shellexpand::tilde(&arguments.global.directory).to_string();
   
    match arguments.command {
        Command::Add { repository } => {
            let mut url = repository.clone();

            // prepend username to repo if needed  
            if !url.contains("/") {
                url = format!("{}/{}", arguments.global.user, url);
            }

            // prepend domain to repo if needed  
            if !url.contains("http://") && !url.contains("https://") {
                url = format!("https://github.com/{}", url);
            }

            let clone_directory = format!("{}/{}", arguments.global.directory, repository);

            println!("cloning... {} -> {}", url, clone_directory);

            let repo = match Repository::clone(&url, clone_directory) {
                Ok(repo) => repo,
                Err(e) => panic!("failed to clone: {}", e),
            };
        },
    }
}
