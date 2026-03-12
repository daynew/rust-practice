all: run

build:
	cargo check

run:
	cargo run

watch:
	find . -name "*.rs" | entr -c cargo run -- you poem.txt

watch_test:
	find . -name "*.rs" | entr -c cargo test

clean:
	cargo clean
