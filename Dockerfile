FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt -y update
RUN apt install -y musl-tools musl-dev
RUN apt-get install -y build-essential
RUN apt install -y gcc-x86-64-linux-gnu

WORKDIR /app

COPY ./ .

# For a musl build, these ENV variables have to be set
ENV RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc'
ENV CC='gcc'
ENV CC_x86_64_unknown_linux_musl=x86_64-linux-gnu-gcc
ENV CC_x86_64-unknown-linux-musl=x86_64-linux-gnu-gcc

RUN cargo build --target x86_64-unknown-linux-musl --release

# We create the final Docker image “from scratch”
FROM scratch

WORKDIR /app

# We copy our binary and the .env file over to
# the final image to keep it small
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/warp-rest-api ./
COPY --from=builder /app/.env ./

CMD ["/app/warp-rest-api"]