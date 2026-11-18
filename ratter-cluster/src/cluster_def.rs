#[derive(Debug, Clone, Copy)]
pub struct ClusterDef {
    pub id: u32,
    pub name: &'static str,
    pub features: &'static [FeatureDef],
    pub attributes: &'static [AttrDef],
    pub events: &'static [EventDef],
    pub commands: &'static [CommandDef],
}

impl ClusterDef {
    pub fn attribute(&self, id: u32) -> Option<&AttrDef> {
        let idx = self
            .attributes
            .binary_search_by_key(&id, |attr| attr.id)
            .ok()?;
        Some(&self.attributes[idx])
    }

    pub fn event(&self, id: u32) -> Option<&EventDef> {
        let idx = self.events.binary_search_by_key(&id, |attr| attr.id).ok()?;
        Some(&self.events[idx])
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Nullable<T> {
    Null,
    Some(T),
}

pub trait Attr {
    const ATTR: AttrDef;
    type Value;
}

pub trait Event {
    const EVENT: EventDef;
}

pub trait Command {
    const COMMAND: CommandDef;
}

pub trait CommandInvoke: Command {
    type Response: CommandResponse;
}

pub trait CommandResponse: Command {}

pub struct BasicAccResponse;

impl Command for BasicAccResponse {
    const COMMAND: CommandDef = CommandDef {
        id: 0x00,
        name: "Y",
        fields: &[],
        conformance: Conformance::Mandatory,
    };
}
impl CommandResponse for BasicAccResponse {}

impl Command for () {
    const COMMAND: CommandDef = CommandDef {
        id: 0x00,
        name: "N",
        fields: &[],
        conformance: Conformance::Mandatory,
    };
}
impl CommandResponse for () {}

#[derive(Debug, Clone, Copy)]
pub struct FeatureDef {
    pub bit: u32,
    pub code: &'static str,
    pub name: &'static str,
    pub summary: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct AttrDef {
    pub id: u32,
    pub name: &'static str,
    pub attr_ty: AttrTypeDef,
    pub conformance: Conformance,
}

#[derive(Debug, Clone, Copy)]
pub enum Conformance {
    Mandatory,
    Optional,
    Feature(u8),
    TooComplicated,
}

#[derive(Debug, Clone, Copy)]
pub struct AttrTypeDef {
    pub ty: AttrValueType,
    pub nullable: bool,
}

pub trait AttrType {
    const ATTR_TYPE: AttrTypeDef;
}

pub trait BaseTy
where
    Self: From<Self::BaseTy> + Into<Self::BaseTy> + Clone,
{
    type BaseTy;
}

impl<T: AttrType> AttrType for Nullable<T> {
    const ATTR_TYPE: AttrTypeDef = T::ATTR_TYPE.nullable();
}

impl<T: AttrType> AttrType for Vec<T> {
    const ATTR_TYPE: AttrTypeDef = AttrTypeDef {
        ty: AttrValueType::List(&T::ATTR_TYPE.ty),
        nullable: false,
    };
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TypeUnsupported;

impl AttrType for TypeUnsupported {
    const ATTR_TYPE: AttrTypeDef = AttrTypeDef {
        ty: AttrValueType::Unsupported,
        nullable: false,
    };
}

macro_rules! impl_base_attr_ty {
    ($ty:ty, $attr_ty:expr) => {
        impl AttrType for $ty {
            const ATTR_TYPE: AttrTypeDef = AttrTypeDef {
                ty: $attr_ty,
                nullable: false,
            };
        }

        impl BaseTy for $ty {
            type BaseTy = $ty;
        }
    };
    ($ty:ty, int $size:expr) => {
        impl_base_attr_ty!($ty, AttrValueType::Int($size, IntStyle::Number));
    };
}

impl_base_attr_ty!(bool, AttrValueType::Bool(BoolStyle::TrueFalse));
impl_base_attr_ty!(String, AttrValueType::String);
impl_base_attr_ty!(f32, AttrValueType::Float);
impl_base_attr_ty!(f64, AttrValueType::Double);

impl_base_attr_ty!(u8, int IntSize::U8);
impl_base_attr_ty!(u16, int IntSize::U16);
impl_base_attr_ty!(u32, int IntSize::U32);
impl_base_attr_ty!(u64, int IntSize::U64);

impl_base_attr_ty!(i8, int IntSize::I8);
impl_base_attr_ty!(i16, int IntSize::I16);
impl_base_attr_ty!(i32, int IntSize::I32);
impl_base_attr_ty!(i64, int IntSize::I64);

macro_rules! impl_int_attr {
    ($name:ident, $ty:ty, $style:expr) => {
        #[derive(derive_more::From, derive_more::Into, Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name(pub $ty);
        impl AttrType for $name {
            const ATTR_TYPE: AttrTypeDef = <$ty>::ATTR_TYPE.int_style($style);
        }

        impl BaseTy for $name {
            type BaseTy = $ty;
        }

        impl crate::tlv::TlvPut for $name {
            fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
                let base_ty: <Self as BaseTy>::BaseTy = self.clone().into();
                base_ty.tlv_put(tag, writer);
            }
        }

        impl crate::tlv::TlvDecode for $name {
            fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, crate::tlv::TlvDecodeErr> {
                let base_value = <Self as BaseTy>::BaseTy::tlv_decode(element, reader)?;
                Ok(base_value.into())
            }
        }
    };
}

impl_int_attr!(Temperature, i16, IntStyle::Temperature);
impl_int_attr!(Humidity, u16, IntStyle::Percent10000);
impl_int_attr!(PowerMw, i64, IntStyle::MilliUnit(MilliUnit::Watt));
impl_int_attr!(AmperageMa, i64, IntStyle::MilliUnit(MilliUnit::Ampere));
impl_int_attr!(VoltageMv, i64, IntStyle::MilliUnit(MilliUnit::Volt));
impl_int_attr!(EnergyMwh, i64, IntStyle::MilliUnit(MilliUnit::WattHour));

impl AttrTypeDef {
    pub const fn new(ty: AttrValueType) -> Self {
        Self {
            ty,
            nullable: false,
        }
    }

    pub const fn map8(variants: &'static [(usize, &'static str)]) -> Self {
        u8::ATTR_TYPE.int_style(IntStyle::Bitmap { variants })
    }

    pub const fn enum8(variants: &'static [(usize, &'static str)]) -> Self {
        u8::ATTR_TYPE.int_style(IntStyle::Enum { variants })
    }

    const fn nullable(mut self) -> Self {
        self.nullable = true;
        self
    }

    const fn int_style(mut self, style: IntStyle) -> Self {
        match &mut self.ty {
            AttrValueType::Int(_, int_style) => {
                *int_style = style;
            }
            _ => panic!("int_style called on non Int AttrValueType"),
        }

        self
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AttrValueType {
    Bool(BoolStyle),
    Int(IntSize, IntStyle),
    Float,
    Double,
    String,
    Struct { fields: &'static [AttrDef] },
    List(&'static AttrValueType),
    Unsupported,
}

#[derive(Debug, Clone, Copy)]
pub enum IntSize {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
}

#[derive(Debug, Clone, Copy)]
pub enum BoolStyle {
    TrueFalse,
    OnOff,
}

#[derive(Debug, Clone, Copy)]
pub enum IntStyle {
    Enum {
        variants: &'static [(usize, &'static str)],
    },
    Bitmap {
        variants: &'static [(usize, &'static str)],
    },
    Number,
    Percent200,
    Percent10000,
    Temperature,
    MilliUnit(MilliUnit),
}

#[derive(Debug, Clone, Copy)]
pub enum MilliUnit {
    Watt,
    Ampere,
    Volt,
    WattHour,
}

#[derive(Debug, Clone, Copy)]
pub struct EventDef {
    pub id: u32,
    pub name: &'static str,
    pub fields: &'static [AttrDef],
    pub conformance: Conformance,
}

#[derive(Debug, Clone, Copy)]
pub struct CommandDef {
    pub id: u32,
    pub name: &'static str,
    pub fields: &'static [AttrDef],
    pub conformance: Conformance,
}
