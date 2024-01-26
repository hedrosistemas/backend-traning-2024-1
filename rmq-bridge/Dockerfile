FROM rust AS build

WORKDIR /build
COPY . .
RUN apt update && \ 
    apt install libssl-dev build-essential cmake pkg-config llvm-dev libclang-dev clang libmosquitto-dev -y && \
    cargo build --release

FROM rust:alpine

WORKDIR /app
COPY --from=build /build/.env /build/target/release/rmq-bridge app/

CMD ["sh", "-c", "./rmq-bridge"]