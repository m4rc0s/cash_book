-- Add migration script here

create table entry (
    id UUID primary key,
    amount DECIMAL NOT NULL,
    kind VARCHAR(60) NOT NULL,
    description VARCHAR(255),
    movement_date date not null,
    created_at timestamp default NOW(),
    updated_at timestamp default NOW()
);

CREATE INDEX IF NOT EXISTS idx_entry_movement_date ON entry(movement_date);