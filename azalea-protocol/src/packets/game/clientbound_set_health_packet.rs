use packet_macros::{GamePacket, McBuf};

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundSetHealthPacket {
    pub health: f32,
    #[var]
    pub food: u32,
    pub saturation: f32,
}