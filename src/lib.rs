#[macro_use]
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
        print!("\x1b[{};0f", 1);
        let width : u32 = env::var("BLOCKISH_WIDTH").unwrap().parse().unwrap();
        let height : u32 = env::var("BLOCKISH_HEIGHT").unwrap().parse().unwrap();
        let raw_slice = slice::from_raw_parts(data, (width * height * 3) as usize);
        render(width as u32, height as u32, &|x, y| {
            let start = (((y * width as u32 + x) * 3)) as usize;
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
