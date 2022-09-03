# struct to map macro使用
1. 创建项目
```shell
cargo new into_map_demo
cd into_map_demo
cargo new into_map --lib
cargo new into_map_derive --lib
```
2. 在into_map_demo cargo.toml添加如下内容：
```toml
[dependencies]
into_map = {path = "into_map"}
into_map_derive = {path = "into_map_derive"}
```
在into_map_demo/src/main.rs添加代码
```rust
use into_map::IntoMapTrait;
use into_map_derive::IntoMap;

// 指定IntoMap trait 来自into_map_derive包的派生宏
#[derive(IntoMap)]
struct User {
    name: String,
    id: usize,
    active: bool,
}

#[derive(IntoMap)]
struct Person {
    id: i64,
    name: String,
    sex: i64,
}

fn main() {
    let user = User {
        name: "daheige".to_string(),
        id: 12,
        active: true,
    };
    let map = user.into_map();
    println!("m:{:?}", map);

    let p = Person {
        id: 123,
        name: "xiaoming".to_string(),
        sex: 1,
    };
    let m = p.into_map();
    println!("m:{:?}", m);
}
```

3. 然后在into_map/src/lib.rs中添加如下内容：
```rust
// 定义IntoMap trait
use std::collections::BTreeMap;

pub trait IntoMapTrait {
    fn into_map(&self) -> BTreeMap<String, String>;
}
```
4. 在into_map_derive cargo.toml中添加如下内容：
```toml
[lib]
#指定这是一个宏的组件库
proc-macro = true

[dependencies]
# syn和quote帮助我们解析标记流实例中的rust代码
# syn 负责ast的内存数据结构解析
# quote是syn补充，允许用户在其中的quote!中生成rust代码，还允许用户替换syn数据类型中的值
syn = { version = "1.0.99",features = ["extra-traits"]}
quote = "1.0.21"
into_map = { path = "../into_map"}
```
在into_map_derive/src/lib.rs添加如下派生宏实现代码：
```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

// 派生宏实现
// 实现IntoMap trait derive
// into_map_derive 在时候的时候是 #[derive(IntoMap)]
#[proc_macro_derive(IntoMap)]
pub fn into_map_derive(input: TokenStream) -> TokenStream {
    // 先遍历结构体得到field_name field_value
    let mut insert_tokes = vec![];
    let parsed_input: DeriveInput = parse_macro_input!(input);
    let struct_name = parsed_input.ident; // 结构体名字
    match parsed_input.data {
        Data::Struct(s) => {
            if let Fields::Named(named_fields) = s.fields {
                let a = named_fields.named; // 字段列表
                for i in a {
                    let field = i.ident.unwrap();
                    let insert_token = quote! {
                        // #field符号在quote!宏中，相当于结构体中的字段名称替换
                        // 这个map是下面的let mut map = BTreeMap::new();
                        map.insert(
                            stringify!(#field).to_string(),
                            self.#field.to_string()
                        );
                    };
                    insert_tokes.push(insert_token);
                }
            }
        }
        other => panic!("IntoMap is not yet impl for:{:?}", other),
    }

    // 代码生成阶段
    let tokens = quote! {
        // 为结构体实现IntoMap trait
        impl IntoMapTrait for #struct_name{
            fn into_map(&self)->BTreeMap<String,String> {
                let mut map = BTreeMap::new();
                #(#insert_tokes)*
                map
            }
        }
    };

    proc_macro::TokenStream::from(tokens)
}
```
5. 回到into_map_demo下
```shell
cargo run
m:{"active": "true", "id": "12", "name": "daheige"}
m:{"id": "123", "name": "xiaoming", "sex": "1"}
```
到这里你的派生宏实现struct to map的功能大功告成了！
