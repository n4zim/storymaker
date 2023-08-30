
run:
	cargo run --release

trace:
	RUST_LOG=big_brain=trace,thirst=trace cargo run --release

generate_worlds:
	cd worlds && cargo run
