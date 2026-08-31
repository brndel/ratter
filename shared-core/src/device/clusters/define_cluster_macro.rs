macro_rules! transform_field {
    ($field_name:ident => $transform:path) => {
        $transform($field_name)
    };
    ($field_name:ident => &$transform:path) => {
        $transform(&$field_name)
    };
    ($field_name:ident) => {
        $field_name.into()
    };
}

macro_rules! filter_listen {
    ($cluster:ident $attr_id:ident $listen:literal) => {
        Some(matter_clusters::r#gen::$cluster::attribute_id::$attr_id)
    };
    ($cluster:ident $attr_id:ident) => {
        None
    }
}


macro_rules! define_cluster {
    (struct $struct_name:ident, enum $enum_name:ident, $cluster:ident {
        $($field_name:ident : $field_ty:ty => $attr_id:ident $($listen:literal)? as $field_enum_variant:ident { $decode_fn:ident $(=> $transform:path)? }),*
    }) => {

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct $struct_name {
    $(
        pub $field_name: crate::device::clusters::DeviceValue<$field_ty>
    ),*
}

impl $struct_name {
    pub const LISTEN_ATTRS: &'static [Option<u32>] = &[
        $(
            crate::device::clusters::define_cluster_macro::filter_listen!($cluster $attr_id $($listen)?)
        ),*
    ];
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum $enum_name {
    $(
        $field_enum_variant {
            $field_name: $field_ty
        }
    ),*
}

impl ChangeEvent for $enum_name {
    type State = $struct_name;

    fn apply(self, state: &mut Self::State, source: crate::event::AttrChangeSource) {
        match self {
            $(
                Self::$field_enum_variant { $field_name } => match source {
                    crate::event::AttrChangeSource::Device => {
                        state.$field_name.device_value = $field_name;
                        state.$field_name.user_value = None;
                    },
                    crate::event::AttrChangeSource::User => state.$field_name.user_value = Some($field_name)
                }
            ),*
        }
    }
}

#[cfg(feature = "backend")]
mod backend_impl {
    #[allow(unused_imports)]
    use super::*;

    impl crate::backend::ClusterState for super::$struct_name {
        const CLUSTER_ID: u32 = matter_clusters::r#gen::$cluster::CLUSTER_ID;
    }

    impl crate::backend::FromEndpoint for super::$struct_name {
        async fn from_endpoint(node: &matter_controller::Node, endpoint: u16) -> anyhow::Result<Self> {
            crate::device::clusters::read_decode!(node, endpoint, [
                $(
                    $field_name = {$cluster, $attr_id, $decode_fn}
                ),*
            ]);

            Ok(Self {
                $(
                    $field_name: crate::device::clusters::DeviceValue::new(crate::device::clusters::define_cluster_macro::transform_field!($field_name $(=> $transform)?))
                ),*
            })
        }
    }

    impl crate::backend::FromAttrChange for super::$enum_name {
        fn from_attr_change(attr: u32, value: &matter_codec::Value) -> anyhow::Result<Self> {
            let mut tlv_bytes = Vec::new();
            let mut writer = matter_codec::TlvWriter::new(&mut tlv_bytes);
            writer.write_value(matter_codec::Tag::Anonymous, &value).expect("writing to vec should not fail");

            let value = match attr {
                $(
                    matter_clusters::r#gen::$cluster::attribute_id::$attr_id => Self::$field_enum_variant {
                        $field_name: {
                            let value = matter_clusters::r#gen::$cluster::$decode_fn(&tlv_bytes)?;
                            crate::device::clusters::define_cluster_macro::transform_field!(value $(=> $transform)?)
                        }
                    }
                ),*,
                _ => return Err(anyhow::anyhow!("unkown attr"))
            };

            Ok(value)
        }
    }
}
    }
}

pub(crate) use {define_cluster, transform_field, filter_listen};
