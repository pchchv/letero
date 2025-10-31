use tracing_subscriber::EnvFilter;

pub fn init_logs() {
    let _filter = EnvFilter::default()
        .add_directive("server=trace".parse().unwrap())
        .add_directive("axum=info".parse().unwrap());
}