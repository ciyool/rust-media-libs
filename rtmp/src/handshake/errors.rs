use std::io;

/// Enumeration that represents the various errors that can occur during the handshaking process
#[derive(Debug, Fail)]
pub enum HandshakeError {
    /// The RTMP specification requires the first byte in the handshake process to start with a
    /// 3, so this error is encountered if any other value is in the first byte.
    #[fail(display = "First byte of the handshake did not start with a 3")]
    BadVersionId,

    /// The RTMP specification requires the 2nd set of 4 bytes to all be zeroes, so this error
    /// is encountered if any of those values are not zeros.
    #[fail(display = "Packet 1's 2nd time field was expected to be empty, but wasn't")]
    NonZeroedTimeInPacket1,

    /// This is encountered when the peer did not send the same timestamp in packet #2 that we
    /// sent them in our packet #1.
    #[fail(display = "Peer did not send the correct time back")]
    IncorrectPeerTime,

    /// This is encountered when the peer did not send back the same random data in their packet
    /// #2 that we sent them in our packet #1.
    #[fail(display = "Peer did not send the correct random data back")]
    IncorrectRandomData,

    /// This is encountered if we try to keep progressing on a handshake handler that has already
    /// completed a successful handshake.
    #[fail(display = "Attempted to continue handshake process after completing handshake")]
    HandshakeAlreadyCompleted,

    /// This occurs when an IO error is encountered while reading the input.
    #[fail(display = "_0")]
    Io(#[cause] io::Error)
}

impl From<io::Error> for HandshakeError {
    fn from(error: io::Error) -> Self {
        HandshakeError::Io(error)
    }
}