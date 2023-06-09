use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, post};
use axum::{Json, Router};

use crate::context::Context;
use crate::model::tickets::{Ticket, TicketCreate};
use crate::web::controller::AppState;
use crate::Result;

async fn create_ticket(
    State(state): State<AppState>,
    context: Context,
    Json(data): Json<TicketCreate>,
) -> Result<(StatusCode, Json<Ticket>)> {
    println!("->> {:<12} - create_ticket", "HANDLER");

    let created = state.create_ticket(context, data).await?;

    Ok((StatusCode::CREATED, Json(created)))
}

async fn get_tickets(State(state): State<AppState>, context: Context) -> Result<Json<Vec<Ticket>>> {
    println!("->> {:<12} - get_tickets", "HANDLER");

    let tickets = state.get_tickets(context).await?;

    Ok(Json(tickets))
}

async fn delete_ticket(
    State(state): State<AppState>,
    context: Context,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - delete_ticket", "HANDLER");

    let deleted = state.delete_ticket(context, id).await?;

    Ok(Json(deleted))
}

pub fn ticket_routes(state: AppState) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket).get(get_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(state)
}
