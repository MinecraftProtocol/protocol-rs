use bytes::Bytes;

// Modules
mod game;
mod handshake;
mod login;
mod status;

// Structs

trait Packet {
    fn write(&self, _: Bytes);

    fn serverbound(&self) -> bool;

    fn clientbound(&self) -> bool;
}

impl dyn Packet {
    fn serverbound(&self) -> bool {
        !self.clientbound()
    }
}