mod conformance;
mod event;
mod command;
mod field;
mod ty;

use std::{
    collections::HashMap, fmt::{Debug, Display}, str::FromStr, sync::LazyLock,
};

use change_case::{constant_case, snake_case};
use nest_struct::nest_struct;
use serde::Deserialize;

use conformance::ConformanceFields;

use crate::cluster_file::{command::CommandEntry, conformance::FeatureList, event::{EventEntry, ImplTlvDecodeContainer}, field::{Field, FieldEntry}, ty::TypeFields};

#[nest_struct]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
pub struct ClusterFile {
    revision_history: nest! {
        revision: Vec<nest! {
            #[serde(rename = "@revision")]
            revision: String,
            #[serde(rename = "@summary")]
            summary: String
        }>
    },
    cluster_ids: nest! {
        cluster_id: Vec<nest! {
            #[serde(rename = "@id")]
            id: Option<IntValue>,
            #[serde(rename = "@name")]
            name: Ident
        }>,
    },
    classification: nest! {
        #[serde(rename = "@hierarchy")]
        hierarchy: String,
        #[serde(rename = "@picsCode")]
        pics_code: String,
        #[serde(rename = "@baseCluster")]
        base_cluster: Option<String>,
    },
    features: Option<
        nest! {
            feature: Vec<nest! {
                #[serde(rename = "@bit")]
                bit: IntValue,
                #[serde(rename = "@code")]
                code: String,
                #[serde(rename = "@name")]
                name: String,
                #[serde(rename = "@summary")]
                summary: String,
            }>
        },
    >,
    data_types: Option<
        nest! {
            #[serde(rename="enum")]
            #[serde(default)]
            enums: Vec<EnumDataType>,
            #[serde(default)]
            bitmap: Vec<BitmapDataType>,
            #[serde(rename="struct")]
            #[serde(default)]
            structs: Vec<StructDataTypeEntry>
        },
    >,
    attributes: Option<
        nest! {
            attribute: Vec<nest! {
                #[serde(rename = "@id")]
                id: IntValue,
                #[serde(rename = "@name")]
                name: Ident,
                #[serde(flatten)]
                ty: TypeFields,
                quality: Option<nest! {
                    #[serde(rename = "@nullable")]
                    #[serde(default)]
                    nullable: BoolValue,
                }>,
                #[serde(flatten)]
                conformance: ConformanceFields,
            }>
        },
    >,
    events: Option<
        nest! {
            event: Vec<EventEntry>,
        },
    >,
    commands: Option<
        nest! {
            command: Vec<CommandEntry>
        },
    >,
    pub cluster_file_name: Option<String>,
}

impl ClusterFile {
    pub fn parse(content: &str) -> Result<Self, quick_xml::de::DeError> {
        quick_xml::de::from_str(content)
    }

    pub fn features(&self) -> FeatureList<'_> {
        self.features
            .as_ref()
            .map(|features| features.feature.as_slice())
            .unwrap_or_default()
    }
}

impl ClusterFileAttributesAttribute {
    fn is_nullable(&self) -> bool {
        self.quality
            .as_ref()
            .is_some_and(|quality| quality.nullable.0)
    }
}

