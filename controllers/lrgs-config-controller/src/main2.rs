use std::{sync::Arc, time::Duration};

use api::v1::{dds_recv::DdsConnection, lrgs::LrgsCluster};
use futures::StreamExt;
use k8s_openapi::api::{apps::v1::StatefulSet, core::v1::{ConfigMap, Secret}};
use kube::{api::{Patch, PatchParams}, runtime::{controller::Action, reflector::ObjectRef, watcher, Controller}, Api, Client, Error, Resource, ResourceExt};
use lrgs::{configmap::created_script_config_map, statefulset::create_statefulset};

mod api;
mod lrgs;

// Context for our reconciler
#[derive(Clone)]
struct Data {
    /// kubernetes client
    client: Client
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let args = Cli::parse();
    // Infer the runtime environment and try to create a Kubernetes Client
    let client = Client::try_default().await?;
    let secrets: Api<Secret> = Api::default_namespaced(client.clone());
    let lrgs_cluster: Api<LrgsCluster> = Api::default_namespaced(client.clone());
    let dds_connections: Api<DdsConnection> = Api::default_namespaced(client.clone());
    let stateful_set: Api<StatefulSet> = Api::default_namespaced(client.clone());
    let user_watch_config = watcher::Config::default().fields("type=LrgsCluster.opendcs.org/ddsuser");


    let context = Arc::new(Data {
        client: client.clone(),
    });

    let user_mapper = |obj: Secret| {
        let binding = obj.metadata.labels.unwrap();
        let name = binding.get("lrgs.opendcs.org/lrgs-cluster").unwrap();        
        let namespace = obj.metadata.namespace.unwrap_or("default".to_string());
        let obj_ref = ObjectRef::new(name).within(&namespace);
        Some(obj_ref)
    };

    let dds_mapper = |obj: DdsConnection| {
        let binding = obj.metadata.labels.unwrap();
        let name = binding.get("lrgs.opendcs.org/lrgs-cluster").unwrap();
        let namespace = obj.metadata.namespace.unwrap_or("default".to_string());
        let obj_ref = ObjectRef::new(name).within(&namespace);
        Some(obj_ref)
    };


    println!("Starting controller");
    Controller::new(lrgs_cluster.clone(), watcher::Config::default())
        .owns(stateful_set, watcher::Config::default())
        .owns(secrets.clone(), watcher::Config::default())
        .watches(secrets.clone(), user_watch_config, user_mapper)
        .watches(dds_connections.clone(), watcher::Config::default(), dds_mapper)
        .run(reconcile, error_policy , context)
        .for_each(|res| async move {
            match res {
                Ok(o) => println!("reconciled {:?}", o),
                Err(e) => println!("reconcile failed: {}", e),
            }
        })
        .await
        ;
    println!("Exiting.");
    Ok(())
}


async fn reconcile(object: Arc<LrgsCluster>, data: Arc<Data>) -> Result<Action, Error>  {
    println!("Processing {:?}",object.spec);
    let oref = object.controller_owner_ref(&()).unwrap();
    let client = &data.client;
    //let name = object.metadata.name.clone();
    let ns = object.metadata.namespace.clone().unwrap_or("default".to_string());
    let stateful_api: Api<StatefulSet> = Api::namespaced(client.clone(), &ns);
    let config_map_api: Api<ConfigMap> = Api::namespaced(client.clone(), &ns);
    let lrgs_config_map = created_script_config_map(ns.clone(), &oref);

    let lrgs_statefulset = create_statefulset(&object);
    println!("{lrgs_statefulset:?}");
    let serverside = PatchParams::apply("mycontroller");
    config_map_api.patch(&lrgs_config_map.name_any(), &serverside, &Patch::Apply(lrgs_config_map)).await?;
    stateful_api.patch(&lrgs_statefulset.name_any(), &serverside, &Patch::Apply(lrgs_statefulset)).await?;
    Ok(Action::requeue(Duration::from_secs(3600 / 2)))
}

fn error_policy(_object: Arc<LrgsCluster>, err: &Error, _ctx: Arc<Data>) -> Action {
    println!("Error {}", err);
    Action::requeue(Duration::from_secs(5))
}

