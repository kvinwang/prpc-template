use rocket::figment::{
    providers::{Format, Toml},
    Figment,
};

pub const CONFIG_FILENAME: &str = "{{app_name}}.toml";
pub const DEFAULT_CONFIG: &str = include_str!("../{{app_name}}.toml");

pub fn load_config_file() -> Figment {
    Figment::from(Toml::string(DEFAULT_CONFIG).nested()).merge(Toml::file(CONFIG_FILENAME).nested())
}
