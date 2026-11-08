use rustigram::{
    bot::{DispatcherBuilder, filter::filter_fn},
    prelude::{filters, handler_fn},
    types::UpdateKind,
};

use crate::{app::state::AppState, handlers};

pub fn register_routes(builder: DispatcherBuilder, state: AppState) -> DispatcherBuilder {
    let connection_state = state.clone();

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
            filter_fn(|ctx| matches!(&ctx.update.kind, UpdateKind::BusinessMessage(_))),
            handler_fn(|ctx| async move { handlers::business::message::handle_message(ctx).await }),
        )
}
