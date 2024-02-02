mod birthday;
mod birthday_store;
use std::path::{Path, PathBuf};

use anyhow::{bail, Result};
pub use birthday::Birthday;
use birthday_store::SqliteBirthdayStore;
use chrono::{Datelike, NaiveDate};
use directories::ProjectDirs;

fn get_db_path() -> Result<PathBuf> {
    let mut data_dir = match std::env::var("BIRTHDAY_DATA") {
        Ok(path) => PathBuf::from(path),
        Err(_) => match ProjectDirs::from("", "", "birthday") {
            Some(project_dirs) => project_dirs.data_dir().to_owned(),
            None => Path::new("./").to_owned(),
        },
    };

    std::fs::create_dir_all(&data_dir)?;
    data_dir.push("test.db");
    Ok(data_dir)
}

fn open_store() -> Result<SqliteBirthdayStore> {
    let db_path = get_db_path()?;
    SqliteBirthdayStore::open(db_path.as_path(), "birthday".to_string())
}

pub fn add(name: String, day: u32, month: u32, year: i32) -> Result<()> {
    let store = open_store()?;
    let birthdate = NaiveDate::from_ymd_opt(year, month, day).expect("Invalid date");
    store.add(name, birthdate)
}

pub fn get_all() -> Result<Vec<Birthday>> {
    let store = open_store()?;
    store.get_all()
}

pub fn get_next(today: NaiveDate) -> Result<Option<Birthday>> {
    let store = open_store()?;
    let mut birthdays = store.get_all()?;
    birthdays.sort_by_key(|birthday| birthday.next(today));
    Ok(birthdays.into_iter().next())
}

pub fn search(
    name: Option<String>,
    year: Option<i32>,
    month: Option<u32>,
    day: Option<u32>,
) -> Result<Vec<Birthday>> {
    let store = open_store()?;
    let mut birthdays = store.get_all()?;

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
    let store = open_store()?;
    store.remove(id)
}
