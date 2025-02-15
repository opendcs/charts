use api::v1::dds_recv::DdsConnection;
//use futures::{StreamExt, TryStreamExt};
use kube::{Client, api::{Api, ListParams }};//, PostParams}};
use std::fs::File;
use std::error::Error;
use simple_xml_builder::XMLElement;


mod api;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// The path to write output to
    #[arg(short, long, default_value = "./")]
    conf_dir: std::path::PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    // Infer the runtime environment and try to create a Kubernetes Client
    let client = Client::try_default().await?;
    let file = File::create(args.conf_dir.join("ddsrecv.conf"))?;
    let ret = create_ddsrecv_conf(client, &file).await;
    Ok(ret?)
}

async fn create_ddsrecv_conf(client: Client, file: &File) -> Result<(), Box<dyn Error>> {

    let mut ddsrecv_conf = XMLElement::new("ddsrecvconf");
    let mut i: i32 = 0;
    // Read pods in the configured namespace into the typed interface from k8s-openapi
    let connections: Api<DdsConnection> = Api::default_namespaced(client);
    // NOTE: review error handling more. No connections is reasonable, need
    // to make sure this would always just be empty and figure out some other error conditions.
    for host in connections.list(&ListParams::default()).await? {
        println!("found dds {}", host.spec.hostname);
        let mut connection = XMLElement::new("connection");
        connection.add_attribute("number", i);
        connection.add_attribute("host", host.spec.hostname);
        let mut enabled = XMLElement::new("enabled");
        enabled.add_text(host.spec.enabled.unwrap_or(true).to_string());
        
        let mut port = XMLElement::new("port");
        port.add_text(host.spec.port);

        let mut name = XMLElement::new("name");
        name.add_text(host.spec.name);
        
        let mut username = XMLElement::new("username");
        if host.spec.secret.is_some() {
            // get from secret and setup password
        } else {
            username.add_text(host.spec.username.unwrap());
            // also setup password
        }

        let mut authenticate = XMLElement::new("authenticate");
        authenticate.add_text("true");

        connection.add_child(port);
        connection.add_child(name);
        connection.add_child(username);
        connection.add_child(authenticate);


        ddsrecv_conf.add_child(connection);
        i = i + 1;
    }
    print!("{}", ddsrecv_conf);
    Ok(ddsrecv_conf.write(file)?)
}