pub fn latest_message_api(talker_uid: u64) -> String {
    format!("https://api.vc.bilibili.com/svr_sync/v1/svr_sync/fetch_session_msgs?sender_device_id=1&talker_id={}&session_type=1&size=1&build=0&mobi_app=web", talker_uid)
}

pub fn message_collect_api(talker_uid: u64, size: u32, end_seqno: u64) -> String {
    format!("https://api.vc.bilibili.com/svr_sync/v1/svr_sync/fetch_session_msgs?sender_device_id=1&talker_id={}&session_type=1&size={}&end_seqno={}&build=0&mobi_app=web", talker_uid, size, end_seqno+1)
}