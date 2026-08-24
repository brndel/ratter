mod connection_wrapper;

use std::{collections::BTreeMap, sync::Arc};

use dioxus::logger::tracing::{error, info};
use futures::Stream;
use matc::{
    clusters::codec::{
        acl_cluster::{self, read_access_control_entries_per_fabric}, admin_commissioning_cluster::{open_commissioning_window, read_window_status}, commissioner_control_cluster::decode_reverse_open_commissioning_window, joint_fabric_datastore_cluster::read_node_acl_list, operational_credential_cluster::{read_fabrics, remove_fabric},
    }, controller::Connection, devman::DeviceManager, messages::{pake1, parse_im_invoke_resp}, onboarding::{OnboardingInfo, encode_manual_pairing_code},
};
use matter_commissioning::{
    CommissioningFlow, DiscoveryCapabilities, Discriminator, Passcode, SetupPayload,
    encode_manual_code,
};
use rand::{Rng, RngExt, rand_core::utils::fill_bytes_via_next_word, rngs::ThreadRng};
use shared_core::{
    asset::asset_registry::AssetRegistry,
    attr_dump::{AttrDump, AttrDumpValue},
    backend::RunAction,
    device::{EndpointAction, EndpointTarget},
    event::{AttrChangeEvent, AttrChangeSource},
    id::{ClusterId, DeviceId, EndpointId},
};
use tokio::sync::{RwLock, mpsc};
use tokio_stream::wrappers::ReceiverStream;

use crate::{connections::connection_wrapper::ConnectionWrapper, event_bus::EventBusSender};

use anyhow::{Result, anyhow};

#[derive(Clone)]
pub struct Connections {
    bus_sender: EventBusSender,
    assets: Arc<RwLock<AssetRegistry>>,
    connections: Arc<RwLock<BTreeMap<u64, Arc<ConnectionWrapper>>>>,
}

impl Connections {
    pub fn new(bus_sender: EventBusSender, assets: Arc<RwLock<AssetRegistry>>) -> Self {
        Self {
            bus_sender,
            assets,
            connections: Default::default(),
        }
    }

    pub async fn connect_to_device(
        &self,
        device_manager: Arc<DeviceManager>,
        node_id: u64,
        force_reconnect: bool,
    ) -> bool {
        let mut connections = self.connections.write().await;
        let should_connect = force_reconnect || {
            let is_disconnected = if let Some(wrapper) = connections.get(&node_id) {
                wrapper.is_disconnected().await
            } else {
                true
            };

            is_disconnected
        };

        if should_connect {
            connections.insert(
                node_id,
                ConnectionWrapper::new(self.bus_sender.clone(), device_manager, node_id),
            );
            true
        } else {
            false
        }
    }

    pub async fn add_connection(&self, connection: Connection, node_id: u64) {
        let mut connections = self.connections.write().await;

        connections.insert(
            node_id,
            ConnectionWrapper::new_from_connection(connection, node_id, self.bus_sender.clone()),
        );
    }

    pub async fn open_commission_window(&self, device: DeviceId) -> Result<String> {

        
        let connection = self
        .get_connection(device)
        .await
        .ok_or(anyhow!("not connected"))?;

        let access_control = acl_cluster::read_acl(&connection, 0).await?;
        info!("access_control: {access_control:?}");

        let (passcode, salt, discriminator) = random_window_secrets()?;

        let discriminator = discriminator & 0x0F00;

        let info = OnboardingInfo {
            discriminator,
            passcode,
            is_short_discriminator: true,
            vendor_id: None,
            product_id: None,
            discovery_capabilities: None,
        };
        let iterations = 2000;

        // let mut salt = [0u8; 32];
        // fill_bytes_via_next_word(&mut salt, || Ok::<_, ()>(rng.next_u32())).unwrap();
        // let key = matc::controller::pin_to_passcode(info.passcode).map_err(|e| anyhow!("{e:?}"))?;
        // let verifier = matc::spake2p::Engine::create_passcode_verifier(&key, &salt, iterations);

        let verifier =
            matter_crypto::pake_passcode_verifier(info.passcode, &salt, iterations)?;

        // if verifier.as_slice() != matter_controller_verifier.as_ref() {
        //     info!("verifiers do not match!!");
        // }

        let payload = matc::clusters::codec::admin_commissioning_cluster::encode_open_commissioning_window(180, verifier.to_vec(), info.discriminator, iterations, salt.to_vec())?;
        info!("payload: {payload:?}");

        let result = connection.invoke_request_timed(0,
            matc::clusters::defs::CLUSTER_ID_ADMINISTRATOR_COMMISSIONING, matc::clusters::defs::CLUSTER_ADMINISTRATOR_COMMISSIONING_CMD_ID_OPENCOMMISSIONINGWINDOW, &payload, 6000).await?;

        let (tag, status) = parse_im_invoke_resp(&result.tlv)?;
            
        info!("open window result: {result:?}");
        info!("tag: {tag}, status: {status}");
        let status = read_window_status(&connection, 0).await?;
        info!("window status: {status:?}");


        // open_commissioning_window(
        //     &connection,
        //     0,
        //     200,
        //     verifier,
        //     info.discriminator,
        //     iterations,
        //     salt.to_vec(),
        // )
        // .await?;

        let code = encode_manual_pairing_code(&info);

        info!(
            "opened commissioning window on discriminator '{}', passcode '{}' and code '{}')",
            info.discriminator, info.passcode, code
        );

        Ok(code)
    }

