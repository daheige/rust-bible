use chrono::prelude::*;
use futures::TryStreamExt;
use sqlx;
use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use sqlx::Row;
use std::env;
use std::time::Duration;
use tokio;

#[derive(Debug, sqlx::FromRow)]
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
    // The fetch query finalizer returns a stream-like type
    // that iterates through the rows in the result sets.
    let sql = "select * from student where id >= ?";
    let records = sqlx::query(sql)
        .bind(1)
        .map(|row: MySqlRow| Stu {
            // 这里需要指定row类型
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

    // 3、使用execute，执行更新操作，返回 affect_rows
    // the executor query finalizer returns the number of affected rows,
    // if any, and drops all received results. In addition, there are fetch,
    // fetch_one, fetch_optional, and fetch_all to receive results.
    let sql = r#"update student set name = ? where id = ?"#;
    let affect_rows = sqlx::query(sql)
        .bind("heige")
        .bind(1)
        .execute(&pool)
        .await?;
    println!("{:?}", affect_rows);

    // 4、使用execute和fetch，执行插入操作，获取自增id
    let sql = r#"insert into student (name,age,id_card,last_update) value(?,?,?,?)"#;
    let affect_rows = sqlx::query(sql)
        .bind("heige")
        .bind(32)
        .bind("abc")
        .bind(chrono::NaiveDate::from_ymd(2022, 04, 13))
        .execute(&pool)
        .await?;
    let id = affect_rows.last_insert_id();
    println!("id = {}", id);

    // ========查询结果集转化为struct========
    // 5、使用fetch获取结果集Vec的流Stream数据
    // To assist with mapping the row into a domain type,
    let sql = "select * from student where id >= ?";
    let mut stream = sqlx::query_as::<_, Stu>(sql).bind(1).fetch(&pool);
    while let Some(user) = stream.try_next().await? {
        println!("{:?}", user);
    }

    // 6、使用fetch_one获取一条结果集
    let sql = "select * from student where id = ?";
    let user: Stu = sqlx::query_as(sql).bind(1).fetch_one(&pool).await?;
    println!("id = {} name = {}", user.id, user.name);

    // 7、使用fetch_all获取多个记录，将所有的结果集放到Vec
    let sql = "select * from student where id >= ?";
    let records: Vec<Stu> = sqlx::query_as(sql).bind(1).fetch_all(&pool).await?;
    for row in records {
        // println!("s = {:?}", row);
        println!("id = {} name = {}", row.id, row.name);
    }

    // 8、事务处理
    let sql = r#"insert into student (name,age,id_card,last_update) value(?,?,?,?)"#;
    let mut conn = pool.begin().await?; // 创建一个conn
    let affect_rows = sqlx::query(sql)
        .bind("daheige")
        .bind(32)
        .bind("abc")
        .bind(chrono::NaiveDate::from_ymd(2022, 04, 13))
        .execute(&mut conn) // 这里是传递可变的conn
        .await?;
    conn.commit().await?; // 提交事务
    let id = affect_rows.last_insert_id(); // 获取插入的id
    println!("id = {}", id);

    Ok(())
}
