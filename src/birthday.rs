use chrono::{Datelike, NaiveDate};

#[derive(Clone)]
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

    pub fn is_today(&self, today: NaiveDate) -> bool {
        self.date.month() == today.month() && self.date.day() == today.day()
    }
}

#[cfg(test)]
mod tests {
    use crate::Birthday;
    use chrono::NaiveDate;

    #[test]
    fn test_age() {
        let birthday = Birthday {
            id: 1,
            name: "Ben Dover".to_string(),
            date: NaiveDate::from_ymd_opt(1990, 5, 3).unwrap(),
        };
        let today = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();

        assert_eq!(birthday.age(today), Some(33))
    }

    #[test]
    fn test_next_birthday_is_this_year() {
        let birthday = Birthday {
            id: 1,
            name: "Ben Dover".to_string(),
            date: NaiveDate::from_ymd_opt(1990, 5, 3).unwrap(),
        };
        let today = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();

        assert_eq!(
            birthday.next(today),
            NaiveDate::from_ymd_opt(2024, 5, 3).unwrap()
        )
    }

    #[test]
    fn test_next_birthday_is_next_year() {
        let birthday = Birthday {
            id: 1,
            name: "Ben Dover".to_string(),
            date: NaiveDate::from_ymd_opt(1990, 5, 3).unwrap(),
        };
        let today = NaiveDate::from_ymd_opt(2024, 6, 1).unwrap();

        assert_eq!(
            birthday.next(today),
            NaiveDate::from_ymd_opt(2025, 5, 3).unwrap()
        )
    }

    #[test]
    fn test_is_today() {
        let birthday = Birthday {
            id: 1,
            name: "Ben Dover".to_string(),
            date: NaiveDate::from_ymd_opt(1990, 5, 3).unwrap(),
        };

        assert!(birthday.is_today(NaiveDate::from_ymd_opt(2024, 5, 3).unwrap()));

        assert!(!birthday.is_today(NaiveDate::from_ymd_opt(2024, 5, 4).unwrap()));

        assert!(!birthday.is_today(NaiveDate::from_ymd_opt(2024, 6, 3).unwrap()));
    }
}
