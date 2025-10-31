use time::macros::format_description;
use tracing_subscriber::{
    EnvFilter,
    fmt::{self, time::LocalTime},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

pub fn init_logs() {
    let filter = EnvFilter::default()
        .add_directive("server=trace".parse().unwrap())
        .add_directive("axum=info".parse().unwrap());

    let timer = LocalTime::new(format_description!(
        "[day]-[month]-[year] [hour]:[minute]:[second]"
    ));
    
    tracing_subscriber::registry().with(filter).with(fmt::layer().with_timer(timer)).init();
}