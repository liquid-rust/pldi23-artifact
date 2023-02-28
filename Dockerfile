FROM ubuntu:22.04

RUN apt-get update && apt-get install -y wget unzip python3

RUN mkdir /root/.local

# Install Rustup
RUN wget -qO- https://sh.rustup.rs | sh -s -- -y --default-toolchain none
ENV PATH=/root/.cargo/bin:$PATH

# Install Stack
RUN wget -qO- https://get.haskellstack.org | sh
ENV PATH=/root/.local/bin:$PATH

# Install Z3
RUN wget https://github.com/Z3Prover/z3/releases/download/z3-4.12.1/z3-4.12.1-x64-glibc-2.35.zip
RUN unzip z3-4.12.1-x64-glibc-2.35.zip
RUN rm z3-4.12.1-x64-glibc-2.35.zip
ENV PATH=/z3-4.12.1-x64-glibc-2.35/bin/:$PATH

# Install tokei
RUN wget https://github.com/XAMPPRocky/tokei/releases/download/v12.1.2/tokei-x86_64-unknown-linux-gnu.tar.gz
RUN tar -xf tokei-x86_64-unknown-linux-gnu.tar.gz && rm tokei-x86_64-unknown-linux-gnu.tar.gz && mv tokei /usr/local/bin

# Build Flux
WORKDIR /usr/artifact/flux
COPY flux ./
RUN cargo install --path flux
RUN cargo install --path flux-bin
RUN rm -rf flux

# Build Fixpoint
WORKDIR /usr/artifact/liquid-fixpoint
COPY liquid-fixpoint ./
RUN stack install
RUN rm -rf liquid-fixpoint
