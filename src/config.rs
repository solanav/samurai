use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::str::FromStr;

pub struct ConfigData {
    pub bind_ip: Ipv4Addr,
    pub debug_ip: Ipv4Addr,
    pub debug_port: u16,
}

pub fn read_config() -> ConfigData {
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("config")).unwrap()
        .merge(config::Environment::with_prefix("SAMURAI")).unwrap();

    let settings = settings.try_into::<HashMap<String, String>>().unwrap();

    ConfigData {
        bind_ip: Ipv4Addr::from_str(settings.get("bind_ip").unwrap()).unwrap(),
        debug_ip: Ipv4Addr::from_str(settings.get("debug_ip").unwrap()).unwrap(),
        debug_port: u16::from_str(settings.get("debug_port").unwrap()).unwrap(),
    }
}