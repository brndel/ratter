use jiff::{Timestamp, Zoned, civil::DateTime, tz::TimeZone};

pub struct EvalCtx<'a, Id, P> {
    pub date_time: DateTime,
    pub timestamp: Timestamp,
    pub timezone: TimeZone,
    pub provider: &'a P,
    pub action_trigger: Option<Id>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConditionEvalResult<T = Timestamp> {
    pub value: Option<bool>,
    pub next_change: Option<T>,
}

pub trait ValueProvider<Id> {
    fn get_bool(&self, id: &Id) -> Option<bool>;
    fn get_int(&self, id: &Id) -> Option<i64>;
}

pub trait DateEval {
    fn evaluate(&self, now: &DateTime) -> ConditionEvalResult<DateTime>;
}

pub trait Eval<Id> {
    fn evaluate(&mut self, ctx: &EvalCtx<Id, impl ValueProvider<Id>>) -> ConditionEvalResult;
}

pub trait EvalDependencies<Id> {
    fn dependencies(&self) -> Vec<Id>
    where
        Id: Clone,
    {
        let mut result = Vec::new();

        self.append_dependencies(&mut result);

        result
    }

    fn append_dependencies(&self, result: &mut Vec<Id>);
}


impl<'a, Id, P> EvalCtx<'a, Id, P> {
    pub fn new(zoned: Zoned, provider: &'a P) -> Self {
        Self {
            date_time: zoned.datetime(),
            timestamp: zoned.timestamp(),
            timezone: zoned.time_zone().clone(),
            provider,
            action_trigger: None
        }
    }

    pub fn now(provider: &'a P) -> Self {
        Self::new(Zoned::now(), provider)
    }

    pub fn with_trigger(mut self, trigger_id: Id) -> Self {
        self.action_trigger = Some(trigger_id);

        self
    }
}

impl<T: Ord> ConditionEvalResult<T> {
    pub fn merge_or(self, other: Self) -> Self {
        Self {
            value: match (self.value, other.value) {
                (None, None) => None,
                (None, Some(v)) | (Some(v), None) => Some(v),
                (Some(a), Some(b)) => Some(a || b),
            },
            next_change: match (self.next_change, other.next_change) {
                (None, None) => None,
                (None, Some(v)) | (Some(v), None) => Some(v),
                (Some(a), Some(b)) => Some(a.min(b)),
            },
        }
    }

    pub fn merge_and(self, other: Self) -> Self {
        Self {
            value: match (self.value, other.value) {
                (None, None) => None,
                (None, Some(_)) | (Some(_), None) => None,
                (Some(a), Some(b)) => Some(a && b),
            },
            next_change: Self::min_timestamp(self.next_change, other.next_change),
        }
    }

    pub fn min_timestamp(a: Option<T>, b: Option<T>) -> Option<T> {
        match (a, b) {
            (None, None) => None,
            (None, Some(v)) | (Some(v), None) => Some(v),
            (Some(a), Some(b)) => Some(a.min(b)),
        }
    }
}

impl ConditionEvalResult<DateTime> {
    pub fn to_timestamp(self, tz: TimeZone) -> ConditionEvalResult<Timestamp> {
        ConditionEvalResult {
            value: self.value,
            next_change: self
                .next_change
                .map(|dt| dt.to_zoned(tz).unwrap().timestamp()),
        }
    }
}
