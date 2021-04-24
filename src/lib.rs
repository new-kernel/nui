#![no_std]

extern crate libn;
use libn::color::vga::Color;

extern "C" { pub fn draw(color: Color) -> Color; }
