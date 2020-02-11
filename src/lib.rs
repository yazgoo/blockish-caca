#[macro_use]
extern crate crossterm;
extern crate redhook;
extern crate blockish;
use blockish::render;
use std::slice;

use std::io;
use std::io::Write;
use std::io::stdout;
use gag::Gag;
use std::env;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct caca_display {
    _unused: [u8; 0],
}
#[doc = " \\e libcaca display context"]
pub type caca_display_t = caca_display;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct caca_canvas {
    _unused: [u8; 0],
}
#[doc = " \\e libcaca canvas"]
pub type caca_canvas_t = caca_canvas;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct caca_dither {
    _unused: [u8; 0],
}
#[doc = " dither structure"]
pub type caca_dither_t = caca_dither;

static mut WIDTH: i32 = 0;
static mut HEIGHT: i32 = 0;

redhook::hook! {
    unsafe fn caca_dither_bitmap(
        _arg1: *mut caca_canvas_t,
        _arg2: ::std::os::raw::c_int,
        _arg3: ::std::os::raw::c_int,
        _width: ::std::os::raw::c_int,
        _height: ::std::os::raw::c_int,
        _arg6: *const caca_dither_t,
        data: *const ::std::os::raw::c_uchar
    ) -> ::std::os::raw::c_int => my_caca_dither_bitmap {
        let (bwidth, bheight) = crossterm::terminal::size().unwrap();
        let width = bwidth as u32 * 8;
        let height = bheight as u32 * 16 - 1;
        let original_width : u32 = WIDTH as u32;
        let original_height : u32 = HEIGHT as u32;
        if (width > original_width) {
            let width = original_width;
        }
        if (height > original_height) {
            let height = original_height;
        }
        print!("\x1b[{};0f", 1);
        let raw_slice = slice::from_raw_parts(data, (original_width * original_height * 3) as usize);
        render(width as u32, height as u32, &|x, y| {
            let start = (( ((y * original_height / height) * original_width as u32 + (x * original_width / width) ) * 3)) as usize;
            let b = (raw_slice[start]) as u8;
            let g = (raw_slice[start + 1]) as u8;
            let r = (raw_slice[start + 2]) as u8;
            (r, g, b)
        });
        0
    }
}

redhook::hook! {
unsafe fn caca_refresh_display(arg1: *mut caca_display_t) -> ::std::os::raw::c_int => my_caca_refresh_display {
    {
        let print_gag = Gag::stdout().unwrap();
        redhook::real!(caca_refresh_display)(arg1)
    }
    }
}

redhook::hook! {
    unsafe fn caca_create_dither(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
        arg5: u32,
        arg6: u32,
        arg7: u32,
        arg8: u32
    ) -> *mut caca_dither_t => my_caca_create_dither {
        let res = redhook::real!(caca_create_dither)(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8);
        WIDTH=arg2;
        HEIGHT=arg3;
        res
    }
}
