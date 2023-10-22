CREATE TABLE IF NOT EXISTS groups(
    id VARCHAR(200) PRIMARY KEY,
    name VARCHAR(200),
    subjects VARCHAR(200),
    roles VARCHAR(200),
    created_at TIMESTAMP,
    updated_at TIMESTAMP
);