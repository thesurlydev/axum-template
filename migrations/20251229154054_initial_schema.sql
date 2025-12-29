-- migrations/20251229154054_initial_schema.sql
CREATE TABLE users (
    id           VARCHAR(36)    PRIMARY KEY,
    username     VARCHAR(64)    NOT NULL UNIQUE,
    email        VARCHAR(128)   NOT NULL,
    created_by   VARCHAR(36),
    created_at   TIMESTAMPTZ    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    modified_by  VARCHAR(36),
    modified_at  TIMESTAMPTZ    NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_users_email ON users(email);

CREATE TABLE user_auth (
    user_id       VARCHAR(36)  PRIMARY KEY,
    password_hash VARCHAR(255) NOT NULL,
    created_at    TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    modified_at   TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (user_id) REFERENCES users(id)
);
