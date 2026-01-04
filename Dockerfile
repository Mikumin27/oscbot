FROM rustlang/rust:nightly-slim AS builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    build-essential \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src \
 && printf 'fn main() {}\n' > src/main.rs \
 && cargo build --release \
 && rm -rf src

COPY src ./src
RUN find src -type f -exec touch {} + \
 && rm -f target/release/oscbot \
 && cargo build --release \
 && mkdir -p /out \
 && cp target/release/oscbot /out/oscbot

FROM git.sulej.net/osc/skins-image:latest

WORKDIR /app/oscbot

RUN install -d -m 755 -o 1000 -g 1000 \
      /app/oscbot

COPY --chown=1000:1000 --from=builder /out/oscbot /app/oscbot/oscbot
COPY --chown=1000:1000 default-danser.json /app/danser/settings/default.json
COPY --chown=1000:1000 src/generate/data /app/oscbot/src/generate/data

USER 1000:1000

ENTRYPOINT ["/app/oscbot/oscbot"]