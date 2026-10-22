use std::time::Duration;

use jiff::Timestamp;

use crate::{
    eval::{
        ConditionEvalResult, DateEval, Eval, EvalCtx, EvalDependencies,
        ValueProvider,
    },
    time::DateTimeCondition,
};

pub enum Condition<Id> {
    Stateless(StatelessCondition<Id, Self>),
    Statefull(StatefullCondition<Id, Self>),
}

pub enum StatelessCondition<Id, Condition> {
    Time(DateTimeCondition),
    External(Id),
    Not(Box<Condition>),
    All { parts: Vec<Condition> },
    Any { parts: Vec<Condition> },
}

pub enum StatefullCondition<Id, Condition> {
    HoldValue(HoldValue<Condition>),
    ActionDuration(ActionDuration<Id>),
}

pub enum ConditionState<Id> {
    Stateless(StatelessCondition<Id, Self>),
    Statefull(StatefullConditionState<Id, Self>),
}

pub enum StatefullConditionState<Id, Condition> {
    HoldValue {
        hold_value: HoldValue<Condition>,
        change_away_from_hold_value: Option<Timestamp>,
    },
    ActionDuration {
        action: ActionDuration<Id>,
        last_trigger: Option<Timestamp>,
    },
}

pub struct HoldValue<Condition> {
    condition: Box<Condition>,
    duration: Duration,
    hold_value: Option<bool>,
}

pub struct ActionDuration<Id> {
    action: Id,
    duration: Duration,
}

impl<Id, Condition> Eval<Id> for StatelessCondition<Id, Condition>
where
    Condition: Eval<Id>,
{
    fn evaluate(&mut self, ctx: &EvalCtx<Id, impl ValueProvider<Id>>) -> ConditionEvalResult {
        match self {
            StatelessCondition::Time(time_condition) => time_condition
                .evaluate(&ctx.date_time)
                .to_timestamp(ctx.timezone.clone()),
            StatelessCondition::External(id) => ConditionEvalResult {
                value: ctx.provider.get_bool(id),
                next_change: None,
            },
            StatelessCondition::Not(condition) => {
                let mut result = condition.evaluate(ctx);
                result.value = result.value.map(|value| !value);
                result
            }
            StatelessCondition::All { parts } => parts.iter_mut().fold(
                ConditionEvalResult {
                    value: None,
                    next_change: None,
                },
                |acc, v| acc.merge_and(v.evaluate(ctx)),
            ),
            StatelessCondition::Any { parts } => parts.iter_mut().fold(
                ConditionEvalResult {
                    value: None,
                    next_change: None,
                },
                |acc, v| acc.merge_or(v.evaluate(ctx)),
            ),
        }
    }
}

impl<Id, Condition> EvalDependencies<Id> for StatelessCondition<Id, Condition>
where
    Id: Clone,
    Condition: EvalDependencies<Id>,
{
    fn append_dependencies(&self, result: &mut Vec<Id>) {
        match self {
            StatelessCondition::Time(_) => (),
            StatelessCondition::External(id) => result.push(id.clone()),
            StatelessCondition::Not(condition) => condition.append_dependencies(result),
            StatelessCondition::All { parts } | StatelessCondition::Any { parts } => {
                for condition in parts {
                    condition.append_dependencies(result);
                }
            }
        }
    }
}

impl<Id, Condition> EvalDependencies<Id> for StatefullConditionState<Id, Condition>
where
    Id: Clone,
    Condition: EvalDependencies<Id>,
{
    fn append_dependencies(&self, result: &mut Vec<Id>) {
        match self {
            StatefullConditionState::HoldValue { hold_value, .. } => {
                hold_value.condition.append_dependencies(result)
            }
            StatefullConditionState::ActionDuration { action, .. } => {
                result.push(action.action.clone())
            }
        }
    }
}

