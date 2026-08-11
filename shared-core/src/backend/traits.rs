use matc::{controller::Connection, tlv::TlvItemValue};

use crate::{
    device::{AttrChange, clusters::ChangeEvent},
    id::EndpointId,
};

pub trait FromEndpoint: Sized {
    fn from_endpoint(
        connection: &Connection,
        endpoint: u16,
    ) -> impl Future<Output = anyhow::Result<Self>>;
}

pub trait FromAttr: Sized {
    fn from_attr(cluster: u32, attr: u32, value: &TlvItemValue) -> anyhow::Result<Self>;
}

pub trait ClusterState {
    const CLUSTER_ID: u32;
}

pub trait FromAttrChange: Sized + ChangeEvent
where
    <Self as ChangeEvent>::State: ClusterState,
{
    fn from_attr_change(attr: u32, value: &TlvItemValue) -> anyhow::Result<Self>;
}

pub trait RunAction<Target, Action> {
    fn run_actions<I: IntoIterator<Item = Action>>(
        &mut self,
        target: Target,
        actions: I,
    ) -> impl Future<Output = anyhow::Result<()>>
    where
        I::IntoIter: Send + Sync,
        I: 'static + Send + Sync;
}

pub trait RunClusterAction {
    type Cluster: ClusterState;

    fn run(
        self,
        connection: &Connection,
        endpoint: EndpointId,
    ) -> impl Future<Output = anyhow::Result<Vec<AttrChange>>>;
}

pub trait EnableDisableChangeAction {
    type Action;

    fn enable_action(&self) -> Vec<Self::Action>;
    fn disable_action(&self) -> Vec<Self::Action>;
    fn change_action(old: &Self, new: &Self) -> Vec<Self::Action>;
}

pub trait DiffAction {
    type Diff;

    fn diff_action(old: Option<&Self>, new: Option<&Self>) -> Self::Diff;
}

impl<T: EnableDisableChangeAction> DiffAction for T {
    type Diff = Vec<T::Action>;

    fn diff_action(old: Option<&Self>, new: Option<&Self>) -> Self::Diff {
        match (old, new) {
            (None, None) => Vec::new(),
            (None, Some(new)) => new.enable_action(),
            (Some(old), None) => old.disable_action(),
            (Some(old), Some(new)) => Self::change_action(old, new),
        }
    }
}

pub trait ControlActions {
    type Clusters<'a>;
    type Action;

    fn actions(cluster: &Self::Clusters<'_>, control: Option<&Self>) -> Vec<Self::Action>;
}
