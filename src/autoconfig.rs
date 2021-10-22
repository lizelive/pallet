struct Network;

enum KnowFormats {
    /// generate a json file
    Json,

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

struct RuntimeConfig {
    ports: Vec<Port>,
    secrets: HashMap<String, String>,
    pass_as: Option<KnowFormats>,
}