impl Display for ClusterFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "#![allow(unused_imports)]\n")?;
        writeln!(f, "use crate::cluster_def::*;")?;
        writeln!(f, "use crate::tlv::*;")?;

        for id in &self.cluster_ids.cluster_id {
            let Some(id_number) = id.id else {
                continue;
            } ;
            writeln!(
                f,
                "pub const {}: ClusterDef = ClusterDef {{",
                constant_case(&id.name.0)
            )?;
            writeln!(f, "  id: {},", id_number)?;
            writeln!(f, "  name: \"{}\",", id.name)?;
            writeln!(f, "  features: &[")?;
            if let Some(features) = &self.features {
                for feature in &features.feature {
                    writeln!(
                        f,
                        "    FeatureDef {{ bit: {}, code: {}, name: {}, summary: {} }},",
                        feature.bit,
                        StringDisplay(&feature.code),
                        StringDisplay(&feature.name),
                        StringDisplay(&feature.summary)
                    )?;
                }
            }
            writeln!(f, "  ],")?;
            writeln!(f, "  attributes: &[")?;
            for attr in self
                .attributes
                .as_ref()
                .iter()
                .flat_map(|i| i.attribute.iter())
            {
                writeln!(f, "    attr::{}::ATTR,", attr.name)?;
            }
            writeln!(f, "  ],")?;
            writeln!(f, "  events: &[")?;
            for event in self.events.as_ref().iter().flat_map(|events| &events.event) {
                writeln!(f, "    event::{}::EVENT,", event.name)?;
            }
            writeln!(f, "  ],")?;
            writeln!(f, "  commands: &[")?;
            for command in self.commands.as_ref().iter().flat_map(|commands| &commands.command) {
                if command.is_to_server() {
                    writeln!(f, "    command::{}::COMMAND,", command.name)?;
                }
            }
            writeln!(f, "  ],")?;
            writeln!(f, "}};")?;
        }

        if let Some(attributes) = &self.attributes {
            writeln!(f, "pub mod attr {{")?;
            writeln!(f, "  use super::*;")?;
            for attr in &attributes.attribute {
                writeln!(f, "  pub struct {};", attr.name)?;
                writeln!(f, "")?;
                writeln!(f, "  impl Attr for {} {{", attr.name)?;
                writeln!(f, "    const ATTR: AttrDef = AttrDef {{")?;
                writeln!(f, "      id: {},", attr.id)?;
                writeln!(f, "      name: {},", attr.name.string())?;
                writeln!(f, "      attr_ty: Self::Value::ATTR_TYPE,")?;
                writeln!(
                    f,
                    "      conformance: {},",
                    attr.conformance.conformance(self.features())
                )?;
                writeln!(f, "    }};")?;
                let type_name = attr.ty.type_name(true);
                if attr.is_nullable() {
                    writeln!(f, "    type Value = Nullable<{}>;", type_name)?;
                } else {
                    writeln!(f, "    type Value = {};", type_name)?;
                }
                writeln!(f, "  }}")?;
                writeln!(f, "")?;
                writeln!(f, "")?;
            }
            writeln!(f, "}}")?;
        }

        if let Some(events) = &self.events {
            writeln!(f, "pub mod event {{")?;
            writeln!(f, "  use super::*;")?;

            for event in &events.event {
                let event = event.entry(self.features());
                writeln!(f, "{}", event)?;
            }

            writeln!(f, "}}")?;
        }

        if let Some(commands) = &self.commands {
            writeln!(f, "pub mod command {{")?;
            writeln!(f, "  use super::*;")?;

            for command in &commands.command {
                let command = command.entry(self.features());
                writeln!(f, "{}", command)?;
            }

            writeln!(f, "}}")?;
        }


        static HARDCODED_IMPORTS: LazyLock<HashMap<&'static str, &'static[&'static str]>> = LazyLock::new(|| {
            HashMap::from_iter([
                ("fixed_label_cluster", &["label_cluster"] as &'static[&'static str]),
                ("user_label_cluster", &["label_cluster"] as &'static[&'static str]),
                ("channel", &["content_launcher"]),
                ("content_launcher", &["media_playback"]),
            ])
        });
        

        writeln!(f, "pub mod ty {{")?;
        writeln!(f, "  pub use crate::globals::*;")?;
        if let Some(hardcoded_imports) = HARDCODED_IMPORTS.get(self.cluster_file_name.as_ref().unwrap().as_str()) {
            for import in *hardcoded_imports {
                writeln!(f, "  pub use crate::clusters::{}::ty::*;", import)?;
            }
        }
        if let Some(data_types) = &self.data_types {
            writeln!(f, "  use super::*;")?;
            for r#enum in &data_types.enums {
                Display::fmt(&r#enum, f)?;
            }
            for bitmap in &data_types.bitmap {
                Display::fmt(&bitmap, f)?;
            }
            for struct_ty in &data_types.structs {
                let struct_ty = struct_ty.entry(self.features());
                Display::fmt(&struct_ty, f)?;
            }
        }
        writeln!(f, "}}")?;

        Ok(())
    }
}

const DEFAULT_DERIVE: &'static str = "#[derive(Debug, Clone, PartialEq)]";

#[nest_struct]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnumDataType {
    #[serde(rename = "@name")]
    name: Ident,
    #[serde(rename = "item")]
    #[serde(default)]
    items: Vec<
        nest! {
            #[serde(rename = "@value")]
            value: IntValue,
            #[serde(rename = "@name")]
            name: Ident,
            #[serde(rename = "@summary")]
            summary: Option<String>,
        },
    >,
}

