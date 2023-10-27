CREATE TABLE IF NOT EXISTS roles(
    id VARCHAR(200) PRIMARY KEY,
    name VARCHAR(200),
    permissions VARCHAR(200),
    created_at TIMESTAMP,
    updated_at TIMESTAMP
);