    pub async fn forget_device(&self, device: DeviceId, cert: &[u8]) -> Result<()> {
        let connection = self
            .get_connection(device)
            .await
            .ok_or(anyhow!("not connected"))?;

        let fabrics = read_fabrics(&connection, 0).await?;

        let Some((idx, fabric)) = fabrics.into_iter().enumerate().find(|(_, fabric)| {
            fabric.node_id == Some(device)
                && fabric
                    .root_public_key
                    .as_ref()
                    .is_some_and(|key| key == cert)
        }) else {
            anyhow::bail!("device {device} has no matching fabric");
        };

        remove_fabric(&connection, 0, idx as u8).await?;

        self.connections.write().await.remove(&device);

        Ok(())
    }
}

impl Connections {
    async fn get_connection(&self, device: DeviceId) -> Option<Arc<Connection>> {
        let connections = self.connections.read().await;

        let connection_wrapper = connections.get(&device)?;

        let connection = connection_wrapper.connection().await?;

        Some(connection)
    }
}

impl RunAction<EndpointTarget, EndpointAction> for Connections {
    async fn run_actions<I: IntoIterator<Item = EndpointAction>>(
        &mut self,
        target: EndpointTarget,
        actions: I,
    ) -> anyhow::Result<()>
    where
        I::IntoIter: Send + Sync,
        I: 'static + Send + Sync,
    {
        let connection = self
            .get_connection(target.device)
            .await
            .ok_or_else(|| anyhow!("device not connected"))?;

        tokio::spawn({
            let bus_sender = self.bus_sender.clone();
            let connection = connection.clone();

            async move {
                for action in actions {
                    if let Ok(attr_changes) = action.run(&connection, target.endpoint).await {
                        for change in attr_changes {
                            bus_sender.send_attr_change(
                                target.device,
                                AttrChangeEvent {
                                    endpoint: target.endpoint,
                                    source: AttrChangeSource::User,
                                    change,
                                },
                            );
                        }
                    }
                }
            }
        });

        Ok(())
    }
}

