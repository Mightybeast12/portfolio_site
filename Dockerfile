
FROM rust:slim-bookworm AS builder


RUN cargo install trunk


RUN rustup target add wasm32-unknown-unknown


WORKDIR /app


COPY . .


RUN trunk clean
 
RUN trunk build --release


FROM nginx:alpine

COPY --from=builder /app/dist /usr/share/nginx/html

COPY nginx.conf /etc/nginx/nginx.conf

EXPOSE 8080


