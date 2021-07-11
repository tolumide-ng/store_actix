# syntax=docker/dockerfile:experimental


# CARGO BUILD
FROM rust:latest as cargo-build
RUN apt-get update
RUN apt-get install musl-tools -y
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/auth_api
COPY Cargo.toml Cargo.toml
RUN mkdir src/
RUN echo "fn main() {println!(\"If you see this, the build broke\")}" > src/main.rs

RUN mkdir -p $HOME/.ssh
RUN ssh-keyscan github.com > $HOME/.ssh/known_hosts
RUN test "$(cat $HOME/.ssh/known_hosts | ssh-keygen -lf -)" = "2048 SHA256:nThbg6kXUpJWGl7E1IGOCspRomTxdCARLviKw6E5SY8 github.com (RSA)"

RUN --mount=type=ssh RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

RUN rm src/main.rs
COPY src/* src
RUN touch src/**
RUN --mount=type=ssh RUSTFLAGS=-Clinker=musl-gcc cargo test --release --target=x86_64-unknown-linux-musl
RUN --mount=type=ssh RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl



# FINAL STAGE
FROM alpine:latest
RUN addgroup -g 1000 appgroup
RUN adduser -D -s /bin/sh -u 1000 -G appgroup appuser
WORKDIR /home/auth_api/bin/
# COPY --from=cargo-build /usr/src/auth_api/target/x86_64-unknown-linux-musl/release/auth_api .
COPY --from=cargo-build /usr/src/auth_api/target/x86_64-unknown-linux-musl/release/auth_api .

RUN chown appuser:appgroup auth_api
USER appuser
CMD ["./auth_api"]