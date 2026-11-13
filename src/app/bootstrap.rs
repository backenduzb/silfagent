use anyhow::{Context, Result};
use rustigram::prelude::{Bot, WebhookConfig};

use crate::{
    app::state::AppState, config::settings::Env, db::connect, routes::register::register_routes,
};

pub async fn run() -> Result<()> {
    let settings = Env::load().context("failed to load application configuration")?;
    let db = connect(&settings.dburl).await?;
    let state = AppState::new(db, settings.activity_url);

    let bot = Bot::new(&settings.tgtoken).context("failed to initialize Telegram bot")?;
    let dispatcher = register_routes(bot.dispatcher(), state).build();

    if settings.debug {
        dispatcher.polling().await.context("polling stopped")?;
    } else {
        let addr = settings.webaddr.context("debug")?;
        let url = settings.weburl.as_deref().context("debug")?;
        let secret = settings.websecret.as_deref();

        let mut set_webhook = bot.client.set_webhook(url);
        if let Some(secret) = secret {
            set_webhook = set_webhook.secret_token(secret);
        }
        set_webhook.await.context("cann't regidter webhook")?;

        let mut config = WebhookConfig::new(addr);
        if let Some(secret) = secret {
            config = config.secret_token(secret);
        }
        dispatcher
            .webhook(config)
            .await
            .context("webhook stopped")?;
    }
    Ok(())
}
