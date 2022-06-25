use azalea_buf::McBuf;
use azalea_chat::component::Component;
use packet_macros::GamePacket;

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundChatPreviewPacket {
    pub query_id: i32,
    pub preview: Option<Component>,
}