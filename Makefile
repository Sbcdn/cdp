REGISTRY=registry.url.pkg
PROJECT=project-id
REPOSITORY=repository-name
TESTNET_PROJECT=testnet-project-id
TESTNET_REPOSITORY=testnet-repository-name
IMAGE=cdp
VERSION=v1.1


build-mainnet-google:
	cargo update
	cargo fetch
	rm -rf .cargo
	mkdir .cargo
	echo "git-fetch-with-cli = true" > .cargo/config
	cp -R ~/.cargo/git .cargo/git
	sudo docker build -t $(REGISTRY)/$(PROJECT)/$(REPOSITORY)/$(IMAGE):$(VERSION) -f Dockerfile --target $(IMAGE) .

push-mainnet-google:
	docker push $(REGISTRY)/$(PROJECT)/$(REPOSITORY)/$(IMAGE):$(VERSION)


build-testnet-google:
	cargo update
	cargo fetch
	rm -rf .cargo
	mkdir .cargo
	echo "git-fetch-with-cli = true" > .cargo/config
	cp -R ~/.cargo/git .cargo/git
	sudo docker build -t $(REGISTRY)/$(TESTNET_PROJECT)/$(TESTNET_REPOSITORY)/$(IMAGE):$(VERSION) -f Dockerfile --target $(IMAGE) .

push-testnet-google:
	docker push $(REGISTRY)/$(TESTNET_PROJECT)/$(TESTNET_REPOSITORY)/$(IMAGE):$(VERSION)