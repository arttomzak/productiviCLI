// Represents a single timed session
// Holds start time, end time, duration, and associated task

#[derive(Debug, sqlx::FromRow)]
pub struct Session {
    id: i32,
    task_id: i32,
    started_at: chrono::DateTime<chrono::Utc>,
    ended_at: Option<chrono::DateTime<chrono::Utc>>,
    duration_secs: Option<i64>,
}
