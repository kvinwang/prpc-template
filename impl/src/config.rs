use rocket::figment::{
    providers::{Format, Toml},
    Figment,
};

pub const CONFIG_FILENAME: &str = "{{app_name}}.toml";
pub const SYSTEM_CONFIG_FILENAME: &str = "/etc/{{app_name}}/{{app_name}}.toml";
pub const DEFAULT_CONFIG: &str = include_str!("../{{app_name}}.toml");

pub fn load_config_figment() -> Figment {
    Figment::from(rocket::Config::default())
        .merge(Toml::string(DEFAULT_CONFIG).nested())
        .merge(Toml::file(SYSTEM_CONFIG_FILENAME).nested())
        .merge(Toml::file(CONFIG_FILENAME).nested())
}
