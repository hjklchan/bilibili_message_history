pub mod collect;

use crate::models::{
    message::Message,
    response::{BilibiliResponse, ResponseData},
};
use reqwest::{blocking::Client, header::HeaderMap};

pub fn get_latest_msg_api(
    headers: HeaderMap,
    talker_uid: u64,
) -> Result<BilibiliResponse<ResponseData<Message>>, String> {
    Client::new()
        .get(collect::latest_message_api(talker_uid))
        .headers(headers.clone())
        .send()
        .map_err(|err| format!("请求时发生错误: {}", err))?
        .json::<BilibiliResponse<ResponseData<Message>>>()
        .map_err(|err| format!("反序列化时发生错误: {}", err))
}

pub fn get_message_collect_api(
    headers: HeaderMap,
    talker_uid: u64,
    size: u32,
    end_seqno: u64,
) -> Result<BilibiliResponse<ResponseData<Message>>, String> {
    Client::new()
        .get(collect::message_collect_api(talker_uid, size, end_seqno))
        .headers(headers.clone())
        .send()
        .map_err(|err| format!("请求时发生错误: {}", err))?
        .json::<BilibiliResponse<ResponseData<Message>>>()
        .map_err(|err| format!("反序列化时发生错误: {}", err))
}
