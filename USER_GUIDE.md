# User Guide

This guide walks you through generating, configuring, and running a web application using this Axum template.

## Table of Contents

- [Overview](#overview)
- [Prerequisites](#prerequisites)
- [Quick Start](#quick-start)
- [Architecture](#architecture)
- [Configuration](#configuration)
- [Database Setup](#database-setup)
- [API Endpoints](#api-endpoints)
- [Authentication](#authentication)
- [Running the Application](#running-the-application)

## Overview

This template generates a production-ready Rust web application using:

- **Axum** - Fast, ergonomic web framework
- **SQLx** - Async SQL toolkit with compile-time query verification
- **PostgreSQL** - Database backend
- **JWT** - Token-based authentication
- **utoipa** - OpenAPI/Swagger documentation

```mermaid
graph TB
    subgraph "Generated Application"
        A[Axum Web Server] --> B[Router]
        B --> C[Auth Middleware]
        C --> D[Handlers]
        D --> E[Services]
        E --> F[Repositories]
        F --> G[(PostgreSQL)]
    end

    H[Client] --> A
    A --> I[Swagger UI]
```

## Prerequisites

Before generating a project, ensure you have:

- **Rust** (1.75 or later) - `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **cargo-generate** - `cargo install cargo-generate`
- **sqlx-cli** - `cargo install sqlx-cli --features postgres`
- **PostgreSQL** (16+) - Running instance with connection credentials
- **jq** (optional) - For parsing JSON responses in scripts

## Quick Start

### 1. Generate the Project

```bash
cargo generate --git https://github.com/thesurlydev/axum-template --name my-app
```

You'll be prompted for:

| Prompt | Description | Default |
|--------|-------------|---------|
| Description | Project description | - |
| GitHub user | Your GitHub username | thesurlydev |
| Port | Server port | 8080 |
| Database support | Enable PostgreSQL | true |
| Database URL | PostgreSQL connection string | postgres://postgres:postgres@localhost:5432/postgres |
| GitHub support | Add GitHub Actions/Dependabot | true |

### 2. Set Up the Database

```bash
cd my-app

# Create the database
sqlx database create

# Run migrations
sqlx migrate run
```

### 3. Configure Environment

The `.env` file is generated with your settings. Ensure `JWT_SECRET_KEY` is set to a secure value for production:

```bash
# Generate a secure secret
openssl rand -base64 32
```

### 4. Run the Application

```bash
cargo run
```

The server starts at `http://localhost:8080` (or your configured port).

### 5. Access Swagger UI

Open `http://localhost:8080/docs` to explore the API documentation.

## Architecture

### Project Structure

```
my-app/
├── src/
│   ├── main.rs              # Application entry point
│   ├── app.rs               # Router and middleware setup
│   ├── lib.rs               # Library exports
│   ├── common/              # Shared utilities
│   │   ├── app_state.rs     # Application state
│   │   ├── bootstrap.rs     # Application bootstrap/startup
│   │   ├── config.rs        # Configuration loading
│   │   ├── dto.rs           # API response types
│   │   ├── error.rs         # Error handling
│   │   ├── hash_util.rs     # Password hashing (Argon2)
│   │   ├── jwt.rs           # JWT utilities
│   │   ├── opentelemetry.rs # OpenTelemetry support (optional)
│   │   ├── pagination.rs    # Pagination utilities
│   │   ├── ts_format.rs     # Timestamp formatting
│   │   └── validated_json.rs # Request validation
│   └── domain/              # Business domains
│       ├── auth/            # Authentication domain
│       │   ├── api/         # Routes and handlers
│       │   ├── domain/      # Models, services, repositories
│       │   ├── dto/         # Data transfer objects
│       │   └── infra/       # PostgreSQL implementations
│       └── user/            # User management domain
│           ├── api/
│           ├── domain/
│           ├── dto/
│           └── infra/
├── migrations/              # SQL migrations
└── .env                     # Environment configuration
```

### Domain-Driven Design

Each domain follows a clean architecture pattern:

```mermaid
graph LR
    subgraph "Domain Layer"
        A[API Layer] --> B[Service Trait]
        B --> C[Repository Trait]
    end

    subgraph "Infrastructure Layer"
        D[Handlers] -.implements.-> A
        E[PostgreSQL Service] -.implements.-> B
        F[PostgreSQL Repository] -.implements.-> C
    end

    F --> G[(Database)]
```

### Request Flow

```mermaid
sequenceDiagram
    participant C as Client
    participant R as Router
    participant M as Middleware
    participant H as Handler
    participant S as Service
    participant DB as PostgreSQL

    C->>R: HTTP Request
    R->>M: Route Match
    M->>M: JWT Validation
    alt Valid Token
        M->>H: Authorized Request
        H->>S: Business Logic
        S->>DB: Query/Mutation
        DB-->>S: Result
        S-->>H: Domain Object
        H-->>C: JSON Response
    else Invalid Token
        M-->>C: 401 Unauthorized
    end
```

## Configuration

### Environment Variables

| Variable | Description | Required | Default |
|----------|-------------|----------|---------|
| `DATABASE_URL` | PostgreSQL connection string | Yes | - |
| `DATABASE_MAX_CONNECTIONS` | Maximum connections in the pool | No | 5 |
| `DATABASE_MIN_CONNECTIONS` | Minimum idle connections in the pool | No | 1 |
| `SERVICE_HOST` | Server bind address | Yes | - |
| `SERVICE_PORT` | Server port | Yes | - |
| `JWT_SECRET_KEY` | Secret for signing JWT tokens | Yes | - |
| `CORS_ALLOWED_ORIGINS` | Allowed CORS origins (comma-separated or `*`) | No | `*` |
| `REQUEST_TIMEOUT_SECS` | Request timeout in seconds | No | 5 |

### Example .env

```env
DATABASE_URL=postgres://postgres:postgres@localhost:5432/myapp
DATABASE_MAX_CONNECTIONS=5
DATABASE_MIN_CONNECTIONS=1
SERVICE_HOST=0.0.0.0
SERVICE_PORT=8080
JWT_SECRET_KEY=your-secure-secret-key-here
CORS_ALLOWED_ORIGINS=*
REQUEST_TIMEOUT_SECS=5
```

## Database Setup

### Schema Overview

```mermaid
erDiagram
    users {
        varchar(36) id PK
        varchar(64) username UK
        varchar(128) email
        varchar(36) created_by
        timestamptz created_at
        varchar(36) modified_by
        timestamptz modified_at
    }

    user_auth {
        varchar(36) user_id PK,FK
        varchar(255) password_hash
        timestamptz created_at
        timestamptz modified_at
    }

    users ||--o| user_auth : "has auth"
```

### Running Migrations

```bash
# Create the database (if it doesn't exist)
sqlx database create

# Run all pending migrations
sqlx migrate run

# Revert the last migration
sqlx migrate revert
```

### Seed Data

The template includes seed data with a default admin user:

- **Username**: `admin`
- **Email**: `shane@surly.dev`
- **Password**: `test`

**Important**: Change this password and email immediately in production.

## API Endpoints

### Health Check

Simple endpoint to verify the server is running.

```mermaid
sequenceDiagram
    participant C as Client
    participant S as Server

    C->>S: GET /health
    S-->>C: 200 OK "OK"
```

**Request:**
```bash
curl http://localhost:8080/health
```

**Response:**
```
OK
```

### Authentication

#### Login

Authenticate and receive a JWT token.

```mermaid
sequenceDiagram
    participant C as Client
    participant S as Server
    participant DB as Database

    C->>S: POST /auth/login
    Note over C,S: {"client_id": "admin", "client_secret": "test"}
    S->>DB: Find user by username
    DB-->>S: User + password hash
    S->>S: Verify password (Argon2)
    S->>S: Generate JWT
    S-->>C: 200 OK
    Note over S,C: {"access_token": "eyJ...", "token_type": "Bearer"}
```

**Request:**
```bash
curl -X POST http://localhost:8080/auth/login \
  -H "Content-Type: application/json" \
  -d '{"client_id":"admin","client_secret":"test"}'
```

**Response:**
```json
{
  "status": 200,
  "message": "success",
  "data": {
    "access_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
    "token_type": "Bearer"
  }
}
```

#### Register

Create authentication credentials for an existing user.

```mermaid
sequenceDiagram
    participant C as Client
    participant S as Server
    participant DB as Database

    C->>S: POST /auth/register
    Note over C,S: {"user_id": "uuid", "password": "secret"}
    S->>S: Hash password (Argon2)
    S->>DB: Insert user_auth record
    DB-->>S: Success
    S-->>C: 200 OK
```

**Request:**
```bash
curl -X POST http://localhost:8080/auth/register \
  -H "Content-Type: application/json" \
  -d '{"user_id":"<user-uuid>","password":"secure-password"}'
```

### User Management

All user endpoints require JWT authentication. Include the token in the `Authorization` header:

```
Authorization: Bearer <your-token>
```

#### List Users

Supports pagination and optional filtering via query parameters.

```mermaid
sequenceDiagram
    participant C as Client
    participant S as Server
    participant DB as Database

    C->>S: GET /users?page=1&page_size=20
    Note over C,S: Authorization: Bearer <token>
    S->>S: Validate JWT
    S->>DB: SELECT * FROM users (with pagination)
    DB-->>S: User list + count
    S-->>C: 200 OK {paginated response}
```

**Query Parameters:**

| Parameter | Type | Description | Default |
|-----------|------|-------------|---------|
| `page` | integer | Page number (1-indexed) | 1 |
| `page_size` | integer | Items per page (max 100) | 20 |
| `id` | string | Filter by user ID | - |
| `username` | string | Filter by username | - |
| `email` | string | Filter by email | - |

**Request:**
```bash
curl "http://localhost:8080/users?page=1&page_size=20" \
  -H "Authorization: Bearer $TOKEN"
```

**Response:**
```json
{
  "status": 200,
  "message": "success",
  "data": {
    "items": [
      {
        "id": "00000000-0000-0000-0000-000000000001",
        "username": "admin",
        "email": "shane@surly.dev",
        "created_by": null,
        "created_at": "2025-01-01T00:00:00+00:00",
        "modified_by": null,
        "modified_at": "2025-01-01T00:00:00+00:00"
      }
    ],
    "total": 1,
    "page": 1,
    "page_size": 20,
    "total_pages": 1
  }
}
```

#### Get User by ID

```mermaid
sequenceDiagram
    participant C as Client
    participant S as Server
    participant DB as Database

    C->>S: GET /users/{id}
    S->>S: Validate JWT
    S->>DB: SELECT * FROM users WHERE id = $1
    alt User Found
        DB-->>S: User
        S-->>C: 200 OK {user}
    else Not Found
        DB-->>S: None
        S-->>C: 404 Not Found
    end
```

**Request:**
```bash
curl http://localhost:8080/users/00000000-0000-0000-0000-000000000001 \
  -H "Authorization: Bearer $TOKEN"
```

#### Create User

```mermaid
sequenceDiagram
    participant C as Client
    participant S as Server
    participant DB as Database

    C->>S: POST /users
    Note over C,S: {"username": "newuser", "email": "new@example.com"}
    S->>S: Validate JWT
    S->>S: Validate input
    S->>DB: INSERT INTO users
    DB-->>S: New user ID
    S->>DB: SELECT created user
    DB-->>S: User
    S-->>C: 201 Created {user}
```

**Request:**
```bash
curl -X POST http://localhost:8080/users \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"username":"newuser","email":"newuser@example.com"}'
```

**Response:**
```json
{
  "status": 201,
  "message": "success",
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "username": "newuser",
    "email": "newuser@example.com",
    "created_by": "00000000-0000-0000-0000-000000000001",
    "created_at": "2025-01-01T12:00:00+00:00",
    "modified_by": "00000000-0000-0000-0000-000000000001",
    "modified_at": "2025-01-01T12:00:00+00:00"
  }
}
```

#### Update User

```mermaid
sequenceDiagram
    participant C as Client
    participant S as Server
    participant DB as Database

    C->>S: PUT /users/{id}
    Note over C,S: {"username": "updated", "email": "updated@example.com"}
    S->>S: Validate JWT
    S->>S: Validate input
    S->>DB: Check user exists
    alt User Found
        S->>DB: UPDATE users SET ...
        DB-->>S: Updated user
        S-->>C: 200 OK {user}
    else Not Found
        S-->>C: 404 Not Found
    end
```

**Request:**
```bash
curl -X PUT http://localhost:8080/users/550e8400-e29b-41d4-a716-446655440000 \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"username":"updateduser","email":"updated@example.com"}'
```

#### Delete User

```mermaid
sequenceDiagram
    participant C as Client
    participant S as Server
    participant DB as Database

    C->>S: DELETE /users/{id}
    S->>S: Validate JWT
    S->>DB: DELETE FROM users WHERE id = $1
    alt Deleted
        DB-->>S: rows_affected = 1
        S-->>C: 204 No Content
    else Not Found
        DB-->>S: rows_affected = 0
        S-->>C: 404 Not Found
    end
```

**Request:**
```bash
curl -X DELETE http://localhost:8080/users/550e8400-e29b-41d4-a716-446655440000 \
  -H "Authorization: Bearer $TOKEN"
```

**Response:**

Returns `204 No Content` on success with an empty response body.

## Authentication

### JWT Token Structure

Tokens are signed using HS256 and contain:

```json
{
  "sub": "user-uuid",
  "exp": 1735689600,
  "iat": 1735603200
}
```

- **sub**: User ID (subject)
- **exp**: Expiration timestamp (24 hours from issue)
- **iat**: Issued at timestamp

### Using Tokens

Include the token in all authenticated requests:

```bash
# Store token in variable
TOKEN=$(curl -s -X POST http://localhost:8080/auth/login \
  -H "Content-Type: application/json" \
  -d '{"client_id":"admin","client_secret":"test"}' | jq -r '.data.access_token')

# Use token in subsequent requests
curl http://localhost:8080/user \
  -H "Authorization: Bearer $TOKEN"
```

### Error Responses

| Status | Message | Cause |
|--------|---------|-------|
| 401 | Missing credentials | No Authorization header |
| 401 | Invalid token | Malformed or expired JWT |
| 401 | Wrong credentials | Invalid username/password |
| 404 | User not found | User doesn't exist |

## Running the Application

### Development

```bash
# Run with auto-reload (requires cargo-watch)
cargo watch -x run

# Run with debug logging
RUST_LOG=debug cargo run
```

### Production

```bash
# Build optimized release
cargo build --release

# Run the binary
./target/release/my-app
```

### Docker (if enabled)

```bash
# Build image
docker build -t my-app .

# Run container
docker run -p 8080:8080 \
  -e DATABASE_URL=postgres://user:pass@host:5432/db \
  -e JWT_SECRET_KEY=your-secret \
  my-app
```

## Troubleshooting

### Common Issues

**Database connection failed**
- Verify PostgreSQL is running
- Check `DATABASE_URL` in `.env`
- Ensure database exists: `sqlx database create`

**JWT_SECRET_KEY must be set**
- Add `JWT_SECRET_KEY` to your `.env` file
- Generate a secure key: `openssl rand -base64 32`

**Migration failed**
- Ensure database exists
- Check for syntax errors in migration files
- Verify database user has CREATE privileges

**401 Unauthorized**
- Verify token is not expired
- Check Authorization header format: `Bearer <token>`
- Ensure JWT_SECRET_KEY matches between token generation and validation
