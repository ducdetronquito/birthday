use anyhow::{bail, Result};
use chrono::{Datelike, NaiveDate, NaiveDateTime};
use rusqlite::Connection;

fn get_db() -> Result<Connection> {
    let db = Connection::open("test.db")?;
    db.execute(
        "CREATE TABLE IF NOT EXISTS birthdays (
             id INTEGER PRIMARY KEY,
             name TEXT NOT NULL,
             date_timestamp INTEGER NOT NULL
         ) STRICT",
        (),
    )?;
    Ok(db)
}

pub struct Birthday {
    pub id: i32,
    pub name: String,
    pub date: NaiveDate,
}

impl Birthday {
    pub fn age(&self, today: NaiveDate) -> Option<u32> {
        today.years_since(self.date)
    }

    pub fn next(&self, today: NaiveDate) -> NaiveDate {
        let birthday_this_year = self.date.with_year(today.year()).unwrap();
        let birthday_next_year = self.date.with_year(today.year() + 1).unwrap();
        if birthday_this_year > today {
            birthday_this_year
        } else {
            birthday_next_year
        }
    }
}

fn to_timestamp(date: NaiveDate) -> i64 {
    date.and_hms_opt(0, 0, 0).unwrap().timestamp()
}

fn from_timestamp(timestamp: i64) -> NaiveDate {
    NaiveDateTime::from_timestamp_opt(timestamp, 0)
        .unwrap()
        .date()
}

pub fn add_birthday(name: String, date: String) -> Result<()> {
    let db = get_db()?;
    let date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")?;
    let timestamp = to_timestamp(date);
    db.execute(
        "INSERT INTO birthdays(name, date_timestamp) VALUES(?1, ?2)",
        (name, timestamp),
    )?;
    Ok(())
}

pub fn get_all_birthdays() -> Result<Vec<Birthday>> {
    let db = get_db()?;
    let mut statement = db.prepare("SELECT id, name, date_timestamp FROM birthdays")?;
    let birthday_iter = statement.query_map([], |row| {
        let id = row.get(0)?;
        let name = row.get(1)?;
        let timestamp = row.get(2)?;
        let date = from_timestamp(timestamp);
        Ok(Birthday { id, name, date })
    })?;
    let birthdays: Result<Vec<Birthday>, rusqlite::Error> = birthday_iter.collect();
    Ok(birthdays.unwrap())
}

pub fn get_next_birthday(today: NaiveDate) -> Result<Option<Birthday>> {
    let mut birthdays = get_all_birthdays()?;
    birthdays.sort_by_key(|birthday| birthday.next(today));
    Ok(birthdays.into_iter().next())
}

pub fn search_birthdays(
    name: Option<String>,
    year: Option<i32>,
    month: Option<u32>,
    day: Option<u32>,
) -> Result<Vec<Birthday>> {
    let mut birthdays = get_all_birthdays()?;

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
