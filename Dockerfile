# STAGE-1
FROM rust as planner

WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# STAGE-2
FROM rust as cacher

WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# STAGE-3
FROM rust as builder

COPY . /app
WORKDIR /app
COPY --from=cacher /app/target /target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release

# STAGE-4
FROM gcr.io/distroless/cc-debian12 as release

COPY --from=builder /app/target/release/convertapitos /app/convertapitos
EXPOSE 5800
WORKDIR /app

CMD ["./convertapitos"]