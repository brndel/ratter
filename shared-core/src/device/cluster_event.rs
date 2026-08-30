use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, derive_more::From)]
#[serde(rename_all = "snake_case")]
pub enum ClusterEvent {
    Button(ButtonClusterEvent),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ButtonClusterEvent {
    Press { count: u8 },
    LongPressStarted,
    LongPressEnded,
}

#[cfg(feature = "backend")]
mod backend_impl {
    use super::*;

    use crate::id::{ClusterId, EventId};
    use matter_clusters::r#gen::switch;
    use matter_codec::Value;

    impl ClusterEvent {
        pub fn from_event(cluster: ClusterId, event_id: EventId, value: &Value) -> Option<Self> {
            match cluster {
                switch::CLUSTER_ID => {
                    ButtonClusterEvent::from_event(event_id, value).map(Self::from)
                }
                _ => None,
            }
        }
    }
    impl ButtonClusterEvent {
        pub fn from_event(event_id: EventId, value: &Value) -> Option<Self> {
            let mut tlv_bytes = Vec::new();
            let mut writer = matter_codec::TlvWriter::new(&mut tlv_bytes);
            writer
                .write_value(matter_codec::Tag::Anonymous, &value)
                .expect("writing to vec should not fail");

            match event_id {
                switch::event_id::SHORT_RELEASE => {
                    // let _event = switch::ShortReleaseEvent::decode(&tlv_bytes).ok()?;

                    // Some(Self::Press)

                    // doing a double press will emit SHORT_RELEASE 2 times in short succession, which can send control events to devices too fast, which results in packet loss
                    // MULTI_PRESS_COMPLETE does also gets emmitted for single press, so its the best choice in this case
                    None
                }
                switch::event_id::MULTI_PRESS_COMPLETE => {
                    let event = switch::MultiPressCompleteEvent::decode(&tlv_bytes).ok()?;

                    Some(Self::Press {
                        count: event.total_number_of_presses_counted,
                    })
                }
                switch::event_id::LONG_PRESS => {
                    let _event = switch::LongPressEvent::decode(&tlv_bytes).ok()?;

                    Some(Self::LongPressStarted)
                }
                switch::event_id::LONG_RELEASE => {
                    let _event = switch::LongReleaseEvent::decode(&tlv_bytes).ok()?;

                    Some(Self::LongPressEnded)
                }
                _ => None,
            }
        }
    }
}
