use sqlx::PgPool;

pub async fn init_db() -> PgPool {
    let connection_string = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPool::connect_lazy(&connection_string).expect("failed to connect to database")
}