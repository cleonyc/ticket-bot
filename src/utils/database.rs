

use crate::SETTINGS;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use uuid::Uuid;
#[derive(Debug)]
pub struct Database {
    pool: Pool<Postgres>,
}
impl Database {
    pub async fn new() -> anyhow::Result<Self> {
        let db_url = SETTINGS
            .write()
            .await
            .get_str("database")
            .expect("NO DATABASE FOUND IN CONFIG");
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await?;

        Ok(Database { pool })
    }
    /// Returns ticket id
    pub async fn open_ticket(&self, channel_id: usize) -> anyhow::Result<usize> {
        let rec = sqlx::query!(
            "INSERT INTO tickets ( current_ticket_status, messages, channel_id )
            VALUES  ( 'open' , '{}' , $1 )
            RETURNING id ;",
            channel_id as i64
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(rec.id as usize)
    }
    pub async fn new_panel(
        &self,
        message_id: usize,
        category_id: usize,
        interaction_id: Uuid,
    ) -> anyhow::Result<()> {
        sqlx::query!(
            "INSERT INTO panels ( message_id, category_id, interaction_id )
            VALUES  ( $1 , $2 , $3 ); ",
            message_id as i64,
            category_id as i64,
            interaction_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
    pub async fn get_category(&self, inter_id: Uuid) -> anyhow::Result<usize> {
        let rec = sqlx::query!(
            "SELECT * 
                FROM panels 
                WHERE interaction_id = $1",
            inter_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(rec.category_id.unwrap() as usize)
    }
    pub async fn channel_is_ticket(&self, channel_id: usize) -> anyhow::Result<bool> {
        let recs = sqlx::query_as!(Ticket, "SELECT id, current_ticket_status as \"current_ticket_status: TicketStatus\", channel_id, messages 
                FROM tickets 
                WHERE channel_id = $1",
                channel_id as i64
            )   
            .fetch_all(&self.pool)
            .await?;

        Ok(recs.len() > 0)
    }
    pub async fn ticket_id(&self, channel_id: usize) -> anyhow::Result<usize> {
        let rec = sqlx::query_as!(Ticket, "SELECT id, current_ticket_status as \"current_ticket_status: TicketStatus\", channel_id, messages 
                FROM tickets 
                WHERE channel_id = $1",
                channel_id as i64
            )   
            .fetch_one(&self.pool)
            .await?;
        Ok(rec.id as usize)
        // Ok(1)
    }
    pub async fn ticket_status(&self, ticket: usize) -> anyhow::Result<TicketStatus> {
        let rec = sqlx::query_as!(Ticket, "SELECT id, current_ticket_status as \"current_ticket_status: TicketStatus\", channel_id, messages 
                FROM tickets 
                WHERE id = $1",
                ticket as i64
            )   
            .fetch_one(&self.pool)
            .await?;
        Ok(rec.current_ticket_status.unwrap())
    }
    pub async fn set_ticket_status(
        &self,
        ticket: usize,
        status: TicketStatus,
    ) -> anyhow::Result<()> {
        sqlx::query_as!(
            Ticket,
            "UPDATE tickets 
                SET current_ticket_status = $1
                WHERE id = $2",
            status as TicketStatus,
            ticket as i64
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
    pub async fn add_message(&self, ticket: usize, message: String) -> anyhow::Result<()> {
        // let m_slice: [String; 1] = [message];
        sqlx::query!(
            "UPDATE tickets
                SET messages = messages || $1
                WHERE id = $2 ",
            &vec!(message),
            ticket as i64
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
    pub async fn is_panel(&self, id: Uuid) -> anyhow::Result<bool> {
        let recs = sqlx::query!(
            "SELECT * 
                FROM panels 
                WHERE interaction_id = $1",
            id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(recs.len() > 0)
    }
    pub async fn get_messages(&self, ticket: usize) -> anyhow::Result<Vec<String>> {
        let rec = sqlx::query!(
            "SELECT messages 
                FROM tickets 
                WHERE id = $1",
            ticket as i64
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(rec.messages.unwrap())
        // Ok(())
    }
}
#[derive(sqlx::Type, Debug, PartialEq)]
#[sqlx(type_name = "ticket_status", rename_all = "snake_case")]
pub enum TicketStatus {
    Open,
    Closed,
    Deleted,
}
// #[derive(Debug)]
// #[sqlx(rename_all = "snake_case")]
pub struct Ticket {
    pub id: i32,
    pub current_ticket_status: Option<TicketStatus>,
    pub messages: Option<Vec<String>>,
    pub channel_id: Option<i64>,
}
