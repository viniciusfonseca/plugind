start-server:
	LIBS_PATH=`pwd` ADDR_LISTEN=0.0.0.0:8080 cargo run --package plugin-mesh --bin plugin-mesh

invoke-example:
	cargo build --package plugin-example --release
	curl -X POST http://localhost:8080 -H "Content-Type: application/json" -d "{\"lib_name\": \"target/release/libplugin_example\", \"params\": \"\"}"
