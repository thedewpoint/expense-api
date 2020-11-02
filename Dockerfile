FROM instrumentisto/rust:nightly-slim-2020-10-24 as build
WORKDIR /usr/src/expense-api
COPY . .
RUN cargo install --locked --path .
FROM gcr.io/distroless/cc-debian10
COPY --from=build /usr/local/cargo/bin/expense-api /usr/local/bin/expense-api
CMD ["expense-api"]