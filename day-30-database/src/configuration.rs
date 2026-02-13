// // ⛳️ Step 5 - Reading A Configuration File
// //
// // We have two groups of configuration values at the moment:
// // • the application port(currently hard-coded to 8000 in main.rs);
// // • the database connection parameters.

// #[derive(serde::Deserialize)]
// pub struct Settings {
//     pub database: DatabaseSettings,
//     pub application_port: u16,
// }

// #[derive(serde::Deserialize, Clone)]
// pub struct DatabaseSettings {
//     pub username: String,
//     pub password: String,
//     pub port: u16,
//     pub host: String,
//     pub database_name: String,
// }

// // // Let’s add config to our dependencies
// // [dependencies]
// // config= "0.14"

// // The we want to read our application settings from a configuration file named configuration.yaml:

// pub fn get_configuration() -> Result<Settings, config::ConfigError> {
//     // Initialise our configuration reader
//     let settings = config::Config::builder()
//         // Add configuration values from a file named `configuration.yaml`.
//         .add_source(config::File::new(
//             "configuration.yaml",
//             config::FileFormat::Yaml,
//         ))
//         .build()?;
//     // Try to convert the configuration values it read into
//     // our Settings type
//     settings.try_deserialize::<Settings>()
// }

// ⛳️ Step 6 - Connecting To Postgres

// Add a connection_string method to give PgConnection::connect a single connection rather than granular connection parameters in teh config yaml.

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialise our configuration reader
    let settings = config::Config::builder()
        // Add configuration values from a file named `configuration.yaml`.
        .add_source(config::File::new(
            "configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?;
    // Try to convert the configuration values it read into
    // our Settings type
    settings.try_deserialize::<Settings>()
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}
