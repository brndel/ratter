mod day_of_week;
mod hour_of_day;

use jiff::civil::DateTime;

use crate::eval::{ConditionEvalResult, DateEval};

pub use day_of_week::*;
pub use hour_of_day::*;

pub enum DateTimeCondition {
    HourOfDay(HourOfDay),
    DayOfWeek(DayOfWeek),
}

impl DateEval for DateTimeCondition {
    fn evaluate(&self, now: &DateTime) -> ConditionEvalResult<DateTime> {
        match self {
            DateTimeCondition::HourOfDay(hour_of_day) => hour_of_day.evaluate(now),
            DateTimeCondition::DayOfWeek(day_of_week) => day_of_week.evaluate(now),
        }
    }
}
