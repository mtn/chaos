run:
	bootimage run -- \
		-serial mon:stdio

unittest:
	cargo test

integration:
	bootimage test

