# syntax=docker/dockerfile:1
#TODO: Förbättra 


ARG RUST_VERSION=1.75.0
ARG APP_NAME=RustWebServer

FROM --platform=$BUILDPLATFORM tonistiigi/xx:1.3.0 AS xx

FROM --platform=$BUILDPLATFORM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /app

COPY --from=xx / /

RUN apk add --no-cache clang lld musl-dev git file

ARG TARGETPLATFORM

RUN xx-apk add --no-cache musl-dev gcc

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/,id=rust-cache-${APP_NAME}-${TARGETPLATFORM} \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
xx-cargo build --locked --release --target-dir ./target
cp ./target/$(xx-cargo --print-target-triple)/release/$APP_NAME /bin/server
xx-verify /bin/server
EOF


FROM alpine:3.18 AS final

ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser


COPY --chown=appuser --from=build /bin/server /bin/
COPY --chown=appuser --chmod=444  ./static  /static/
COPY --chown=appuser --chmod=666  ./log.txt  /log.txt

EXPOSE 7151

CMD ["/bin/server"]
