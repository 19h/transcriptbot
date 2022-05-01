FROM rustlang/rust:nightly

RUN apt-get update
RUN DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    build-essential \
    ca-certificates \
    curl \
    git \
    libssl-dev \
    openssl \
    pkg-config \
    cmake

ADD . /my-source

RUN    cd /my-source \
    && cargo rustc --verbose --release \
    && mv /my-source/target/release/transcript-bot /transcript-bot \
    && rm -rfv /my-source

CMD ["/transcript-bot"]
