use jiff::Timestamp;
use shared_core::id::{AttrPath, ClusterItemPath, EventPath};
use toasty::{Deferred, create, stmt::Uuid};

use crate::db::PersistDb;

#[derive(toasty::Model)]
pub struct ClusterItemId {
    #[key]
    #[auto]
    pub id: Uuid,
    pub item: ClusterItemPath,
    // pub device: DeviceId,
    // pub endpoint: EndpointId,
    // pub cluster: ClusterId,
    // pub item: u32,
    // pub item_is_attr: bool
}

#[derive(toasty::Model)]
#[key(attribute_id, timestamp)]
pub struct AttributeChange {
    attribute_id: Uuid,
    #[belongs_to]
    attribute: Deferred<ClusterItemId>,

    #[index]
    timestamp: Timestamp,

    value: i64,
}

#[derive(toasty::Model)]
#[key(event_id, timestamp)]
pub struct EventLog {
    event_id: Uuid,
    #[belongs_to]
    event: Deferred<ClusterItemId>,

    #[index]
    timestamp: Timestamp,
    value: Vec<u8>,
}

impl PersistDb {
    async fn get_or_create_item_id(&self, item_path: ClusterItemPath) -> anyhow::Result<Uuid> {
        if let Some(id) = self.value_id_cache.read().await.get(&item_path) {
            Ok(id.clone())
        } else {
            let mut db = self.db.clone();

            let id = {
                let id = ClusterItemId::filter(ClusterItemId::fields().item().eq(item_path))
                    .select(ClusterItemId::fields().id())
                    .first()
                    .exec(&mut db)
                    .await?;

                if let Some(id) = id {
                    id
                } else {
                    let result_attr = toasty::create!(ClusterItemId { item: item_path })
                        .exec(&mut db)
                        .await?;

                    result_attr.id
                }
            };

            self.value_id_cache.write().await.insert(item_path, id);

            Ok(id)
        }
    }
}

impl PersistDb {
    pub async fn store_attribute_change(
        &self,
        attribute_path: AttrPath,
        value: i64,
    ) -> anyhow::Result<()> {
        let attribute_id = self.get_or_create_item_id(attribute_path.into()).await?;

        create!(AttributeChange {
            attribute_id,
            timestamp: Timestamp::now(),
            value: value
        })
        .exec(&mut self.db.clone())
        .await?;

        Ok(())
    }

    pub async fn query_attribute_changes(
        &self,
        attribute_path: AttrPath,
        start: Timestamp,
        end: Timestamp,
    ) -> anyhow::Result<Vec<(Timestamp, i64)>> {
        let attr_id = self.get_or_create_item_id(attribute_path.into()).await?;

        let result = AttributeChange::filter(
            AttributeChange::fields()
                .attribute_id()
                .eq(attr_id)
                .and(AttributeChange::fields().timestamp().between(start, end)),
        )
        .select((
            AttributeChange::fields().timestamp(),
            AttributeChange::fields().value(),
        ))
        .exec(&mut self.db.clone())
        .await?;

        Ok(result)
    }
}

impl PersistDb {
    pub async fn log_event(
        &self,
        event_path: EventPath,
        value: Vec<u8>
    ) -> anyhow::Result<()> {
        let event_id = self.get_or_create_item_id(event_path.into()).await?;

        create!(EventLog {
            event_id,
            timestamp: Timestamp::now(),
            value: value
        })
        .exec(&mut self.db.clone())
        .await?;

        Ok(())
    }

    pub async fn query_events(
        &self,
        event_path: EventPath,
        start: Timestamp,
        end: Timestamp
    ) -> anyhow::Result<Vec<(Timestamp, Vec<u8>)>> {
        let event_id = self.get_or_create_item_id(event_path.into()).await?;

        let result = EventLog::filter(
            EventLog::fields()
                .event_id()
                .eq(event_id)
                .and(EventLog::fields().timestamp().between(start, end)),
        )
        .select((
            EventLog::fields().timestamp(),
            EventLog::fields().value(),
        ))
        .exec(&mut self.db.clone())
        .await?;

        Ok(result)
    }
}
