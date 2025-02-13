use api::v1;
use kube::CustomResourceExt;

mod api;

fn main() {
    print!("{}", serde_yaml::to_string(&v1::dds_recv::DdsRecv::crd()).unwrap())
}