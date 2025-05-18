use authentication::{get_users, hash_password, save_users, LoginRole, User};
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
    Add {
        username: String,
        password: String,

        /// Inidicates whether the user is admin
        #[arg(long)]
        admin: Option<bool>,
    },

    /// Delete user
    Delete {
        username: String,
    },

    /// Change password
    ChangePassword {
        username: String,
        password: String,
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

fn delete_user(username: String) {
    let mut users = get_users();

    if users.contains_key(&username) {
        users.remove(&username);
        authentication::save_users(users);
    } else {
        println!("user {} does not exist", username);
    }
}

fn change_password(username: String, password: String) {
    let mut users = get_users();

    if let Some(user) = users.get_mut(&username) {
        user.password = hash_password(&password);
        save_users(users);
    } else {
        println!("user {} does not exist", username);
    }
}

fn main() {
    let args = Args::parse();

    match args.command {
        Some(Commands::List) => {
            list_users();
        },

        Some(Commands::Add { username, password, admin }) => {
            add_user(username, password, admin.unwrap_or(false));
        },

        Some(Commands::Delete { username }) => {
            delete_user(username);
        },

        Some(Commands::ChangePassword { username, password }) => {
            change_password(username, password);
        }

        None => println!("incorrect syntax: run with --help for more information"),
    }
}
