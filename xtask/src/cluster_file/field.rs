use std::fmt::Display;

use nest_struct::nest_struct;
use serde::Deserialize;

use crate::cluster_file::{
    Ident, IntValue, conformance::{Conformance, ConformanceFields, FeatureList}, ty::{TypeFields, TypeName},
};

#[nest_struct]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldEntry {
    #[serde(rename = "@id")]
    id: IntValue,
    #[serde(rename = "@name")]
    name: Ident,
    #[serde(flatten)]
    ty: TypeFields,
    #[serde(flatten)]
    conformance: ConformanceFields,
}

#[nest_struct]
pub struct Field<'a> {
    pub id: IntValue,
    pub name: Ident,
    pub ty: TypeName<'a>,
    pub conformance: Conformance,
}

impl FieldEntry {
    pub fn entry(&self, features: FeatureList) -> Field<'_> {
        Field {
            id: self.id,
            name: self.name.clone(),
            ty: self.ty.type_name(true),
            conformance: self.conformance.conformance(features),
        }
    }

    pub fn entry_no_prefix(&self, features: FeatureList) -> Field<'_> {
        Field {
            id: self.id,
            name: self.name.clone(),
            ty: self.ty.type_name(false),
            conformance: self.conformance.conformance(features),
        }
    }

}

impl Display for Field<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AttrDef {{ id: {}, name: {}, attr_ty: <{}>::ATTR_TYPE, conformance: {} }}", self.id, self.name.string(), self.ty, self.conformance)
    }
}
