#####################
### Build Backend ###
#####################

FROM rust:1.64.0-alpine3.16 as builder_backend

# Install musl libc

RUN apk --no-cache add musl-dev

# Create backend rust package

WORKDIR /usr/src

RUN USER=root cargo new server

COPY server/Cargo.toml server/Cargo.lock /usr/src/server/

WORKDIR /usr/src/server

# Dummy build to cache dependencies

RUN cargo build --target x86_64-unknown-linux-musl

# Copy in sources

COPY server/src /usr/src/server/src/

## Touch main.rs to prevent cached build

RUN touch /usr/src/server/src/main.rs

# Build application

RUN cargo build --target x86_64-unknown-linux-musl


######################
### Build Frontend ###
######################

FROM node:18.9.1-alpine3.16 AS builder_frontend

# Create frontend nodejs package

COPY web/package.json web/yarn.lock /usr/src/web/

WORKDIR /usr/src/web

# Dummy build to cache dependencies

RUN yarn install

# Copy in sources

COPY web/tailwind.config.js web/webpack.config.js /usr/src/web/
COPY web/src /usr/src/web/src/

# Build application

RUN yarn build


###############
### Runtime ###
###############

FROM alpine:3.16 AS runtime

RUN apk --no-cache add sqlite

# Copy binary and built frontend code
COPY --from=builder_backend /usr/src/server/target/x86_64-unknown-linux-musl/debug/server /cs348/
COPY --from=builder_frontend /usr/src/web/build /cs348/public/

# Copy DB and Rocket.toml
COPY db/store.db server/Rocket.toml /cs348/

EXPOSE 3030

WORKDIR /cs348

CMD ["/cs348/server"]
