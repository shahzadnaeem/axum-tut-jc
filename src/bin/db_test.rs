use chrono::prelude::*;
use core::time;
use std::thread;

use axum_tut::{database::Database, error::Result, model::tickets::Ticket};

#[tokio::main]
async fn main() -> Result<()> {
    let db = Database::new().await;

    let id = create_ticket(&db).await?;

    println!("\nAdded new Ticket ID: {}", id);

    let added_ticket = get_ticket(&db, id).await?;

    if let Some(ticket) = added_ticket {
        println!(
            "\tTicket ID: {}, Created_by_uid: {}, Title: {}, Done: {}\n",
            ticket.id, ticket.created_by_uid, ticket.title, ticket.done
        );
    } else {
        println!("ERROR: Failed to get added ticket");
    }

    let tickets = get_tickets(&db).await?;

    println!("All Tickets");
    for ticket in tickets {
        println!(
            "\tID: {}, Created_by_uid: {}, Title: {}, Done: {}",
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

async fn get_ticket(db: &Database, id: i64) -> Result<Option<Ticket>> {
    let ticket = sqlx::query!(
        r#"SELECT * FROM tickets
        WHERE id = ?1"#,
        id
    )
    .fetch_optional(&db.pool)
    .await?;

    Ok(ticket.map_or(None, |rec| {
        Some(Ticket {
            id: rec.id as u64,
            created_by_uid: rec.created_by_uid as u64,
            title: rec.title.clone(),
            done: rec.done,
        })
    }))
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
