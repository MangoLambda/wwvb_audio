use chrono::{DateTime, Local, NaiveDate, Timelike};
use chrono::offset::TimeZone;

#[derive(Debug, PartialEq)]
pub enum Dst {
    NotInEffect,
    StartsToday,
    InEffect,
    EndsToday
}

pub fn get_dst(date_time: &DateTime<Local>) -> Dst {

    let winter_offset = get_local_timezone_winter_time_offset();
    let summer_offset = get_local_timezone_summer_time_offset();
     
     
    let is_dst_time_zone = winter_offset != summer_offset;
    if !is_dst_time_zone {
        return Dst::NotInEffect;
    }

    let start_of_day = date_time.with_hour(0).unwrap().with_minute(0).unwrap();
    let end_of_day = date_time.with_hour(23).unwrap().with_minute(0).unwrap();
    
    let start_of_day_offset = start_of_day.offset().local_minus_utc();
    let end_of_day_offset = end_of_day.offset().local_minus_utc();

    match (start_of_day_offset, end_of_day_offset) {
        (start, end) if start < end => return Dst::StartsToday,
        (start, end) if start > end => return Dst::EndsToday,
        (start, _) if start == summer_offset => return Dst::InEffect,
        _ => return Dst::NotInEffect,
    }
}

fn get_local_timezone_summer_time_offset() -> i32 {
    let naive_summer_date = NaiveDate::from_ymd_opt(2021, 6, 1).unwrap().and_hms_opt(2, 0, 0).unwrap();
    let local_summer_date: DateTime<Local> = Local.from_local_datetime(&naive_summer_date).unwrap();
    local_summer_date.offset().local_minus_utc()
}

fn get_local_timezone_winter_time_offset() -> i32 {
    let naive_winter_date = NaiveDate::from_ymd_opt(2021, 1, 1).unwrap().and_hms_opt(2, 0, 0).unwrap();
    let local_winter_date: DateTime<Local> = Local.from_local_datetime(&naive_winter_date).unwrap();
    local_winter_date.offset().local_minus_utc()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dst_not_in_effect() {
        let date_time = Local.with_ymd_and_hms(2021, 1, 15, 0, 0, 0).single().unwrap();
        assert_eq!(get_dst(&date_time), Dst::NotInEffect);
    }

    #[test]
    fn test_dst_in_effect() {
        let date_time = Local.with_ymd_and_hms(2021, 7, 15, 0, 0, 0).single().unwrap();
        assert_eq!(get_dst(&date_time), Dst::InEffect);
    }

    #[test]
    fn test_dst_starts_today() {
        let date_time = Local.with_ymd_and_hms(2021, 3, 14, 0, 0, 0).single().unwrap();
        assert_eq!(get_dst(&date_time), Dst::StartsToday);
    }

    #[test]
    fn test_dst_ends_today() {
        let date_time = Local.with_ymd_and_hms(2021, 11, 7, 0, 0, 0).single().unwrap();
        assert_eq!(get_dst(&date_time), Dst::EndsToday);
    }
}
