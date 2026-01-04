FROM rust:slim AS chef

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    build-essential \
 && rm -rf /var/lib/apt/lists/*

RUN rustup toolchain install nightly --profile minimal && \
    rustup default nightly && \
    cargo install cargo-chef --locked

WORKDIR /app

FROM chef AS planner

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

ARG BUILD_PROFILE=release
ENV CARGO_BUILD_JOBS=1

RUN if [ "$BUILD_PROFILE" = "release" ]; then \
        cargo build --locked --release; \
    else \
        cargo build --locked; \
    fi \
 && mkdir -p /out \
 && cp "/app/target/${BUILD_PROFILE}/oscbot" /out/oscbot

FROM git.sulej.net/osc/skins-image:latest

ENV OSC_BOT_DANSER_PATH=/app/danser
ENV PATH="/app/danser:${PATH}"
WORKDIR /app/oscbot

COPY --from=builder /out/oscbot /app/oscbot/oscbot
COPY default-danser.json /app/oscbot/default-danser.json
COPY default-danser.json /app/danser/settings/default.json
COPY src/generate/data /app/oscbot/src/generate/data

RUN mkdir -p \
      /app/oscbot/Songs \
      /app/oscbot/Skins \
      /app/oscbot/Replays \
      /app/oscbot/videos \
      /app/oscbot/videoForRegen \
 && chmod +x /app/oscbot/oscbot \
 && chown -R 1000:1000 /app/oscbot /app/danser

USER 1000:1000
CMD ["/app/oscbot/oscbot"]
