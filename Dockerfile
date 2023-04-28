FROM rust:latest as builder
WORKDIR /oc-dt-monitor
COPY . /oc-dt-monitor/
RUN cargo build --release

FROM debian:latest
RUN bash -c "$(curl -L https://raw.githubusercontent.com/oracle/oci-cli/master/scripts/install/install.sh)"
COPY --from=builder /oc-dt-monitor/target/release/oc-dt-monitor /usr/bin/oc-dt-monitor
CMD ["/usr/bin/oc-dt-monitor", "start"]