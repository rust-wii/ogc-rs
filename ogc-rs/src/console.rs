//! The ``console`` module of ``ogc-rs``.
//!
//! This module implements a safe wrapper around the console functions.

use crate::{mem_cached_to_uncached, OgcError, Result};
use std::ptr;

pub struct Console(());

impl Console {
    /// Initializes the console subsystem with given parameters.
    pub fn init(xstart: i32, ystart: i32, xres: i32, yres: i32, stride: i32) -> Console {
        unsafe {
            let framebuffer = mem_cached_to_uncached!(ogc_sys::SYS_AllocateFramebuffer(
                ogc_sys::VIDEO_GetPreferredMode(ptr::null_mut())
            ));

            ogc_sys::CON_Init(framebuffer, xstart, ystart, xres, yres, stride);
        }

        Console(())
    }

    /// Initialize stdout console.
    pub fn init_stdout(xorigin: i32, yorigin: i32, width: i32, height: i32) -> Result<()> {
        unsafe {
            let init = ogc_sys::CON_InitEx(
                ogc_sys::VIDEO_GetPreferredMode(ptr::null_mut()),
                xorigin,
                yorigin,
                width,
                height,
            );

            if init < 0 {
                Err(OgcError::Console(
                    "Failed to allocate memory for framebuffer!".into(),
                ))
            } else {
                Ok(())
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
        let coords: (i32, i32) = (0, 0);

        unsafe {
            ogc_sys::CON_GetMetrics(coords.0 as *mut i32, coords.1 as *mut i32);
        }

        coords
    }

    /// Retrieve the current cursor position of the current console.
    pub fn get_position() -> (i32, i32) {
        let coords: (i32, i32) = (0, 0);

        unsafe {
            ogc_sys::CON_GetPosition(coords.0 as *mut i32, coords.1 as *mut i32);
        }

        coords
    }
}
