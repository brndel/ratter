use std::collections::BTreeMap;

use anyhow::{anyhow, bail};
use dioxus::fullstack::reqwest;
use matter_codec::Value;
use matter_controller::{Node, ReadPath};
use nest_struct::nest_struct;
use serde::Deserialize;
use shared_core::{
    id::{AttrId, ClusterId},
    thread::{NeighborTableStruct, RouteTableStruct, RoutingRole, ThreadDeviceData},
};

const THREAD_DIAGNOSTICS_CLUSTER: ClusterId = 0x0035;
const ROUTING_ROLE: AttrId = 0x0001;
const NEIGHBOR_TABLE: AttrId = 0x0007;
const ROUTE_TABLE: AttrId = 0x0008;

pub async fn read_thread_data(node: &Node, ext_address: Option<u64>) -> anyhow::Result<ThreadDeviceData> {
    let values = node
        .read(&[
            ReadPath::concrete(0, THREAD_DIAGNOSTICS_CLUSTER, ROUTING_ROLE),
            ReadPath::concrete(0, THREAD_DIAGNOSTICS_CLUSTER, NEIGHBOR_TABLE),
            ReadPath::concrete(0, THREAD_DIAGNOSTICS_CLUSTER, ROUTE_TABLE),
        ])
        .await?;

    let mut routing_role = None;
    let mut neighbor_table = None;
    let mut route_table = None;

    for (path, value) in values {
        match path.attribute {
            ROUTING_ROLE => {
                let Value::Uint(role) = value else {
                    bail!("ROUTING_ROLE is not an Uint");
                };

                let role = match role {
                    0 => RoutingRole::Unspecified,
                    1 => RoutingRole::Unassigned,
                    2 => RoutingRole::SleepyEndDevice,
                    3 => RoutingRole::EndDevice,
                    4 => RoutingRole::Reed,
                    5 => RoutingRole::Router,
                    6 => RoutingRole::Leader,
                    role => bail!("unkown role {role}"),
                };

                routing_role = Some(role);
            }
            NEIGHBOR_TABLE => {
                let Value::Array(values) = value else {
                    bail!("NEIGHBOR_TABLE is not an Array");
                };

                let mut entries = Vec::new();
                for value in values {
                    let Value::Structure(fields) = value else {
                        bail!("NEIGHBOR_TABLE array entry is not structured");
                    };
                    let mut fields = fields.into_iter().map(|(_, field)| field);

                    entries.push(NeighborTableStruct {
                        address: uint(fields.next().unwrap())?,
                        age: uint(fields.next().unwrap())? as _,
                        rloc_16: uint(fields.next().unwrap())? as _,
                        link_frame_counter: uint(fields.next().unwrap())? as _,
                        mle_frame_counter: uint(fields.next().unwrap())? as _,
                        lqi: uint(fields.next().unwrap())? as _,
                        average_rssi: int(fields.next().unwrap())? as _,
                        last_rssi: int(fields.next().unwrap())? as _,
                        frame_error_rate: uint(fields.next().unwrap())? as _,
                        message_error_rate: uint(fields.next().unwrap())? as _,
                        rx_on_when_idle: bool(fields.next().unwrap())?,
                        full_thread_device: bool(fields.next().unwrap())?,
                        full_network_data: bool(fields.next().unwrap())?,
                        is_child: bool(fields.next().unwrap())?,
                    });
                }

                neighbor_table = Some(entries);
            }
            ROUTE_TABLE => {
                let Value::Array(values) = value else {
                    bail!("ROUTE_TABLE is not an Array");
                };

                let mut entries = Vec::new();
                for value in values {
                    let Value::Structure(fields) = value else {
                        bail!("ROUTE_TABLE array entry is not structured");
                    };
                    let mut fields = fields.into_iter().map(|(_, field)| field);

                    entries.push(RouteTableStruct {
                        address: uint(fields.next().unwrap())?,
                        rloc_16: uint(fields.next().unwrap())? as _,
                        router_id: uint(fields.next().unwrap())? as _,
                        next_hop: uint(fields.next().unwrap())? as _,
                        path_cost: uint(fields.next().unwrap())? as _,
                        lqi_in: uint(fields.next().unwrap())? as _,
                        lqi_out: uint(fields.next().unwrap())? as _,
                        age: uint(fields.next().unwrap())? as _,
                        allocated: bool(fields.next().unwrap())?,
                        link_established: bool(fields.next().unwrap())?,
                    });
                }

                route_table = Some(entries);
            }
            attr => bail!("unkown attr 0x{attr:x}"),
        }
    }

    dbg!(&routing_role, &neighbor_table, &route_table);

    Ok(ThreadDeviceData {
        address: ext_address,
        role: routing_role.ok_or_else(|| anyhow!("routing_role missing"))?,
        neighbor_table: neighbor_table.ok_or_else(|| anyhow!("neighbor_table missing"))?,
        route_table: route_table.ok_or_else(|| anyhow!("route_table missing"))?,
    })
}

fn uint(value: Value) -> anyhow::Result<u64> {
    match value {
        Value::Uint(v) => Ok(v),
        v => Err(anyhow::anyhow!("value is {v:?}, should be Uint")),
    }
}
fn int(value: Value) -> anyhow::Result<i64> {
    match value {
        Value::Int(v) => Ok(v),
        v => Err(anyhow::anyhow!("value is {v:?}, should be Int")),
    }
}

fn bool(value: Value) -> anyhow::Result<bool> {
    match value {
        Value::Bool(v) => Ok(v),
        v => Err(anyhow::anyhow!("value is {v:?}, should be Bool")),
    }
}

pub async fn read_otbr_address_map(rest_endpoint: &str) -> anyhow::Result<BTreeMap<String, u64>> {
    let result = reqwest::get(rest_endpoint).await.unwrap();
    let response = result.json::<OtbrDiagnosticsResponse>().await?;

    Ok(response.data
        .into_iter()
        .map(|entry| {
            let ext_addr = u64::from_str_radix(&entry.ext_address, 16).unwrap();

            (entry.omr_ipv6_address, ext_addr)
        })
        .collect())
}


#[nest_struct]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OtbrDiagnosticsResponse {
    data: Vec<nest! {
        ext_address: String,
        omr_ipv6_address: String
    }>,
}