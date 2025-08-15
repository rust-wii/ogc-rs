#![no_std]
#![no_main]

extern crate alloc;

use ogc_rs::{mp3player::MP3Player, prelude::*};

#[no_mangle]
extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let mp3 = include_bytes!("../mp3.mp3");

    let video = Video::init();
    let mut asnd = Asnd::init();
    let mut player = MP3Player::new(asnd);

    Input::init(ControllerType::Gamecube);
    Input::init(ControllerType::Wii);

    let gcn_ctrl = Input::new(ControllerType::Gamecube, ControllerPort::One);
    let wii_ctrl = Input::new(ControllerType::Wii, ControllerPort::One);

    Console::init(&video);
    Video::configure(&video.render_config);
    unsafe {
        Video::set_next_framebuffer(video.framebuffer);
    }
    Video::set_black(false);
    Video::flush();
    Video::wait_vsync();

    println!("Hello World!");

    player.play_buffer(mp3);

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
