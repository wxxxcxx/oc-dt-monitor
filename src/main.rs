use std::collections::HashMap;

use oc::OracleCloud;
use structopt::StructOpt;
use tracing::{info, warn, Level};
use tracing_subscriber::FmtSubscriber;

mod oc;
mod opt;

static mut CLEAN: bool = false;

macro_rules! clean_not_println {
    ($($arg:tt)+) => {
        if unsafe{CLEAN ==false} {
            println!($($arg)+);
        }
    };
}

fn run(oc: &OracleCloud, opt: &opt::Opt) -> oc::Result<()> {
    let data_transfer = oc.query_data_transfer()?;
    info!("Current data transfer usage: {}GB", data_transfer);
    if opt.clean {
        println!("{}", data_transfer);
    } else {
        clean_not_println!("Current data transfer usage: {}GB", data_transfer);
    }
    if data_transfer > opt.threshold as f64 {
        info!("Data transfer exceeds the threshold ({}GB)", opt.threshold);
        clean_not_println!("Data transfer exceeds the threshold ({}GB)", opt.threshold);
        if opt.auto_stop {
            info!("The flag `auto_stop` is enabled, prepare to stop instances");
            clean_not_println!("The flag `auto_stop` is enabled, prepare to stop instances");
            let instances = oc.list_instances()?;
            let instances = instances.iter().fold(HashMap::new(), |mut map, instance| {
                map.insert(
                    instance.0.to_owned(),
                    (instance.1.to_owned(), instance.2.to_owned()),
                );
                map
            });
            info!("Found instances:");
            clean_not_println!("Found instances:");
            for (id, (name, state)) in &instances {
                info!("{}({}): {}", name, id, state);
            }
            if let Some(need_stop_instances) = &opt.instances {
                for instance in need_stop_instances {
                    match instances.get(instance) {
                        Some((name, state)) => {
                            if state != "RUNNING" {
                                continue;
                            }
                            info!(
                                "Prepare to stop instance: {}({}): {}",
                                name, instance, state
                            );
                            clean_not_println!(
                                "Prepare to stop instance: {}({}): {}",
                                name,
                                instance,
                                state
                            );
                            oc.stop_instance(instance, &opt.stop_method)?;
                            info!("Instance {} stopped", instance);
                            clean_not_println!("Instance {} stopped", instance);
                        }
                        None => {
                            info!("Instance {} not found", instance);
                            clean_not_println!("Instance {} not found", instance);
                        }
                    }
                }
            } else {
                for (id, (name, state)) in instances {
                    if state != "RUNNING" {
                        continue;
                    }
                    info!("Prepare to stop instance: {}({}): {}", name, id, state);
                    oc.stop_instance(&id, &opt.stop_method)?;
                    clean_not_println!("Instance {} stopped", id)
                }
            }
        }
    }
    Ok(())
}

fn main() {
    let opt = opt::Opt::from_args();
    unsafe {
        CLEAN = opt.clean;
    }
    let subscriber = FmtSubscriber::builder()
        .with_max_level(if opt.debug { Level::TRACE } else { Level::WARN })
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let oc: oc::OracleCloud = oc::OracleCloud::new(
        opt.tenant_id.clone(),
        opt.path.to_str().unwrap().to_string(),
        opt.config.to_str().unwrap().to_string(),
    );

    match &opt.command {
        Some(opt::Command::Start(start)) => {
            info!("Start monitor with interval: {}s", start.interval);
            loop {
                match run(&oc, &opt) {
                    Ok(_) => {}
                    Err(e) => {
                        warn!("Error: {}", e);
                    }
                }
                std::thread::sleep(std::time::Duration::from_secs(start.interval));
            }
        }
        None => match run(&oc, &opt) {
            Ok(_) => {}
            Err(e) => {
                warn!("Error: {}", e);
            }
        },
    }
}
