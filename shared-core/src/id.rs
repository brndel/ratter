pub type DeviceId = u64;
pub type EndpointId = u16;
pub type ClusterId = u32;
pub type AttrId = u32;
pub type EventId = u32;

pub type AssetId = u64;

pub type VendorId = u16;
pub type ProductId = u16;



#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AttrPath {
    pub device: DeviceId,
    pub endpoint: EndpointId,
    pub cluster: ClusterId,
    pub attribute: AttrId
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct EventPath {
    pub device: DeviceId,
    pub endpoint: EndpointId,
    pub cluster: ClusterId,
    pub event: EventId
}

#[cfg_attr(feature = "backend", derive(toasty::Embed))]
#[cfg_attr(feature = "backend", unique(device, endpoint, cluster, item, item_is_attr))]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ClusterItemPath {
    pub device: DeviceId,
    pub endpoint: EndpointId,
    pub cluster: ClusterId,
    pub item: u32,
    /// - `true`: the `item` value referrs to an attribute id
    /// - `false`: the `item` value referrs to an event id
    pub item_is_attr: bool
}

impl From<AttrPath> for ClusterItemPath {
    fn from(value: AttrPath) -> Self {
        Self { device: value.device, endpoint: value.endpoint, cluster: value.cluster, item: value.attribute, item_is_attr: true }
    }
}

impl From<EventPath> for ClusterItemPath {
    fn from(value: EventPath) -> Self {
        Self { device: value.device, endpoint: value.endpoint, cluster: value.cluster, item: value.event, item_is_attr: true }
    }
}