# Pixel Points

A simple leaderboard / high-score server written in Rust.

## Setting up the database

It uses a postgres database. To spin it up, go to the `launch-database` directory and
run `docker compose up -d` (after having installed `docker-compose`). Then add an
environment variable pointing to this database:

```
export DATABASE_URL="postgresql://pixel:pixel@localhost:5432"
```

Now we are ready to initialize the database with our schema. Do this using the
`sqlx` CLI like this:

```
cargo sqlx database setup
```

## Running the server

Spin up the server with a `cargo run` and try hitting it with
`curl localhost:3000/v1/games`!
