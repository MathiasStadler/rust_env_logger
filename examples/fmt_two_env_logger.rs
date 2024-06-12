use log::{debug, error, log_enabled, info, Level};
use std::io::Write;


fn main(){
// env_logger::init();
env_logger::builder()
    .format(|buf, record| {
     //   writeln!(buf, "{}: {}", record.level(), record.args())
writeln!(
                buf,
                "{}:{} {} - {}",
                // record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                // chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args(),
                record
            )
    })
    .init();

debug!("this is a debug {}", "message");
error!("this is printed by default");

if log_enabled!(Level::Info) {
    let x = 3 * 4; // expensive computation
    info!("the answer was: {}", x);
}
}
