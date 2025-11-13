use std::time::Duration;
use time::OffsetDateTime;
use sqlx::{PgPool, query};
use tracing::{info, error};
use tokio::{spawn, time::sleep};

const CLEANUP_INTERVAL: u64 = 60 * 60;

/// Starts a task that periodically deletes expired sessions from the database.
///
/// The task runs every `CLEANUP_INTERVAL` seconds.
///
/// The task is spawned in a separate thread, so it will not block the calling thread.
///
/// The task logs a message to the tracing log every time it runs,
/// indicating how many sessions were deleted.
///
/// The task logs an error to the tracing log if it encounters an error while running.
pub fn start_cleanup_task(db: PgPool) {
    spawn(async move {
        loop {
            match query!(
                "DELETE FROM Sessions WHERE ExpiresAt < $1",
                OffsetDateTime::now_utc()
            )
            .execute(&db)
            .await
            {
                Ok(count) => info!("deleted {} expired sessions", count.rows_affected()),
                Err(err) => error!("failed to delete expired sessions: {}", err),
            }

            sleep(Duration::from_secs(CLEANUP_INTERVAL)).await;
        }
    });
}