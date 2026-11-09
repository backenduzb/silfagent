use rustigram::{
    bot::{DispatcherBuilder, filter::filter_fn},
    prelude::{filters, handler_fn},
    types::UpdateKind,
};

use crate::{app::state::AppState, handlers};

pub fn register_routes(builder: DispatcherBuilder, state: AppState) -> DispatcherBuilder {
    let connection_state = state.clone();
    let story_state = state.clone();
    let message_state = state.clone();
    let deleted_state = state.clone();
    let edited_state = state.clone();

    builder
        .on(
            filters::command("start"),
            handler_fn(move |ctx| {
                let state = state.clone();
                async move { handlers::start::start_with_state(ctx, state).await }
            }),
        )
        .on(
            filter_fn(|ctx| matches!(&ctx.update.kind, UpdateKind::BusinessConnection(_))),
            handler_fn(move |ctx| {
                let state = connection_state.clone();
                async move { handlers::business::connection::handle(ctx, state).await }
            }),
        )
        .on(
            filter_fn(|ctx| matches!(&ctx.update.kind, UpdateKind::Message(_))),
            handler_fn(move |ctx| {
                let state = story_state.clone();
                async move { handlers::business::story::handle_story(ctx, state).await }
            }),
        )
        .on(
            filter_fn(|ctx| matches!(&ctx.update.kind, UpdateKind::BusinessMessage(_))),
            handler_fn(move |ctx| {
                let state = message_state.clone();
                async move { handlers::business::message::handle_message(ctx, state).await }
            }),
        )
        .on(
            filter_fn(|ctx| matches!(&ctx.update.kind, UpdateKind::DeletedBusinessMessages(_))),
            handler_fn(move |ctx| {
                let state = deleted_state.clone();
                async move { handlers::business::deleted::handle_deleted(ctx, state).await }
            }),
        )
        .on(
            filter_fn(|ctx| matches!(&ctx.update.kind, UpdateKind::EditedBusinessMessage(_))),
            handler_fn(move |ctx| {
                let state = edited_state.clone();
                async move { handlers::business::edited::handle_edited(ctx, state).await }
            }),
        )
}
