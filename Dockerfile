FROM redhat/ubi8
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly

RUN source /usr/local/cargo/env

COPY . tinyrenderer/

WORKDIR /tinyrenderer

RUN cargo install cargo-make

RUN cargo make

CMD cargo make serve
