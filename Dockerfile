FROM rust:latest AS base

RUN rustup target add x86_64-unknown-linux-musl
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk wasm-bindgen-cli

RUN apt-get update && \
    apt-get install -y musl musl-dev musl-tools pkg-config libssl-dev

FROM base AS debug
WORKDIR /usr/src/myrustblog
COPY .git .git

RUN echo "=== GIT STATUS ===" && git status && git diff || true

FROM base AS build

RUN USER=root cargo new --bin /usr/src/myrustblog
WORKDIR /usr/src/myrustblog

COPY Cargo.toml Cargo.lock ./
COPY backend/Cargo.toml backend/
COPY frontend/Cargo.toml frontend/build.rs frontend/
COPY common/Cargo.toml common/

RUN mkdir backend/src frontend/src common/src \
    && touch frontend/src/main.rs common/src/main.rs \
    && mv src/main.rs backend/src/main.rs
RUN cargo build --target x86_64-unknown-linux-musl --profile backend-release
RUN rm backend/src/main.rs frontend/src/main.rs common/src/main.rs

COPY . .

RUN rm ./target/x86_64-unknown-linux-musl/backend-release/deps/backend*
RUN cargo build --target x86_64-unknown-linux-musl --profile backend-release
RUN cd frontend && trunk build --release

FROM gcr.io/distroless/static

COPY --from=build /usr/src/myrustblog/target/x86_64-unknown-linux-musl/backend-release/backend /usr/local/bin/backend
COPY --from=build /usr/src/myrustblog/frontend/dist /usr/local/bin/dist

WORKDIR /usr/local/bin
CMD [ "/usr/local/bin/backend" ]
