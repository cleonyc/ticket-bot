-- Add migration script here
CREATE TYPE ticket_status as ENUM ('open', 'closed', 'deleted');
CREATE TABLE IF NOT EXISTS tickets (
    id SERIAL PRIMARY KEY,
    current_ticket_status ticket_status,
    messages TEXT[], 
    channel_id BIGINT
    
);