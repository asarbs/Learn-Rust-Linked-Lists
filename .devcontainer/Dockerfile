FROM debian


USER root

ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID

COPY .devcontainer/library-scripts/*.sh /tmp/library-scripts/
RUN apt-get update
RUN apt-get upgrade -y \
    && bash /tmp/library-scripts/common-debian.sh
RUN apt-get install -y bash \
bash-completion \
python3 \
python3-pip git \
wget \
lsb-release \
software-properties-common \
gnupg \
gdb \
cmake \
libxrandr-dev \
libxcursor-dev \
libudev-dev \
libfreetype-dev \
libopenal-dev \
libflac-dev \
libvorbis-dev \
libgl1-mesa-dev \
libegl1-mesa-dev

SHELL ["/bin/bash", "-c"]

EXPOSE 3000

USER ${USERNAME}

RUN curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh -s -- -y

ENTRYPOINT [ "/usr/local/share/docker-init.sh" ]
CMD [ "sleep", "infinity" ]
