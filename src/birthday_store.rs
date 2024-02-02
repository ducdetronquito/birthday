use std::path::Path;

use anyhow::Result;
use chrono::{NaiveDate, NaiveDateTime};
use rusqlite::Connection;

use crate::Birthday;

pub struct SqliteBirthdayStore {
    conn: Connection,
}

impl SqliteBirthdayStore {
    pub fn open(data_dir: &Path, store_name: String) -> Result<SqliteBirthdayStore> {
        let mut db_path = data_dir.to_owned();
        db_path.push(store_name + ".db");
        let conn = Connection::open(db_path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS birthdays (
                 id INTEGER PRIMARY KEY,
                 name TEXT NOT NULL,
                 date_timestamp INTEGER NOT NULL
             ) STRICT",
            (),
        )?;
        Ok(SqliteBirthdayStore { conn })
    }

    pub fn add(&self, name: String, date: NaiveDate) -> Result<()> {
        let timestamp = to_timestamp(date);
        self.conn.execute(
            "INSERT INTO birthdays(name, date_timestamp) VALUES(?1, ?2)",
            (name, timestamp),
        )?;
        Ok(())
    }

    pub fn get_all(&self) -> Result<Vec<Birthday>> {
        let mut statement = self
            .conn
            .prepare("SELECT id, name, date_timestamp FROM birthdays")?;
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

    pub fn remove(&self, id: i32) -> Result<Option<Birthday>> {
        let mut statement = self
            .conn
            .prepare("DELETE FROM birthdays WHERE id = :id RETURNING id, name, date_timestamp")?;
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
}

fn to_timestamp(date: NaiveDate) -> i64 {
    date.and_hms_opt(0, 0, 0).unwrap().timestamp()
}

fn from_timestamp(timestamp: i64) -> NaiveDate {
    NaiveDateTime::from_timestamp_opt(timestamp, 0)
        .unwrap()
        .date()
}
