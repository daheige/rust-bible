use yaml_rust::{YamlLoader, YamlEmitter};
use serde::{Serialize, Deserialize};
use serde_yaml;

fn main() {
    println!("Hello, world!");
    let s = r#"
    foo:
        - "abc"
        - "list2"
    bar:
        - 1
        - 2.0
    "#;

    let docs = YamlLoader::load_from_str(s).unwrap();
    let doc = &docs[0];

    // debug
    println!("{:?}", doc);

    // index access for map or array
    println!("foo.0 = {}", doc["foo"][0].as_str().unwrap());
    println!("bar.0 = {}", doc["bar"][1].as_f64().unwrap());
    assert_eq!(doc["foo"][0].as_str().unwrap(), "abc");
    assert_eq!(doc["bar"][1].as_f64().unwrap(), 2.0);

    // dump the yaml object to string
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(doc).unwrap();
    }

    println!("{}", out_str);
}

/*
输出yaml字符串
---
foo:
  - abc
  - list2
bar:
  - 1
  - 2.0
 */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Conf {
    foo: Vec<String>,
    bar: Vec<f64>,
    point: Point,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::Read;
    use yaml_rust::{YamlLoader};

    // 导入serde_yaml用来解析yaml
    use serde_yaml;

    #[test]
    fn read_yaml2() {
        let mut s = String::new();
        let mut f = File::open("test.yaml").unwrap();
        f.read_to_string(&mut s).unwrap();
        println!("s = {}", s); // 打印文件读到的内容

        // 解析yaml
        let docs = YamlLoader::load_from_str(&s).unwrap();
        let doc = &docs[0];

        // debug
        println!("{:?}", doc);

        // index access for map or array
        println!("foo.0 = {}", doc["foo"][0].as_str().unwrap());
        println!("bar.0 = {}", doc["bar"][1].as_f64().unwrap());
    }

    #[test]
    fn yaml_parse_and_format() {
        let mut s = String::new();
        File::open("test.yaml").unwrap().read_to_string(&mut s).unwrap();
        println!("s = {}", s); // 打印文件读到的内容

        // 通过serde_yaml将yaml解析到指定的结构体中
        let c: super::Conf = serde_yaml::from_str(&s).unwrap();
        println!("{:?}", c);

        // 将yaml序列化为string
        let s = serde_yaml::to_string(&c).unwrap();
        println!("s = {}", s);
    }
}
