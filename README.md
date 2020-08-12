# live-helper

A nginx based docker image that is used to provide simple rtmp streaming server with authentication,
clients can see live video in hls format. A simple rust cli programs is used as helper to decode rtmp to hls files.

Important files:
 - /src/main.rs
 - /docker/nginx.conf
 - /docker/Dockerfile
