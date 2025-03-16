use std::collections::BTreeMap;

use k8s_openapi::{api::{apps::v1::{StatefulSet, StatefulSetSpec}, core::v1::{PersistentVolumeClaim, PersistentVolumeClaimSpec, PersistentVolumeClaimTemplate, PodTemplateSpec, VolumeResourceRequirements}}, apimachinery::pkg::{api::resource::Quantity, apis::meta::v1::{LabelSelector, OwnerReference}}};
use kube::{api::ObjectMeta, Resource};

use crate::api::v1::lrgs::LrgsCluster;



pub fn create_statefulset(lrgs_spec: &LrgsCluster) -> StatefulSet {
    let owner_ref = lrgs_spec.controller_owner_ref(&()).unwrap();

    let mut labels: BTreeMap<String,String> = BTreeMap::new();
    labels.insert("app.kubernetes.io/name".to_string(), "lrgs".to_string());

    let pod_spec = pod_spec_template(lrgs_spec, &owner_ref, &labels);
    let pvct = claim_templates(lrgs_spec, &owner_ref, &labels);


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
        volume_claim_templates: Some(pvct), 
    };

    StatefulSet {
        metadata: ObjectMeta {
            name: lrgs_spec.metadata.name.clone(),
            owner_references: Some(vec![owner_ref]),
            labels: Some(labels.clone()),
            ..ObjectMeta::default()
        },
        spec: Some(the_spec),
        ..Default::default()
    }
}


fn pod_spec_template(_lrgs_spec: &LrgsCluster, owner_ref: &OwnerReference, labels: &BTreeMap<String,String>) -> PodTemplateSpec {
    PodTemplateSpec {
        metadata: Some(ObjectMeta {
            labels: Some(labels.clone()),            
            owner_references: Some(vec![owner_ref.clone()]),
            annotations: None,
            creation_timestamp: None,
            deletion_grace_period_seconds: None,
            deletion_timestamp: None,
            finalizers: None,
            generate_name: None,
            generation: None,            
            resource_version: None,
            self_link: None,
            uid: None,
            managed_fields: None,
            name: None,
            namespace: None,
        }),
        spec: None,
    }
}

fn claim_templates(lrgs_spec: &LrgsCluster, owner_ref: &OwnerReference, _labels: &BTreeMap<String,String>) -> Vec<PersistentVolumeClaim> {
    vec![
        PersistentVolumeClaim {
            metadata: ObjectMeta {
                name: Some("archive".to_string()),
                owner_references: Some(vec![owner_ref.clone()]),
                ..Default::default()
            },
            spec: Some(PersistentVolumeClaimSpec {
                storage_class_name: Some(lrgs_spec.spec.storage_class.clone()),
                resources: Some(VolumeResourceRequirements { 
                    limits: None, 
                    requests: Some(
                        BTreeMap::from(
                            [("storage".to_string(), Quantity(lrgs_spec.spec.storage_size.clone()))]
                        )
                    ),
                }),
                access_modes: Some(vec!["ReadWriteOnce".to_string()]),
                ..Default::default()
            }),
            status: None,
        }
    ]
}