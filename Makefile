PLUGIND_VERSION := $(shell cargo get workspace.package.version)

install:
	cargo install --path ./pluginctl

docker-release:
	docker build -t distanteagle16/plugind:$(PLUGIND_VERSION) .