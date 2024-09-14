
FROM rust:slim-bookworm as builder


RUN cargo install trunk


RUN rustup target add wasm32-unknown-unknown


WORKDIR /app


COPY . .


RUN trunk build --release


FROM nginx:alpine


COPY --from=builder /app/dist /usr/share/nginx/html


EXPOSE 80


