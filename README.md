To setup Docker builds:
- start database: `docker compose up database` from here
- perform migrations: `sqlx db setup` from the `server` directory
- build offline data: `cargo sqlx prepare --workspace` from here (put `.sqlx` into git!)
- build the docker app: `docker compose build`
- stop the database, so it is started from the main launch: `docker compose down database`
- 