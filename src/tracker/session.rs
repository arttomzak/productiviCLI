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

    let task = sqlx::query!(
        // ! is a macro, $1 is first param
        "SELECT id FROM tasks WHERE name = $1",
        task_name
    )
    .fetch_optional(pool) // expect one row or crash with none
    .await
    .expect("db error, you might wanna peep that");

    let task_id = match task {
        Some(t) => t.id, // some task t exists, use its id
        None => {
            sqlx::query!(
                "INSERT INTO tasks (name) VALUES ($1) RETURNING id",
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

    // println!("Wrote session start for: {}", task_name);
}

pub async fn stop_session(pool: &sqlx::PgPool) {
    let session = sqlx::query!("SELECT id FROM sessions WHERE ended_at IS NULL",)
        .fetch_optional(pool)
        .await
        .expect("Db error yo");

    match session {
        Some(s) => {
            sqlx::query!(
                "UPDATE sessions SET ended_at = NOW(), duration_secs = EXTRACT(EPOCH FROM NOW() - started_at) WHERE id = $1",
                s.id
            )
            .execute(pool)
            .await
            .expect("Couldn't write into sessions table");
        }
        None => println!("no session active!"),
    };

    // println!("end to session written successfully!");
}

pub async fn get_active_session(
    pool: &sqlx::PgPool,
) -> Option<(String, chrono::DateTime<chrono::Utc>)> {
    let row = sqlx::query!(
        "SELECT tasks.name, sessions.started_at
        FROM sessions
        JOIN tasks on tasks.id = sessions.task_id
        WHERE sessions.ended_at IS NULL"
    ) // the join lets us grab .name and .started_at
    .fetch_optional(pool)
    .await
    .expect("db error yo");

    row.map(|r| (r.name, r.started_at)) // if theres a val in r, transform it
}

// // eventually wanna migrate to having it constantly show how long the current task has been running for but the work here will be useful
// pub async fn session_session(pool: &sqlx::PgPool) {
//     let session = sqlx::query!(
//         "SELECT id FROM sessions WHERE ended_at IS NULL",
//     )
//     .fetch_optional(pool)
//     .await
//     .expect("Db error yo");

//     match session {
//         Some(s) => {
//             let cur_duration = s.duration_secs // have to calculate this
//             println!("Tracking {} for {}", s.task_name, s.time),
//         },
//         None => {
//             println!("No session active!"),
//         }
//     };
// }

// pub async fn day_sessions(pool: &sqlx::PgPool) {
//     let sessions = sqlx::query!(
//         "SELECT"
//     )
// }
