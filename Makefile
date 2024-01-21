PROJECT = my_os
ARCH = x86_64

.PHONY: build build-bootimage

build:
	cargo build \
		-Z build-std=core,compiler_builtins \
		-Z build-std-features=compiler-builtins-mem  \
		--target ${ARCH}-${PROJECT}.json
