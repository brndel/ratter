macro_rules! define_cluster {
    (struct $struct_name:ident, enum $enum_name:ident, $cluster_mod:ident, $cluster_id:ident {
        $($field_name:ident : $field_ty:ty => $attr_id:ident as $field_enum_variant:ident { $read_fn:ident, $decode_fn:ident }),*
    }) => {

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct $struct_name {
    $(
        pub $field_name: crate::device::clusters::DeviceValue<$field_ty>
    ),*
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
    impl crate::backend::ClusterState for super::$struct_name {
        const CLUSTER_ID: u32 = matc::clusters::defs::$cluster_id;
    }

    impl crate::backend::FromEndpoint for super::$struct_name {
        async fn from_endpoint(connection: &matc::controller::Connection, endpoint: u16) -> anyhow::Result<Self> {
            Ok(Self {
                $(
                    $field_name: crate::device::clusters::DeviceValue::new(matc::clusters::codec::$cluster_mod::$read_fn(connection, endpoint).await?.into())
                ),*
            })
        }
    }

    impl crate::backend::FromAttrChange for super::$enum_name {
        fn from_attr_change(attr: u32, value: &matc::tlv::TlvItemValue) -> anyhow::Result<Self> {
            let value = match attr {
                $(
                    matc::clusters::defs::$attr_id => Self::$field_enum_variant {
                        $field_name: matc::clusters::codec::$cluster_mod::$decode_fn(value)?.into()
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

pub(crate) use define_cluster;
