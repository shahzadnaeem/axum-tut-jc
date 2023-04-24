use chrono::prelude::*;
use core::time;
use std::thread;

use axum_tut::{database::Database, error::Result, model::tickets::Ticket};

#[tokio::main]
async fn main() -> Result<()> {
    let db = Database::new().await;

    let id = create_ticket(&db).await?;

    println!("\nAdded new Ticket ID: {}\n", id);

    let tickets = get_tickets(&db).await?;

    for ticket in tickets {
        println!(
            "Ticket ID: {}, Created_by_uid: {}, Title: {}, Done: {}",
            ticket.id, ticket.created_by_uid, ticket.title, ticket.done
        );
    }

    Ok(())
}

fn new_ticket_title() -> String {
    thread::sleep(time::Duration::from_secs(1));
    let local_now = Local::now();
    let title_text = local_now.format("Ticket created at %H:%M:%S").to_string();

    title_text
}

fn random_uid() -> i64 {
    static mut NEXT_UID: i64 = 100;

    unsafe {
        NEXT_UID += 1;
        NEXT_UID
    }
}

async fn create_ticket(db: &Database) -> Result<i64> {
    let uid = random_uid();
    let title = new_ticket_title();

    let ticket_id = sqlx::query!(
        r#"INSERT INTO tickets(created_by_uid, title)
        VALUES(?1,?2)"#,
        uid,
        title
    )
    .execute(&db.pool)
    .await?
    .last_insert_rowid();

    Ok(ticket_id)
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
