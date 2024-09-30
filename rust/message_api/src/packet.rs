//! This module contains the definition of the `MessagePacket` struct.

use allocator_api2::alloc::Allocator;
use crate::header::MessageHeader;

/// A packet containing a message and its header.
pub struct MessagePacket {
    pub(crate) data: *mut MessageHeader,
    pub(crate) len: usize,
}

impl MessagePacket {
    /// Creates a new message packet.
    pub unsafe fn new(data: *mut MessageHeader, len: usize) -> Self {
        Self { data, len }
    }

    /// Returns the message header.
    pub fn header(&self) -> &MessageHeader {
        unsafe { &*(self.data as *const MessageHeader) }
    }

    /// Returns a mutable reference to the message header.
    pub fn header_mut(&self) -> &mut MessageHeader {
        unsafe { &mut *self.data }
    }
}
