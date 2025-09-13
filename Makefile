setup-docker:
	sudo systemctl start docker
	sudo chmod 777 /var/run/docker.sock

compose-up:
	docker compose down
	docker compose up -d

compose-up-build:
	docker compose down
	docker compose up --build -d

invoke-example:
	cargo build --package plugin-example --release
	pluginctl deploy plugin-example/plugind.toml
	curl -X POST http://localhost:8080/invocations \
		-H "Content-Type: application/json" \
		-H "X-Plugin: plugin-example"
