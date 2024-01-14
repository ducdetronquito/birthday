use std::path::{Path, PathBuf};

use anyhow::Result;
use chrono::{NaiveDate, NaiveDateTime};
use directories::ProjectDirs;
use rusqlite::Connection;

use crate::Birthday;

pub fn add(name: String, date: String) -> Result<()> {
    let db = get_db()?;
    let date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")?;
    let timestamp = to_timestamp(date);
    db.execute(
        "INSERT INTO birthdays(name, date_timestamp) VALUES(?1, ?2)",
        (name, timestamp),
    )?;
    Ok(())
}

fn get_db_path() -> Result<PathBuf> {
    let mut data_dir = match ProjectDirs::from("", "", "birthday") {
        Some(project_dirs) => project_dirs.data_dir().to_owned(),
        None => Path::new("./").to_owned(),
    };
    std::fs::create_dir_all(&data_dir)?;
    data_dir.push("test.db");
    Ok(data_dir)
}

pub fn get_all() -> Result<Vec<Birthday>> {
    let db = get_db()?;
    let mut statement = db.prepare("SELECT id, name, date_timestamp FROM birthdays")?;
    let birthday_iter = statement.query_map([], |row| {
        let id = row.get(0)?;
        let name = row.get(1)?;
        let timestamp = row.get(2)?;
        let date = from_timestamp(timestamp);
        Ok(Birthday { id, name, date })
    })?;
    let birthdays = birthday_iter.collect::<Result<Vec<Birthday>, rusqlite::Error>>()?;
    Ok(birthdays)
}

pub fn remove(id: i32) -> Result<Option<Birthday>> {
    let db = get_db()?;
    let mut statement =
        db.prepare("DELETE FROM birthdays WHERE id = :id RETURNING id, name, date_timestamp")?;
    let birthday_iter = statement.query_map(&[(":id", id.to_string().as_str())], |row| {
        let id = row.get(0)?;
        let name = row.get(1)?;
        let timestamp = row.get(2)?;
        let date = from_timestamp(timestamp);
        Ok(Birthday { id, name, date })
    })?;
    let birthdays = birthday_iter.collect::<Result<Vec<Birthday>, rusqlite::Error>>()?;
    if birthdays.is_empty() {
        Ok(None)
    } else {
        Ok(Some(birthdays[0].clone()))
    }
}

fn get_db() -> Result<Connection> {
    let db_path = get_db_path()?;
    let db = Connection::open(db_path)?;
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

fn to_timestamp(date: NaiveDate) -> i64 {
    date.and_hms_opt(0, 0, 0).unwrap().timestamp()
}

fn from_timestamp(timestamp: i64) -> NaiveDate {
    NaiveDateTime::from_timestamp_opt(timestamp, 0)
        .unwrap()
        .date()
}
