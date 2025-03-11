use std::collections::BTreeMap;

use k8s_openapi::{api::{apps::v1::{StatefulSet, StatefulSetSpec}, core::v1::PodTemplateSpec}, apimachinery::pkg::apis::meta::v1::LabelSelector};
use kube::{api::ObjectMeta, Resource};

use crate::api::v1::lrgs::LrgsCluster;



pub fn create_statefulset(lrgs_spec: &LrgsCluster) -> StatefulSet {
    let oref = lrgs_spec.controller_owner_ref(&()).unwrap();

    let mut labels: BTreeMap<String,String> = BTreeMap::new();
    labels.insert("app.kubernetes.io/name".to_string(), "lrgs".to_string());

    let pod_spec = PodTemplateSpec::default();
    let the_spec = StatefulSetSpec {
        replicas: Some(lrgs_spec.spec.replicas),
        selector: LabelSelector { 
            match_expressions: None, 
            match_labels: Some(labels.clone())
            },
        min_ready_seconds: Some(10),
        ordinals: None,
        persistent_volume_claim_retention_policy: None,
        pod_management_policy: None,
        revision_history_limit: None,
        service_name: "lrgs".to_string(),
        template: pod_spec,
        update_strategy: None,
        volume_claim_templates: None, 
    };

    StatefulSet {
        metadata: ObjectMeta {
            name: lrgs_spec.metadata.name.clone(),
            owner_references: Some(vec![oref]),
            labels: Some(labels.clone()),
            ..ObjectMeta::default()
        },
        spec: Some(the_spec),
        ..Default::default()
    }
}