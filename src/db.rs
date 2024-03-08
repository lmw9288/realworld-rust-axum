use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};


pub async fn my_pool() -> Pool<MySql> {
    let pool = MySqlPoolOptions::new()
        .max_connections(20)
        .connect("mysql://root@localhost:3306/realworld-axum-sqlx")
        .await
        .expect("database can not connect");
    pool
}
