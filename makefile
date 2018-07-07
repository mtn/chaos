run:
	bootimage run -- \
		-serial mon:stdio \
		-device isa-debug-exit,iobase=0xf4,iosize=0x04 \
		-display none

unittest:
	cargo test

integration:
	bootimage test

