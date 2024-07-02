use diesel::pg::PgConnection;
use diesel::r2d2;

pub fn init_db_pool(url: &str) -> r2d2::Pool<r2d2::ConnectionManager<PgConnection>> {
    let manager = r2d2::ConnectionManager::<PgConnection>::new(url);

    r2d2::Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not establish DB connection pool")
}
