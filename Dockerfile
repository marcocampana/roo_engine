FROM rust:latest

WORKDIR /usr/src/myapp
COPY . .

RUN cargo build --release
EXPOSE 3030

CMD ["cargo", "run", "--", "--address",  "0.0.0.0", "--port",  "3030"]