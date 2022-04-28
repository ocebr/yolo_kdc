FROM rust as builder

RUN apt-get update
RUN apt install libssl-dev
RUN apt install -y clang llvm-dev libclang-dev

COPY ./src /home/src/
COPY ./Cargo.toml ./home/Cargo.toml
COPY ./.env /home/.env
EXPOSE 8040

WORKDIR /home/


RUN cargo build --release

RUN  cp ./target/release/yolo_kdc /bin/yolo_kdc


FROM gcr.io/distroless/cc

COPY --from=builder --chown=1:1 ${HOME}/bin/yolo_kdc  /app/main
COPY --from=builder --chown=1:1 /home/.env app/.env
EXPOSE 8040
WORKDIR /app
USER 1000
CMD [ "./main" ]


