struct Pair<K, V> {
    key: K,
    value: V,
}

impl<K: ToJSON, V: ToJSON> ToJSON for Pair<K, V> {
    fn to_json(&self) -> String {
        format!(
r#"{{
    key: {},
    value: {}
}}"#,
            self.key.to_json(),
            self.value.to_json()
        )
    }
}

impl ToJSON for String {
    fn to_json(&self) -> String {
        self.clone()
    }
}

trait ToJSON {
    fn to_json(&self) -> String;
}

fn main() {
    let pair = Pair {
        key: "test".to_string(),
        value: "asd".to_string(),
    };
    println!("{}", pair.to_json());
}
