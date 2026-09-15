use serde::{Deserialize, Serialize};

use crate::id::DeviceId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadGraphMessage {
    pub device: DeviceId,
    pub kind: ThreadGraphMessageKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreadGraphMessageKind {
    Discovered(ThreadDeviceData),
    Error(String),
    NotReadyYet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadDeviceData {
    pub address: Option<u64>,
    pub role: RoutingRole,
    pub neighbor_table: Vec<NeighborTableStruct>,
    pub route_table: Vec<RouteTableStruct>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoutingRole {
    Unspecified,
    Unassigned,
    SleepyEndDevice,
    EndDevice,
    Reed,
    Router,
    Leader,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeighborTableStruct {
    pub address: u64,
    pub age: u32,
    pub rloc_16: u16,
    pub link_frame_counter: u32,
    pub mle_frame_counter: u32,
    pub lqi: u8,
    pub average_rssi: i8,
    pub last_rssi: i8,
    pub frame_error_rate: u8,
    pub message_error_rate: u8,
    pub rx_on_when_idle: bool,
    pub full_thread_device: bool,
    pub full_network_data: bool,
    pub is_child: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteTableStruct {
    pub address: u64,
    pub rloc_16: u16,
    pub router_id: u8,
    pub next_hop: u8,
    pub path_cost: u8,
    pub lqi_in: u8,
    pub lqi_out: u8,
    pub age: u8,
    pub allocated: bool,
    pub link_established: bool,
}
