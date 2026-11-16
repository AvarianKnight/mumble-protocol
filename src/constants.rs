//! Constants used throughought the projects

/// Maximum size of an encrypted Mumble packet.
pub const MAX_UDP_PACKET_SIZE: usize = 1024;

/// The maximum size a TCP packet can be on the send and receive path.
pub const MAX_TCP_PACKET_SIZE: usize = 8192;

/// Size in bytes of the AES key used in `CryptState`.
pub const KEY_SIZE: usize = 16;

/// Size in bytes of blocks for the AES primitive.
pub const BLOCK_SIZE: usize = std::mem::size_of::<u128>();
