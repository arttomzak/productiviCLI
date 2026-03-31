// Loads environment variables from .env
// Will hold database URL and any other config values

pub fn load() -> String {
    dotenvy::dotenv().ok(); // .ok() keeps us from crashing on an empty env call
    std::env::var("DATABASE_URL").expect("DATABASE_URL is not set yo") // .expect(xyz) at the end will give a val or crash w xyz message

    // cool rust thing, if you don't have a semicolon on the last line
    // itll auto return
}