impl Display for EnumDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{DEFAULT_DERIVE}")?;
        writeln!(f, "#[derive(Copy)]")?;
        writeln!(f, "#[allow(non_camel_case_types)]")?;
        writeln!(f, "pub enum {} {{", self.name)?;
        for item in &self.items {
            if let Some(summary) = &item.summary {
                writeln!(f, "  ///{}", summary)?;
            }
            writeln!(f, "  {} = {},", item.name, item.value)?;
        }
        writeln!(f, "}}")?;

        writeln!(f, "impl AttrType for {} {{", self.name)?;
        writeln!(f, "  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[")?;
        for item in &self.items {
            writeln!(f, "  ({}, \"{}\"),", item.value, item.name)?;
        }
        writeln!(f, "  ]);")?;
        writeln!(f, "}}")?;

        writeln!(f, "impl TlvPut for {} {{", self.name)?;
        writeln!(f, "  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {{")?;
        writeln!(f, "    writer.put_uint(tag, *self as u64).unwrap();")?;
        writeln!(f, "  }}")?;
        writeln!(f, "}}")?;

        writeln!(f, "impl TryFrom<u64> for {} {{", self.name)?;
        writeln!(f, "  type Error = ();")?;
        writeln!(f, "  fn try_from(value: u64) -> Result<Self, ()> {{")?;
        writeln!(f, "    match value {{")?;
        for item in &self.items {
            writeln!(f, "      {} => Ok(Self::{}),", item.value, item.name)?;
        }
        writeln!(f, "      _ => Err(())")?;
        writeln!(f, "    }}")?;
        writeln!(f, "  }}")?;
        writeln!(f, "}}")?;

        writeln!(f, "impl TlvDecode for {} {{", self.name)?;
        writeln!(f, "  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {{")?;
        writeln!(f, "    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)")?;
        writeln!(f, "  }}")?;
        writeln!(f, "}}")?;

        Ok(())
    }
}

#[nest_struct]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BitmapDataType {
    #[serde(rename = "@name")]
    name: Ident,
    #[serde(rename = "bitfield")]
    bitfields: Vec<
        nest! {
            #[serde(rename = "@bit")]
            bit: Option<IntValue>,
            #[serde(rename = "@from")]
            from: Option<IntValue>,
            #[serde(rename = "@to")]
            to: Option<IntValue>,
            #[serde(rename = "@name")]
            name: Ident,
            #[serde(rename = "@summary")]
            summary: Option<String>,
        },
    >,
}

impl Display for BitmapDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "bitflags::bitflags! {{")?;
        writeln!(f, "  {DEFAULT_DERIVE}")?;
        writeln!(f, "  #[derive(Copy)]")?;
        writeln!(f, "  pub struct {}: u32 {{", self.name)?;
        for item in &self.bitfields {
            if let Some(summary) = &item.summary {
                writeln!(f, "    ///{}", summary)?;
            }
            if let Some(bit) = &item.bit {
                writeln!(f, "    const {} = 1 << {};", item.name, bit.decimal())?;
            } else if let (Some(from), Some(to)) = (&item.from, &item.to) {
                write!(f, "    const {} = (0", item.name)?;
                for i in from.0..=to.0 {
                    write!(f, "| {}", Decimal(i))?;
                }
                writeln!(f, ");")?;
            } else {
                panic!("bit or from+to must be set")
            }
        }
        writeln!(f, "  }}")?;
        writeln!(f, "}}")?;

        writeln!(f, "impl AttrType for {} {{", self.name)?;
        writeln!(f, "  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[")?;
        for item in &self.bitfields {
            if let Some(bit) = &item.bit {
                writeln!(f, "    ({}, \"{}\"),", bit.decimal(), item.name)?;
            } else if let (Some(from), Some(to)) = (&item.from, &item.to) {
                for i in from.0..=to.0 {
                    writeln!(f, "    ({}, \"{}\"),", Decimal(i), item.name)?;
                }
            } else {
                panic!("bit or from+to must be set")
            }
        }
        writeln!(f, "  ]);")?;
        writeln!(f, "}}")?;


        writeln!(f, "impl TlvPut for {} {{", self.name)?;
        writeln!(f, "  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {{")?;
        writeln!(f, "    writer.put_uint(tag, self.bits() as u64).unwrap();")?;
        writeln!(f, "  }}")?;
        writeln!(f, "}}")?;

        writeln!(f, "impl From<u64> for {} {{", self.name)?;
        writeln!(f, "  fn from(value: u64) -> Self {{")?;
        writeln!(f, "    Self::from_bits_retain(value as u32)")?;
        writeln!(f, "  }}")?;
        writeln!(f, "}}")?;

        
        writeln!(f, "impl TlvDecode for {} {{", self.name)?;
        writeln!(f, "  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {{")?;
        writeln!(f, "    Ok(Self::from(u64::tlv_decode(element, reader)?))")?;
        writeln!(f, "  }}")?;
        writeln!(f, "}}")?;

        Ok(())
    }
}

