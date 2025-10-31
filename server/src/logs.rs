use time::macros::format_description;
use tracing_subscriber::{
    EnvFilter,
    fmt::{self, time::LocalTime},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

pub fn init_logs() -> tracing_appender::non_blocking::WorkerGuard {
    let filter = EnvFilter::default()
        .add_directive("server=trace".parse().unwrap())
        .add_directive("axum=info".parse().unwrap());

    let timer = LocalTime::new(format_description!(
        "[day]-[month]-[year] [hour]:[minute]:[second]"
    ));

    let exe_path = std::env::current_exe().expect("failed to get executable path");
    let exe_folder = exe_path.parent().expect("failed to get executable folder");
    let (non_blocking, _guard) = tracing_appender::non_blocking(tracing_appender::rolling::daily(
        exe_folder.join("logs"),
        "server",
    ));

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().with_timer(timer))
        .with(fmt::layer().with_writer(non_blocking).with_ansi(false))
        .init();

    _guard
}