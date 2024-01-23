dev-simulator-run:
	cd device-simulator && yarn start

rmq-build:
	cd rmq-bridge && cargo build

rmq-run:
	cd rmq-bridge && cargo run