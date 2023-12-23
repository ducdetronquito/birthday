use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version)]
#[command(about = "🎂 CLI tool to remember birthdays of people you know")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    #[command(about = "Add a person's birthday")]
    Add { name: String, date: String },
    #[command(about = "Show all birthdays")]
    All {},
    #[command(about = "Show the next birthday")]
    Next {},

    #[command(about = "Search for birthdays")]
    #[command(arg_required_else_help(true))]
    Search {
        #[arg(short, long, help = "match for names containing <NAME>")]
        name: Option<String>,
        #[arg(short, long, help = "match for a specific <DATE>")]
        date: Option<String>,
    },
    #[command(about = "Show today's birthdays")]
    Today {},
}

fn main() {
    let args = Cli::parse();
    println!("You ran cli with: {:?}", args);
}
