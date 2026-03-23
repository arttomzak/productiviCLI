-- making the sessions table, this'll keep track of a sessions timestamps and duration
CREATE TABLE sessions(
    id SERIAL PRIMARY KEY,
    task_id INTEGER NOT NULL REFERENCES tasks(id),
    started_at TIMESTAMPTZ NOT NULL,
    ended_at TIMESTAMPTZ,
    duration_secs BIGINT
);

-- speeds up future queries on a specific task
CREATE INDEX idx_sessions_task_id ON sessions(task_id);