use crate::id::AttrId;

pub enum NumericUnit {
    MilliSi(MilliSiUnit),
    Percent { max_value: i64 },
    Celcius,
}

pub enum BoolUnit {
    On,
    Occupied,
    Open
}

pub enum PercentResolution {
    /// 0 -> 0%
    /// 100 -> 100%
    /// The Maximum resolution is 1%
    P100,
    /// 0 -> 0%
    /// 200 -> 100%
    /// The Maximum resolution is 0.5%
    P200,
    /// 0 -> 0%
    /// 10000 -> 100%
    /// The Maximum resolution is 0.01%
    P10000,
}

pub enum MilliSiUnit {
    Volt,
    Watt,
    Ampare,
    WattHour,
}

pub trait ValueAttribute {
    fn attr_id(&self) -> AttrId;
    fn value(&self) -> ValueAttributeKind;
}

pub enum ValueAttributeKind {
    Numeric(Box<dyn NumericValueAttribute>),
    Bool(Box<dyn NumericValueAttribute>),
}

pub trait NumericValueAttribute {
    fn unit(&self) -> NumericUnit;
    fn read(&self, tlv: &[u8]) -> Option<i64>;
}
pub trait BoolValueAttribute {
    fn unit(&self) -> BoolUnit;
    fn read(&self, tlv: &[u8]) -> Option<bool>;
}
