use time::macros::format_description;
use tracing_subscriber::{
    EnvFilter,
    fmt::{self, time::LocalTime},
};

pub fn init_logs() {
    let _filter = EnvFilter::default()
        .add_directive("server=trace".parse().unwrap())
        .add_directive("axum=info".parse().unwrap());

    let _timer = LocalTime::new(format_description!(
        "[day]-[month]-[year] [hour]:[minute]:[second]"
    ));
}