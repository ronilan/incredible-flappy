FROM ubuntu:24.04

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        curl \
        unzip \
    && rm -rf /var/lib/apt/lists/*

RUN curl -fL \
    https://github.com/ronilan/incredible_app_template/releases/latest/download/incredible_app_template-terminal-linux.zip \
    -o /tmp/incredible_app_template.zip \
    && unzip -o /tmp/incredible_app_template.zip -d /usr/local/bin \
    && rm /tmp/incredible_app_template.zip \
    && chmod +x /usr/local/bin/incredible_app_template

CMD ["/bin/bash", "-i"]
