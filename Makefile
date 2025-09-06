compose-up:
	docker compose -f ./compose-setup/docker-compose.yaml down
	docker compose -f ./compose-setup/docker-compose.yaml up -d

invoke-example:
	cargo build --package plugin-example --release
	pluginctl deploy plugin-example/plugind.toml
	curl -X POST http://localhost:8080/invocations \
		-H "Content-Type: application/json" \
		-H "X-Plugin: plugin-example"
