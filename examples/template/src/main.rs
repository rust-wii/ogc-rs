#![no_std]
#![feature(start)]

extern crate alloc;
use ogc_rs::{input::controller::{Button, ControllerType}, prelude::*};

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let video = Video::init();
    Input::init(ControllerType::Gamecube);
    Input::init(ControllerType::Wii);

    let gcn_ctrl = Input::new(ControllerPort::One, ControllerType::Gamecube);
    let wii_ctrl = Input::new(ControllerPort::One, ControllerType::Gamecube);

    Console::init(&video);
    Video::configure(video.render_config.into());
    Video::set_next_framebuffer(video.framebuffer);
    Video::set_black(false);
    Video::flush();
    Video::wait_vsync();

    println!("Hello World!");

    loop {
        Input::update(ControllerType::Gamecube);
        Input::update(ControllerType::Wii); 

        if gcn_ctrl.is_button_down(Button::Start) {
            break 0;
        }

        if wii_ctrl.is_button_down(Button::Home) {
            break 0;
        }

        Video::wait_vsync();
    }
}
