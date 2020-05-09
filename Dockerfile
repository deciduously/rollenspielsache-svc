ARG BASE_IMAGE=ekidd/rust-musl-builder:latest

FROM ${BASE_IMAGE} AS builder

# Compile
ADD --chown=rust:rust . ./
RUN cargo build --release

# Copy the statically-linked binary into runtime container
FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/rollenspielsache-svc /usr/local/bin
CMD ["/usr/local/bin/rollenspielsache-svc", "-a", "0.0.0.0", "-p", "9292"]