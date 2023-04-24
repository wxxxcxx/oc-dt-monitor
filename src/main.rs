use tracing::Level;
use tracing_subscriber::FmtSubscriber;

mod oc;

const config: &str = "~/.oci/config";
fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let oc = oc::OracleCloud::new(config.to_string());
    println!("{:?}", oc.list_instances("ocid1.tenancy.oc1..aaaaaaaak4ktoonlhts54ighhtnuuaeh52sqasebovimqdlekd3nqr6vqlva"));
    // println!("{:?}", oc.query_data_transfer("ocid1.tenancy.oc1..aaaaaaaak4ktoonlhts54ighhtnuuaeh52sqasebovimqdlekd3nqr6vqlva"));
    // println!(
    //     "{:?}",
    //     oc.query_data_transfer(
    //         "ocid1.tenancy.oc1..aaaaaaaak4ktoonlhts54ighhtnuuaeh52sqasebovimqdlekd3nqr6vqlva"
    //     )
    // );
    // println!(
    //     "{:?}",
    //     oc.stop_instance(
    //         "ocid1.instance.oc1.us-sanjose-1.anzwuljrzy3jtficpcyssgehp5u3cmv3a7k6qtmup5pgquhelgwm6fbmedfa",
    //         true
    //     )
    // );
}
