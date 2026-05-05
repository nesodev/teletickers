use chrono::{DateTime, Duration, NaiveDate, Utc};

pub fn is_past_date(date: NaiveDate) -> bool {
    let today = chrono::Local::now().naive_local().date();
    date < today
}

pub fn is_future_date(date: NaiveDate) -> bool {
    let today = chrono::Local::now().naive_local().date();
    date > today
}

pub fn days_until(date: NaiveDate) -> i64 {
    let today = chrono::Local::now().naive_local().date();
    (date - today).num_days()
}

pub fn add_days(date: DateTime<Utc>, days: i64) -> DateTime<Utc> {
    date + Duration::days(days)
}

pub fn format_datetime(dt: DateTime<Utc>) -> String {
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn format_date(date: NaiveDate) -> String {
    date.format("%Y-%m-%d").to_string()
}
