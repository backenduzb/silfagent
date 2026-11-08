use dotenvy::dotenv;
use std::env;
use std::time::Duration;

pub struct Env {
    pub tgtoken: String,
    pub webaddr: Option<std::net::SocketAddr>,
    pub weburl: Option<String>,
    pub websecret: Option<String>,
    pub debug: bool,
    pub dburl: String,
}

pub const THINKING_TIME: Duration = Duration::from_secs(1);
pub const STREAM_INTERVAL: Duration = Duration::from_millis(150);
pub const CHUNK_SIZE: usize = 24;

impl Env {
    pub fn load() -> anyhow::Result<Self> {
        dotenv().ok();

        let tgtoken = env::var("BOT_TOKEN")?;
        let dburl = env::var("DATABASE_URL")?;
        let debug = env::var("DEBUG")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()?;

        let (webaddr, weburl, websecret) = if debug {
            (None, None, None)
        } else {
            (
                Some(env::var("WEBHOOK_HOST")?.parse()?),
                Some(env::var("WEBHOOK_URL")?),
                Some(env::var("WEBHOOK_SECRET")?),
            )
        };

        Ok(Self {
            tgtoken,
            webaddr,
            weburl,
            websecret,
            debug,
            dburl,
        })
    }
}
