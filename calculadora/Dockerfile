ARG APP_NAME=calculadora

FROM rust:alpine AS build
ARG APP_NAME
WORKDIR /app

RUN apk add --no-cache musl-dev gcc

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --locked --release && \
    cp ./target/release/$APP_NAME /bin/server

FROM gcr.io/distroless/static-debian12 AS final

COPY --from=build /bin/server /bin/server

ENV ROCKET_ADDRESS=0.0.0.0

EXPOSE 8000

CMD ["/bin/server"]