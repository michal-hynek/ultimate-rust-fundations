use authentication::{get_users, LoginRole, User};
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

    /// Add new user
    AddUser {
        username: String,
        password: String,

        /// Inidicates whether the user is admin
        #[arg(long)]
        admin: Option<bool>,
    },
}

fn list_users() {
    println!("{:<20}{:<20}", "Username", "Role");
    println!("{:-<40}", "");

    let users = authentication::get_users();

    users.iter().for_each(|(_, user)| {
        println!("{:<20}{:<20?}", user.username, user.role);
    });
}

fn add_user(username: String, password: String, is_admin: bool) {
    let mut users = get_users();

    if users.contains_key(&username) {
        println!("user {} already exists", username);
        return;
    }

    let role = if is_admin {
        LoginRole::Admin
    } else {
        LoginRole::User
    };


    let new_user = User::new( &username, &password, role);
    users.insert(username, new_user);
    authentication::save_users(users);
}

fn main() {
    let args = Args::parse();

    match args.command {
        Some(Commands::List) => {
            list_users();
        },

        Some(Commands::AddUser { username, password, admin }) => {
            add_user(username, password, admin.unwrap_or(false));
        },

        None => println!("incorrect syntax: run with --help for more information"),
    }
}
