use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DiffConfig {
     pub profiles: HashMap<String, DiffProfile>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiffProfile{

}