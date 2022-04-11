use std::env;
use mysql::*;
use mysql::prelude::*;
use chrono::prelude::*;

struct Stu {
    id: i64,
    name: String,
    age: i32,
    id_card: String,
    date: String,
}

fn main() {
    println!("Hello, world!");
    let address = env::var("DB_DSN").ok().unwrap_or_else(|| {
        "mysql://root:root1234@127.0.0.1:3306/test".to_string()
    });

    let opt = Opts::from_url(&address).unwrap();
    let pool = Pool::new(opt).unwrap(); // create mysql pool
    let mut conn = pool.get_conn().unwrap(); // get mysql connection

    // 流式查询 结果式逐行读取，好处就是整个数据永远不会存在内存中，如果是大量数据，用query_iter笔记好
    println!("=========query_iter tuple=========");
    conn.query_iter("select * from student").unwrap().for_each(|row| {
        let res: (i64, String, i32, String, String) = from_row(row.unwrap());
        println!("id = {} name = {} age = {} id_card = {} date = {:?}",
                 res.0, res.1, res.2, res.3, res.4);
    });

    println!("=========query vec=========");
    // 聚合查询 将查询结果映射到Vec中，每个元素都是一个元组
    let res: Vec<(i64, String, i32, String, String)> = conn.
        query("select * from student").
        unwrap();
    for row in res {
        println!("id = {} name = {} age = {} id_card = {} date = {:?}",
                 row.0, row.1, row.2, row.3, row.4);
    }

    println!("=========query_map result to struct====");
    // 查询结果映射到结构体中
    let res = conn.query_map("select * from student", |(id, name, age, id_card, update)| {
        Stu {
            id,
            name,
            age,
            id_card,
            date: update,
        }
    }).expect("query failed.");
    for row in res {
        println!("id = {} name = {} age = {} id_card = {} date = {:?}",
                 row.id, row.name, row.age, row.id_card, row.date);
    }

    println!("=========query_first result to struct====");
    // query_first 单条数据查询 返回的结果是Option<T> 需要将其解包两次才可以获得实际数据
    let res = conn.query_first("select * from student where id = 1").map(
        |row| {
            row.map(|(id, name, age, id_card, update)| {
                Stu {
                    id,
                    name,
                    age,
                    id_card,
                    date: update,
                }
            })
        });
    match res.unwrap() {
        Some(stu) => {
            println!("id = {} name = {} age = {} id_card = {} date = {:?}",
                     stu.id, stu.name, stu.age, stu.id_card, stu.date);
        }
        None => println!("sorry no student"),
    }

    // params! 命名参数进行参数绑定
    let stu = conn.exec_first("select id, name, age, id_card from student where id = :id",
                              params! {"id"=>1},
    ).map(|row| {
        row.map(|(id, name, age, id_card)| {
            Stu {
                id,
                name,
                age,
                id_card,
                date:"".to_string(),
            }
        })
    }).unwrap().unwrap();
    println!("id = {} name = {} age = {} id_card = {} date = {:?}",
             stu.id, stu.name, stu.age, stu.id_card, stu.date);
}

/*
总结
    经常使用的时间处理库： chrono
    流式查询使用： query_iter
    输出到Vec使用： query
    映射到结构体使用： query_map
    获取单条数据使用： query_first
    命名参数查询使用： exec_first
    from_row 和 from_value 的使用
 */