use dotenvy::dotenv;
use std::env;
use std::net::SocketAddr;
use std::time::Duration;

pub struct Env {
    pub tgtoken: String,
    pub webaddr: Option<SocketAddr>,
    pub weburl: Option<String>,
    pub websecret: Option<String>,
    pub debug: bool,
    pub dburl: String,
    pub activity_url: String,
}

pub const THINKING_TIME: Duration = Duration::from_secs(1);
pub const STREAM_INTERVAL: Duration = Duration::from_millis(150);
pub const CHUNK_SIZE: usize = 24;

impl Env {
    pub fn load() -> anyhow::Result<Self> {
        dotenv().ok();

        let tgtoken = env::var("BOT_TOKEN")?;
        let dburl = env::var("DATABASE_URL")?;
        let activity_url = env::var("ACTIVITY_URL")?;
        let debug = env::var("DEBUG")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()?;

        let (webaddr, weburl, websecret) = if debug {
            (None, None, None)
        } else {
            let host = env::var("WEBHOOK_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
            let port = env::var("WEBHOOK_PORT")
                .or_else(|_| env::var("PORT"))
                .unwrap_or_else(|_| "8080".to_string())
                .parse::<u16>()?;
            let addr: SocketAddr = format!("{host}:{port}").parse()?;

            (
                Some(addr),
                Some(env::var("WEBHOOK_URL")?),
                env::var("WEBHOOK_SECRET")
                    .ok()
            )
        };

        Ok(Self {
            tgtoken,
            webaddr,
            weburl,
            websecret,
            debug,
            dburl,
            activity_url,
        })
    }
}
