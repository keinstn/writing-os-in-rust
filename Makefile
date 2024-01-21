PROJECT = my_os
ARCH = x86_64

.PHONY: build build-bootimage run-qemu

build:
	cargo build \
		-Z build-std=core,compiler_builtins \
		-Z build-std-features=compiler-builtins-mem  \
		--target ${ARCH}-${PROJECT}.json

build-bootimage:
	cargo bootimage

run-qemu:
	qemu-system-x86_64 \
		-drive format=raw,file=target/${ARCH}-${PROJECT}/debug/bootimage-${PROJECT}.bin
