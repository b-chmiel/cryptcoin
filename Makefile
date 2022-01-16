run:
	cargo run

test:
	cargo test

run-docker:
	docker build . -t cryptcoin
	docker run --rm -it cryptcoin