FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

# Include lld linker to improve build times either by using environment variable
# RUSTFLAGS="-C link-arg=-fuse-ld=lld" or with Cargo's configuration file (i.e see .cargo/config.toml).
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
   && apt-get -y install clang busybox protobuf-compiler \
   && apt-get upgrade -y \
   && apt-get autoremove -y && apt-get clean -y \
   && rm -rf /var/lib/apt/lists/*

# Set build argument for architecture
ARG TARGETARCH
RUN apt-get update \
    && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends wget \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

