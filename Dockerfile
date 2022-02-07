FROM redhat/ubi8

RUN yum install --assumeyes gcc make openssl openssl-devel pkgconf-pkg-config openssl-libs perl-core #compat-openssl10

RUN yum clean all -y

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly

# RUN source /usr/local/cargo/env
# RUN echo "source $HOME/.cargo/env" >> ~/.cshrc #doesn't work
# RUN echo "source $HOME/.cargo/env" >> ~/.tcshrc  #doesn't work
ENV PATH "$PATH:/root/.cargo/bin"

COPY . /root/tinyrenderer/

WORKDIR /root/tinyrenderer

RUN cargo install cargo-make

RUN cargo make

RUN cargo install fd-find
CMD cargo make serve
# CMD /bin/bash
