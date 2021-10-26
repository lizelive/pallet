use std::collections::HashMap;

use serde::{Deserialize, Serialize};
struct Network;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PassAss {
    /// generate a json file
    ConnectionFileArg,
    /// set every value as an enviorment varible
    Env
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Protocol {
    Tcp,
    Udp,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Port {
    /// dns label, up to 10 characters
    /// unique per task
    /// used to show user what they have running
    name: String,
    /// which port. if not set auto assign
    port: Option<u16>,
    protocol: Protocol,
    /// the host to use
    host: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    ports: Vec<Port>,
    config: HashMap<String, String>,
    pass_as: Option<PassAss>,
}

impl Config {
    fn to_map(&self){

    }

    fn find_ports(&mut self){

    }
}

