use std::collections::{BTreeMap, BTreeSet};

use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::id::{AssetId, EndpointId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceAsset {
    pub config: DeviceAssetConfig,
    pub commission_timestamp: Timestamp,
    pub endpoints: BTreeMap<EndpointId, BTreeSet<DeviceAssetDeviceKind>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceAssetConfig {
    pub name: String,
    pub room: Option<AssetId>,
    pub labels: BTreeSet<AssetId>,
}

#[cfg(feature = "backend")]
impl crate::backend::DirectoryAsset for DeviceAsset {
    const DIRECTORY_NAME: &'static str = "device";
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum DeviceAssetDeviceKind {
    #[serde(alias = "extended_color_light")]
    ColorLight,
    OnOffPlugInUnit,
    Switch,
    OccupancySensor,
}

impl TryFrom<u32> for DeviceAssetDeviceKind {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            // 0x000E => Ok("Aggregator"),
            // 0x002D => Ok("Air Purifier"),
            // 0x002C => Ok("Air Quality Sensor"),
            // 0x0028 => Ok("Basic Video Player"),
            // 0x0018 => Ok("Battery Storage"),
            // 0x0013 => Ok("Bridged Node"),
            // 0x0029 => Ok("Casting Video Client"),
            // 0x0023 => Ok("Casting Video Player"),
            // 0x0105 => Ok("Color Dimmer Switch"),
            0x010C => Ok(Self::ColorLight),
            // 0x0015 => Ok("Contact Sensor"),
            // 0x0024 => Ok("Content App"),
            // 0x0840 => Ok("Control Bridge"),
            // 0x0077 => Ok("Cook Surface"),
            // 0x0078 => Ok("Cooktop"),
            // 0x050D => Ok("Device Energy Management"),
            // 0x0101 => Ok("Dimmable Light"),
            // 0x010B => Ok("Dimmable Plug-In Unit"),
            // 0x0104 => Ok("Dimmer Switch"),
            // 0x0075 => Ok("Dishwasher"),
            // 0x000A => Ok("Door Lock"),
            // 0x000B => Ok("Door Lock Controller"),
            // 0x0510 => Ok("Electrical Sensor"),
            // 0x050C => Ok("Energy EVSE"),
            0x010D => Ok(Self::ColorLight),
            // 0x007A => Ok("Extractor Hood"),
            // 0x002B => Ok("Fan"),
            // 0x0306 => Ok("Flow Sensor"),
            0x000F => Ok(Self::Switch),
            // 0x0309 => Ok("Heat Pump"),
            // 0x0307 => Ok("Humidity Sensor"),
            // 0x0130 => Ok("Joint Fabric Administrator"),
            // 0x007C => Ok("Laundry Dryer"),
            // 0x0073 => Ok("Laundry Washer"),
            // 0x0106 => Ok("Light Sensor"),
            // 0x0079 => Ok("Microwave Oven"),
            // 0x0027 => Ok("Mode Select"),
            // 0x0110 => Ok("Mounted Dimmable Load Control"),
            // 0x010F => Ok("Mounted On/Off Control"),
            // 0x0090 => Ok("Network Infrastructure Manager"),
            0x0107 => Ok(Self::OccupancySensor),
            // 0x0100 => Ok("On/Off Light"),
            // 0x0103 => Ok("On/Off Light Switch"),
            0x010A => Ok(Self::OnOffPlugInUnit),
            // 0x0850 => Ok("On/Off Sensor"),
            // 0x0014 => Ok("OTA Provider"),
            // 0x0012 => Ok("OTA Requestor"),
            // 0x007B => Ok("Oven"),
            // 0x0011 => Ok("Power Source"),
            // 0x0305 => Ok("Pressure Sensor"),
            // 0x0303 => Ok("Pump"),
            // 0x0304 => Ok("Pump Controller"),
            // 0x0044 => Ok("Rain Sensor"),
            // 0x0070 => Ok("Refrigerator"),
            // 0x0074 => Ok("Robotic Vacuum Cleaner"),
            // 0x0072 => Ok("Room Air Conditioner"),
            // 0x0016 => Ok("Root Node"),
            // 0x0019 => Ok("Secondary Network Interface"),
            // 0x0076 => Ok("Smoke CO Alarm"),
            // 0x0017 => Ok("Solar Power"),
            // 0x0022 => Ok("Speaker"),
            // 0x0071 => Ok("Temperature Controlled Cabinet"),
            // 0x0302 => Ok("Temperature Sensor"),
            // 0x0301 => Ok("Thermostat"),
            // 0x030A => Ok("Thermostat Controller"),
            // 0x0091 => Ok("Thread Border Router"),
            // 0x002A => Ok("Video Remote Control"),
            // 0x0041 => Ok("Water Freeze Detector"),
            // 0x050F => Ok("Water Heater"),
            // 0x0043 => Ok("Water Leak Detector"),
            // 0x0042 => Ok("Water Valve"),
            // 0x0202 => Ok("Window Covering"),
            // 0x0203 => Ok("Window Covering Controller"),
            _ => Err(()),
        }
    }
}
