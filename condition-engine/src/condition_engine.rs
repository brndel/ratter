use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use tokio::sync::mpsc::Sender;

use crate::{
    condition::{Condition, ConditionState, ToStated},
    eval::{ConditionEvalResult, Eval, EvalCtx, EvalDependencies, ValueProvider},
};

pub struct ConditionEngine<Provider, ConditionId, ExternalId> {
    conditions: HashMap<ConditionId, ConditionWrapper<ExternalId>>,
    dependencies: HashMap<ExternalId, HashSet<ConditionId>>,
    value_provider: Provider,
    tx: Sender<ConditionEvent<ConditionId>>,
}

pub struct ConditionEvent<Id> {
    pub id: Id,
    pub from: Option<bool>,
    pub to: Option<bool>,
}

struct ConditionWrapper<Id> {
    condition: ConditionState<Id>,
    dependencies: Vec<Id>,
    result: ConditionEvalResult,
}

impl<Provider, ConditionId, ExternalId> ConditionEngine<Provider, ConditionId, ExternalId>
where
    ConditionId: Clone + Hash + Eq,
    ExternalId: Clone + Hash + Eq,
    Provider: ValueProvider<ExternalId>,
{
    pub fn new(provider: Provider, tx: Sender<ConditionEvent<ConditionId>>) -> Self {
        Self {
            conditions: Default::default(),
            dependencies: Default::default(),
            value_provider: provider,
            tx,
        }
    }

    pub async fn add_condition(&mut self, id: ConditionId, condition: Condition<ExternalId>) {
        self.remove_condition(&id).await;

        let ctx = EvalCtx::now(&self.value_provider);
        let wrapper = ConditionWrapper::new(condition, &ctx);

        for dep in &wrapper.dependencies {
            let conditions = self.dependencies.entry(dep.clone()).or_default();
            conditions.insert(id.clone());
        }

        if wrapper.result.value != None {
            self.tx
                .send(ConditionEvent {
                    id: id.clone(),
                    from: None,
                    to: wrapper.result.value,
                })
                .await
                .unwrap();
        }

        self.conditions.insert(id, wrapper);
    }

    pub async fn remove_condition(&mut self, id: &ConditionId) {
        if let Some(condition) = self.conditions.remove(id) {
            if condition.result.value != None {
                self.tx
                    .send(ConditionEvent {
                        id: id.clone(),
                        from: condition.result.value,
                        to: None,
                    })
                    .await
                    .unwrap();
            }
            for dep in condition.dependencies {
                if let Some(deps) = self.dependencies.get_mut(&dep) {
                    deps.remove(id);
                }
            }
        }
    }

    pub async fn notify_external_change(&mut self, id: &ExternalId) {
        let Some(dependants) = self.dependencies.get(id) else {
            return;
        };

        let ctx = EvalCtx::now(&self.value_provider);

        for condition_id in dependants.clone() {
            if let Some(condition) = self.conditions.get_mut(&condition_id) {
                let old_value = condition.result.value;

                condition.update(&ctx);

                if old_value != condition.result.value {
                    self.tx
                        .send(ConditionEvent {
                            id: condition_id,
                            from: old_value,
                            to: condition.result.value,
                        })
                        .await
                        .unwrap();
                }
            };
        }
    }

    pub async fn call_trigger(&mut self, id: ExternalId) {
        let Some(dependants) = self.dependencies.get(&id) else {
            return;
        };

        let ctx = EvalCtx::now(&self.value_provider).with_trigger(id);

        for condition_id in dependants.clone() {
            if let Some(condition) = self.conditions.get_mut(&condition_id) {
                let old_value = condition.result.value;

                condition.update(&ctx);

                if old_value != condition.result.value {
                    self.tx
                        .send(ConditionEvent {
                            id: condition_id,
                            from: old_value,
                            to: condition.result.value,
                        })
                        .await
                        .unwrap();
                }
            };
        }
    }

    pub fn get_value(&self, id: &ConditionId) -> Option<bool> {
        self.conditions
            .get(id)
            .and_then(|condition| condition.result.value)
    }
}

impl<Id: PartialEq> ConditionWrapper<Id> {
    pub fn new(condition: Condition<Id>, ctx: &EvalCtx<Id, impl ValueProvider<Id>>) -> Self
    where
        Id: Clone,
    {
        let mut condition = condition.to_stated();
        let result = condition.evaluate(ctx);
        let dependencies = condition.dependencies();

        Self {
            condition,
            dependencies,
            result,
        }
    }

    pub fn update(&mut self, ctx: &EvalCtx<Id, impl ValueProvider<Id>>) {
        self.result = self.condition.evaluate(ctx);
    }
}
