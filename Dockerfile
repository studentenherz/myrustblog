FROM rust:latest AS base

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli

FROM base AS build

WORKDIR /usr/src/myrustblog
COPY . .

RUN cargo build --profile backend-release
RUN cd frontend && trunk build --release

FROM gcr.io/distroless/cc-debian12

COPY --from=build /usr/src/myrustblog/target/backend-release/backend /usr/local/bin/backend
COPY --from=build /usr/src/myrustblog/frontend/dist /usr/local/bin/dist
COPY .env /usr/local/bin/.env

WORKDIR /usr/local/bin
CMD [ "backend" ]
