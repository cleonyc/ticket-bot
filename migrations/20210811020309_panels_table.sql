-- Add migration script here
CREATE TABLE IF NOT EXISTS panels(
    id SERIAL PRIMARY KEY,
    message_id BIGINT,
    category_id BIGINT,
    interaction_id UUID 
);