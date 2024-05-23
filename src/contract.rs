pub mod query {
    use crate::msg::ValueResp;

    pub fn value() -> ValueResp {
        ValueResp { value: 0 }
    }
    pub fn value_incremented(n: u64) -> ValueResp {
        ValueResp { value: n + 1 }
    }
}
