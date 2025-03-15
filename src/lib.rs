use chrono::Local;
use models::message::{ImageMessage, Message, ShareMessage, TextMessage, ViewerKind};
use reqwest::header::HeaderMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;

pub mod api;
pub mod models;

// format_message 格式化消息
//
// 消息格式化
// 并不处理消息的存储
pub fn format_message(message: &Message) -> Result<String, String> {
    match message.msg_type {
        // 文本消息
        1 => {
            let deserialized = serde_json::from_str::<TextMessage>(&message.content)
                .map_err(|err| format!("反序列化 [文本消息 (TextMessage)] 时发生错误: {}", err))?;

            Ok(deserialized.into())
        }
        // 图片消息
        2 => {
            let deserialized = serde_json::from_str::<ImageMessage>(&message.content)
                .map_err(|err| format!("反序列化 [文本消息 (TextMessage)] 时发生错误: {}", err))?;

            Ok(deserialized.into())
        }
        // 分享消息
        7 => {
            let deserialized = serde_json::from_str::<ShareMessage>(&message.content)
                .map_err(|err| format!("反序列化 [分享消息 (ShareMessage)] 时发生错误: {}", err))?;

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
    let bilibili_response = api::get_latest_msg_api(headers.clone(), talker_uid)
        .map_err(|err| format!("反序列化时发生错误: {}", err))?;
    // 获取最新的 end_seqno
    let mut mutable_end_seqno = bilibili_response.data.max_seqno;
    // 获取最新一条消息的 timestamp
    #[allow(unused)]
    let timestamp = bilibili_response
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

    if fs::exists(&path_buf).map_err(|err| format!("检查文件是否存在时发生错误: {}", err))?
    {
        fs::remove_file(&path_buf).map_err(|err| format!("删除文件时发生错误: {}", err))?;
    }

    let mut file =
        fs::File::create(&path_buf).map_err(|err| format!("创建文件时发生错误: {}", err))?;

    println!("Start getting the message data");

    loop {
        let bilibili_response =
            api::get_message_collect_api(headers.clone(), talker_uid, size, mutable_end_seqno)
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
                    let nickname = person_nickname(viewer, message, &talker_nickname, talker_uid);
                    let formatted_message = format_message(&message).unwrap();
                    let message_text = format!(
                        "[{}]{}: {}\n",
                        message.datetime(),
                        nickname,
                        formatted_message,
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
                let nickname = person_nickname(viewer, message, &talker_nickname, talker_uid);
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