impl<Id, Condition> Eval<Id> for StatefullConditionState<Id, Condition>
where
    Condition: Eval<Id>,
    Id: PartialEq
{
    fn evaluate(&mut self, ctx: &EvalCtx<Id, impl ValueProvider<Id>>) -> ConditionEvalResult {
        match self {
            StatefullConditionState::HoldValue {
                hold_value:
                    HoldValue {
                        condition,
                        duration,
                        hold_value,
                    },
                change_away_from_hold_value,
            } => {
                let new_value = condition.evaluate(ctx);

                if new_value.value == *hold_value {
                    *change_away_from_hold_value = None;
                } else if change_away_from_hold_value.is_none() {
                    *change_away_from_hold_value = Some(ctx.timestamp);
                }

                match *change_away_from_hold_value {
                    Some(timestamp) => {
                        let hold_duration_end = timestamp + *duration;
                        let is_in_hold_range = ctx.timestamp < hold_duration_end;
                        let value = if is_in_hold_range {
                            *hold_value
                        } else {
                            new_value.value
                        };
                        let next_change = is_in_hold_range.then_some(hold_duration_end);

                        ConditionEvalResult {
                            value,
                            next_change: ConditionEvalResult::min_timestamp(
                                next_change,
                                new_value.next_change,
                            ),
                        }
                    }
                    None => ConditionEvalResult {
                        value: *hold_value,
                        next_change: new_value.next_change,
                    },
                }
            }
            StatefullConditionState::ActionDuration {
                action:
                    ActionDuration {
                        action,
                        duration,
                    },
                last_trigger,
            } => {
                if Some(&*action) == ctx.action_trigger.as_ref() {
                    *last_trigger = Some(ctx.timestamp);
                }

                let Some(last_trigger) = *last_trigger else {
                    return ConditionEvalResult {
                        value: Some(false),
                        next_change: None,
                    };
                };

                let trigger_end = last_trigger + *duration;
                let is_in_trigger_range = ctx.timestamp < trigger_end;
                let next_change = is_in_trigger_range.then_some(trigger_end);

                ConditionEvalResult {
                    value: Some(is_in_trigger_range),
                    next_change,
                }
            }
        }
    }
}

impl<Id> Eval<Id> for ConditionState<Id> where Id: PartialEq
{
    fn evaluate(&mut self, ctx: &EvalCtx<Id, impl ValueProvider<Id>>) -> ConditionEvalResult {
        match self {
            ConditionState::Stateless(condition) => condition.evaluate(ctx),
            ConditionState::Statefull(condition) => condition.evaluate(ctx),
        }
    }
}

impl<Id> EvalDependencies<Id> for ConditionState<Id>
where
    Id: Clone,
{
    fn append_dependencies(&self, result: &mut Vec<Id>) {
        match self {
            ConditionState::Stateless(condition) => condition.append_dependencies(result),
            ConditionState::Statefull(condition) => condition.append_dependencies(result),
        }
    }
}

pub trait ToStated<Stated> {
    fn to_stated(self) -> Stated;
}

impl<Id> ToStated<ConditionState<Id>> for Condition<Id> {
    fn to_stated(self) -> ConditionState<Id> {
        match self {
            Condition::Stateless(condition) => ConditionState::Stateless(condition.to_stated()),
            Condition::Statefull(condition) => ConditionState::Statefull(condition.to_stated()),
        }
    }
}

impl<Id, C1, C2> ToStated<StatefullConditionState<Id, C2>> for StatefullCondition<Id, C1>
where
    C1: ToStated<C2>,
{
    fn to_stated(self) -> StatefullConditionState<Id, C2> {
        match self {
            StatefullCondition::HoldValue(hold_value) => StatefullConditionState::HoldValue {
                hold_value: hold_value.to_stated(),
                change_away_from_hold_value: None,
            },
            StatefullCondition::ActionDuration(action) => StatefullConditionState::ActionDuration {
                action,
                last_trigger: None,
            },
        }
    }
}

impl<Id, C1, C2> ToStated<StatelessCondition<Id, C2>> for StatelessCondition<Id, C1>
where
    C1: ToStated<C2>,
{
    fn to_stated(self) -> StatelessCondition<Id, C2> {
        match self {
            StatelessCondition::Time(date_time_condition) => {
                StatelessCondition::Time(date_time_condition)
            }
            StatelessCondition::External(id) => StatelessCondition::External(id),
            StatelessCondition::Not(condition) => StatelessCondition::Not(condition.to_stated()),
            StatelessCondition::All { parts } => StatelessCondition::All {
                parts: parts
                    .into_iter()
                    .map(|condition| condition.to_stated())
                    .collect(),
            },
            StatelessCondition::Any { parts } => StatelessCondition::Any {
                parts: parts
                    .into_iter()
                    .map(|condition| condition.to_stated())
                    .collect(),
            },
        }
    }
}

impl<C1, C2> ToStated<HoldValue<C2>> for HoldValue<C1>
where
    C1: ToStated<C2>,
{
    fn to_stated(self) -> HoldValue<C2> {
        HoldValue {
            condition: self.condition.to_stated(),
            duration: self.duration,
            hold_value: self.hold_value,
        }
    }
}

impl<C1, C2> ToStated<Box<C2>> for Box<C1>
where
    C1: ToStated<C2>,
{
    fn to_stated(self) -> Box<C2> {
        Box::new((*self).to_stated())
    }
}
