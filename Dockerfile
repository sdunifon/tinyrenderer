FROM redhat/ubi8

RUN yum install --assumeyes gcc make openssl openssl-devel pkgconf-pkg-config
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly

# RUN source /usr/local/cargo/env
# RUN echo "source $HOME/.cargo/env" >> ~/.cshrc #doesn't work
# RUN echo "source $HOME/.cargo/env" >> ~/.tcshrc  #doesn't work
ENV PATH "$PATH:/root/.cargo/bin"
RUN cargo install fd-find
COPY . /root/tinyrenderer/

WORKDIR /root/tinyrenderer

RUN cargo install cargo-make

RUN cargo make
RUN yum clean all -y
CMD cargo make serve
# CMD /bin/bash
