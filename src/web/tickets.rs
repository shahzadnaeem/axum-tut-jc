use axum::extract::{Path, State};
use axum::routing::{delete, post};
use axum::{Json, Router};

use crate::model::{ModelController, Ticket, TicketCreate};
use crate::Result;

async fn create_ticket(
    State(controller): State<ModelController>,
    Json(data): Json<TicketCreate>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - create_ticket", "HANDLER");

    let created = controller.create_ticket(data).await?;

    Ok(Json(created))
}

async fn get_tickets(State(controller): State<ModelController>) -> Result<Json<Vec<Ticket>>> {
    println!("->> {:<12} - get_tickets", "HANDLER");

    let tickets = controller.get_tickets().await?;

    Ok(Json(tickets))
}

async fn delete_ticket(
    State(controller): State<ModelController>,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - delete_ticket", "HANDLER");

    let deleted = controller.delete_ticket(id).await?;

    Ok(Json(deleted))
}

pub fn ticket_routes(controller: ModelController) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket).get(get_tickets))
        .route("/ticksts/:id", delete(delete_ticket))
        .with_state(controller)
}
