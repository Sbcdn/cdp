REGISTRY=default-route-openshift-image-registry.apps.testnet.drasil.org
GOOGLEREG=europe-west2-docker.pkg.dev
PROJECT_GOOGLE=tqjwvihfudnb
REPO_GOOGLE=wm-staging-registry
PROJECT=drasil
PROJECT_MAINNET=mdrsl
IMAGE=cdp
VERSION=v1.1

# europe-west2-docker.pkg.dev/tqjwvihfudnb/wm-staging-registry

build:
	cargo update
	cargo fetch
	rm -rf .cargo
	mkdir .cargo
	echo "git-fetch-with-cli = true" > .cargo/config
	cp -R ~/.cargo/git .cargo/git
	sudo docker build -t $(REGISTRY)/$(PROJECT)/$(IMAGE):$(VERSION) -f Dockerfile --target $(IMAGE) .

build-mainnet:
	cargo update
	cargo fetch
	rm -rf .cargo
	mkdir .cargo
	echo "git-fetch-with-cli = true" > .cargo/config
	cp -R ~/.cargo/git .cargo/git
	sudo docker build -t $(REGISTRY)/$(PROJECT_MAINNET)/$(IMAGE):$(VERSION) -f Dockerfile --target $(IMAGE) .


run:
	docker run -t $(IMAGE):$(VERSION) $(IMAGE)

push:
	sudo docker push $(REGISTRY)/$(PROJECT)/$(IMAGE):$(VERSION)

push-mainnet:
	sudo docker push $(REGISTRY)/$(PROJECT_MAINNET)/$(IMAGE):$(VERSION)

build-mainnet-google:
	cargo update
	cargo fetch
	rm -rf .cargo
	mkdir .cargo
	echo "git-fetch-with-cli = true" > .cargo/config
	cp -R ~/.cargo/git .cargo/git
	sudo docker build -t $(GOOGLEREG)/$(PROJECT_GOOGLE)/$(REPO_GOOGLE)/$(IMAGE):$(VERSION) -f Dockerfile --target $(IMAGE) .

push-mainnet-google:
	docker push $(GOOGLEREG)/$(PROJECT_GOOGLE)/$(REPO_GOOGLE)/$(IMAGE):$(VERSION)

TESTNET_PROJECT_GOOGLE=efvgtwmyqlpe
TESTNET_REPO_GOOGLE=preview-testnet-registry

build-testnet-google:
	cargo update
	cargo fetch
	rm -rf .cargo
	mkdir .cargo
	echo "git-fetch-with-cli = true" > .cargo/config
	cp -R ~/.cargo/git .cargo/git
	sudo docker build -t $(GOOGLEREG)/$(TESTNET_PROJECT_GOOGLE)/$(TESTNET_REPO_GOOGLE)/$(IMAGE):$(VERSION) -f Dockerfile --target $(IMAGE) .

push-testnet-google:
	docker push $(GOOGLEREG)/$(TESTNET_PROJECT_GOOGLE)/$(TESTNET_REPO_GOOGLE)/$(IMAGE):$(VERSION)