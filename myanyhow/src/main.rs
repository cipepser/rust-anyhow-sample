extern crate anyhow;
extern crate serde;
extern crate serde_json;
extern crate thiserror;

use anyhow::{Context, Result};
use serde::{Serialize, Deserialize};
use thiserror::Error;
use crate::ClusterMapError::InvalidGroup;

#[derive(Serialize, Deserialize, Debug)]
struct ClusterMap {
    name: String,
    group: i32,
}

#[derive(Error, Debug)]
pub enum ClusterMapError {
    #[error("Invalid range of group (expected in 0-100), got {0}")]
    InvalidGroup(i32),
}

impl ClusterMap {
    fn validate(self) -> Result<Self> {
        if self.group < 0 || self.group > 100 {
            Err(InvalidGroup(self.group).into())
        } else {
            Ok(self)
        }
    }
}

fn get_cluster_info(path: &str) -> Result<ClusterMap> {
    let config = std::fs::read_to_string(&path)
        .with_context(|| format!("failed to read config file: {}", path))?;
    let map: ClusterMap = serde_json::from_str(&config)?;
    let map = map.validate()?;
    Ok(map)
}


fn main() {
    let _ = match get_cluster_info("cluster.json") {
        Ok(cm) => println!("{:?}", cm),
        Err(err) => println!("{:?}", err),
    };
}
