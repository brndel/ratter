

/// Demo application for DeviceManager - simplified Matter device interaction.
///
/// Usage:
///   # First-time setup (creates CA, controller certs, config):
///   cargo run --example devman_demo -- init ./matter-data --fabric-id 1000 --controller-id 100
///
///   # Commission a device:
///   cargo run --example devman_demo -- -d ./matter-data commission 192.168.1.100:5540 300 123456 "kitchen light"
///
///   # Commission a device using manual pairing code and discovery:
///   cargo run --example devman_demo -- -d ./matter-data commission-with-discovery "0251-520-0076" 300 "kitchen light"
///
///   # Commission a Wi-Fi device over BLE (requires --features ble):
///   cargo run --features ble --example devman_demo -- -d ./matter-data commission-ble \
///     "MT:Y.K908..." 300 "kitchen light" HomeWifi --password "secret"
///
///   # Commission a Thread device over BLE (requires --features ble):
///   cargo run --features ble --example devman_demo -- -d ./matter-data commission-ble-thread \
///     "MT:Y.K908..." 300 "kitchen sensor" 0e080000000000010000000300001235...
///
///   # Scan for BLE commissionable devices (requires --features ble):
///   cargo run --features ble --example devman_demo -- scan-ble --timeout-secs 5
///
///   # Discover commissionable Matter devices via mDNS:
///   cargo run --example devman_demo -- -d ./matter-data discover-commissionable --timeout-secs 5
///
///   # List registered devices:
///   cargo run --example devman_demo -- -d ./matter-data list
///
///   # Send ON/OFF commands:
///   cargo run --example devman_demo -- -d ./matter-data on "kitchen light"
///   cargo run --example devman_demo -- -d ./matter-data off "kitchen light"
///
///   # List all attributes on all endpoints:
///   cargo run --example devman_demo -- -d ./matter-data list-attributes "kitchen light"
use anyhow::{Error, Result};
use clap::{Parser, Subcommand};
#[cfg(feature = "ble")]
use matc::{NetworkCreds, ble};
use matc::{
    clusters::{
        self, codec::{color_control::{self, UpdateFlags}, commissioner_control_cluster::commission_node, descriptor_cluster, level_control, on_off},
    }, controller::{self, Connection}, devman::{DeviceManager, ManagerConfig}, tlv::{self, TlvItem, TlvItemValue},
};
use tokio::sync::Mutex;
use std::{collections::HashMap, sync::{Arc, LazyLock}, time::{Duration, Instant}};

const DEFAULT_DATA_DIR: &str = "./matter-data";
const DEFAULT_LOCAL_ADDRESS: &str = "0.0.0.0:5555";

#[derive(Parser, Debug)]
#[command(name = "devman_demo", about = "Matter device manager demo")]
struct Cli {
    #[clap(long)]
    #[arg(global = true, default_value_t = false)]
    verbose: bool,

    /// Data directory (config, certs, device registry)
    #[clap(short, long)]
    #[arg(global = true, default_value_t = DEFAULT_DATA_DIR.to_string())]
    data_dir: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initialize device manager (create CA, controller, config)
    Init {
        #[clap(long, default_value_t = 1000)]
        fabric_id: u64,

        #[clap(long, default_value_t = 100)]
        controller_id: u64,

        #[clap(long, default_value_t = DEFAULT_LOCAL_ADDRESS.to_string())]
        local_address: String,
    },
    /// Commission a device
    Commission {
        /// Device address (ip:port)
        address: String,
        /// Node ID to assign
        node_id: u64,
        /// Commissioning PIN
        pin: u32,
        /// Friendly name
        name: String,
    },
    CommissionWithDiscovery {
        /// Manual pairing code
        pairing_code: String,
        /// Node ID to assign
        node_id: u64,
        /// Friendly name
        name: String,
    },
    /// Commission a Wi-Fi device over BLE (requires --features ble)
    #[cfg(feature = "ble")]
    CommissionBle {
        /// Manual or QR pairing code
        pairing_code: String,
        /// Node ID to assign
        node_id: u64,
        /// Friendly name
        name: String,
        /// Wi-Fi SSID to provision
        ssid: String,
        /// Wi-Fi password to provision
        #[clap(long, default_value = "")]
        password: String,
    },
    /// Commission a Thread device over BLE (requires --features ble)
    #[cfg(feature = "ble")]
    CommissionBleThread {
        /// Manual or QR pairing code
        pairing_code: String,
        /// Node ID to assign
        node_id: u64,
        /// Friendly name
        name: String,
        /// Thread operational dataset (hex-encoded, from `ot-ctl dataset active -x`)
        dataset: String,
    },
    /// Scan for BLE commissionable Matter devices (requires --features ble)
    #[cfg(feature = "ble")]
    ScanBle {
        /// Scan duration in seconds
        #[clap(long, default_value_t = 5)]
        timeout_secs: u64,
    },
    /// Discover commissionable Matter devices via mDNS
    DiscoverCommissionable {
        /// Discovery window in seconds
        #[clap(long, default_value_t = 5)]
        timeout_secs: u64,
    },
    /// List registered devices
    List,
    Control {
        /// Device name or node ID
        device: String,
        #[command(subcommand)]
        command: ControlCommand,
    },
    /// List all attributes on all clusters on all endpoints
    ptmributes {
        /// Device name or node ID
        device: String,
    },
    /// Write attribute
    WriteAttribute {
        /// Device name or node ID
        device: String,
        /// Endpoint ID
        endpoint: u16,
        /// Cluster ID
        cluster: u32,
        /// Attribute ID
        attribute: u32,
        /// Value to write
        value: String,
    },
    /// Remove device from registry
    Remove {
        /// Device name or node ID
        device: String,
    },
    /// Rename device
    Rename {
        /// Current device name or node ID
        device: String,
        /// New name
        new_name: String,
    },
}

