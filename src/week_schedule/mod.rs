use std::collections::BTreeMap;

use crate::get_week_schedule_time_index;
use chrono::{NaiveTime, Weekday};
use std::ops::Bound::Excluded;
use std::ops::Bound::Included;

#[derive(Debug, Clone)]
pub struct WeekSchedule<T> {
    pub events: BTreeMap<u32, Vec<T>>,
}

impl<T: Clone> Default for WeekSchedule<T> {
    fn default() -> Self {
        Self {
            events: BTreeMap::new(),
        }
    }
}

impl<T: Clone> WeekSchedule<T> {
    pub fn register_event(&mut self, evnt: T, week: Weekday, time: NaiveTime) {
        let indx = get_week_schedule_time_index(week, time);

        self.events.entry(indx).or_default().push(evnt)
    }

    pub fn query(
        &self,
        from_week: Weekday,
        from_time: NaiveTime,
        to_week: Weekday,
        to_time: NaiveTime,
    ) -> Vec<T> {
        if self.events.is_empty() {
            return vec![];
        }

        let from_index = get_week_schedule_time_index(from_week, from_time);
        let to_index = get_week_schedule_time_index(to_week, to_time);

        print!(
            "from_index: {}, to_index_index: {}. Is: {}",
            from_index,
            to_index,
            from_index < to_index
        );

        if from_index < to_index {
            return self
                .events
                .range((Excluded(from_index), Included(to_index)))
                .flat_map(|(_, src)| src)
                .cloned()
                .collect::<Vec<_>>();
        }

        let last_indx = *self.events.last_key_value().unwrap().0;
        let first_indx = *self.events.first_key_value().unwrap().0;

        let mut from_2_last = self
            .events
            .range((Excluded(from_index), Included(last_indx)))
            .flat_map(|(_, src)| src)
            .cloned()
            .collect::<Vec<_>>();

        let first_2_to = self
            .events
            .range((Included(first_indx), Included(to_index)))
            .flat_map(|(_, src)| src)
            .cloned()
            .collect::<Vec<_>>();

        from_2_last.extend(first_2_to);

        from_2_last
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::NaiveTime;
    use chrono::Weekday;

    #[test]
    fn test_week_schedule_s_2_b() {
        let mut week_schedule = WeekSchedule::default();

        week_schedule.register_event("Event 1", Weekday::Mon, NaiveTime::from_hms(9, 0, 0));
        week_schedule.register_event("Event 2", Weekday::Tue, NaiveTime::from_hms(9, 0, 0));
        week_schedule.register_event("Event 3", Weekday::Wed, NaiveTime::from_hms(9, 0, 0));
        week_schedule.register_event("Event 4", Weekday::Thu, NaiveTime::from_hms(9, 0, 0));
        week_schedule.register_event("Event 5", Weekday::Fri, NaiveTime::from_hms(9, 0, 0));
        week_schedule.register_event("Event 6", Weekday::Sat, NaiveTime::from_hms(9, 0, 0));
        week_schedule.register_event("Event 7", Weekday::Sun, NaiveTime::from_hms(9, 0, 0));

        let events = week_schedule.query(
            Weekday::Mon,
            NaiveTime::from_hms(9, 0, 0),
            Weekday::Fri,
            NaiveTime::from_hms(9, 0, 0),
        );

        assert_eq!(
            events,
            vec!["Event 2", "Event 3", "Event 4", "Event 5"]
        );
    }

    #[test]
    fn test_week_schedule_b_2_s() {
        let mut week_schedule = WeekSchedule::default();

        week_schedule.register_event("Event 1", Weekday::Mon, NaiveTime::from_hms(9, 0, 0));
        week_schedule.register_event("Event 2", Weekday::Tue, NaiveTime::from_hms(9, 0, 0));
        week_schedule.register_event("Event 3", Weekday::Wed, NaiveTime::from_hms(9, 0, 0));
        week_schedule.register_event("Event 4", Weekday::Thu, NaiveTime::from_hms(9, 0, 0));
        week_schedule.register_event("Event 5", Weekday::Fri, NaiveTime::from_hms(9, 0, 0));
        week_schedule.register_event("Event 6", Weekday::Sat, NaiveTime::from_hms(9, 0, 0));
        week_schedule.register_event("Event 7", Weekday::Sun, NaiveTime::from_hms(9, 0, 0));

        let events = week_schedule.query(
            Weekday::Fri,
            NaiveTime::from_hms(9, 0, 0),
            Weekday::Tue,
            NaiveTime::from_hms(9, 0, 0),
        );

        assert_eq!(events, vec!["Event 6", "Event 7", "Event 1", "Event 2"]);
    }
}