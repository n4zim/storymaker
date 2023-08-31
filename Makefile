
run:
	cargo run --release

debug:
	RUST_LOG=big_brain=debug,sequence=debug cargo run --release

trace:
	RUST_LOG=big_brain=trace,sequence=debug,thirst=trace cargo run --release

generate_worlds:
	cd worlds && cargo run
