//! The ``console`` module of ``ogc-rs``.
//!
//! This module implements a safe wrapper around the console functions.

use crate::mem_k0_to_k1;
use std::ptr;

struct Console(());

impl Console {
    /// Initializes the console subsystem with given parameters.
    pub fn init(xstart: i32, ystart: i32, xres: i32, yres: i32, stride: i32) -> Console {
        unsafe {
            let framebuffer = mem_k0_to_k1(ogc_sys::SYS_AllocateFramebuffer(
                ogc_sys::VIDEO_GetPreferredMode(ptr::null()),
            ));

            ogc_sys::CON_Init(framebuffer, xstart, ystart, xres, yres, stride);
        }

        Console(())
    }

    /// Initialize stdout console.
    pub fn init_stdout(xorigin: i32, yorigin: i32, width: i32, height: i32) -> Result<()> {
        unsafe {
            let init = ogc_sys::CON_InitEx(
                ogc_sys::VIDEO_GetPreferredMode(ptr::null()),
                xorigin,
                yorigin,
                width,
                height,
            );

            match init {
                -1 => Err(OgcError::Console("Failed to allocate memory for framebuffer!".into())),
                0 => Ok(()),
            }
        }
    }

    /// Enable or disable the USB gecko console.
    pub fn enable_gecko(channel: i32, safe: i32) {
        unsafe {
            ogc_sys::CON_EnableGecko(channel, safe);
        }
    }

    /// Retrieve the columns and rows of the current console
    pub fn get_metrics() -> (i32, i32) {
        let mut coords: (i32, i32) = (0, 0);

        unsafe {
            ogc_sys::CON_GetMetrics(coords.0, coords.1);
        }

        coords
    }

    /// Retrieve the current cursor position of the current console.
    pub fn get_position() -> (i32, i32) {
        let mut coords: (i32, i32) = (0, 0);

        unsafe {
            ogc_sys::CON_GetPosition(coords.0, coords.1);
        }

        coords
    }
}