impl Connections {
    pub async fn dump_all_attrs(
        &self,
        device: u64,
        include_root_endpoint: bool,
        skip_errors: bool,
    ) -> Option<impl Stream<Item = AttrDump> + use<>> {
        let connection = self.get_connection(device).await?;

        let (tx, rx) = mpsc::channel(16);

        tokio::spawn(async move {
            use matc::clusters::codec::*;

            async fn dump_cluster(
                tx: &mpsc::Sender<AttrDump>,
                conn: &Connection,
                endpoint: EndpointId,
                cluster: ClusterId,
                skip_errors: bool,
            ) -> Result<()> {
                info!("dump cluster {}, 0x{:x}", endpoint, cluster);
                let attrs = get_attribute_list(cluster);

                for (attr, attr_name) in attrs {
                    info!(
                        "dump attr {}, 0x{:x}, 0x{:x} ({})",
                        endpoint, cluster, attr, attr_name
                    );
                    let value = match conn.read_request2(endpoint, cluster, attr).await {
                        Ok(value) => Ok(decode_attribute_json(cluster, attr, &value)),
                        Err(err) => {
                            if skip_errors {
                                continue;
                            } else {
                                Err(format!("{err}"))
                            }
                        }
                    };
                    tx.send(AttrDump {
                        endpoint,
                        cluster,
                        attr,
                        value: AttrDumpValue {
                            attr_name: attr_name.to_owned(),
                            value,
                        },
                    })
                    .await?;
                }

                Ok(())
            }

            async fn dump_endpoint(
                tx: &mpsc::Sender<AttrDump>,
                conn: &Connection,
                endpoint: EndpointId,
                skip_errors: bool,
            ) -> Result<()> {
                let servers = descriptor_cluster::read_server_list(conn, endpoint).await?;

                for cluster in servers {
                    dump_cluster(&tx, conn, endpoint, cluster, skip_errors).await?;
                }

                let clients = descriptor_cluster::read_client_list(conn, endpoint).await?;

                for cluster in clients {
                    dump_cluster(&tx, conn, endpoint, cluster, skip_errors).await?;
                }

                Ok(())
            }

            async fn dump_connection(
                tx: &mpsc::Sender<AttrDump>,
                conn: &Connection,
                include_root_endpoint: bool,
                skip_errors: bool,
            ) -> Result<()> {
                let endpoints = descriptor_cluster::read_parts_list(conn, 0).await?;

                if include_root_endpoint {
                    dump_endpoint(&tx, conn, 0, skip_errors).await?;
                }

                for endpoint in endpoints {
                    dump_endpoint(&tx, conn, endpoint, skip_errors).await?;
                }

                Ok(())
            }

            if let Err(err) =
                dump_connection(&tx, &connection, include_root_endpoint, skip_errors).await
            {
                error!("Error while dumping attrs: {err}");
            }
        });

        Some(ReceiverStream::new(rx))
    }
}

#[cfg(test)]
mod tests {
    use matc::onboarding::decode_manual_pairing_code;
    use matter_commissioning::{encode_qr, parse_manual_code};

    use super::*;

    #[test]
    fn encode_decode_pairing_code() {
        // let (passcode, salt, discriminator) = random_window_secrets().unwrap();
        let (passcode, discriminator) = (20_202_021, 0x0100);

        let payload = SetupPayload {
            version: 0,
            vendor_id: None,
            product_id: None,
            commissioning_flow: CommissioningFlow::Standard,
            discovery_capabilities: DiscoveryCapabilities::ON_NETWORK,
            discriminator: Discriminator::new(discriminator).unwrap(),
            passcode: Passcode::new(passcode).unwrap(),
        };
        let code = encode_manual_code(&payload);
        println!("code: '{code}' with passcode '{passcode}' and disc: '{discriminator}'");
        let onboarding = parse_manual_code(&code).unwrap();

        let onboarding_matc = decode_manual_pairing_code(&code).unwrap();

        assert_eq!(
            payload.discriminator.as_u16(),
            onboarding.discriminator.as_u16()
        );
        assert_eq!(payload.passcode.as_u32(), onboarding.passcode.as_u32());
    }
}

/// Generate a valid `(passcode, salt, discriminator)` for an enhanced window.
///
/// Passcode is a fresh 27-bit value with the spec's trivial values excluded;
/// salt is 32 random bytes; discriminator is a random 12-bit value.
///
/// # Errors
/// Returns [`Error::Operational`] if the system RNG fails or no valid passcode
/// is found within the retry budget (practically never — ~12 values excluded).
pub(crate) fn random_window_secrets() -> Result<(u32, [u8; 32], u16), anyhow::Error> {
    use matter_commissioning::setup::Passcode;
    let rng = |buf: &mut [u8]| matter_crypto::random_bytes(buf).map_err(|e| anyhow!("rng: {e}"));
    let mut salt = [0u8; 32];
    rng(&mut salt)?;
    let mut db = [0u8; 2];
    rng(&mut db)?;
    let discriminator = u16::from_le_bytes(db) & 0x0FFF;
    // Passcode: draw 27-bit values until one is spec-valid (Passcode::new rejects
    // out-of-range and the disallowed-trivial set).
    for _ in 0..64 {
        let mut pb = [0u8; 4];
        rng(&mut pb)?;
        let candidate = u32::from_le_bytes(pb) & 0x07FF_FFFF; // 27-bit
        if Passcode::new(candidate).is_ok() {
            return Ok((candidate, salt, discriminator));
        }
    }
    Err(anyhow!("could not generate a valid passcode"))
}
