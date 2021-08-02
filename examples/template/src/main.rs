#![no_std]
#![feature(start)]

extern crate alloc;
use ogc_rs::prelude::*;

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let video = Video::init();
    Input::init(ControllerType::Gamecube);
    Input::init(ControllerType::Wii);

    let mut gcn_ctrl = Input::new(ControllerType::Gamecube, ControllerPort::One);
    let mut wii_ctrl = Input::new(ControllerType::Wii, ControllerPort::One);
    
    Console::init(&video);
    Video::configure(video.render_config.into());
    Video::set_next_framebuffer(video.framebuffer);
    Video::set_black(false);
    Video::flush();
    Video::wait_vsync();

    println!("Hello World!");

    loop {
        gcn_ctrl.update();
        wii_ctrl.update();

        if gcn_ctrl.is_button_in_state(Button::Start, ButtonState::ButtonDown) {
            break 0;
                     
        }

        if wii_ctrl.is_button_in_state(Button::Home, ButtonState::ButtonDown) {
            break 0;    
        }
    
        Video::wait_vsync();
    }
}

