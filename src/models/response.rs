use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BilibiliResponse<T> {
    pub code: i32,
    pub msg: String,
    pub message: String,
    pub ttl: i32,
    pub data: T,
}

#[derive(Debug, Deserialize)]
pub struct ResponseData<T> {
    pub messages: Option<Vec<T>>,
    pub min_seqno: u64,
    pub max_seqno: u64,
    pub has_more: i32,
}
