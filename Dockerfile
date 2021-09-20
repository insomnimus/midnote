FROM rust:1.54.0-buster

# install clippy
RUN rustup component add clippy

# Install C dependencies.
RUN apt-get update && \
	DEBIAN_FRONTEND=noninteractive apt-get install --no-install-recommends -y \
	build-essential \
	pkg-config \
	libasound2-dev

RUN mkdir /source
COPY Cargo.lock Cargo.toml /source/
COPY src /source/src/
COPY entrypoint.sh /source/

RUN chmod 755 /source/entrypoint.sh

ENTRYPOINT [ "/source//entrypoint.sh" ]