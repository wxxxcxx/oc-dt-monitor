FROM rust:latest as builder

WORKDIR /oc-dt-monitor

COPY . /oc-dt-monitor/

RUN cargo build --release

FROM debian:latest

WORKDIR /oc-dt-monitor

ENV OCDTM_CONFIG=/oc-dt-monitor/config.yaml \
    OCDTM_EXECUTABLE=/root/bin/oci \
    OCDTM_STOP_METHOD=soft \
    OCDTM_TENANT_ID= \
    OCDTM_THRESHOLD=1000 \
    OCDTM_STOP_INSTANCES= \
    OC_INTERVAL=3600

RUN apt update && apt install -y curl && apt clean

RUN bash -c "$(curl -L https://raw.githubusercontent.com/oracle/oci-cli/master/scripts/install/install.sh)" -- --accept-all-defaults

COPY --from=builder /oc-dt-monitor/target/release/oc-dt-monitor /oc-dt-monitor/oc-dt-monitor

CMD ["/usr/bin/oc-dt-monitor", "start"]