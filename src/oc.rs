use std::process::Command;
use std::{ffi::OsStr, ops::Deref};

use serde_json::{json, Value};
use thiserror::Error;

use chrono::{DateTime, Datelike, Duration, TimeZone, Utc};
use tracing::debug;

#[derive(Error, Debug)]
pub enum Error {
    #[error("format error")]
    FormatError(#[from] serde_json::Error),
    #[error("encode error")]
    EncodeError(#[from] std::string::FromUtf8Error),
    #[error("io error")]
    IOError(#[from] std::io::Error),
    #[error("server error `{0}`")]
    ServerError(String),
    #[error("command error `{0}`")]
    CommandError(String),
    #[error("unknown error")]
    FieldNotFound(String),
    #[error("unknown error")]
    Unknown,
}

type Result<T> = std::result::Result<T, Error>;

fn get_month_start() -> DateTime<Utc> {
    let now = Utc::now();
    let (year, month, _) = (now.year(), now.month(), 1);
    Utc.with_ymd_and_hms(year, month, 1, 0, 0, 0).unwrap()
}

fn get_month_end() -> DateTime<Utc> {
    let now = Utc::now();
    let (year, month, _) = (now.year(), now.month(), 1);
    let next_month = month % 12 + 1;
    let end_of_month = Utc.with_ymd_and_hms(year, next_month, 1, 0, 0, 0).unwrap();
    end_of_month
}

fn execute<S>(cmd: &[S]) -> Result<String>
where
    S: AsRef<OsStr> + Clone,
{
    let mut command = Command::new(&cmd[0]);
    let command = command.args(&cmd[1..]);
    let output = command.output()?;
    let response = String::from_utf8(output.stdout)?;
    if response.is_empty() {
        Err(Error::CommandError(String::from_utf8(output.stderr)?))
    } else {
        Ok(response)
    }
}

pub struct OracleCloud {
    config: String,
}

impl OracleCloud {
    fn invoke(&self, command: Vec<&str>) -> Result<Value> {
        debug!("execute command: {}", command.join(" "));
        let result = execute(&command)?;
        debug!("result {}", result);
        match result.strip_prefix("ServiceError:") {
            Some(result) => {
                let value: Value = serde_json::from_str(&result)?;
                let message = value
                    .get("message")
                    .and_then(|value| value.as_str())
                    .and_then(|value| Some(value.to_string()))
                    .ok_or(Error::ServerError("Unknown Error".to_string()))?;
                Err(Error::ServerError(message))
            }
            None => {
                let value = serde_json::from_str(&result)?;
                Ok(value)
            }
        }
    }

    pub fn new(config: String) -> Self {
        Self { config }
    }

    pub fn list_compartment(&self) -> Result<Vec<String>> {
        let command = vec![
            "oci",
            "iam",
            "compartment",
            "list",
            "--config-file",
            &self.config,
        ];
        let json = self.invoke(command)?;

        json.get("data")
            .and_then(|value| value.as_array())
            .and_then(|list| {
                list.iter()
                    .map(|value| {
                        value
                            .get("compartment-id")
                            .and_then(|value| value.as_str())
                            .and_then(|value| Some(value.to_string()))
                    })
                    .collect::<Option<Vec<String>>>()
            })
            .ok_or(Error::FieldNotFound("compartment-id".to_string()))
    }

    pub fn list_instances(&self, compartment_id: &str) -> Result<Vec<(String, String, String)>> {
        let command = vec![
            "oci",
            "compute",
            "instance",
            "list",
            "--config-file",
            &self.config,
            "--compartment-id",
            compartment_id,
        ];
        let json = self.invoke(command)?;
        json.get("data")
            .and_then(|value| value.as_array())
            .and_then(|list| {
                list.iter()
                    .map(|value| {
                        let id = value
                            .get("id")
                            .and_then(|value| value.as_str())
                            .and_then(|value| Some(value.to_string()));
                        let name = value
                            .get("display-name")
                            .and_then(|value| value.as_str())
                            .and_then(|value| Some(value.to_string()));
                        let state = value
                            .get("lifecycle-state")
                            .and_then(|value| value.as_str())
                            .and_then(|value| Some(value.to_string()));
                        id.zip(name).zip(state).map(|((a, b), c)| (a, b, c))
                    })
                    .collect::<Option<Vec<(String, String, String)>>>()
            })
            .ok_or(Error::FieldNotFound("id".to_string()))
    }

    pub fn query_data_transfer(&self, tenant_id: &str) -> Result<f64> {
        let month_start = get_month_start().to_rfc3339();
        let month_end = get_month_end().to_rfc3339();
        let command = vec![
            "oci",
            "usage-api",
            "usage-summary",
            "request-summarized-usages",
            "--granularity",
            "MONTHLY",
            "--tenant-id",
            tenant_id,
            "--time-usage-started",
            &month_start,
            "--time-usage-ended",
            &month_end,
            "--group-by",
            "[\"skuName\", \"skuPartNumber\", \"unit\", \"tenantName\"]",
        ];
        let json = self.invoke(command)?;
        json.pointer("/data/items")
            .and_then(|value| value.as_array())
            .and_then(|array| {
                array
                    .iter()
                    .find(|value| {
                        value
                            .pointer("/sku-name")
                            .and_then(|value| value.as_str())
                            .and_then(|value| Some(value == "Outbound Data Transfer Zone 1"))
                            .unwrap_or(false)
                    })
                    .and_then(|value| {
                        value
                            .pointer("/computed-quantity")
                            .and_then(|value| value.as_f64())
                    })
            })
            .ok_or(Error::FieldNotFound("Outbound Data Transfer".to_string()))
    }

    pub fn stop_instance(&self, instance_id: &str, soft_stop: bool) -> Result<()> {
        let action = if soft_stop { "SOFTSTOP" } else { "STOP" };
        let command = vec![
            "oci",
            "compute",
            "instance",
            "action",
            "--action",
            action,
            "--config-file",
            &self.config,
            "--instance-id",
            instance_id,
        ];
        self.invoke(command)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const config: &str = "~/.oci/config";
}
