use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use shared_core::{device::device_registry::{DeviceConnectionStage, DeviceSubscriptionStatus}, event::DeviceStatusEvent, id::{AttrPath, ClusterItemPath, DeviceId, EventPath}};
use toasty::{Deferred, create, stmt::Uuid};

use crate::{Result, persist_db::PersistDb};

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

    /// TLV encoded value
    value: Vec<u8>,
}

#[derive(toasty::Model)]
#[key(event_id, timestamp)]
pub struct EventLog {
    event_id: Uuid,
    #[belongs_to]
    event: Deferred<ClusterItemId>,

    #[index]
    timestamp: Timestamp,
    /// TLV encoded event data
    value: Vec<u8>,
}

#[derive(toasty::Model)]
#[key(device_id, timestamp)]
pub struct DeviceStatusLog {
    device_id: DeviceId,
    #[index]
    timestamp: Timestamp,
    #[column(type = text)]
    status: toasty::Json<DeviceStatusLogEvent>
}

#[derive(Serialize, Deserialize)]
pub enum DeviceStatusLogEvent {
    Connecting {
        stage: DeviceConnectionStage,
    },
    Connected,
    SubscriptionStatus {
        status: DeviceSubscriptionStatus,
    },
}

impl<'a> From<&'a DeviceStatusEvent> for DeviceStatusLogEvent {
    fn from(value: &'a DeviceStatusEvent) -> Self {
        match value {
            DeviceStatusEvent::Connecting { stage } => Self::Connecting { stage: stage.clone() },
            DeviceStatusEvent::Connected { device: _ } => Self::Connected,
            DeviceStatusEvent::SubscriptionStatus { status } => Self::SubscriptionStatus { status: status.clone() },
        }
    }
}


impl PersistDb {
    async fn get_or_create_item_id(&self, item_path: ClusterItemPath) -> Result<Uuid> {
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
    pub async fn log_attribute_change(&self, attribute_path: AttrPath, value: Vec<u8>) -> Result<()> {
        let attribute_id = self.get_or_create_item_id(attribute_path.into()).await?;

        create!(AttributeChange {
            attribute_id,
            timestamp: Timestamp::now(),
            value
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
    ) -> Result<Vec<(Timestamp, Vec<u8>)>> {
        let attr_id = self.get_or_create_item_id(attribute_path.into()).await?;

        let result = AttributeChange::filter(
            AttributeChange::fields()
                .attribute_id()
                .eq(attr_id)
                .and(AttributeChange::fields().timestamp().ge(start))
                .and(AttributeChange::fields().timestamp().le(end)),
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
    pub async fn log_event(&self, event_path: EventPath, value: Vec<u8>) -> Result<()> {
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
        end: Timestamp,
    ) -> Result<Vec<(Timestamp, Vec<u8>)>> {
        let event_id = self.get_or_create_item_id(event_path.into()).await?;

        let result = EventLog::filter(
            EventLog::fields()
                .event_id()
                .eq(event_id)
                .and(EventLog::fields().timestamp().ge(start))
                .and(EventLog::fields().timestamp().le(end)),
        )
        .select((EventLog::fields().timestamp(), EventLog::fields().value()))
        .exec(&mut self.db.clone())
        .await?;

        Ok(result)
    }
}

impl PersistDb {
    pub async fn log_device_status(&self, device_id: DeviceId, status: &DeviceStatusEvent) -> Result<()> {
        create!(DeviceStatusLog {
            device_id,
            timestamp: Timestamp::now(),
            status: DeviceStatusLogEvent::from(status)
        })
        .exec(&mut self.db.clone())
        .await?;

        Ok(())
    }

    pub async fn query_device_status(
        &self,
        device_id: DeviceId,
        start: Timestamp,
        end: Timestamp,
    ) -> Result<Vec<(Timestamp, DeviceStatusLogEvent)>> {
        let result = EventLog::filter(
            DeviceStatusLog::fields()
                .device_id()
                .eq(device_id)
                .and(DeviceStatusLog::fields().timestamp().ge(start))
                .and(DeviceStatusLog::fields().timestamp().le(end)),
        )
        .select((DeviceStatusLog::fields().timestamp(), DeviceStatusLog::fields().status()))
        .exec(&mut self.db.clone())
        .await?;

        let result = result.into_iter().map(|(timestamp, status)| (timestamp, status.0)).collect();

        Ok(result)
    }
}


#[cfg(test)]
mod tests {

    use matter_clusters::{
        r#gen::electrical_power_measurement::{self, decode_active_power, encode_active_power},
        types::Nullable,
    };

    use super::*;

    #[tokio::test]
    async fn insert_and_query() {
        let db = PersistDb::new_memory().await.unwrap();

        let path = AttrPath {
            device: 10,
            endpoint: 1,
            cluster: electrical_power_measurement::CLUSTER_ID,
            attribute: electrical_power_measurement::attribute_id::ACTIVE_POWER,
        };

        db.log_attribute_change(path, encode_active_power(Nullable::Value(30)))
            .await
            .unwrap();
        db.log_attribute_change(path, encode_active_power(Nullable::Value(15)))
            .await
            .unwrap();

        let start = Timestamp::now();
        db.log_attribute_change(path, encode_active_power(Nullable::Value(8)))
            .await
            .unwrap();
        db.log_attribute_change(path, encode_active_power(Nullable::Value(20)))
            .await
            .unwrap();

        let end = Timestamp::now();

        let values = db.query_attribute_changes(path, start, end).await.unwrap();

        let expected_values = [Nullable::Value(8), Nullable::Value(20)];

        for ((_timestamp, value), expected_value) in values.into_iter().zip(expected_values) {
            let value = decode_active_power(&value).unwrap();
            assert_eq!(value, expected_value);
        }
    }
}
