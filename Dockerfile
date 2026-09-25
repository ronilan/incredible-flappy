FROM ubuntu:24.04

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        curl \
        unzip \
    && rm -rf /var/lib/apt/lists/*

RUN curl -fL \
    https://github.com/ronilan/incredible-flappy/releases/latest/download/incredible_flappy-terminal-linux.zip \
    -o /tmp/incredible_flappy.zip \
    && unzip -o /tmp/incredible_flappy.zip -d /usr/local/bin \
    && rm /tmp/incredible_flappy.zip \
    && chmod +x /usr/local/bin/incredible_flappy

CMD ["/bin/bash", "-i"]