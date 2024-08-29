use chrono::{NaiveTime, Timelike, Weekday};

pub fn get_week_schedule_time_index(wd: Weekday, time: NaiveTime) -> u32 {

    let w = wd as u32 * 1000000;
    let h = time.hour() * 10000;
    let m = time.minute() * 100;
    let s = time.second() * 1;

    w + h + m + s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_week_schedule_time_index() {
        let wd = Weekday::Mon;
        let time = NaiveTime::from_hms(12, 0, 0);
        assert_eq!(get_week_schedule_time_index(wd, time), 120000);
    }

    #[test]
    fn test_day_value() {
        let mon = get_week_schedule_time_index(Weekday::Mon, NaiveTime::from_hms(0, 0, 0));
        let tue = get_week_schedule_time_index(Weekday::Tue, NaiveTime::from_hms(0, 0, 0));

        println!("mon: {}, tue: {}", mon, tue);

        assert_eq!(mon < tue, true);
    }

    #[test]
    fn test_h_value() {
        let mon1 = get_week_schedule_time_index(Weekday::Mon, NaiveTime::from_hms(0, 0, 59));
        let mon2 = get_week_schedule_time_index(Weekday::Mon, NaiveTime::from_hms(1, 0, 0));
        let mon3 = get_week_schedule_time_index(Weekday::Mon, NaiveTime::from_hms(9, 0, 0));
        let mon4 = get_week_schedule_time_index(Weekday::Mon, NaiveTime::from_hms(23, 0, 0));

        let result = mon1 < mon2 && mon2 < mon3 && mon3 < mon4;

        assert_eq!(result, true);
    }

    #[test]
    fn test_get_week_schedule_time_index_sun() {
        let wd = Weekday::Sun;
        let time = NaiveTime::from_hms(23, 59, 59);
        assert_eq!(get_week_schedule_time_index(wd, time), 6235959);
    }

    #[test]
    fn test_get_week_schedule_time_index_sun2() {
        let wd = Weekday::Sun;
        let time = NaiveTime::from_hms(1, 59, 59);
        assert_eq!(get_week_schedule_time_index(wd, time), 6015959);
    }
}
