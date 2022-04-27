use bytes::Bytes;

// Modules
mod game;
mod handshake;
mod login;
mod status;

// Structs

trait Packet {
    fn write(_: Bytes);

    fn serverbound() -> bool;

    fn clientbound() -> bool;
}

impl dyn Packet {
    fn serverbound() -> bool {
        !from_server()
    }
}