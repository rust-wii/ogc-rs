use crate::ffi;

/// Error code returned when the given size for the queue is too large.
pub const ERROR_TOOMANY: i32 = ffi::MQ_ERROR_TOOMANY;

/// Flag indicating that an operation should block when run.
pub const MSG_BLOCK: u32 = ffi::MQ_MSG_BLOCK;
/// Flag indicating that an operation should **not** block when run.
pub const MSG_NOBLOCK: u32 = ffi::MQ_MSG_NOBLOCK;

/// Alias for message pointer.
pub type MQMsg = ffi::mqmsg_t;

/// A message queue handle
#[derive(Debug)]
#[repr(transparent)]
pub struct MQBox {
	handle: ffi::mqbox_t,
}

impl MQBox {
	/// Initializes a message queue that can store the given number of messages.
	pub fn new(count: u32) -> Result<Self, i32> {
        let mut mqbox = core::mem::MaybeUninit::uninit();
        unsafe {
            let res = ffi::MQ_Init(mqbox.as_mut_ptr(), count);

            if res < 0 {
                Err(res)
            } else {
                Ok(MQBox {
                	handle: mqbox.assume_init()
                })
            }
        }
	}
	
	/// Sends a message to the given message queue. Returns `true` on success.
	pub fn send(&mut self, msg: MQMsg, flags: u32) -> bool {
		unsafe {
			ffi::MQ_Send(self.handle, msg, flags) != 0
		}
	}
	
	/// Receives a message from the given message queue. Returns `true` on
	/// success.
	pub fn receive(&mut self, msg: &mut MQMsg, flags: u32) -> bool {
		unsafe {
			ffi::MQ_Receive(self.handle, msg, flags) != 0
		}
	}
	
	/// Jams a message in front of the given message queue. Returns `true` on
	/// success.
	pub fn jam(&mut self, msg: MQMsg, flags: u32) -> bool {
		unsafe {
			ffi::MQ_Jam(self.handle, msg, flags) != 0
		}
	}
}

impl Drop for MQBox {
	fn drop(&mut self) {
		unsafe {
			ffi::MQ_Close(self.handle);
		}
	}
}
