use std::collections::BTreeMap;

use k8s_openapi::{api::core::v1::ConfigMap, apimachinery::pkg::apis::meta::v1::OwnerReference, ByteString};
use kube::api::ObjectMeta;




pub fn created_script_config_map(namespace: String, owner_ref: &OwnerReference) -> ConfigMap {
    let script = String::from_utf8(Vec::from(include_bytes!("lrgs.sh"))).unwrap_or_default();
    ConfigMap {
        metadata: ObjectMeta {
            name: Some("lrgs-scripts".to_string()),
            namespace: Some(namespace),
            owner_references: Some(vec![owner_ref.clone()]),
            ..Default::default()
        },
        data: Some(
            BTreeMap::from([("lrgs.sh".to_string(), script)])
        ),
        ..Default::default()
    }
}