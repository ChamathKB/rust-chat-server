FROM ubuntu:18.04
 
RUN apt-get update && apt-get install -y curl
RUN apt-get install build-essential -y

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN mkdir /rust-chat-server

COPY ./ /rust-chat-server/
WORKDIR /rust-chat-server

RUN cargo build --release

CMD ["./target/release/rust-chat-server"]

EXPOSE 9090