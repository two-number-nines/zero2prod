use config::Config;
#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub host: String,
    pub username: String,
    pub password: String,
    pub port: u16,
    pub database_name: String,
}

pub fn get_config() -> Result<Settings, config::ConfigError>{
    let settings = Config::builder()
        .add_source(
            config::File::new("config.yaml", config::FileFormat::Yaml)
        )
        .build()?;
    settings.try_deserialize::<Settings>()
}