#[derive(Subcommand, Debug)]
enum ControlCommand {
    /// Send ON command
    On,
    /// Send OFF command
    Off,
    /// Send Toggle command
    Toggle,
    SetLevel {
        /// Brightness level 0-255
        level: u8,
    },
    SetHueSat {
        /// Hue 0-255
        hue: u8,
        /// Saturatio 0-100
        saturation: u8,
    },
    EnableColorLoop,
    Flash,
    ListenForButton,
}

async fn connect_by_name_or_id(
    dm: &DeviceManager,
    device: &str,
) -> Result<Arc<matc::controller::Connection>> {
    static CONNECTION_CACHE: LazyLock<Mutex<HashMap<String, Option<Arc<Connection>>>>> = LazyLock::new(Default::default);


    let mut cache = CONNECTION_CACHE.lock().await;
    if let Some(connection) = cache.get(&device.to_owned()) {
        match connection {
            Some(connection) => return Ok(connection.clone()),
            None => return Err(anyhow::anyhow!("cache for device {device} has error")),
        }
    }


    let result = 
    // Try parsing as node_id first, fall back to name lookup
    if let Ok(node_id) = device.parse::<u64>() &&
        dm.get_device(node_id)?.is_some() {
            dm.connect(node_id).await
    } else {
        dm.connect_by_name(device).await
    };

    match result {
        Ok(conn) => {
            let conn = Arc::new(conn);
            cache.insert(device.to_owned(), Some(conn.clone()));
            Ok(conn)
        },
        Err(err) => {
            cache.insert(device.to_owned(), None);
            Err(err)
        },
    }
}

fn resolve_node_id(dm: &DeviceManager, device: &str) -> Result<u64> {
    if let Ok(node_id) = device.parse::<u64>() {
        if dm.get_device(node_id)?.is_some() {
            return Ok(node_id);
        }
    }
    let dev = dm
        .get_device_by_name(device)?
        .ok_or_else(|| anyhow::anyhow!("device '{}' not found", device))?;
    Ok(dev.node_id)
}

async fn print_cluster_attributes(
    connection: &controller::Connection,
    endpoint: u16,
    cluster: u32,
) {
    match clusters::names::get_cluster_name(cluster) {
        Some(v) => println!("    {} (0x{:x})", v, cluster),
        None => println!("    unknown cluster - id 0x{:x}", cluster),
    }
    let attrlist = clusters::codec::get_attribute_list(cluster);
    for attr in attrlist {
        let out = connection.read_request2(endpoint, cluster, attr.0).await;
        if let Ok(out) = out {
            println!(
                "      attr 0x{:x} {}: {}",
                attr.0,
                attr.1,
                clusters::codec::decode_attribute_json(cluster, attr.0, &out)
            );
        }
    }
}

async fn print_endpoint_attributes(connection: &controller::Connection, endpoint: u16) {
    let resptlv = connection
        .read_request2(
            endpoint,
            clusters::defs::CLUSTER_ID_DESCRIPTOR,
            clusters::defs::CLUSTER_DESCRIPTOR_ATTR_ID_SERVERLIST,
        )
        .await
        .unwrap();
    println!("  clusters:");
    if let tlv::TlvItemValue::List(l) = resptlv {
        for c in l {
            if let tlv::TlvItemValue::Int(cluster) = c.value {
                print_cluster_attributes(connection, endpoint, cluster as u32).await;
            }
        }
    }
}

