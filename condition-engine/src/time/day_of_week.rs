use jiff::civil::{DateTime, Weekday};
use serde::{Deserialize, Serialize, de::Error};

use crate::eval::{ConditionEvalResult, DateEval};

#[derive(Debug, Clone)]
pub struct DayOfWeek {
    pub weekday: Weekday,
}

impl Serialize for DayOfWeek {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.weekday.to_monday_zero_offset().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for DayOfWeek {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self {
            weekday: Weekday::from_monday_zero_offset(i8::deserialize(deserializer)?)
                .map_err(D::Error::custom)?,
        })
    }
}

impl DateEval for DayOfWeek {
    fn evaluate(&self, now: &DateTime) -> ConditionEvalResult<DateTime> {
        let value = now.weekday() == self.weekday;

        let next_change = {
            let day = if self.weekday == now.weekday() {
                now.tomorrow().ok()
            } else {
                now.nth_weekday(1, self.weekday).ok()
            };

            day.map(|day| day.start_of_day())
        };

        ConditionEvalResult {
            value: Some(value),
            next_change,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn monday_condition() -> DayOfWeek {
        DayOfWeek {
            weekday: Weekday::Monday,
        }
    }

    mod monday {
        use super::*;

        fn now() -> DateTime {
            DateTime::new(2026, 10, 5, 10, 0, 0, 0).unwrap()
        }

        #[test]
        fn eval() {
            let condition = monday_condition();

            let now = now();
            let tuesday_start = DateTime::new(2026, 10, 6, 0, 0, 0, 0).unwrap();

            assert_eq!(
                condition.evaluate(&now),
                ConditionEvalResult {
                    value: Some(true),
                    next_change: Some(tuesday_start)
                }
            )
        }

        #[test]
        fn eval_on_next_change() {
            let condition = monday_condition();

            let now = now();

            let next_change = condition.evaluate(&now).next_change.unwrap();

            assert_eq!(
                condition.evaluate(&next_change),
                ConditionEvalResult {
                    value: Some(false),
                    next_change: Some(DateTime::new(2026, 10, 12, 0, 0, 0, 0).unwrap())
                }
            )
        }
    }

    mod friday {
        use super::*;

        fn now() -> DateTime {
            DateTime::new(2026, 10, 2, 10, 0, 0, 0).unwrap()
        }

        #[test]
        fn eval() {
            let condition = monday_condition();

            let now = now();
            let next_monday_start = DateTime::new(2026, 10, 5, 0, 0, 0, 0).unwrap();

            assert_eq!(
                condition.evaluate(&now),
                ConditionEvalResult {
                    value: Some(false),
                    next_change: Some(next_monday_start)
                }
            )
        }

        #[test]
        fn eval_on_next_change() {
            let condition = monday_condition();

            let now = now();
            let next_tuesday_start = DateTime::new(2026, 10, 6, 0, 0, 0, 0).unwrap();

            let next_change = condition.evaluate(&now).next_change.unwrap();

            assert_eq!(
                condition.evaluate(&next_change),
                ConditionEvalResult {
                    value: Some(true),
                    next_change: Some(next_tuesday_start)
                }
            )
        }
    }
}
