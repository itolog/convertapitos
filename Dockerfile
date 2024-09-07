# STAGE-1: Planning stage
FROM rust as planner
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# STAGE-2: Caching dependencies
FROM rust as cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN apt-get update && apt-get install -y pkg-config alsa-utils libasound2-dev libasound2
RUN cargo chef cook --release --recipe-path recipe.json

# STAGE-3: Building the Rust project
FROM rust as builder
COPY . /app
WORKDIR /app
COPY --from=cacher /app/target /target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN apt-get update && apt-get install -y pkg-config alsa-utils libasound2-dev libasound2
RUN cargo build --release

# STAGE-4: Preparing the release image with Debian Bookworm Slim
FROM debian:bookworm-slim as release
WORKDIR /app

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y libasound2 && apt-get clean && rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/convertapitos /app/convertapitos

EXPOSE 5800

CMD ["./convertapitos"]
