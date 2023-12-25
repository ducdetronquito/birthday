use anyhow::Result;
use birthday::Birthday;
use chrono::Utc;
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
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    println!("You ran cli with: {:?}", cli);
    match cli.command {
        Command::Add { name, date } => birthday::add_birthday(name, date),
        Command::All {} => {
            let birthdays = birthday::get_all_birthdays()?;
            print_birthdays(birthdays);
            Ok(())
        }
        Command::Next {} => {
            let today = Utc::now().date_naive();
            let maybe_birthday = birthday::get_next_birthday(today)?;
            if let Some(birthday) = maybe_birthday {
                print_birthdays(vec![birthday]);
            }
            Ok(())
        }
        Command::Search {
            name,
            year,
            month,
            day,
        } => {
            let birthdays = birthday::search_birthdays(name, year, month, day)?;
            print_birthdays(birthdays);
            Ok(())
        }
        Command::Today {} => {
            let today = Utc::now().date_naive();
            let birthdays = birthday::get_birthdays_for_date(today)?;
            print_birthdays(birthdays);
            Ok(())
        }
    }
}

fn print_birthdays(birthdays: Vec<Birthday>) {
    let today = Utc::now().date_naive();
    for birthday in birthdays {
        let age = match birthday.age(today) {
            Some(age) => age.to_string(),
            None => "".to_owned(),
        };
        println!(
            "Name={} Birthdate={} Age={}",
            birthday.name, birthday.date, age
        );
    }
}