async fn all_attributes(connection: &controller::Connection) {
    let resptlv = connection
        .read_request2(
            0,
            clusters::defs::CLUSTER_ID_DESCRIPTOR,
            clusters::defs::CLUSTER_DESCRIPTOR_ATTR_ID_PARTSLIST,
        )
        .await
        .unwrap();
    if let tlv::TlvItemValue::List(l) = resptlv {
        for part in l {
            if let tlv::TlvItemValue::Int(v) = part.value {
                println!("endpoint {}", v);
                print_endpoint_attributes(connection, v as u16).await;
            }
        }
    }
    println!("endpoint 0");
    print_endpoint_attributes(connection, 0).await;
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let log_level = if cli.verbose {
        log::LevelFilter::Trace
    } else {
        log::LevelFilter::Error
    };
    env_logger::Builder::new()
        .parse_default_env()
        .target(env_logger::Target::Stdout)
        .filter_level(log_level)
        .format_timestamp(Some(env_logger::TimestampPrecision::Millis))
        .init();

    let data_dir = &cli.data_dir;

    match cli.command {
        Commands::Init {
            fabric_id,
            controller_id,
            local_address,
        } => {
            let config = ManagerConfig {
                fabric_id,
                controller_id,
                local_address,
            };
            DeviceManager::create(data_dir, config).await?;
            println!("Device manager initialized in {}", data_dir);
        }
        Commands::Commission {
            address,
            node_id,
            pin,
            name,
        } => {
            let dm = DeviceManager::load(data_dir).await?;
            let conn = dm.commission(&address, pin, node_id, &name).await?;
            println!("Commissioned '{}' (node {}) at {}", name, node_id, address);

            // List endpoints
            if let Ok(server_list) = descriptor_cluster::read_server_list(&conn, 0).await {
                println!("Supported clusters:");
                for c in server_list {
                    match clusters::names::get_cluster_name(c) {
                        Some(name) => println!("  {}", name),
                        None => println!("  unknown (0x{:x})", c),
                    }
                }
            }
        }
        Commands::CommissionWithDiscovery {
            pairing_code,
            node_id,
            name,
        } => {
            let dm = DeviceManager::load(data_dir).await?;
            let connection = dm
                .commission_with_code(&pairing_code, node_id, &name)
                .await?;
            println!("Commissioned '{}' (node {})", name, node_id);

            let mut endpoints = descriptor_cluster::read_parts_list(&connection, 0).await?;
            println!("Endpoints: {:?}", endpoints);
            endpoints.push(0);
            for ep in endpoints {
                let server_list = descriptor_cluster::read_server_list(&connection, ep).await?;
                let names = server_list
                    .iter()
                    .map(|c| match clusters::names::get_cluster_name(*c) {
                        Some(name) => name.to_string(),
                        None => format!("unknown (0x{:x})", c),
                    })
                    .collect::<Vec<_>>();
                println!("Supported clusters on endpoint {:?}: {:?}", ep, names);
            }
        }
        #[cfg(feature = "ble")]
        Commands::CommissionBle {
            pairing_code,
            node_id,
            name,
            ssid,
            password,
        } => {
            let dm = DeviceManager::load(data_dir).await?;
            println!(
                "Scanning for BLE commissionable device (pairing code: {})",
                pairing_code
            );
            let conn = dm
                .commission_ble_with_code(
                    &pairing_code,
                    node_id,
                    &name,
                    NetworkCreds::WiFi {
                        ssid: ssid.into_bytes(),
                        creds: password.into_bytes(),
                    },
                )
                .await?;
            println!("Commissioned '{}' (node {})", name, node_id);

            if let Ok(server_list) = descriptor_cluster::read_server_list(&conn, 0).await {
                println!("Supported clusters:");
                for c in server_list {
                    match clusters::names::get_cluster_name(c) {
                        Some(name) => println!("  {}", name),
                        None => println!("  unknown (0x{:x})", c),
                    }
                }
            }
        }
        #[cfg(feature = "ble")]
        Commands::CommissionBleThread {
            pairing_code,
            node_id,
            name,
            dataset,
        } => {
            let dm = DeviceManager::load(data_dir).await?;
            let dataset =
                hex::decode(&dataset).map_err(|e| anyhow::anyhow!("dataset hex decode: {}", e))?;
            println!(
                "Scanning for BLE commissionable device (pairing code: {})",
                pairing_code
            );
            let conn = dm
                .commission_ble_with_code(
                    &pairing_code,
                    node_id,
                    &name,
                    NetworkCreds::Thread { dataset },
                )
                .await?;
            println!("Commissioned '{}' (node {})", name, node_id);

            if let Ok(server_list) = descriptor_cluster::read_server_list(&conn, 0).await {
                println!("Supported clusters:");
                for c in server_list {
                    match clusters::names::get_cluster_name(c) {
                        Some(name) => println!("  {}", name),
                        None => println!("  unknown (0x{:x})", c),
                    }
                }
            }
        }
        #[cfg(feature = "ble")]
        Commands::ScanBle { timeout_secs } => {
            println!(
                "Scanning BLE for commissionable Matter devices ({}s)",
                timeout_secs
            );
            let devices = ble::scan_commissionable(Duration::from_secs(timeout_secs)).await?;
            if devices.is_empty() {
                println!("No BLE commissionable devices found.");
            } else {
                println!(
                    "{:<6} {:<6} {:<6} {:<3} {:<5} {:<20} Address",
                    "Disc", "VID", "PID", "CM", "RSSI", "Name"
                );
                println!("{}", "-".repeat(80));
                for d in devices {
                    let rssi = d.rssi.map(|v| v.to_string()).unwrap_or_else(|| "-".into());
                    let name = d.name.as_deref().unwrap_or("");
                    println!(
                        "{:<6} 0x{:04x} 0x{:04x} {:<3} {:<5} {:<20} {}",
                        d.discriminator,
                        d.vendor_id,
                        d.product_id,
                        if d.cm_flag { "y" } else { "n" },
                        rssi,
                        name,
                        d.address
                    );
                }
            }
        }
        Commands::DiscoverCommissionable { timeout_secs } => {
            let dm = DeviceManager::load(data_dir).await?;
            println!(
                "Discovering commissionable devices for {}s...",
                timeout_secs
            );
            let devices = dm
                .discover_commissionable_devices(Duration::from_secs(timeout_secs))
                .await?;
            if devices.is_empty() {
                println!("No commissionable devices found.");
            } else {
                println!("{:<20} {:<6} {:<15} IPs", "Name", "Disc", "Device");
                println!("{}", "-".repeat(70));
                for (instance, info) in devices {
                    println!(
                        "{:<20} {:<6} {:<15} {:?}",
                        info.name.unwrap_or_default(),
                        info.discriminator.unwrap_or_default(),
                        instance,
                        info.ips
                    );
                }
            }
        }
        Commands::List => {
            let dm = DeviceManager::load(data_dir).await?;
            let devices = dm.list_devices()?;
            if devices.is_empty() {
                println!("No devices registered.");
            } else {
                println!("{:<10} {:<25} Name", "Node ID", "Address");
                println!("{}", "-".repeat(60));
                for d in devices {
                    println!("{:<10} {:<25} {}", d.node_id, d.address, d.name);
                }
            }
        }
        Commands::Control { device, command } => {
            let dm = DeviceManager::load(data_dir).await?;

            handle_command(&dm, &device, command).await?;
        }
        Commands::ptmributes { device } => {
            let dm = DeviceManager::load(data_dir).await?;
            let mut conn = connect_by_name_or_id(&dm, &device).await?;
            all_attributes(&mut conn).await;
        }
        Commands::Remove { device } => {
            let dm = DeviceManager::load(data_dir).await?;
            let node_id = resolve_node_id(&dm, &device)?;
            dm.remove_device(node_id)?;
            println!("Removed device (node {})", node_id);
        }
        Commands::Rename { device, new_name } => {
            let dm = DeviceManager::load(data_dir).await?;
            let node_id = resolve_node_id(&dm, &device)?;
            dm.rename_device(node_id, &new_name)?;
            println!("Renamed device (node {}) to '{}'", node_id, new_name);
        }
        Commands::WriteAttribute {
            device,
            endpoint,
            cluster,
            attribute,
            value,
        } => {
            let dm = DeviceManager::load(data_dir).await?;
            let conn = connect_by_name_or_id(&dm, &device).await?;
            println!(
                "Writing attribute 0x{:x} on cluster 0x{:x} at endpoint {} of device '{}'",
                attribute, cluster, endpoint, device
            );
            let mut tlv_value = tlv::TlvBuffer::new();
            tlv_value.write_string(2, &value)?;
            conn.write_request(endpoint, cluster, attribute, &tlv_value.data)
                .await?;
            println!("Write successful");
        }
    }

    Ok(())
}

