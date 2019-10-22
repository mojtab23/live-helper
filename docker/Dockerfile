FROM ubuntu:bionic

LABEL maintainer="Mojtaba Zarezadeh <mojtab.zare@gmail.com>"

#RUN apt update && apt install -y software-properties-common

RUN add-apt-repository ppa:nginx/stable
RUN apt install -y ffmpeg nginx libnginx-mod-rtmp

CMD ["nginx" ,"-V"]
