use reqwest::{blocking::Client, header::HeaderMap};

use crate::models::{message::Message, response::{BilibiliResponse, ResponseData}};

pub fn get_latest_msg_api(
    headers: HeaderMap,
    talker_uid: u64,
) -> Result<BilibiliResponse<ResponseData<Message>>, String> {
    let api: String = format!("https://api.vc.bilibili.com/svr_sync/v1/svr_sync/fetch_session_msgs?sender_device_id=1&talker_id={}&session_type=1&size=1&build=0&mobi_app=web", talker_uid);
    let http_response = Client::new()
        .get(api)
        .headers(headers.clone())
        .send()
        .map_err(|err| format!("请求时发生错误: {}", err))?;
    http_response
        .json::<BilibiliResponse<ResponseData<Message>>>()
        .map_err(|err| format!("反序列化时发生错误: {}", err))
}

pub fn get_message_collect_api(
    headers: HeaderMap,
    talker_uid: u64,
    size: u32,
    end_seqno: u64,
) -> Result<BilibiliResponse<ResponseData<Message>>, String> {
    let api = format!("https://api.vc.bilibili.com/svr_sync/v1/svr_sync/fetch_session_msgs?sender_device_id=1&talker_id={}&session_type=1&size={}&end_seqno={}&build=0&mobi_app=web", talker_uid, size, end_seqno+1);
    Client::new()
        .get(api)
        .headers(headers.clone())
        .send()
        .map_err(|err| format!("请求时发生错误: {}", err))?
        .json::<BilibiliResponse<ResponseData<Message>>>()
        .map_err(|err| format!("反序列化时发生错误: {}", err))
}