async fn handle_command(dm: &DeviceManager, device: &str, command: ControlCommand) -> Result<()> {
    let conn = connect_by_name_or_id(&dm, &device).await?;

    match command {
        ControlCommand::On  => {
            on_off::on(&conn, 1).await?;
            println!("ON sent to '{}'", device);
        }
        ControlCommand::Off => {
            on_off::off(&conn, 1).await?;
            println!("OFF sent to '{}'", device);
        }
        ControlCommand::Toggle => {

            let start = Instant::now();
            println!("SENDING TOGGLE to '{}'", device);
            on_off::toggle(&conn, 1).await?;
            let duration = Instant::now().duration_since(start);

            println!(
                "TOGGLE sent to '{}', took {} ms",
                device,
                duration.as_millis()
            );
        }
        ControlCommand::SetLevel { level } => {
            level_control::move_to_level_with_on_off(&conn, 1, level, None, 0x00, 0x00).await?;
        }
        ControlCommand::SetHueSat {
            hue,
            saturation,
        } => {

            color_control::move_to_hue_and_saturation(&conn, 1, hue, saturation, 100, 0x00, 0x00)
                .await?;
        }
        ControlCommand::EnableColorLoop => {
            use color_control::updateflags::*;
            
            color_control::color_loop_set(&conn, 1, UPDATE_ACTION | UPDATE_TIME | UPDATE_DIRECTION | UPDATE_START_HUE , color_control::ColorLoopAction::Activatefromcolorloopstartenhancedhue, color_control::ColorLoopDirection::Decrement, 1, 20, 0x00, 0x00).await?;
        }
        ControlCommand::Flash => {
            
            for i in 0..10 {
                println!("i: {i}");

                println!("red");
                color_control::move_to_hue_and_saturation(&conn, 1, 5, 80, 5, 0x00, 0x00)
                    .await?;

                tokio::time::sleep(Duration::from_millis(50 * 100)).await;

                println!("white");
                color_control::move_to_hue_and_saturation(&conn, 1, 5, 10, 5, 0x00, 0x00)
                    .await?;
                tokio::time::sleep(Duration::from_millis(50 * 100)).await;
            }

        
        }
        ControlCommand::ListenForButton => {
            let conn = connect_by_name_or_id(&dm, &device).await?;
            
            let mut sub = conn.subscribe_events(None, None, None, true).await?;

            let mut counter = 0;
            println!("listening");
            while let Some(next) = sub.next().await {
                for ev in next.event_reports {
                    let endpoint = ev.endpoint.unwrap_or(0);
                    let cluster = ev.cluster.unwrap_or(0);
                    let event = ev.event.unwrap_or(0);
                    let ev_idx = ev.event_number.unwrap_or(0);
                    let data = ev.data.unwrap_or(tlv::TlvItemValue::Nil());

                    // println!("ev#{ev_idx:010}/{counter:03}: {endpoint:02}:{cluster:02} ev {event:02} ({:?})", data);
                    
                    print!("{endpoint:02}: ");

                    match event {
                        1 => {
                            println!("PRESS")
                        }
                        2 => {
                            println!("LONG PRESS START")
                        }
                        3 => {
                            println!("RELEASE")
                        }
                        4 => {
                            println!("LONG PRESS RELEASE")
                        }
                        5 => {
                            let count = match data {
                                tlv::TlvItemValue::List(tlv_items) => {
                                    if let Some(TlvItemValue::Int(value)) = tlv_items.iter().filter_map(|item| (item.tag == 1).then_some(&item.value)).next() {
                                        Some(*value)
                                    } else {
                                        None
                                    }
                                },
                                _ => {
                                    None
                                }
                            }.unwrap_or(999);
                            println!("ONGOING MULTIPRESS ({count})");
                        }
                        6 => {
                            let count = match data {
                                tlv::TlvItemValue::List(tlv_items) => {
                                    if let Some(TlvItemValue::Int(value)) = tlv_items.iter().filter_map(|item| (item.tag == 1).then_some(&item.value)).next() {
                                        Some(*value)
                                    } else {
                                        None
                                    }
                                },
                                _ => {
                                    None
                                }
                            }.unwrap_or(999);
                            println!("MULTIPRESS END ({count})")
                        }
                        _ => {
                            println!("UNKOWN")
                        }
                    }

                    if endpoint == 1 { // turn right

                    }

                    if endpoint == 2 { // turn left

                    }
                }

                if counter > 50 {
                    break;
                }
                counter += 1;
            }
            println!("finished");

            println!("unsubscribing");
            conn.im_unsubscribe_all().await?;
            println!("unsubscribed");
        }
    }

    Ok(())
}
