FROM rust:1.61 as rust_base

#------------
# Get trunk
#------------
# ENV VERSION=v0.16.0
# WORKDIR /tmp
# RUN curl -L -o trunk.tar.gz https://github.com/thedodd/trunk/releases/download/${VERSION}/trunk-x86_64-unknown-linux-gnu.tar.gz; \
#   tar -xzf trunk.tar.gz; \
#   cp trunk /usr/local/bin
RUN cargo install trunk --locked trunk

#------------
# Build the app
#------------
WORKDIR /usr/src/pizza

COPY assets assets
COPY Cargo* .
COPY index.html .
COPY src src
RUN rustup target add wasm32-unknown-unknown \
  && trunk build --release

# FROM app_builder as installer
FROM nginx:1.23
COPY --from=rust_base /usr/src/pizza/dist /usr/share/nginx/html
