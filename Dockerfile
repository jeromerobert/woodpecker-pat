# Docker image for Centos 7 / Amazon Linux 2 compatible builds
FROM centos:7
# Adapted from https://github.com/rust-lang/docker-rust/

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN curl -O https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init
RUN chmod +x rustup-init
RUN ./rustup-init -y --profile minimal --no-modify-path
RUN yum install -y gcc sqlite-devel
