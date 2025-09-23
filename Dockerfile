FROM alpine:3.22

RUN apk add curl
RUN apk add alpine-sdk

RUN adduser -D user
USER user

#installs Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/home/user/.cargo/bin:${PATH}"
RUN rustup install 1.90 && rustup default 1.90

WORKDIR /home/user/app
COPY --chown=user . .
RUN cargo install --locked --path .

ENTRYPOINT ["piet_programming_language"]
