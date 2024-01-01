mod output;
use anyhow::Result;
use chrono::{Datelike, Utc};
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
        #[arg(short, long, help = "match for a specific <YEAR>")]
        year: Option<i32>,
        #[arg(short, long, help = "match for a specific <MONTH>")]
        month: Option<u32>,
        #[arg(short, long, help = "match for a specific <DAY>")]
        day: Option<u32>,
    },
    #[command(about = "Show today's birthdays")]
    Today {},
    #[command(about = "Forget a birthday by ID")]
    Forget { id: i32 },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let today = Utc::now().date_naive();
    match cli.command {
        Command::Add { name, date } => birthday::add(name, date),
        Command::All {} => {
            let birthdays = birthday::get_all()?;
            output::output(birthdays, today);
            Ok(())
        }
        Command::Next {} => {
            let maybe_birthday = birthday::get_next(today)?;
            if let Some(birthday) = maybe_birthday {
                output::output(vec![birthday], today);
            }
            Ok(())
        }
        Command::Search {
            name,
            year,
            month,
            day,
        } => {
            let birthdays = birthday::search(name, year, month, day)?;
            output::output(birthdays, today);
            Ok(())
        }
        Command::Today {} => {
            let today = Utc::now().date_naive();
            let birthdays = birthday::search(
                None,
                Some(today.year()),
                Some(today.month()),
                Some(today.day()),
            )?;
            output::output(birthdays, today);
            Ok(())
        }
        Command::Forget { id } => {
            let maybe_birthday = birthday::forget(id)?;
            match maybe_birthday {
                Some(birthday) => output::birthday_forgotten(birthday),
                None => output::no_birthday_found(),
            }
            Ok(())
        }
    }
}
