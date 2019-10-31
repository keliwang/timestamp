use structopt::StructOpt;
use chrono::prelude::*;

fn main() {
    let opt = Opt::from_args();
    let dt = match opt {
        Opt::Now {} => {
            Utc::now()
        }
        Opt::Fmt { timestamp } => {
            Utc.timestamp_millis(timestamp)
        }
    };
    println!("timestamp:\t{}", dt.timestamp_millis());
    println!("utc:\t\t{}", dt.to_rfc3339());
    println!("local:\t\t{}", dt.with_timezone(&Local).to_rfc3339());
}

#[derive(StructOpt, Debug)]
#[structopt(name = "timestamp")]
enum Opt {
    Now {},
    Fmt {
        #[structopt(name = "timestamp in milliseconds")]
        timestamp: i64
    },
}
