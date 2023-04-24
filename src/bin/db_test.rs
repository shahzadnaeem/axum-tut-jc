use axum_tut::{database::Database, error::Result, model::tickets::Ticket};

#[tokio::main]
async fn main() -> Result<()> {
    let pool = Database::new().await;

    let tickets = get_tickets(&pool).await?;

    for ticket in tickets {
        println!(
            "Ticket ID: {}, Created_by_uid: {}, Title: {}, Done: {}",
            ticket.id, ticket.created_by_uid, ticket.title, ticket.done
        );
    }

    Ok(())
}

async fn get_tickets(db: &Database) -> Result<Vec<Ticket>> {
    let tickets = sqlx::query!(
        r#"SELECT id, created_by_uid, title, done
        FROM tickets
        ORDER BY id"#
    )
    .fetch_all(&db.pool)
    .await?;

    Ok(tickets
        .iter()
        .map(|rec| Ticket {
            id: rec.id as u64,
            created_by_uid: rec.created_by_uid as u64,
            title: rec.title.clone(),
            done: rec.done,
        })
        .collect())
}
