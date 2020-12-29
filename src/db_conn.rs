use diesel::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

use dotenv::dotenv;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn connect() {
    dotenv().ok();

    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
}
