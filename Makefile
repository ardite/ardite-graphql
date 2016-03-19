target/debug/ardite-graphql:
	cargo build --package ardite-graphql --bin ardite-graphql

target/release/ardite-graphql:
	cargo build --package ardite-graphql --bin ardite-graphql --release
