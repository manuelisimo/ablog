FROM rust:1.45 as backend

WORKDIR /usr/src/ablog
COPY . .
RUN cargo build --release

FROM node as frontend
WORKDIR /usr/src/fe
COPY ./fe .
RUN yarn install
RUN yarn build

FROM debian:stable
RUN apt-get update && apt-get install -y \
    fortunes \
    fortune-mod \ 
    libsqlite3-0 \
    libssl1.1

RUN ln -s /usr/games/fortune /usr/local/bin/fortune
COPY --from=frontend /usr/src/fe/dist /usr/share/ablog
COPY --from=backend /usr/src/ablog/target/release/ablog /usr/bin/ablog
COPY --from=backend /usr/src/ablog/static /usr/share/ablog

ENV DATABASE_URL=file:/var/ablog/ablog.db \
    RUST_LOG="ablog=info,actix_web=info" \
    STATIC_PATH=/static \
    STATIC_DIR=/usr/share/ablog/ \
    TLS_PRIVATE_KEY=/var/ablog/key.pem \
    TLS_CERTIFICATE=/var/ablog/cert.pem
EXPOSE 80 443
CMD ["ablog"]
