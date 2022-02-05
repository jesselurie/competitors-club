FROM ubuntu:20.04

# metadata
ARG VCS_REF
ARG BUILD_DATE
ARG IMAGE_NAME

# show backtraces
ENV RUST_BACKTRACE 1

# install tools and dependencies
RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y \
        libssl1.1 \
        ca-certificates \
        curl && \
# apt cleanup
    apt-get autoremove -y && \
    apt-get clean && \
    find /var/lib/apt/lists/ -type f -not -name lock -delete; \
# add user and link ~/.local/share/crown-sterling to /data
    useradd -m -u 1000 -U -s /bin/sh -d /competitors-club competitors-club && \
    mkdir -p /data /competitors-club/.local/share && \
    mkdir /chain-spec && \ 
    chown -R competitors-club:competitors-club /chain-spec && \
    chown -R competitors-club:competitors-club /data && \
    chown -R competitors-club:competitors-club /competitors-club && \
    ln -s /data /competitors-club/.local/share/competitors-club

# add crown-sterling binary to docker image
COPY ./competitors-club-node /usr/local/bin
COPY ./mainnetSpecRaw.json /chain-spec

USER competitors-club

# check if executable works in this container
RUN /usr/local/bin/competitors-club-node --version

EXPOSE 30330 
EXPOSE 9933
EXPOSE 9944

VOLUME ["/competitors-club"]
ENTRYPOINT ["/usr/local/bin/competitors-club-node", "--chain=/chain-spec/mainnetSpecRaw.json"]