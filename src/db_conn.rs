use diesel::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

use dotenv::dotenv;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn connect() {
    dotenv().ok();
}
