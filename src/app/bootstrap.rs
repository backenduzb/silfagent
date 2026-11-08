use anyhow::{Context, Result};
use rustigram::prelude::{Bot, WebhookConfig};

use crate::{
    app::state::AppState, config::settings::Env, db::connect, routes::register::register_routes,
};

pub async fn run() -> Result<()> {
    let settings = Env::load().context("failed to load application configuration")?;
    let db = connect(&settings.dburl).await?;
    let state = AppState::new(db);

    let bot = Bot::new(&settings.tgtoken).context("failed to initialize Telegram bot")?;
    let dispatcher = register_routes(bot.dispatcher(), state).build();

    if settings.debug {
        dispatcher.polling().await.context("polling stopped")?;
    } else {
        let addr = settings
            .webaddr
            .context("webhook address is required when DEBUG=false")?;
        let secret = settings
            .websecret
            .as_deref()
            .context("webhook secret is required when DEBUG=false")?;
        dispatcher
            .webhook(WebhookConfig::new(addr).secret_token(secret))
            .await
            .context("webhook stopped")?;
    }
    Ok(())
}
