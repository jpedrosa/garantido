// lib.rs
// Copyright (c) 2019 Joao Pedrosa

// #[cfg(windows)] extern crate winapi;
// use std::io::Error;


use winapi::um::winuser::{MOUSEINPUT, MOUSEEVENTF_MOVE, MOUSEEVENTF_ABSOLUTE,
    INPUT_u, INPUT, INPUT_MOUSE, SendInput, MOUSEEVENTF_LEFTDOWN,
    MOUSEEVENTF_LEFTUP, GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN,
    SM_CXVIRTUALSCREEN, SM_CYVIRTUALSCREEN, GetCursorPos};
use winapi::shared::windef::{POINT};


pub fn mouse_input(dx: i32, dy: i32, flags: u32) -> INPUT {
    let mi = MOUSEINPUT{dx: dx, dy: dy, mouseData: 0, dwFlags: flags, time: 0,
        dwExtraInfo: 0};
    let mut u: INPUT_u = unsafe { std::mem::zeroed() }; 
    unsafe {
        *u.mi_mut() = mi;
    }
    INPUT{ type_: INPUT_MOUSE, u: u}
}

pub fn send_input(input: &mut INPUT) {
    unsafe {
        SendInput(1, input, std::mem::size_of::<INPUT>() as i32 );
    }
}

pub fn do_move_cursor(x: i32, y: i32) {
    let mut input = mouse_input(x, y,
        MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE);
    send_input(&mut input);
}

#[derive(Debug)]
pub struct ScreenSize {
    pub w: i32,
    pub h: i32
}

pub fn screen_size() -> ScreenSize {
    let w = unsafe { GetSystemMetrics(SM_CXSCREEN) };
    let h = unsafe { GetSystemMetrics(SM_CYSCREEN) };
    ScreenSize {w: w, h: h}
}

pub fn virtual_screen_size() -> ScreenSize {
    let w = unsafe { GetSystemMetrics(SM_CXVIRTUALSCREEN) };
    let h = unsafe { GetSystemMetrics(SM_CYVIRTUALSCREEN) };
    ScreenSize {w: w, h: h}
}

pub fn move_cursor(x: i32, y: i32) {
    let sz = screen_size();
    let x = (x * 65536) / (sz.w - 1);
    let y = (y * 65536) / (sz.h - 1);
    do_move_cursor(x, y);
}

pub fn left_click() {
    let mut inputdown = mouse_input(0, 0, MOUSEEVENTF_LEFTDOWN);
    let mut inputup = mouse_input(0, 0, MOUSEEVENTF_LEFTUP);
    send_input(&mut inputdown);
    send_input(&mut inputup);
}

#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

pub fn cursor_position() -> Point {
    let mut point = POINT{x: 0, y: 0};
    unsafe {
        GetCursorPos(&mut point);
    }
    Point { x: point.x, y: point.y }
}

