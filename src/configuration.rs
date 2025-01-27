use secrecy::{ExposeSecret, SecretBox};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: SecretBox<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    config::Config::builder()
        .add_source(config::File::with_name("configuration"))
        .build()?
        .try_deserialize()
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> SecretBox<str> {
        SecretBox::from(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        ))
    }

    pub fn connection_string_without_db(&self) -> SecretBox<str> {
        SecretBox::from(format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port
        ))
    }
}
