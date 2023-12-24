use anyhow::Result;
use chrono::{NaiveDate, NaiveDateTime};
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
    pub name: String,
    pub date: NaiveDate,
}

impl Birthday {
    pub fn age(&self, today: NaiveDate) -> Option<u32> {
        today.years_since(self.date)
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
    let mut statement = db.prepare("SELECT name, date_timestamp FROM birthdays")?;
    let birthday_iter = statement.query_map([], |row| {
        let name = row.get(0)?;
        let timestamp = row.get(1)?;
        let date = from_timestamp(timestamp);
        Ok(Birthday { name, date })
    })?;
    let birthdays: Result<Vec<Birthday>, rusqlite::Error> = birthday_iter.collect();
    Ok(birthdays.unwrap())
}
