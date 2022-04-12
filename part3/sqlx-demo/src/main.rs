use chrono::prelude::*;
use futures::TryStreamExt;
use sqlx;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::Row;
use std::env;
use std::time::Duration;
use tokio;

#[derive(Debug)]
struct Stu {
    id: i64,
    name: String,
    age: i32,
    id_card: String,
    last_update: NaiveDate, // 时间格式
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let default_dsn = "mysql://root:root1234@localhost/test";
    let dsn = env::var("DB_DSN").unwrap_or(default_dsn.to_string());
    let pool = MySqlPoolOptions::new()
        .max_connections(50) // 最大连接数
        .min_connections(3) // 最小连接数
        .max_lifetime(Duration::from_secs(1800)) // 最大生命周期
        .idle_timeout(Duration::from_secs(600)) // 空闲连接的生命周期
        .connect_timeout(Duration::from_secs(10)) // 连接超时
        .connect(&dsn)
        .await?;

    // 通过mysql ?占位符，bind 绑定参数形式查询
    let row: (i64,) = sqlx::query_as("select ?")
        .bind(120i64)
        .fetch_one(&pool)
        .await?;

    println!("res: {}", row.0);
    assert_eq!(row.0, 120);

    // 1、使用fetch，获取cursor游标，自己处理row
    let sql = "select * from student where id >= ?";
    let mut rows = sqlx::query(sql).bind(1).fetch(&pool);
    while let Some(row) = rows.try_next().await? {
        let stu = Stu {
            id: row.get("id"),
            name: row.get("name"),
            age: row.get("age"),
            id_card: row.get("id_card"),
            last_update: row.get("last_update"),
        };

        println!("stu = {:?}", stu);
    }

    // 2、使用fetch，加map，自动处理row
    let sql = "select * from student where id >= ?";
    let records = sqlx::query(sql)
        .bind(1)
        .map(|row: sqlx::mysql::MySqlRow| Stu {
            id: row.get("id"),
            name: row.get("name"),
            age: row.get("age"),
            id_card: row.get("id_card"),
            last_update: row.get("last_update"),
        })
        .fetch(&pool);

    tokio::pin!(records);

    while let Some(s) = records.try_next().await? {
        println!("s = {:?}", s);
    }

    // 3、使用execute，执行更新操作，返回affect_rows
    let sql = r#"update student set name = ? where id = ?"#;
    let mut affect_rows = sqlx::query(sql)
        .bind("heige")
        .bind(1)
        .execute(&pool)
        .await?;
    println!("{:?}", affect_rows);

    Ok(())
}
