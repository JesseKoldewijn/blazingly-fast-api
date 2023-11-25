use std::env;

pub struct ServerConfig {
    pub port: u16,
}

impl ServerConfig {
    const STD_PORT: u16 = 8000;
    const PORT_ENV_VAR: &'static str = "PORT";

    #[allow(unused)]
    pub fn from_environment() -> ServerConfig {
        ServerConfig {
            port: Self::port_from_env(),
        }
    }

    pub fn port_from_env() -> u16 {
        env::var(Self::PORT_ENV_VAR)
            .map(|val| val.parse::<u16>().unwrap_or(Self::STD_PORT))
            .unwrap_or(Self::STD_PORT)
    }
}
