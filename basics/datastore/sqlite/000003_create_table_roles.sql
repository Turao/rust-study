CREATE TABLE IF NOT EXISTS roles(
    id VARCHAR(200) PRIMARY KEY,
    name VARCHAR(200),
    created_at TIMESTAMP,
    updated_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS roles_permissions(
    role_id VARCHAR(200),
    permission_id VARCHAR(200),
    FOREIGN KEY (role_id) REFERENCES roles(id),
    FOREIGN KEY (permission_id) REFERENCES permissions(id)
);