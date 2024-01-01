use birthday::Birthday;
use chrono::{Datelike, NaiveDate};
use tabled::{settings::Style, Table, Tabled};

#[derive(Tabled)]
struct DisplayedBirthday {
    #[tabled(rename = "Id")]
    id: i32,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Birthday")]
    birthday: String,
    #[tabled(rename = "Age")]
    age: String,
    #[tabled(rename = "Next birthday")]
    next_birthday: String,

    #[tabled(skip)]
    next_birthday_in_days: i64,
}

impl DisplayedBirthday {
    fn from_birthday(birthday: &Birthday, today: NaiveDate) -> DisplayedBirthday {
        let age = birthday.age(today).unwrap();
        let birthyear = birthday.date.year();
        let next_birthday_in_days = if birthday.is_today(today) {
            0
        } else {
            (birthday.next(today) - today).num_days()
        };
        let next_birthday = match next_birthday_in_days {
            0 => "today 🎂".to_string(),
            1 => "1 day".to_string(),
            _ => format!("{next_birthday_in_days} days"),
        };
        DisplayedBirthday {
            id: birthday.id,
            name: birthday.name.clone(),
            birthday: birthday.date.format("%d %B").to_string(),
            age: format!("{age} ({birthyear})"),
            next_birthday,
            next_birthday_in_days,
        }
    }
}

pub fn output(birthdays: Vec<Birthday>, today: NaiveDate) {
    if birthdays.is_empty() {
        no_birthday_found();
        return;
    }
    let mut displayed_birthdays: Vec<DisplayedBirthday> = birthdays
        .iter()
        .map(|birthday| DisplayedBirthday::from_birthday(birthday, today))
        .collect();
    displayed_birthdays.sort_by_key(|birthday| birthday.next_birthday_in_days);
    let table = Table::new(displayed_birthdays)
        .with(Style::rounded())
        .to_string();
    println!("{table}")
}

pub fn no_birthday_found() {
    println!("No birthday found 🔎");
}

pub fn birthday_forgotten(birthday: Birthday) {
    println!("Birthday of '{}' has been forgotten 🗑️", birthday.name)
}