#[nest_struct]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructDataTypeEntry {
    #[serde(rename = "@name")]
    name: Ident,
    #[serde(rename = "field")]
    fields: Vec<FieldEntry>,
}

pub struct StructDataType<'a> {
    name: &'a Ident,
    fields: Vec<Field<'a>>
}

impl StructDataTypeEntry {
    pub fn entry(&self, features: FeatureList) -> StructDataType<'_> {
        StructDataType { name: &self.name, fields: self.fields.iter().map(|field| field.entry_no_prefix(features)).collect() }
    }
}

impl Display for StructDataType<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{DEFAULT_DERIVE}")?;
        writeln!(f, "pub struct {} {{", self.name)?;
        for field in &self.fields {
            writeln!(
                f,
                "  pub {}: {},",
                field.name.snake_case(),
                field.ty
            )?;
        }
        writeln!(f, "}}")?;

        writeln!(f, "impl AttrType for {} {{", self.name)?;
        writeln!(
            f,
            "  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct {{ fields: &["
        )?;
        for field in &self.fields {
            writeln!(
                f,
                "    AttrDef {{ id: {}, name: \"{}\", attr_ty: <{}>::ATTR_TYPE, conformance: Conformance::Mandatory }},",
                field.id,
                field.name,
                field.ty
            )?;
        }
        writeln!(f, "  ]}});")?;
        writeln!(f, "}}")?;


        writeln!(f, "impl TlvPut for {} {{", self.name)?;
        writeln!(f, "  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {{")?;
        writeln!(f, "    writer.start_structure(tag).unwrap();")?;
        for field in &self.fields {
            writeln!(f, "    self.{}.tlv_put(matter_codec::Tag::Context({}), writer);", field.name.snake_case(), field.id)?;
        }
        writeln!(f, "    writer.end_container().unwrap();")?;
        writeln!(f, "  }}")?;
        writeln!(f, "}}")?;

        writeln!(f, "{}", ImplTlvDecodeContainer { name: &self.name, fields: &self.fields })?;

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
struct IntValue(u32);

impl<'de> Deserialize<'de> for IntValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        let (num, radix) = if let Some(hex) = value.strip_prefix("0x") {
            (hex, 16)
        } else {
            (value.as_str(), 10)
        };
        use serde::de::Error;
        let id = u32::from_str_radix(num, radix).map_err(|err| D::Error::custom(err))?;

        Ok(Self(id))
    }
}

impl IntValue {
    pub fn decimal(&self) -> Decimal {
        Decimal(self.0)
    }
}

impl Display for IntValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:04x}", self.0)
    }
}

struct Decimal(u32);

impl Display for Decimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Default)]
struct BoolValue(bool);

impl<'de> Deserialize<'de> for BoolValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        use serde::de::Error;
        let value = bool::from_str(&value).map_err(|err| D::Error::custom(err))?;

        Ok(Self(value))
    }
}

#[derive(Debug, Clone)]
struct Ident(String);

impl Ident {
    pub fn string(&self) -> StringDisplay<'_, Self> {
        StringDisplay(&self)
    }

    pub fn snake_case(&self) -> String {
        let name = snake_case(&self.0);
        match name.as_str() {
            "type" | "use" | "match" => return format!("r#{}", name),
            _ => name,
        }
    }
}

impl<'de> Deserialize<'de> for Ident {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut value = String::deserialize(deserializer)?;

        value = value.replace("-", "_");
        if value.starts_with(char::is_numeric) {
            value = format!("_{}", value);
        }

        Ok(Self(value))
    }
}

impl Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

struct StringDisplay<'a, T: Display>(&'a T);

impl<'a, T: Display> Display for StringDisplay<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.0.to_string().replace('"', "\\\""))
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalsStructs {
    #[serde(rename = "struct")]
    pub structs: Vec<StructDataTypeEntry>,
}

impl GlobalsStructs {
    pub fn parse(content: &str) -> Result<Self, quick_xml::de::DeError> {
        quick_xml::de::from_str(content)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalsEnums {
    #[serde(rename = "enum")]
    pub enums: Vec<EnumDataType>,
}

impl GlobalsEnums {
    pub fn parse(content: &str) -> Result<Self, quick_xml::de::DeError> {
        quick_xml::de::from_str(content)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalsBitmaps {
    #[serde(rename = "bitmap")]
    pub bitmaps: Vec<BitmapDataType>,
}

impl GlobalsBitmaps {
    pub fn parse(content: &str) -> Result<Self, quick_xml::de::DeError> {
        quick_xml::de::from_str(content)
    }
}
