use clap::{Arg, ArgAction, Command};
use tracing::{info, error, Level};
use tracing_subscriber::FmtSubscriber;
use daemonize::Daemonize;

use std::process::Command as OsCommand;
use std::fs::File;

mod identify;

fn main() {
    // Start the log tracing
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed"); // Set it globally

    let mut dependencies_ok = true;

    // We first need to identify if the right tools are available, otherwise alert the user if so.
    match OsCommand::new("gcc").output() {
        Ok(_) => {},
        Err(e) => {
            error!("`gcc` was not found!");
            dependencies_ok = false;
        }, 
    }

    match OsCommand::new("git").output() {
        Ok(_) => {},
        Err(e) => {
            error!("`git` was not found!");
            dependencies_ok = false;
        }, 
    }

    if !dependencies_ok {
        return;
    }

    info!("All dependent programs are available on the system!");

    // Lets check arguments to see what the user wants the program to do
    let matches = Command::new("pacman")
        .about("package manager utility")
        .version("5.2.1")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Pacman Development Team")
        // Build subcommand
        .subcommand(
            Command::new("build")
                .short_flag('B')
                .long_flag("build")
                .about("Build tools for project construction and identification.")
                .arg(
                    Arg::new("identify")
                        .short('i')
                        .long("identify")
                        .help("Identify the configuration required to run the project.")
                        .conflicts_with("compile")
                        .action(ArgAction::Set)
                        .num_args(1..),
                )
                .arg(
                    Arg::new("compile")
                        .long("compile")
                        .short('c')
                        .conflicts_with("identify")
                        .help("Compile a project folder into a container to be executed.")
                        .action(ArgAction::Set)
                        .num_args(1..),
                ),
        )
        .subcommand(
            Command::new("server")
                .short_flag('S')
                .long_flag("server")
                .about("Run the server daemon for ShuttleManager.")
        )
        .get_matches();

    match matches.subcommand() {
        Some(("build", query_matches)) => {
            if let Some(packages) = query_matches.get_many::<String>("identify") {
                let comma_sep = packages.map(|s| s.as_str()).collect::<Vec<_>>();
                info!("Retrieving info for {}...", comma_sep.join(", "));
                identify::identify(comma_sep[0].to_string());
            } else if let Some(queries) = query_matches.get_many::<String>("compile") {
                let comma_sep = queries.map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
                info!("Attempting to compile for {comma_sep}...");
            } else {
                println!("Displaying locally available projects...");
            }
        }
        Some(("server", query_matches)) => { 
            info!("Attempting to run as a server...");
            let stdout = File::create("/tmp/ShuttleManager.out").unwrap();
            let stderr = File::create("/tmp/ShuttleManager.err").unwrap();
            let daemon = Daemonize::new()
                .working_directory("/tmp")
                .stdout(stdout)
                .stderr(stderr);
            match daemon.start() {
                Ok(_) => info!("Successfully started ShuttleManager daemon!"),
                Err(e) => error!("Failed to start ShuttleManager daemon: {}", e),
            }
        }
        _ => {
            error!("Invalid command!");
        },
    }
}
