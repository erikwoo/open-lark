use std::env;

use dotenvy::dotenv;
use log::info;

use open_lark::{
    client::ws_client::LarkWsClient,
    event::dispatcher::EventDispatcherHandler,
    service::im::v1::{
        p2_im_message_read_v1::P2ImMessageReadV1, p2_im_message_receive_v1::P2ImMessageReceiveV1,
    },
};

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();

    let event_handler = EventDispatcherHandler::builder()
        .register_p2_im_message_receive_v1(handle_p2_im_message_receive_v1)
        .register_p2_im_message_read_v1(handle_p2_im_message_read_v1)
        .build();

    LarkWsClient::open(&app_id, &app_secret, event_handler)
        .await
        .unwrap();
}

fn handle_p2_im_message_receive_v1(data: P2ImMessageReceiveV1) {
    info!("p2.im.message.receive_v1: {:?}", data);
}

fn handle_p2_im_message_read_v1(data: P2ImMessageReadV1) {
    info!("p2.im.message.message_read_v1: {:?}", data);
}
