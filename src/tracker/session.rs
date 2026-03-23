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


pub async fn start_session(pool: &sqlx::PgPool, task_name: &str) {
    // look up task id

    let task = sqlx::query!( // ! is a macro, $1 is first param
        "SELECT id FROM tasks WHERE name = $1",
        task_name
    )
    .fetch_optional(pool) // expect one row or crash with none
    .await
    .expect("db error, you might wanna peep that");

    let task_id = match task {
        Some(t) => t.id, // some task t exists, use its id
        None => {
            sqlx::query!("INSERT INTO tasks (name) VALUES ($1) RETURNING id",
            task_name
        )
        .fetch_one(pool) // crashes if no row written
        .await
        .expect("Failed to make a task") // shouldn't ever not get a response back i assume that been written
        .id
        }
    };

    sqlx::query!(
        "INSERT INTO sessions (task_id, started_at) VALUES ($1, NOW())",
        task_id
    )
    .execute(pool)
    .await
    .expect("Couldn't write into sessions table");

    println!("Wrote session start for: {}", task_name);
}