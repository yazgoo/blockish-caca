extern crate blockish;
extern crate crossterm;
extern crate redhook;
use blockish::ThreadedEngine;
use gag::Gag;
use std::env;
use std::slice;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct caca_display {
    _unused: [u8; 0],
}
#[doc = " \\e libcaca display context"]
pub type CacaDisplayT = caca_display;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct caca_canvas {
    _unused: [u8; 0],
}
#[doc = " \\e libcaca canvas"]
pub type CacaCanvasT = caca_canvas;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct caca_dither {
    _unused: [u8; 0],
}
#[doc = " dither structure"]
pub type CacaDitherT = caca_dither;

static mut WIDTH: i32 = 0;
static mut HEIGHT: i32 = 0;
static mut BYTES_PER_PIXEL: u32 = 0;
static mut TERMINAL_WIDTH: i32 = 0;
static mut TERMINAL_HEIGHT: i32 = 0;
static mut TERMINAL_SIZE_CHANGED: bool = false;

static mut ENGINE: Option<blockish::ThreadedEngine> = None;

redhook::hook! {
    unsafe fn caca_dither_bitmap(
        _arg1: *mut CacaCanvasT,
        _arg2: ::std::os::raw::c_int,
        _arg3: ::std::os::raw::c_int,
        _width: ::std::os::raw::c_int,
        _height: ::std::os::raw::c_int,
        _arg6: *const CacaDitherT,
        data: *const ::std::os::raw::c_uchar
    ) -> ::std::os::raw::c_int => my_caca_dither_bitmap {
        let mut bwidth = 80;
        let mut bheight = 20;
        if let Ok(columns_string) = env::var("COLUNMS") {
            if let Ok(columns) = columns_string.parse() {
                bwidth = columns;
                if TERMINAL_WIDTH != columns {
                    TERMINAL_WIDTH = columns;
                    TERMINAL_SIZE_CHANGED = true;
                }
            }
        }
        if let Ok(lines_string) = env::var("LINES") {
            if let Ok(lines) = lines_string.parse() {
                bheight = lines;
                if TERMINAL_HEIGHT != lines {
                    TERMINAL_HEIGHT = lines;
                    TERMINAL_SIZE_CHANGED = true;
                }
            }
        }
        match crossterm::terminal::size() {
            Ok(res) => {
                bwidth = res.0 as i32;
                bheight = res.1 as i32;
                if bwidth != TERMINAL_WIDTH || bheight != TERMINAL_HEIGHT {
                    TERMINAL_SIZE_CHANGED = true;
                }
                TERMINAL_WIDTH = bwidth;
                TERMINAL_HEIGHT = bheight;
            }
            Err(_) => {
            }
        }
        let width = bwidth as u32 * 8;
        let height = bheight as u32 * 16 - 1;
        if TERMINAL_SIZE_CHANGED {
            ENGINE = Some(ThreadedEngine::new(width as u32, height as u32, true));
        }
        let original_width : u32 = WIDTH as u32;
        let original_height : u32 = HEIGHT as u32;
        print!("\x1b[{};0f", 1);
        let raw_slice = slice::from_raw_parts(data, (original_width * original_height * BYTES_PER_PIXEL) as usize);
        match &mut ENGINE {
            Some(engine) => {
                engine.render(&|x, y| {
                    let start = (( ((y * original_height / height) * original_width as u32 + (x * original_width / width) ) * BYTES_PER_PIXEL)) as usize;
                    let b = (raw_slice[start]) as u8;
                    let g = (raw_slice[start + 1]) as u8;
                    let r = (raw_slice[start + 2]) as u8;
                    (r, g, b, 0)
                });
            }
            None => {}
        };
        0
    }
}

redhook::hook! {
unsafe fn caca_refresh_display(arg1: *mut CacaDisplayT) -> ::std::os::raw::c_int => my_caca_refresh_display {
    {
        let _print_gag = Gag::stdout().unwrap();
        redhook::real!(caca_refresh_display)(arg1)
    }
    }
}

redhook::hook! {
    unsafe fn caca_create_dither(
        _arg1: ::std::os::raw::c_int,
        _arg2: ::std::os::raw::c_int,
        _arg3: ::std::os::raw::c_int,
        _arg4: ::std::os::raw::c_int,
        _arg5: u32,
        _arg6: u32,
        _arg7: u32,
        _arg8: u32
    ) -> *mut CacaDitherT => my_caca_create_dither {
        let res = redhook::real!(caca_create_dither)(_arg1, _arg2, _arg3, _arg4, _arg5, _arg6, _arg7, _arg8);
        WIDTH=_arg2;
        HEIGHT=_arg3;
        BYTES_PER_PIXEL=_arg1 as u32 / 8;
        res
    }
}
