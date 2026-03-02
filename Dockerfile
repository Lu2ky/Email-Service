FROM rust:alpine AS builder

WORKDIR /EmailService
COPY * .
RUN cargo install
RUN cargo build


FROM alpine:latest AS runner
WORKDIR /EmailService
COPY --from=builder /EmailService/target/debug/Email-Service /EmailService/
RUN touch .env
EXPOSE 8000
CMD [ "./Email-Service" ]