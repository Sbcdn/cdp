REGISTRY=default-route-openshift-image-registry.apps.testnet.drasil.org
GOOGLEREG=europe-west2-docker.pkg.dev
PROJECT_GOOGLE=mystic-torus-339723
REPO_GOOGLE=drasil-mainnet
PROJECT=drasil
PROJECT_MAINNET=mdrsl
IMAGE=cdp
VERSION=v1.1

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