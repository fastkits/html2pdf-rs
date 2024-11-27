
FROM rust:1.80.1 as builder
RUN sed -i 's|deb.debian.org|mirrors.tuna.tsinghua.edu.cn|g' /etc/apt/sources.list.d/debian.sources
COPY . /rs-html2pdf
WORKDIR /rs-html2pdf
RUN apt update \
  && apt install -y cmake libclang-dev wget gnupg ca-certificates lsb-release protobuf-compiler --no-install-recommends
RUN rustup target list --installed
RUN cargo build --release

FROM ubuntu:24.04
COPY SourceHanSerifCN-Regular-1.otf /usr/share/fonts/
COPY SourceHanSansCN-Regular.otf /usr/share/fonts/
# RUN sed -i 's/https:\/\/mirrors.aliyun.com/http:\/\/mirrors.cloud.aliyuncs.com/g' /etc/apt/sources.list
COPY google-chrome-stable_current_amd64.deb .
RUN apt-get update
RUN rm -rf /etc/localtime && ln -sf /usr/share/zoneinfo/Asia/Shanghai /etc/localtime && echo 'Asia/Shanghai' >/etc/timezone
RUN dpkg -i google-chrome-stable_current_amd64.deb || apt-get install -y -f
RUN rm -rf google-chrome-stable_current_amd64.deb
EXPOSE 8080
COPY --from=builder /rs-html2pdf/target/release/rs-html2pdf /usr/local/bin/rs-html2pdf
CMD ["rs-html2pdf"]
