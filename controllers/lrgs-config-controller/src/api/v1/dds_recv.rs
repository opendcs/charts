use std::fmt::Debug;

use schemars::{schema::SchemaObject, JsonSchema};
use serde::{Deserialize, Serialize};
use kube::CustomResource;
use garde::Validate;
use schemars::visit::{Visitor, visit_schema_object};

pub struct MyVisitor;

impl Visitor for MyVisitor {
    fn visit_schema_object(&mut self, schema: &mut SchemaObject) {
        schema.extensions.insert("test".to_string(), serde_json::json!("a test"));
        visit_schema_object(self, schema);
    }
}

// Our custom resource
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema, Validate)]
#[kube(group = "lrgs.opendcs.org", version = "v1", kind = "DdsConnection", namespaced)]
#[serde(rename_all = "camelCase")]
#[schemars(schema_with="add_one_of")]
pub struct DdsConnectionSpec {
    #[garde(ascii, length(min=1))]
    pub name: String,
    #[garde(ascii, length(min=1))]
    pub hostname: String,
    #[serde(default = "port_default")]
    #[garde(range(min=1, max=65535))]
    pub port: i32,
    #[garde(skip)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[garde(skip)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[garde(skip)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[garde(skip)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>
}

#[allow(unused)]
fn add_one_of(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
    let mut schema = String::json_schema(gen);
    
    // doesn't seem to provide the control desired.
    
    schema
}

fn port_default() -> i32 {
    16003
}
