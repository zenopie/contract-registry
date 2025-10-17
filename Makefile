# use the optimized contract build enviroment
.PHONY: build-mainnet-reproducible
build-mainnet-reproducible:
	docker run --rm -v "$$(pwd)":/contract \
		--mount type=volume,source="$$(basename "$$(pwd)")_cache",target=/contract/target \
		--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
		ghcr.io/scrtlabs/secret-contract-optimizer:1.0.13
