FROM rust:alpine AS build

RUN apk add musl-dev --no-cache

WORKDIR /src
COPY . .

RUN cargo build --release

FROM alpine

WORKDIR /app
COPY --from=build /src/target/release/rust-basic-crud .

ENTRYPOINT [ "./rust-basic-crud" ]
