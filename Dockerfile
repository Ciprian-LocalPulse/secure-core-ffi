# syntax=docker/dockerfile:1

# ---- Etapa 1: build nucleu Rust ----
FROM rust:1.79-slim AS builder

WORKDIR /usr/src/security-core-ffi

# Cache dependințe
COPY Cargo.toml ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch || true

# Copiem sursele reale
COPY src ./src

RUN cargo build --release --lib

# ---- Etapa 2: imagine finală minimă ----
FROM debian:bookworm-slim AS runtime

LABEL maintainer="Ciprian Ștefan Pleșca" \
      org.opencontainers.image.title="security-core-ffi" \
      org.opencontainers.image.description="Nucleu de securitate criptografic (AES-256-GCM) expus prin FFI" \
      org.opencontainers.image.licenses="Apache-2.0" \
      org.opencontainers.image.authors="Ciprian Ștefan Pleșca" \
      org.opencontainers.image.source="https://github.com/CiprianStefanPlesca/security-core-ffi"

WORKDIR /opt/security-core-ffi

COPY --from=builder /usr/src/security-core-ffi/target/release/libsecurity_core.so ./lib/
COPY bindings ./bindings
COPY README.md LICENSE ./

RUN useradd --no-create-home --shell /usr/sbin/nologin securitycore
USER securitycore

ENV LD_LIBRARY_PATH=/opt/security-core-ffi/lib

CMD ["sh", "-c", "echo 'Nucleu de securitate compilat: lib/libsecurity_core.so'"]
