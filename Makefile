PROJECT = my_os
ARCH = x86_64

.PHONY: build bootimage run

build:
	cargo build \
		-Z build-std=core,compiler_builtins,alloc \
		-Z build-std-features=compiler-builtins-mem  \
		--target ${ARCH}-${PROJECT}.json

bootimage:
	cargo bootimage

run:
	qemu-system-x86_64 \
		-drive format=raw,file=target/${ARCH}-${PROJECT}/debug/bootimage-${PROJECT}.bin
