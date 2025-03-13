use bilibili_msg_history::models::message::ViewerKind;
use clap::{command, Parser};

#[derive(Parser)]
#[command(version = "0.0.1", author = "hjklchan", about, long_about = None)]
struct Args {
    #[arg(long, help = "对方的 uid")]
    talker_uid: u64,
    #[arg(long, short, value_parser = clap::value_parser!(u32).range(20..=200), help = "每次循环获取的消息数量")]
    size: Option<u32>,
    #[arg(long, short, help = "观看视角: [0] 代表自己的视角, [1] 代表对方视角")]
    viewer: Option<u8>,
    #[arg(long, short, help = "消息发送放的昵称")]
    talker_nickname: Option<String>,
    #[arg(long, help = "保存路径, 如 \"C:/Users/hjkl1/Desktop\"")]
    save_path: String,
    #[arg(long, short, help = "当前网页登录的 Cookie")]
    cookie: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let config = bilibili_msg_history::Config {
        cookie: args.cookie,
        talker_uid: args.talker_uid,
        size: args.size.unwrap_or(200),
        viewer_kind: match args.viewer.unwrap_or(0) {
            0 => ViewerKind::FirstPerson,
            1 => ViewerKind::ThirdPerson,
            _ => ViewerKind::FirstPerson,
        },
        save_path: args.save_path,
        talker_nickname: args.talker_nickname.unwrap_or("TA".to_string()),
    };

    bilibili_msg_history::run(Some(config))?;

    Ok(())
}
