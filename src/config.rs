use std::{env, net::SocketAddr};

pub struct Config {
    pub addr: SocketAddr,
    pub database_url: String,
}

impl Config {
    /// HOST defaults to 0.0.0.0 because the app is reached from outside its container or VM.
    pub fn from_env() -> Result<Self, String> {
        let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into());
        let port = env::var("PORT").unwrap_or_else(|_| "3000".into());
        let addr = format!("{host}:{port}")
            .parse()
            .map_err(|e| format!("invalid HOST/PORT ({host}:{port}): {e}"))?;
        // mode=rwc creates the file on first run; the directory must exist (see main).
        let database_url =
            env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://data/app.db?mode=rwc".into());
        Ok(Self { addr, database_url })
    }
}
