.PHONY: build

build:
	cargo build \
		-Z build-std=core,compiler_builtins \
		-Z build-std-features=compiler-builtins-mem  \
		--target x86_64_target.json
