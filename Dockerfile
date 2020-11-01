FROM rustlang/rust:nightly-slim as build
WORKDIR /usr/src/expense-api
COPY . .

RUN cargo install --locked --verbose --path .

FROM alpine:latest

COPY --from=build /usr/local/.cargo/bin/expense-api /usr/local/bin/expense-api

CMD ["expense-api"]