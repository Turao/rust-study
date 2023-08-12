CREATE TABLE IF NOT EXISTS permissions(
    id VARCHAR(200) PRIMARY KEY,
    name VARCHAR(200),
    operation VARCHAR(200),
    created_at TIMESTAMP,
    updated_at TIMESTAMP
);