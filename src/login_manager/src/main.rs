use clap::{Parser, Subcommand};

/// Simple User Management App
#[derive(Parser)]
#[command()]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all users
    List,
}

fn list_users() {
    println!("{:<20}{:<20}", "Username", "Role");
    println!("{:-<40}", "");

    let users = authentication::get_users();

    users.iter().for_each(|(_, user)| {
        println!("{:<20}{:<20?}", user.username, user.role);
    });
}

fn main() {
    let args = Args::parse();

    match args.command {
        Some(Commands::List) => {
            list_users();
        },

        None => println!("incorrect syntax: run with --help for more information"),
    }
}
