use std::fmt::Display;

use chrono::{DateTime, Local};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Message {
    pub sender_uid: u64,
    pub receiver_id: u64,
    pub receiver_type: u8,
    pub msg_type: u8,
    pub msg_seqno: u64,
    pub content: String,
    pub timestamp: u64,
}

impl Message {
    pub fn datetime(&self) -> String {
        let dt: DateTime<Local> = DateTime::from_timestamp(self.timestamp as i64, 0)
            .unwrap()
            .into();
        dt.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}

#[derive(Deserialize)]
pub struct ImageMessage {
    // 是否为原图
    pub original: u8,
    // 图片地址
    pub url: String,
    #[serde(rename = "imageType")]
    pub image_type: String,
    // pub size: f32,
    pub height: u32,
    pub width: u32,
}

impl Display for ImageMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted_content = format!(
            "[图片][{}原图] {}",
            {
                if self.original == 1 {
                    ""
                } else {
                    "非"
                }
            },
            self.url
        );
        write!(f, "{}", formatted_content)
    }
}

impl From<ImageMessage> for String {
    fn from(value: ImageMessage) -> Self {
        value.to_string()
    }
}

#[derive(Deserialize)]
pub struct TextMessage {
    pub content: String,
}

impl From<TextMessage> for String {
    fn from(value: TextMessage) -> Self {
        value.to_string()
    }
}

impl Display for TextMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum ViewerKind {
    #[default]
    // 第一人称 (我看)
    FirstPerson,
    // 第三人称 (TA 看)
    ThirdPerson,
}
