use std::sync::Arc;

use shared_core::{
    asset::automation::{SceneAction, SceneTarget},
    backend::RunAction,
    device::{EndpointAction, EndpointTarget},
};
use tokio::sync::RwLock;

use crate::{node_connections::NodeConnections, controls::Controls};

pub struct AutomationActionRunner {
    pub connections: NodeConnections,
    pub controls: Arc<RwLock<Controls>>,
}

impl RunAction<EndpointTarget, EndpointAction> for AutomationActionRunner {
    async fn run_actions<I: IntoIterator<Item = EndpointAction>>(
        &mut self,
        target: EndpointTarget,
        actions: I,
    ) -> anyhow::Result<()>
    where
        I::IntoIter: Send + Sync,
        I: 'static + Send + Sync,
    {
        let mut connections = self.connections.clone();
        connections.run_actions(target, actions).await
    }
}

impl RunAction<SceneTarget, SceneAction> for AutomationActionRunner {
    async fn run_actions<I: IntoIterator<Item = SceneAction>>(
        &mut self,
        target: SceneTarget,
        actions: I,
    ) -> anyhow::Result<()>
    where
        I::IntoIter: Send + Sync,
        I: 'static + Send + Sync,
    {
        let mut controls = self.controls.write().await;
        controls.run_actions(target, actions).await
    }
}
