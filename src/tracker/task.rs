// Represents a named task (e.g. "coding", "reading", "gym")
// Tasks are the labels your sessions get grouped under

// derive - a macro that makes traits for the struct
// 
#[derive(Debug, sqlx::FromRow)]
pub struct Task {
    id: i32,
    name: String,
    created_at: chrono::DateTime<chrono::Utc>,
}