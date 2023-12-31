FROM rust:1.75-slim-buster as build
# Found at https://dev.to/rogertorres/first-steps-with-docker-rust-30oi

# create a new empty shell project
RUN USER=root cargo new --bin bench-metrics
WORKDIR /bench-metrics

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN apt update && apt install pkg-config libssl-dev -y && rm -rf /var/lib/apt/lists/*

# this build step will cache your dependencies
RUN cargo build --release && \
    rm src/*.rs && \
    rm ./target/release/deps/bench_metrics* && \
    rm ./target/release/bench-metrics* 

# copy your source tree
COPY ./src ./src

# build for release
RUN cargo build --release

# our final base
FROM debian:buster-slim

RUN apt update && apt install libssl1.1 -y && rm -rf /var/lib/apt/lists/*

# copy the build artifact from the build stage
COPY --from=build /bench-metrics/target/release/bench-metrics .

# set the startup command to run your binary
CMD ["./bench-metrics"]