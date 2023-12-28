use std::path::Path;
use std::process::{Command as ProcessCommand};

use clap::{Args, Parser, Subcommand};
use git2::{Cred, RemoteCallbacks};

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
    #[arg(short, long, env, default_value = "~/gitkit")]
    directory: String,

    /// SSH key used to clone repositories 
    #[arg(short, long, env, default_value = "~/.ssh/id_ed25519")]
    ssh_key: String,
}

#[derive(Subcommand)]
enum Command {
    /// Clone a git repository
    Add {
        repository: String,
    },
    /// Generate a cd command to be executed in your shell
    Cd {
        repository: String,
    },
}

fn main() {
    let mut arguments = Arguments::parse();

    // expand ~ in arguments 
    arguments.global.directory = shellexpand::tilde(&arguments.global.directory).to_string();
    arguments.global.ssh_key = shellexpand::tilde(&arguments.global.ssh_key).to_string();
   
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

            let clone_directory = format!("{}/{}", arguments.global.directory, repository);

            // HTTP
            // if !url.contains("http://") && !url.contains("https://") {
            //     url = format!("https://github.com/{}", url);
            // }
            // let repo = match Repository::clone(&url, clone_directory) {
            //     Ok(repo) => repo,
            //     Err(e) => panic!("failed to clone: {}", e),
            // };

            println!("cloning... {} -> {}", url, clone_directory);

            // prepare callbacks
            let mut callbacks = RemoteCallbacks::new();
            callbacks.credentials(|_url, username_from_url, _allowed_types| {
                Cred::ssh_key(
                    username_from_url.unwrap(),
                    None,
                    Path::new(&arguments.global.ssh_key),
                    None,
                )
            });

            // prepare fetch options
            let mut fo = git2::FetchOptions::new();
            fo.remote_callbacks(callbacks);

            // prepare builder
            let mut builder = git2::build::RepoBuilder::new();
            builder.fetch_options(fo);

            // clone the project
            match builder.clone(
              &url,
              Path::new(&clone_directory),
            ) {
                Ok(_) => println!("ok"),
                Err(e) => panic!("{}", e),
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
    }
}
