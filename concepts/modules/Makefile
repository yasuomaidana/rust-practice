test:
	echo "Running tests"
	cargo test
	echo "All tests passed"
	cargo test -p color-string file_reader
	echo "All child package tests passed"

release:
	cargo build --release