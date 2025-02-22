
use crate::api::v1::dds_recv::DdsConnection;
use k8s_openapi::api::core::v1::Secret;
//use futures::{StreamExt, TryStreamExt};
use kube::{
    api::{Api, ListParams},
    Client,
};
use simple_xml_builder::XMLElement;
//, PostParams}};

use std::error::Error;
use std::fs::File;

use super::password_file;


pub async fn create_ddsrecv_conf(client: Client, file: File) -> Result<(), Box<dyn Error>> {
    let mut ddsrecv_conf = XMLElement::new("ddsrecvconf");
    let mut i: i32 = 0;
    // Read pods in the configured namespace into the typed interface from k8s-openapi
    let connections: Api<DdsConnection> = Api::default_namespaced(client.clone());
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
        username.add_text(host.spec.username);

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

pub async fn create_password_file(client: Client, file: File) -> Result<(), Box<dyn Error>> {
    let users: Api<Secret> = Api::default_namespaced(client.clone());
    let params = ListParams::default().fields("type=lrgs.opendcs.org/ddsuser");
    let mut pw_file = password_file::PasswordFile::new(file);
    for user in users.list(&params).await? {
        let data = user.data;
        if data.is_some() {
            let data = data.unwrap();
            let username = String::from_utf8(data.get("username").unwrap().0.clone())?;
            let password = String::from_utf8(data.get("password").unwrap().0.clone())?;
            let roles = data.get("roles");
            let roles = match roles {
                Some(_) => String::from_utf8(roles.unwrap().0.clone())?
                    .split(",")
                    .map(|i| String::from(i))
                    .collect(),
                None => vec![],
            };
            pw_file.add_user(password_file::DdsUser {
                username,
                password,
                roles,
            });
        }
    }
    Ok(pw_file.write_file()?)
}
