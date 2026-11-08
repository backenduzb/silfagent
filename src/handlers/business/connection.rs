use crate::app::state::AppState;
use crate::db::repositories::business;
use crate::db::repositories::user;
use rustigram::prelude::*;
use rustigram::types::UpdateKind;

pub async fn handle(ctx: Context, state: AppState) -> BotResult<()> {
    let UpdateKind::BusinessConnection(connection) = &ctx.update.kind else {
        return Ok(());
    };

    let profile = user::upsert_profile(
        &state.db,
        connection.user.id,
        connection.user.username.clone(),
        connection.user.first_name.clone(),
    )
    .await?;

    business::upsert(
        &state.db,
        profile.id,
        connection.id.clone(),
        connection.is_enabled,
    )
    .await?;

    Ok(())
}
