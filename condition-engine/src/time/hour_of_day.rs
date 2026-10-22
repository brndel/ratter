use std::ops::Range;

use jiff::civil::{DateTime, Time};
use serde::{Deserialize, Serialize};

use crate::eval::{ConditionEvalResult, DateEval};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HourOfDay {
    range: Range<Time>,
}

impl HourOfDay {
    pub fn is_inverted(&self) -> bool {
        self.range.end < self.range.start
    }

    /// Returns earlier..later
    fn ordered_range(&self) -> Range<Time> {
        if self.is_inverted() {
            self.range.end..self.range.start
        } else {
            self.range.clone()
        }
    }
}

impl DateEval for HourOfDay {
    fn evaluate(&self, now: &DateTime) -> ConditionEvalResult<DateTime> {
        let value = self.ordered_range().contains(&now.time()) ^ self.is_inverted();

        let next_change = {
            let Range {
                start: earlier,
                end: later,
            } = self.ordered_range();

            let time = now.time();

            if time < earlier {
                now.with().time(earlier).build().ok()
            } else if time < later {
                now.with().time(later).build().ok()
            } else {
                now.tomorrow()
                    .ok()
                    .and_then(|dt| dt.with().time(earlier).build().ok())
            }
        };

        ConditionEvalResult {
            value: Some(value),
            next_change,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::eval::ConditionEvalResult;

    use super::*;

    fn work_time() -> HourOfDay {
        HourOfDay {
            range: Time::new(9, 0, 0, 0).unwrap()..Time::new(17, 0, 0, 0).unwrap(),
        }
    }

    fn sleep_time() -> HourOfDay {
        HourOfDay {
            range: Time::new(22, 0, 0, 0).unwrap()..Time::new(8, 0, 0, 0).unwrap(),
        }
    }

    #[test]
    fn work_inside() {
        let time = work_time();
        let now = DateTime::new(2004, 02, 12, 15, 0, 0, 0).unwrap();

        assert_eq!(
            time.evaluate(&now),
            ConditionEvalResult {
                value: Some(true),
                next_change: Some(DateTime::new(2004, 02, 12, 17, 0, 0, 0).unwrap())
            }
        )
    }

    #[test]
    fn work_outside() {
        let time = work_time();
        let now = DateTime::new(2004, 02, 12, 20, 0, 0, 0).unwrap();

        assert_eq!(
            time.evaluate(&now),
            ConditionEvalResult {
                value: Some(false),
                next_change: Some(DateTime::new(2004, 02, 13, 9, 0, 0, 0).unwrap())
            }
        )
    }

    #[test]
    fn sleep_inside_same_day() {
        let time = sleep_time();
        let now = DateTime::new(2004, 02, 12, 1, 0, 0, 0).unwrap();

        assert_eq!(
            time.evaluate(&now),
            ConditionEvalResult {
                value: Some(true),
                next_change: Some(DateTime::new(2004, 02, 12, 8, 0, 0, 0).unwrap())
            }
        )
    }

    #[test]
    fn sleep_inside_next_day() {
        let time = sleep_time();
        let now = DateTime::new(2004, 02, 12, 23, 0, 0, 0).unwrap();

        assert_eq!(
            time.evaluate(&now),
            ConditionEvalResult {
                value: Some(true),
                next_change: Some(DateTime::new(2004, 02, 13, 8, 0, 0, 0).unwrap())
            }
        )
    }

    #[test]
    fn sleep_outside() {
        let time = sleep_time();
        let now = DateTime::new(2004, 02, 12, 15, 0, 0, 0).unwrap();

        assert_eq!(
            time.evaluate(&now),
            ConditionEvalResult {
                value: Some(false),
                next_change: Some(DateTime::new(2004, 02, 12, 22, 0, 0, 0).unwrap())
            }
        )
    }
}
