FROM rust:latest as build

COPY . .

RUN mkidr -p /app

RUN cp /target/release/posts-server /app

RUN ubuntu:latest

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get -y update && \
    apt-get -y upgrade && \
    apt-get -y install ca-certificate libssl-dev libpq-dev

COPY --from=build /app/posts-server /usr/local/bin

ENTRYPOINT [ "posts-server" ]