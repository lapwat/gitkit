use std::process::{Command as ProcessCommand};

use clap::{Args, Parser, Subcommand};

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
    /// GitHub username
    #[arg(short, long, env, default_value = "$USER")]
    user: String,

    /// Directory where your repositories are stored
    #[arg(short, long, env, default_value = "~/projects")]
    directory: String,

    /// Directory where your test repositories are stored
    #[arg(short, long, env, default_value = "~/tests")]
    tests_directory: String,
}

#[derive(Subcommand)]
enum Command {
    /// Clone a git repository
    Add {
        repository: String,
    },
    /// Clone a git test repository
    Test {
        repository: String,
    },
    /// Generate a cd command to be executed in your shell
    Cd {
        repository: String,
    },
    /// Commit all modifications and push them to remote 
    Sync {
        repository: String,

        /// Commit message 
        #[arg(short, long, default_value = "commit")]
        message: String,
    },
}

fn main() {
    let mut arguments = Arguments::parse();

    // expand ~ in arguments
    arguments.global.directory = shellexpand::tilde(&arguments.global.directory).to_string();
    arguments.global.tests_directory = shellexpand::tilde(&arguments.global.tests_directory).to_string();
   
    match arguments.command {
        Command::Add { repository } => {
            let mut url = repository.clone();

            // prepend username to repo if needed  
            if !url.contains("/") {
                url = format!("{}/{}", arguments.global.user, url);
            }

            // prepend domain to repo if needed  

            // SSH
            if !url.contains("git@") {
                 url = format!("git@github.com:{}", url);
            }

            let command = format!("mkdir -p {} && cd {} && git clone {}", arguments.global.directory, arguments.global.directory, url);

            let (code, output, error) = run_script::run_script!(command).unwrap();
            if code != 0 {
                println!("Error: {}", error);
            } 
            if !output.is_empty() {
                println!("Output: {}", output);
            }
        },
        Command::Test { repository } => {
            let mut url = repository.clone();

            // prepend username to repo if needed  
            if !url.contains("/") {
                url = format!("{}/{}", arguments.global.user, url);
            }

            // prepend domain to repo if needed  

            // SSH
            if !url.contains("git@") {
                 url = format!("git@github.com:{}", url);
            }

            let command = format!("mkdir -p {} && cd {} && git clone {}", arguments.global.tests_directory, arguments.global.tests_directory, url);

            let (code, output, error) = run_script::run_script!(command).unwrap();
            if code != 0 {
                println!("Error: {}", error);
            }
            if !output.is_empty() {
                println!("Output: {}", output);
            }
        },
        Command::Cd { repository } => {
            let command = format!("cd {}/{}", arguments.global.directory, repository);
            println!("{}", command);

            ProcessCommand::new("sh")
                .arg(command)
                .output()
                .expect("failed to change directory");
        },
        Command::Sync { repository, message } => {
            let command = format!("cd {}/{} && git add . && git commit -m '{}' && git push", arguments.global.directory, repository, message);

            let (code, output, error) = run_script::run_script!(command).unwrap();
            if code != 0 {
                println!("Error: {}", error);
            } 
            if !output.is_empty() {
                println!("Output: {}", output);
            }
        },
    }
}
