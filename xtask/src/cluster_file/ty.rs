use std::{borrow::Cow, fmt::Display, str::FromStr};

use nest_struct::nest_struct;
use serde::Deserialize;
use strum::EnumString;

#[derive(Debug, EnumString)]
#[strum(serialize_all = "kebab-case")]
pub enum Ty {
    Bool,
    Uint8,
    Uint16,
    #[strum(serialize = "uint24")]
    #[strum(serialize = "uint32")]
    Uint32,
    #[strum(serialize = "uint48")]
    #[strum(serialize = "uint56")]
    #[strum(serialize = "uint64")]
    Uint64,
    Int8,
    #[strum(serialize = "int16")]
    #[strum(serialize = "int16s")]
    Int16,
    #[strum(serialize = "int24")]
    #[strum(serialize = "int32")]
    Int32,
    #[strum(serialize = "int48")]
    #[strum(serialize = "int56")]
    #[strum(serialize = "int64")]
    Int64,
    String,
    Octstr,
    #[strum(serialize = "single")]
    Float,
    Double,
    List,
    Temperature,
    Percent,
    Percent100ths,
    #[strum(serialize = "amperage-mA")]
    AmperageMa,
    #[strum(serialize = "power-mW")]
    PowerMw,
    #[strum(serialize = "power-mVAR")]
    PowerMVAR,
    #[strum(serialize = "power-mVA")]
    PowerMVA,
    #[strum(serialize = "voltage-mV")]
    VoltageMv,
    #[strum(serialize = "energy-mWh")]
    EnergyMwh,
    #[strum(serialize = "energy-mVAh")]
    EnergyMvah,
    #[strum(serialize = "energy-mVARh")]
    EnergyMvarh,
    EpochUs,
    EpochS,
    ElapsedS,
    SystemtimeUs,
    SystimeMs,
    PosixMs,
    NodeId,
    EndpointNo,
    EndpointId,
    ClusterId,
    AttributeId,
    DevtypeId,
    VendorId,
    GroupId,
    FabricIdx,
    FabricId,
    MessageId,
    SubjectId,
    Status,
    Tag,
    Namespace,
    Money,
    Enum8,
    Enum16,
    Map16,
    Hwadr,
    Ipv6adr,
    Ipv6pre,
    #[strum(default)]
    Named(String),
}

#[nest_struct]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeFields {
    #[serde(rename = "@type")]
    ty: Ty,
    entry: Option<nest! {
        #[serde(rename = "@type")]
        ty: Ty,
    }>,
}

impl TypeFields {
    pub fn type_name(&self, add_ty_prefix: bool) -> TypeName<'_> {
        let list_entry = self.entry.as_ref().map(|entry| &entry.ty);

        self.ty.type_name(list_entry, add_ty_prefix)
    }
}

pub enum TypeName<'a> {
    Unsupported {
        ty: String,
    },
    Type {
        name: Cow<'a, str>,
        add_ty_prefix: bool,
        is_scalar: bool,
    },
}

impl Display for TypeName<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeName::Unsupported { ty } => write!(f, "TypeUnsupported /* {ty} */"),
            TypeName::Type {
                name,
                add_ty_prefix,
                ..
            } => {
                if *add_ty_prefix {
                    write!(f, "ty::")?;
                }
                write!(f, "{}", name)
            }
        }
    }
}

impl<'a> TypeName<'a> {
    pub fn scalar(name: &'a str) -> Self {
        Self::Type { name: Cow::Borrowed(name), add_ty_prefix: false, is_scalar: true }
    }

    pub fn is_supported(&self) -> bool {
        matches!(self, Self::Type { .. })
    }

    pub fn is_scalar(&self) -> bool {
        matches!(self, Self::Type { is_scalar: true, .. })
    }
}

impl Ty {
    pub fn type_name(&self, list_entry: Option<&Self>, add_ty_prefix: bool) -> TypeName<'_> {
        match self {
            Ty::Bool => TypeName::scalar("bool"),
            Ty::Uint8 => TypeName::scalar("u8"),
            Ty::Uint16 => TypeName::scalar("u16"),
            Ty::Uint32 => TypeName::scalar("u32"),
            Ty::Uint64 => TypeName::scalar("u64"),
            Ty::Int8 => TypeName::scalar("i8"),
            Ty::Int16 => TypeName::scalar("i16"),
            Ty::Int32 => TypeName::scalar("i32"),
            Ty::Int64 => TypeName::scalar("i64"),
            Ty::Float => TypeName::scalar("f32"),
            Ty::Double => TypeName::scalar("f64"),
            Ty::String => TypeName::scalar("String"),
            Ty::Temperature => TypeName::scalar("Temperature"),
            Ty::AmperageMa => TypeName::scalar("AmperageMa"),
            Ty::PowerMw => TypeName::scalar("PowerMw"),
            Ty::VoltageMv => TypeName::scalar("VoltageMv"),
            Ty::EnergyMwh => TypeName::scalar("EnergyMwh"),
            Ty::List => {
                let list_entry = list_entry.unwrap();
                TypeName::Type { name: Cow::Owned(format!("Vec<{}>", list_entry.type_name(None, add_ty_prefix))), add_ty_prefix: false, is_scalar: false }
            } 
            Ty::Named(name) if name.ends_with("ID") => TypeName::scalar("u16"),
            Ty::Named(name) => {
                TypeName::Type { name: Cow::Borrowed(name.as_str()), add_ty_prefix, is_scalar: !name.ends_with("Struct") }
            }
            name => return TypeName::Unsupported { ty: format!("{name:?}") },
        }
    }
}

impl<'de> Deserialize<'de> for Ty {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        let Ok(ty) = Ty::from_str(&value);
        if let Ty::Named(name) = &ty {
            if name.starts_with(char::is_lowercase) {
                println!("WARNING: Named type {} is lowercase", name);
            }
        }

        Ok(ty)
    }
}
