use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::str::FromStr;

pub struct ConfigData {
    pub ip: Ipv4Addr,
}

pub fn read_config() -> ConfigData {
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("config")).unwrap()
        .merge(config::Environment::with_prefix("SAMURAI")).unwrap();

    let settings = settings.try_into::<HashMap<String, String>>().unwrap();

    ConfigData {
        ip: Ipv4Addr::from_str(settings.get("ip").unwrap()).unwrap(),
    }
}