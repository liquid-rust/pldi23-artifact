FROM ubuntu:22.04

RUN apt-get update && apt-get install -y wget unzip python3 build-essential pkg-config curl libssl-dev default-jdk vim nano

# Install Rustup
RUN wget -qO- https://sh.rustup.rs | sh -s -- -y --profile minimal
ENV PATH=/root/.cargo/bin:$PATH

# Install Stack
RUN wget -qO- https://get.haskellstack.org | sh
ENV PATH=/root/.local/bin:$PATH

# Install Z3
RUN wget https://github.com/Z3Prover/z3/releases/download/z3-4.12.1/z3-4.12.1-x64-glibc-2.35.zip &&\
    unzip z3-4.12.1-x64-glibc-2.35.zip &&\
    rm z3-4.12.1-x64-glibc-2.35.zip
ENV PATH=/z3-4.12.1-x64-glibc-2.35/bin/:$PATH

# Build Flux
WORKDIR /flux
COPY flux ./
RUN cargo install --path flux && cargo install --path flux-bin && rm -rf /flux

# Build Fixpoint
WORKDIR /liquid-fixpoint
COPY liquid-fixpoint ./
RUN stack install && rm -rf /liquid-fixpoint /root/.stack

# Build Prusti
WORKDIR /prusti-dev
COPY prusti-dev ./
RUN ./x.py setup && ./x.py build --release
ENV PATH=/prusti-dev/target/release:$PATH

WORKDIR /src
