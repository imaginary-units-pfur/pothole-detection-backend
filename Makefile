docker: database-setup docker-build docker-run

# Database bullshit
database-setup:
	docker compose up -d database; \
		cd server; \
		sqlx db setup; \
		cd ..; \
		cargo sqlx prepare --workspace; \
		docker compose build; \
		docker compose down database
cargo-clean:
	cargo clean
docker-build:
	docker compose build
docker-run:
	docker compose up
