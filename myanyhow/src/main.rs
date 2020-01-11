extern crate anyhow;
extern crate serde;
extern crate serde_json;

use anyhow::{Context, Result};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct ClusterMap {
    name: String,
    group: i32,
}

fn get_cluster_info(path: &str) -> Result<ClusterMap> {
    let config = std::fs::read_to_string(&path)
        .with_context(|| format!("failed to read config file: {}", path))?;
    let map: ClusterMap = serde_json::from_str(&config)?;
    Ok(map)
}

fn main() {
    let _ = match get_cluster_info("cluster.json") {
        Ok(cm) => println!("{:?}", cm),
        Err(err) => println!("{:?}", err),
    };
}
