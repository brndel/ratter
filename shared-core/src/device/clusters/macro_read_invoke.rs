#[macro_export]
macro_rules! invoke {
    ($node:ident, $endpoint:ident, $cluster:ident, $command:ident, $encode:ident($($params:expr),*)) => {
        $node.invoke_tlv(matter_controller::CommandPath {
                            endpoint: $endpoint,
                            cluster: matter_clusters::r#gen::$cluster::CLUSTER_ID,
                            command: matter_clusters::r#gen::$cluster::command_id::$command,
                        }, matter_clusters::r#gen::$cluster::$encode($($params),*))
    }
}

#[macro_export]
macro_rules! read_decode {
    ($node:ident, $endpoint:ident, [$($var_name:ident = {$cluster:ident, $attribute_id:ident, $decode_fn:ident}),*]) => {
        #[allow(unused_parens)]
        let ($($var_name),*) = {
            let values = $node.read(&[
                $(
                    matter_controller::ReadPath::concrete($endpoint, matter_clusters::r#gen::$cluster::CLUSTER_ID, matter_clusters::r#gen::$cluster::attribute_id::$attribute_id)
                ),*
            ]).await?;

            let mut tlv_bytes = Vec::new();
            $(
                let mut $var_name = None;
            )*

            for (path, value) in values {
                tlv_bytes.clear();
                let mut writer = matter_codec::TlvWriter::new(&mut tlv_bytes);
                writer
                    .write_value(matter_codec::Tag::Anonymous, &value)
                    .expect("writing to vec should not fail");

                match path {
                    $(
                        matter_controller::AttributePath {endpoint: _, cluster: matter_clusters::r#gen::$cluster::CLUSTER_ID, attribute: matter_clusters::r#gen::$cluster::attribute_id::$attribute_id } => {
                            $var_name = Some(matter_clusters::r#gen::$cluster::$decode_fn(&tlv_bytes)?);
                        }
                    ),*
                    _ => (),
                }
            }

            ($($var_name.expect("at least one argument did not get sent back")),*)
        };
    }
}

pub use {invoke, read_decode};
