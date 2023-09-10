docker: cargo-clean docker-build docker-run
cargo-clean:
	cargo clean
docker-build:
	docker compose build
docker-run:
	docker compose up
