extern crate anyhow;
extern crate serde;
extern crate serde_json;

use anyhow::{Context, Result};
use serde::{Serialize, Deserialize};
use crate::ClusterMapError::InvalidGroup;

#[derive(Serialize, Deserialize, Debug)]
struct ClusterMap {
    name: String,
    group: i32,
}

impl ClusterMap {
    fn validate(self) -> Result<Self> {
        if self.group < 0 || self.group > 100 {
            Err(InvalidGroup(self.group).into())
//            Err(anyhow::format_err!("Invalid range of group (expected in 0-100), got {}", self.group))
//            anyhow::bail!(InvalidGroup(self.group))
        } else {
            Ok(self)
        }
    }
}

#[derive(Debug)]
pub enum ClusterMapError {
    //    #[error("Invalid range of group (expected in 0-100), got {0}")]
//    #[error(display = "Invalid range of group (expected in 0-100), got {0}")]
    InvalidGroup(i32),
}

impl std::error::Error for ClusterMapError {}

//impl From<anyhow::Error> for ClusterMapError {
//    fn from(err: anyhow::Error) -> Self { ClusterMapError::InvalidGroup(111) }
//}

//impl From<i32> for ClusterMapError {
//    fn from(t: i32) -> Self { ClusterMapError::InvalidGroup(t) }
//}


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
//        Err(err) => match err {
//            InvalidGroup(group) => println!("got {:?}", group),
//            _ => println!("{:?}", err),
//        }
        Err(err) => println!("{:?}", err),
    };
}
