// 定义IntoMap trait
pub trait IntoMapTrait {
    fn into_map(&self) -> std::collections::BTreeMap<String, String>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
