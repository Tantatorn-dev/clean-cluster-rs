use futures::{StreamExt, TryStreamExt};
use kube::api::{Api, ResourceExt, ListParams, PostParams, WatchEvent};
use k8s_openapi::api::core::v1::Pod;
use kube::Client;

#[tokio::main]
async fn main() -> Result<(), kube::Error> {
    let client = Client::try_default().await?;

    let pods: Api<Pod> = Api::namespaced(client, "user-tsuksangwarn");

    let lp = ListParams::default()
            .fields(&format!("metadata.name={}", "shell-demo"))
            .timeout(10);
    let mut stream = pods.watch(&lp, "0").await?.boxed();

    while let Some(status) = stream.try_next().await? {
        match status {
            WatchEvent::Added(o) => println!("Added {}", o.name()),
            WatchEvent::Modified(o) => {
                let s = o.status.as_ref().expect("status exists on pod");
                let phase = s.phase.clone().unwrap_or_default();
                println!("Modified: {} with phase: {}", o.name(), phase);
            }
            WatchEvent::Deleted(o) => println!("Deleted {}", o.name()),
            WatchEvent::Error(e) => println!("Error {}", e),
            _ => {}
        }
    }

    Ok(())
}
