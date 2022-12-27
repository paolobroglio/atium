FROM rust:1.66-alpine

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

RUN apk add  --no-cache ffmpeg

CMD ["video-converter"]