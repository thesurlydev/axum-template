# Database

The intent of this document is to provide a guide for how to establish a database, tables, and seed data for the application.

## Install sqlx

```shell
cargo install sqlx-cli --features postgres
```

## Create the database

The following command will use the `DATABASE_URL` environment variable (in `.env`) to create the database.

```shell
sqlx database create
```

## Run the migrations

```shell
sqlx migrate run
```
