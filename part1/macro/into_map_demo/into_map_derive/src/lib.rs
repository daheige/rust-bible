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
        // 下面的行建议放到调用的位置
        // use into_map::IntoMapTrait;
        // 为结构体实现IntoMap trait
        impl IntoMapTrait for #struct_name{
            fn into_map(&self)->std::collections::BTreeMap<String,String> {
                let mut map = std::collections::BTreeMap::new();
                #(#insert_tokes)*
                map
            }
        }
    };

    proc_macro::TokenStream::from(tokens)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
