FROM scratch

COPY target/x86_64-unknown-linux-musl/debug/client /client

ENTRYPOINT ["/client"]