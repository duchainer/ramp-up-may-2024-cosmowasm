pub mod query {
    use crate::msg::ValueResp;

    pub fn value(n: u64) -> ValueResp {
        ValueResp { value: n + 1 }
    }
}
