mod birthday;
mod birthday_store;
use anyhow::{bail, Result};
pub use birthday::Birthday;
use chrono::{Datelike, NaiveDate};

pub fn add(name: String, day: u32, month: u32, year: i32) -> Result<()> {
    let birthdate = NaiveDate::from_ymd_opt(year, month, day).expect("Invalid date");
    birthday_store::add(name, birthdate)
}

pub fn get_all() -> Result<Vec<Birthday>> {
    birthday_store::get_all()
}

pub fn get_next(today: NaiveDate) -> Result<Option<Birthday>> {
    let mut birthdays = birthday_store::get_all()?;
    birthdays.sort_by_key(|birthday| birthday.next(today));
    Ok(birthdays.into_iter().next())
}

pub fn search(
    name: Option<String>,
    year: Option<i32>,
    month: Option<u32>,
    day: Option<u32>,
) -> Result<Vec<Birthday>> {
    let mut birthdays = birthday_store::get_all()?;

    if let Some(name) = name {
        birthdays.retain(|birthday| birthday.name.contains(&name));
    }

    if let Some(year) = year {
        birthdays.retain(|birthday| birthday.date.year() == year);
    }

    if let Some(month) = month {
        if !(1..=12).contains(&month) {
            bail!("Month must be between 1 and 12");
        }
        birthdays.retain(|birthday| birthday.date.month() == month);
    }

    if let Some(day) = day {
        if !(1..=31).contains(&day) {
            bail!("Month must be between 1 and 31");
        }
        birthdays.retain(|birthday| birthday.date.day() == day);
    }

    Ok(birthdays)
}

pub fn forget(id: i32) -> Result<Option<Birthday>> {
    birthday_store::remove(id)
}
