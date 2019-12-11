FROM ubuntu:bionic

LABEL maintainer="Mojtaba Zarezadeh <mojtab.zare@gmail.com>"

RUN apt update && apt install -y software-properties-common

RUN add-apt-repository ppa:nginx/stable
RUN apt install -y ffmpeg nginx libnginx-mod-rtmp

COPY docker/nginx.conf /etc/nginx/nginx.conf
COPY target/release/live-helper-service /usr/local/bin/live-helper-service

RUN live-helper-service -V

VOLUME /data

EXPOSE 80 8080 1935

CMD ["nginx", "-g", "daemon off;"]
