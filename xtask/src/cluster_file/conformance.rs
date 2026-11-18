

use std::fmt::Display;

use nest_struct::nest_struct;
use serde::Deserialize;

use crate::cluster_file::ClusterFileFeaturesFeature;

#[nest_struct]
#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ConformanceFields {
    mandatory_conform: Option<
        nest! {
            feature: Option<nest! {
                #[serde(rename = "@name")]
                name: String
            }>
        },
    >,
    optional_conform: Option<nest! {
        
    }>,
}

pub type FeatureList<'a> = &'a [ClusterFileFeaturesFeature];

impl ConformanceFields {
    pub fn conformance(&self, features: FeatureList) -> Conformance {
        match (&self.mandatory_conform, &self.optional_conform) {
            (Some(mandatory), _) => {
                if let Some(feature) = mandatory.feature.as_ref() {
                    let feature = features
                        .iter()
                        .find(|f| f.code == feature.name)
                        .expect("unkown feature");
                    Conformance::Feature(feature.bit.0)
                } else {
                    Conformance::Mandatory
                }
            }
            (None, Some(_)) => Conformance::Optional,
            (None, None) => Conformance::TooComplicated,
        }
    }
}

pub enum Conformance {
    Mandatory,
    Feature(u32),
    Optional,
    TooComplicated,
}

impl Display for Conformance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Conformance::Mandatory => write!(f, "Conformance::Mandatory"),
            Conformance::Feature(feature) => write!(f, "Conformance::Feature({})", feature),
            Conformance::Optional => write!(f, "Conformance::Optional"),
            Conformance::TooComplicated => write!(f, "Conformance::TooComplicated"),
        }
    }
}