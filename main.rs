extern crate sdl;

use sdl::event::{Event, Key};
use sdl::video::RGB;
use sdl::video::{SurfaceFlag, VideoFlag};

// detect which flag should be drawn next and then draws it
fn next_flag(selected_flag: &mut i32, screen: &sdl::video::Surface) {
    *selected_flag = *selected_flag + 1;
    if *selected_flag > 2 {
        *selected_flag = 0;
    };
    match *selected_flag {
        0 => draw_lgbtflag(screen),
        1 => draw_transflag(screen),
        2 => draw_nonbinaryflag(screen),
        _ => print!("how did this happen"),
    }
}

fn main() {
    let mut selected_flag = 0;
    let mut can_press = true;

    sdl::init(&[sdl::InitFlag::Video]);
    sdl::wm::set_caption("rust-sdl demo - pride", "rsdl-pride");

    // SDL initialization
    let screen = match sdl::video::set_video_mode(
        800,
        600,
        32,
        &[SurfaceFlag::HWSurface],
        &[VideoFlag::DoubleBuf],
    ) {
        Ok(screen) => screen,
        Err(err) => panic!("failed to set video mode: {}", err),
    };

    draw_lgbtflag(&screen); // draw starting flag

    'main: loop {
        'event: loop {
            screen.flip();
            match sdl::event::poll_event() {
                Event::Quit => break 'main,
                Event::None => break 'event,
                Event::Key(k, _, _, _) if k == Key::Escape => break 'main,
                Event::Key(k, _, _, _) if k == Key::Return => {
                    // bandaid fix to fix SDL recognizing letting go of a key as pressing
                    if can_press == true{
                        next_flag(&mut selected_flag, &screen);
                        can_press = false;
                    }
                    else {
                        can_press = true;
                    }
                }
                _ => {}
            }
        }
    }

    sdl::quit();
}

fn draw_lgbtflag(screen: &sdl::video::Surface) {
    screen.fill_rect(Some(sdl::Rect {x: 0, y: 0, w: 800, h: 600}), RGB(0xE4, 0x03, 0x03));
    screen.fill_rect(Some(sdl::Rect {x: 0, y: 100, w: 800, h: 600}), RGB(0xFF, 0x8C, 0x00));
    screen.fill_rect(Some(sdl::Rect {x: 0, y: 200, w: 800, h: 600}), RGB(0xFF, 0xED, 0x00));
    screen.fill_rect(Some(sdl::Rect {x: 0, y: 300, w: 800, h: 600}), RGB(0x00, 0x80, 0x26));
    screen.fill_rect(Some(sdl::Rect {x: 0, y: 400, w: 800, h: 600}), RGB(0x24, 0x40, 0x8E));
    screen.fill_rect(Some(sdl::Rect {x: 0, y: 500, w: 800, h: 600}), RGB(0x73, 0x29, 0x82));
}

fn draw_transflag(screen: &sdl::video::Surface) {
    screen.fill_rect(Some(sdl::Rect {x: 0, y: 0, w: 800, h: 600}), RGB(0x5B, 0xCE, 0xFA));
    screen.fill_rect(Some(sdl::Rect {x: 0, y: 120, w: 800, h: 600}), RGB(0xF5, 0xA9, 0xB8));
    screen.fill_rect(Some(sdl::Rect {x: 0, y: 240, w: 800, h: 600}), RGB(0xFF, 0xFF, 0xFF));
    screen.fill_rect(Some(sdl::Rect {x: 0, y: 360, w: 800, h: 600}), RGB(0xF5, 0xA9, 0xB8));
    screen.fill_rect(Some(sdl::Rect {x: 0, y: 480, w: 800, h: 600}), RGB(0x5B, 0xCE, 0xFA));
}

fn draw_nonbinaryflag(screen: &sdl::video::Surface) {
    screen.fill_rect(Some(sdl::Rect {x: 0, y: 0, w: 800, h: 600}), RGB(0xFC, 0xF4, 0x34));
    screen.fill_rect(Some(sdl::Rect {x: 0, y: 150, w: 800, h: 600}), RGB(0xFF, 0xFF, 0xFF));
    screen.fill_rect(Some(sdl::Rect {x: 0, y: 300, w: 800, h: 600}), RGB(0x9C, 0x59, 0xD1));
    screen.fill_rect(Some(sdl::Rect {x: 0, y: 450, w: 800, h: 600}), RGB(0x2C, 0x2C, 0x2C));

}
