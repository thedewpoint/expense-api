FROM rustlang/rust:nightly as build

WORKDIR /usr/src/expense-api
COPY . .

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/local/cargo/bin/expense-api /usr/local/bin/expense-api

CMD ["expense-api"]