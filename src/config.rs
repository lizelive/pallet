use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use url::quirks::port;

use crate::errors::Error;
struct Network;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PassAss {
    /// generate a json file
    ConnectionFileArg,
    /// set every value as an enviorment varible
    Env,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Protocol {
    Tcp,
    Udp,
}
impl Default for Protocol {
    
}
#[derive(Serialize, Deserialize, Debug, Clone, Getters)]
pub struct Port {
    /// dns label, up to 10 characters
    /// unique per task
    /// used to show user what they have running
    name: String,
    /// which port. if not set auto assign
    port: Option<u16>,
    protocol: Protocol,
    /// the host to use
    addr: IpAddr,
}

impl Default for Port {
    fn default() -> Self {
        Self {
            name: Default::default(),
            port: Default::default(),
            protocol: Protocol::Tcp,
            addr: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    ports: Vec<Port>,
    secrets: HashMap<String, String>,
    pass_as: Option<PassAss>,
}

impl Config {
    fn to_map(&mut self) -> Result<HashMap<String, String>, Error>  {
        let mut out = self.secrets.clone();
        self.ports = Config::find_ports(self.ports)?;
        Ok(out)
    }

    fn find_ports(ports: Vec<Port>) -> Result<Vec<Port>, Error> {
        {
            let sockets = Vec::new();
            for mut port in ports {
                let socket = std::net::UdpSocket::bind((port.addr.clone(), 0))?;
                port.port = Some(socket.local_addr()?.port());
                sockets.push(socket);
            }
            Ok::<_, std::io::Error>(ports)
        }
        .or(Err(Error::NetConfigFailed))
    }
}
