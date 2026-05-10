FROM rust:1.94 AS build
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=build /app/target/release/degcolor /usr/local/bin/degcolor
CMD ["degcolor"]
