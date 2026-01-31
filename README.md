# Rust Task API

A RESTful API built with Rust, Actix-web, and PostgreSQL for managing tasks.

## Features

- Create, Read, Update, and Delete tasks
- PostgreSQL database with SQLx
- Docker support with Docker Compose
- Pagination support
- Input validation with Serde
- Automatic timestamps (created_at, updated_at)

## Tech Stack

- **Rust** - Programming language
- **Actix-web** - Web framework
- **SQLx** - Async SQL toolkit
- **PostgreSQL** - Database
- **Docker** - Containerization
- **Serde** - Serialization/Deserialization

## Prerequisites

- Rust 1.87.0 or higher
- Docker and Docker Compose
- PostgreSQL 18 (or use Docker)

## Installation

### 1. Clone the repository

```bash
git clone <repository-url>
cd rust-api
```

### 2. Set up environment variables

Copy the example environment file and configure it:

```bash
cp .env.example .env
```

Edit `.env` with your configuration.

### 3. Start PostgreSQL with Docker

```bash
docker-compose up -d
```

This will start:
- PostgreSQL on port 5432
- pgAdmin on port 5050

### 4. Run database migrations

```bash
# Install SQLx CLI if not already installed
cargo install sqlx-cli --no-default-features --features postgres

# Run migrations
sqlx migrate run
```

### 5. Run the application

```bash
# Development mode with auto-reload
cargo install cargo-watch
cargo watch -q -c -w src/ -x run

# Or regular run
cargo run
```

The API will be available at `http://127.0.0.1:8080`

## API Endpoints

### Health Check

```http
GET /api/healthchecker
```

**Response:**
```json
{
  "status": "success",
  "message": "Health check: API is up and running smoothly."
}
```

### Create Task

```http
POST /api/tasks
Content-Type: application/json

{
  "title": "My Task",
  "content": "Task description"
}
```

**Response:** `201 Created`
```json
{
  "status": "success",
  "data": {
    "id": "uuid",
    "title": "My Task",
    "content": "Task description",
    "created_at": "2026-01-31T12:00:00Z",
    "updated_at": "2026-01-31T12:00:00Z"
  }
}
```

### Get All Tasks

```http
GET /api/tasks?page=1&limit=10
```

**Query Parameters:**
- `page` (optional): Page number (default: 1)
- `limit` (optional): Items per page (default: 10)

**Response:** `200 OK`
```json
{
  "status": "success",
  "result": 2,
  "tasks": [
    {
      "id": "uuid",
      "title": "Task 1",
      "content": "Description 1",
      "created_at": "2026-01-31T12:00:00Z",
      "updated_at": "2026-01-31T12:00:00Z"
    }
  ]
}
```

### Get Task by ID

```http
GET /api/tasks/{id}
```

**Response:** `200 OK`
```json
{
  "status": "success",
  "task": {
    "id": "uuid",
    "title": "My Task",
    "content": "Task description",
    "created_at": "2026-01-31T12:00:00Z",
    "updated_at": "2026-01-31T12:00:00Z"
  }
}
```

### Update Task

```http
PATCH /api/tasks/{id}
Content-Type: application/json

{
  "title": "Updated Title",
  "content": "Updated Content"
}
```

**Note:** Both fields are optional. Only provided fields will be updated.

**Response:** `200 OK`
```json
{
  "status": "success",
  "task": {
    "id": "uuid",
    "title": "Updated Title",
    "content": "Updated Content",
    "created_at": "2026-01-31T12:00:00Z",
    "updated_at": "2026-01-31T13:00:00Z"
  }
}
```

### Delete Task

```http
DELETE /api/tasks/{id}
```

**Response:** `204 No Content`

## Project Structure

```
rust-api/
├── src/
│   ├── main.rs           # Application entry point
│   ├── model.rs          # Database models
│   ├── schema.rs         # Request/Response schemas
│   └── services.rs       # API route handlers
├── migrations/           # Database migrations
├── docker-compose.yml    # Docker configuration
├── Cargo.toml           # Rust dependencies
├── .env                 # Environment variables (not in git)
└── .env.example         # Example environment variables
```

## Database Schema

### Tasks Table

| Column      | Type                     | Constraints           |
|-------------|-------------------------|-----------------------|
| id          | UUID                    | PRIMARY KEY, DEFAULT  |
| title       | VARCHAR(255)            | NOT NULL              |
| content     | TEXT                    | NOT NULL              |
| created_at  | TIMESTAMP WITH TIMEZONE | DEFAULT NOW()         |
| updated_at  | TIMESTAMP WITH TIMEZONE | DEFAULT NOW()         |

## Development

### Running Tests

```bash
cargo test
```

### Linting

```bash
cargo clippy
```

### Formatting

```bash
cargo fmt
```

### Database Commands

```bash
# Create a new migration
sqlx migrate add -r migration_name

# Run migrations
sqlx migrate run

# Revert last migration
sqlx migrate revert

# Create database
sqlx database create

# Drop database
sqlx database drop
```

## Docker

### Access pgAdmin

1. Open `http://localhost:5050` in your browser
2. Login with credentials from `.env`:
   - Email: `admin@admin.com`
   - Password: `password123`
3. Add server:
   - Host: `postgres`
   - Port: `5432`
   - Database: `rust_admin`
   - Username: `admin`
   - Password: `password123`

### Access PostgreSQL CLI

```bash
docker exec -it postgres psql -U admin -d rust_admin
```

## Environment Variables

See `.env.example` for all available environment variables.

## Error Handling

The API returns standard HTTP status codes:

- `200 OK` - Successful GET/PATCH request
- `201 Created` - Successful POST request
- `204 No Content` - Successful DELETE request
- `404 Not Found` - Resource not found
- `500 Internal Server Error` - Server error

Error response format:
```json
{
  "status": "error",
  "message": "Error description"
}
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'feat: add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License.

## Contact

For questions or support, please open an issue on GitHub.
