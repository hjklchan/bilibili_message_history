use models::message::{ImageMessage, Message, TextMessage, ViewerKind};
use chrono::Local;
use models::response::{BilibiliResponse, ResponseData};
use reqwest::blocking::Client;
use reqwest::header::HeaderMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;

pub mod models;

pub mod api_collect {
    // use super::*;
}

// format_message
//
// 消息格式化
// 并不处理消息的存储
pub fn format_message(message: &Message) -> Result<String, String> {
    match message.msg_type {
        // 文本消息
        1 => {
            let deserialized = serde_json::from_str::<TextMessage>(&message.content)
                .map_err(|err| format!("反序列化 [文本消息 (TextMessage)] 时错先错误: {}", err))?;

            Ok(deserialized.into())
        }
        // 图片消息
        2 => {
            let deserialized = serde_json::from_str::<ImageMessage>(&message.content)
                .map_err(|err| format!("反序列化 [文本消息 (TextMessage)] 时错先错误: {}", err))?;

            // let formatted_content = format!(
            //     "[图片][{}原图] {}",
            //     {
            //         if deserialized.original == 1 {
            //             ""
            //         } else {
            //             "非"
            //         }
            //     },
            //     deserialized.url
            // );
            Ok(deserialized.into())
        }
        // 其他消息
        _ => Ok("[其他消息]".to_owned()),
    }
}

pub fn person_nickname<'a>(
    viewer: ViewerKind,
    message: &'a Message,
    nickname: &'a str,
    talker_uid: u64,
) -> &'a str {
    match viewer {
        ViewerKind::FirstPerson => {
            if message.sender_uid == talker_uid {
                return nickname;
            }

            return "我";
        }
        ViewerKind::ThirdPerson => {
            if message.receiver_id == talker_uid {
                return nickname;
            }

            return "我";
        }
    }
}

#[deprecated(since = "0.0.2", note = "下个版本不在使用，将会被 Args 结构体代替")]
#[derive(Debug, Default)]
pub struct Config {
    pub cookie: String,
    pub talker_uid: u64,
    pub size: u32,
    pub viewer_kind: ViewerKind,
    pub talker_nickname: String,
    pub save_path: String,
}

pub fn run(config: Option<Config>) -> Result<(), String> {
    let config = config.ok_or("缺省配置")?;

    let size = config.size;
    let talker_uid = config.talker_uid;
    // let talker_uid: u64 = 319521269;
    let cookie = config.cookie;
    let viewer = config.viewer_kind;
    let talker_nickname = config.talker_nickname;

    let mut headers = HeaderMap::new();
    headers.insert(
        "Cookie",
        cookie
            .parse()
            .map_err(|err| format!("解析 Cookie 时发生错误: {}", err))?,
    );

    // 获取最新的一条消息的元数据
    let api: String = format!("https://api.vc.bilibili.com/svr_sync/v1/svr_sync/fetch_session_msgs?sender_device_id=1&talker_id={}&session_type=1&size=1&build=0&mobi_app=web", talker_uid);
    let http_response = Client::new()
        .get(api)
        .headers(headers.clone())
        .send()
        .map_err(|err| format!("请求时发生错误: {}", err))?;
    let bilibili_response = http_response
        .json::<BilibiliResponse<ResponseData<Message>>>()
        .map_err(|err| format!("反序列化时发生错误: {}", err))?;
    // 获取最新的 end_seqno
    let mut mutable_end_seqno = bilibili_response.data.max_seqno;
    // 获取最新一条消息的 timestamp
    let _timestamp = bilibili_response
        .data
        .messages
        .map(|messages| {
            messages
                .iter()
                .next()
                .map_or(0, |message| message.timestamp)
        })
        .unwrap_or_default();

    // 创建目录和文件

    let datetime = Local::now().format("%Y-%m-%d").to_string();
    let filepath = format!("{}/{}.txt", config.save_path, datetime);
    let path_buf = PathBuf::from(filepath);

    if fs::exists(&path_buf).unwrap() {
        fs::remove_file(&path_buf).unwrap();
    }

    let mut file = fs::File::create(&path_buf).unwrap();

    println!("Start getting the message data");

    loop {
        let api = format!("https://api.vc.bilibili.com/svr_sync/v1/svr_sync/fetch_session_msgs?sender_device_id=1&talker_id={}&session_type=1&size={}&end_seqno={}&build=0&mobi_app=web", talker_uid, size, mutable_end_seqno+1);
        let bilibili_response = Client::new()
            .get(api)
            .headers(headers.clone())
            .send()
            .map_err(|err| format!("请求时发生错误: {}", err))?
            .json::<BilibiliResponse<ResponseData<Message>>>()
            .map_err(|err| format!("反序列化时发生错误: {}", err))?;

        if bilibili_response.code != 0 {
            break;
        }

        // 如果 has_more 为 0 说明聊天记录已经到底了
        // 但是当前数据的 messages 仍可能会有聊天数据
        if bilibili_response.data.has_more == 0 {
            if let Some(messages) = bilibili_response.data.messages {
                messages.iter().for_each(|message| {
                    // 其他逻辑
                    let nickname = person_nickname(
                        viewer,
                        message,
                        &talker_nickname,
                        talker_uid,
                    );
                    let message_text = format!(
                        "[{}]{}: {}\n",
                        message.datetime(),
                        nickname,
                        format_message(&message).unwrap()
                    );

                    if let Err(err) = file.write(message_text.as_bytes()) {
                        eprintln!("消息写入时发生错误: {}", err);
                    }
                });
            }

            break;
        }

        if let Some(messages) = bilibili_response.data.messages {
            messages.iter().for_each(|message| {
                // 其他逻辑
                let nickname = person_nickname(
                    viewer,
                    message,
                    &talker_nickname,
                    talker_uid,
                );
                let message_text = format!(
                    "[{}]{}: {}\n",
                    message.datetime(),
                    nickname,
                    format_message(&message).unwrap()
                );

                if let Err(err) = file.write(message_text.as_bytes()) {
                    eprintln!("消息写入时发生错误: {}", err);
                }
            });
        }

        mutable_end_seqno = bilibili_response.data.min_seqno - 1;

        std::thread::sleep(Duration::from_millis(500));
    }

    println!("Done.");

    Ok(())
}