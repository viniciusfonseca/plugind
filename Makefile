compose-up:
	docker compose -f ./compose-setup/docker-compose.yaml down
	docker compose -f ./compose-setup/docker-compose.yaml up -d

invoke-example:
	cargo build --package plugin-example --release
	pluginctl deploy plugin-example/plugin.toml
	curl -X POST http://localhost:8080 -H "Content-Type: application/json" -d "{\"lib_name\": \"plugin-example\", \"params\": \"\"}"
