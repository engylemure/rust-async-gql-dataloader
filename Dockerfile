FROM rust:1.42.0-slim

RUN apt-get update; \
    apt-get install -y --no-install-recommends \
    git \
    bash \
    nano \
    curl \
    wget \
    iputils-ping \
    pkg-config \
    libssl-dev \
    patch

ENV DOCKERIZE_VERSION v0.6.1
RUN wget https://github.com/jwilder/dockerize/releases/download/$DOCKERIZE_VERSION/dockerize-linux-amd64-$DOCKERIZE_VERSION.tar.gz \
    && tar -C /usr/local/bin -xzvf dockerize-linux-amd64-$DOCKERIZE_VERSION.tar.gz \
    && rm dockerize-linux-amd64-$DOCKERIZE_VERSION.tar.gz

COPY ./app /usr/src/app
COPY docker/ /files
RUN cp -rf /files/* /
RUN rm -rf /files

WORKDIR /usr/src/app
RUN cargo install systemfd cargo-watch
# Comment the below line if the build fails and uncomment it for running in production
# RUN cargo build --release

ENTRYPOINT ["dockerize", "-template", "/env.tmpl:/usr/src/app/.env"]

CMD ["bash", "/start.sh"]