use std::ops::Range;

use jiff::Timestamp;

pub fn timestamp_avg(values: impl Iterator<Item = (i64, Timestamp)>, range: Range<Timestamp>, start_value: Option<i64>) -> Option<i64> {
    weight_avg(TimestampIter {
        time_range: range,
        start_value,
        inner: values,
        current_value: None,
    })
}

struct TimestampIter<T: Iterator> {
    time_range: Range<Timestamp>,
    start_value: Option<i64>,
    inner: T,
    current_value: Option<T::Item>
}

impl<T> Iterator for TimestampIter<T> where T: Iterator<Item = (i64, Timestamp)> {
    type Item = (i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        let next_value = self.inner.next();

        let next_value = if self.current_value.is_none() { // if true, this is the first element of the iter

            if next_value.is_none() { // if true, the iterator is empty and we dont want to emit any values
                return None;
            }

            if let Some(start_value) = self.start_value {
                self.current_value = Some((start_value, self.time_range.start));

                next_value
            } else { // There is no explicit start value provided, we just skip it
                self.current_value = next_value;

                self.inner.next()
            }

        } else {
            next_value
        };

        let next_timestamp = if let Some((_, timestamp)) = next_value {
            timestamp
        } else {
            self.time_range.end
        };

        let current_value = self.current_value;
        self.current_value = next_value;

        if let Some((current_value, current_timestamp)) = current_value {
            let duration_secs = current_timestamp.duration_until(next_timestamp).as_secs();

            Some((current_value, duration_secs))
        } else {
            None
        }
    }
}


/// values iterator has shape (value, weight)
pub fn weight_avg(values: impl Iterator<Item = (i64, i64)>) -> Option<i64> {
    let mut sum = 0;
    let mut count = 0;

    for (value, weight) in values {
        println!("value {value} with weight {weight}");
        sum += value * weight;
        count += weight;
    }

    sum.checked_div(count)
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

use super::*;

    #[test]
    fn simple() {
        let start = Timestamp::UNIX_EPOCH;

        let t1 = start + Duration::from_secs(10);
        let t2 = start + Duration::from_secs(20);
        let t3 = start + Duration::from_secs(30);

        let end = start + Duration::from_secs(40);

        let value = timestamp_avg([(100, t1), (200, t2), (300, t3)].into_iter(), start..end, None);

        assert_eq!(value, Some(200));
    }


    #[test]
    fn start_value() {
        let start = Timestamp::UNIX_EPOCH;

        let t1 = start + Duration::from_secs(10);
        let t2 = start + Duration::from_secs(20);
        let t3 = start + Duration::from_secs(30);

        let end = start + Duration::from_secs(40);

        let value = timestamp_avg([(100, t1), (200, t2), (300, t3)].into_iter(), start..end, Some(1000));

        assert_eq!(value, Some(400));
    }
}
