FROM rust AS build

WORKDIR /build
COPY . .
RUN apt update && apt install libssl-dev build-essential cmake pkg-config llvm-dev libclang-dev clang libmosquitto-dev -y
RUN cargo build --release

FROM rust:alpine

WORKDIR /app
COPY .env ./
# RUN apt install libssl-dev build-essential cmake pkg-config llvm-dev libclang-dev clang libmosquitto-dev
COPY --from=build /build/target/release/rmq-bridge ./rmq-bridge

CMD ["./rmq-bridge"]