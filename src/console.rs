//! The ``console`` module of ``ogc-rs``.
//!
//! This module implements a safe wrapper around the console functions.

use crate::{video::Video, OgcError, Result};
use alloc::string::String;
use core::ptr;

/// Represents the console service.
/// No console control can be done until an instance of this struct is created.
/// This service can only be created once!
pub struct Console;

/// Implementation of the console service.
impl Console {
    /// Initializes the console subsystem with video.
    pub fn init(video: &Video) -> Self {
        unsafe {
            ogc_sys::CON_Init(
                video.framebuffer,
                20,
                20,
                video.render_config.framebuffer_width as i32,
                video.render_config.extern_framebuffer_height as i32,
                (video.render_config.framebuffer_width * 2) as i32,
            );
        }

        Self
    }

    /// Initialize stdout console.
    pub fn init_stdout(xorigin: i32, yorigin: i32, width: i32, height: i32) -> Result<()> {
        let init = unsafe {
            ogc_sys::CON_InitEx(
                ogc_sys::VIDEO_GetPreferredMode(ptr::null_mut()),
                xorigin,
                yorigin,
                width,
                height,
            )
        };

        if init < 0 {
            Err(OgcError::Console(
                "Failed to allocate memory for framebuffer!".into(),
            ))
        } else {
            Ok(())
        }
    }

    /// Enable or disable the USB gecko console.
    pub fn enable_gecko(channel: i32, safe: i32) {
        unsafe {
            ogc_sys::CON_EnableGecko(channel, safe);
        }
    }

    /// Retrieve the columns and rows of the current console.
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

    /// Print a formatted string to the console screen through ``printf``.
    pub fn print(formatted_string: &str) {
        // Create a buffer.
        let mut buffer = String::new();

        // Credit to ``lemarcuspoilus`` on github for this method.
        let offset_to_contents = {
            let mut it = formatted_string.char_indices();
            loop {
                let (i, ch) = match it.next() {
                    Some(pair) => pair,
                    None => return,
                };
                match ch {
                    '\n' | '\r' => buffer.push(ch),
                    _ => break i,
                }
            }
        };

        buffer.push_str(&formatted_string[offset_to_contents..]);
        buffer.push('\0');

        // Print the buffer.
        unsafe {
            ogc_sys::printf(buffer.as_ptr());
        }
    }
}
