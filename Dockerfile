FROM jac18281828/rustdev:latest

ARG PROJECT=rusthello
WORKDIR /workspaces/${PROJECT}
COPY . .
RUN chown -R jac:jac .
USER jac

ENV PATH=/home/jac/.cargo/bin:$PATH
# source $HOME/.cargo/env
RUN rustc --version
