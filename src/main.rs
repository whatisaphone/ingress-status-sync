#![warn(future_incompatible, rust_2018_compatibility, rust_2018_idioms, unused)]
#![warn(clippy::pedantic)]
// #![warn(clippy::cargo)]
#![cfg_attr(feature = "strict", deny(warnings))]

use k8s_openapi::api::{
    core::v1::{Endpoints, Node},
    networking::v1::Ingress,
};
use kube::{
    api::{Patch, PatchParams},
    Api,
    Client,
};
use serde_json::json;
use std::error::Error;
use tracing::{debug, info};

const MANAGER: &str = "ingress-status-sync.wiaph.one";
const ANNOTATION: &str = "ingress-status-sync.wiaph.one/enabled";
const TARGET_NAMESPACE: &str = "ingress-nginx";
const TARGET_SERVICE: &str = "ingress-nginx-controller";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt().with_env_filter("info").init();

    let client = Client::try_default().await?;

    let target_ips = get_target_ips(&client).await?;
    info!(?target_ips);

    for ing in Api::all(client.clone()).list(&<_>::default()).await? {
        process_ingress(&client, &ing, target_ips.iter().map(String::as_str)).await?;
    }
    Ok(())
}

async fn get_target_ips(client: &Client) -> Result<Vec<String>, Box<dyn Error>> {
    let eps = <Api<Endpoints>>::namespaced(client.clone(), TARGET_NAMESPACE)
        .get(TARGET_SERVICE)
        .await?;
    let node_names = eps
        .subsets
        .into_iter()
        .flatten()
        .flat_map(|ss| ss.addresses)
        .flatten()
        .flat_map(|ea| ea.node_name);
    let mut result = Vec::new();
    for node_name in node_names {
        let node = <Api<Node>>::all(client.clone()).get(&node_name).await?;
        for address in node
            .status
            .unwrap()
            .addresses
            .into_iter()
            .flatten()
            .filter(|a| a.type_ == "InternalIP")
        {
            result.push(address.address);
        }
    }
    Ok(result)
}

async fn process_ingress(
    client: &Client,
    ing: &Ingress,
    target_ips: impl IntoIterator<Item = &str>,
) -> Result<(), Box<dyn Error>> {
    let namespace = ing.metadata.namespace.as_deref().unwrap();
    let name = ing.metadata.name.as_deref().unwrap();
    let enabled = ing
        .metadata
        .annotations
        .as_ref()
        .map_or(false, |anns| anns.contains_key(ANNOTATION));
    if !enabled {
        debug!(namespace, ingress = name, "skipping ingress");
        return Ok(());
    }
    info!(namespace, ingress = name, "setting ingress status");
    let status_list: Vec<_> = target_ips
        .into_iter()
        .map(|ip| json!({ "ip": ip }))
        .collect();
    let patch = json!({
        "status": {
            "loadBalancer": {
                "ingress": status_list,
            },
        },
    });
    <Api<Ingress>>::namespaced(client.clone(), &namespace)
        .patch_status(name, &PatchParams::apply(MANAGER), &Patch::Merge(patch))
        .await?;
    Ok(())
}
