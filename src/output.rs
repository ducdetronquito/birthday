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
}

impl DisplayedBirthday {
    fn from_birthday(birthday: &Birthday, today: NaiveDate) -> DisplayedBirthday {
        let age = birthday.age(today).unwrap();
        let birthyear = birthday.date.year();
        let next_birthday_in_days = (birthday.next(today) - today).num_days();
        let next_birthday = if birthday.is_today(today) {
            "today".to_string()
        } else {
            match next_birthday_in_days {
                1 => "1 day".to_string(),
                _ => format!("{next_birthday_in_days} days"),
            }
        };
        DisplayedBirthday {
            id: birthday.id,
            name: birthday.name.clone(),
            birthday: birthday.date.format("%d %B").to_string(),
            age: format!("{age} ({birthyear})"),
            next_birthday,
        }
    }
}

pub fn output(birthdays: Vec<Birthday>, today: NaiveDate) {
    let displayed_birthdays: Vec<DisplayedBirthday> = birthdays
        .iter()
        .map(|birthday| DisplayedBirthday::from_birthday(birthday, today))
        .collect();
    let table = Table::new(displayed_birthdays)
        .with(Style::rounded())
        .to_string();
    println!("{table}")
}
