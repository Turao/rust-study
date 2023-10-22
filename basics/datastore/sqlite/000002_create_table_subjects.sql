CREATE TABLE IF NOT EXISTS subjects(
    id VARCHAR(200) PRIMARY KEY,
    version INTEGER,
    name VARCHAR(200),
    roles VARCHAR(200),
    created_at TIMESTAMP,
    updated_at TIMESTAMP
);