// SPDX-License-Identifier: GPL-2.0
/*
 * UHID Example
 *
 * Copyright (c) 2012-2013 David Herrmann <dh.herrmann@gmail.com>
 *
 * The code may be used by anyone for any purpose,
 * and can serve as a starting point for developing
 * applications using uhid.
 */

/*
 * UHID Example
 * This example emulates a basic 3 buttons mouse with wheel over UHID. Run this
 * program as root and then use the following keys to control the mouse:
 *   q: Quit the application
 *   1: Toggle left button (down, up, ...)
 *   2: Toggle right button
 *   3: Toggle middle button
 *   a: Move mouse left
 *   d: Move mouse right
 *   w: Move mouse up
 *   s: Move mouse down
 *   r: Move wheel up
 *   f: Move wheel down
 *
 * Additionally to 3 button mouse, 3 keyboard LEDs are also supported (LED_NUML,
 * LED_CAPSL and LED_SCROLLL). The device doesn't generate any related keyboard
 * events, though. You need to manually write the EV_LED/LED_XY/1 activation
 * input event to the evdev device to see it being sent to this device.
 *
 * If uhid is not available as /dev/uhid, then you can pass a different path as
 * first argument.
 * If <linux/uhid.h> is not installed in /usr, then compile this with:
 *   gcc -o ./uhid_test -Wall -I./include ./samples/uhid/uhid-example.c
 * And ignore the warning about kernel headers. However, it is recommended to
 * use the installed uhid.h if available.
 */

// External libc and Linux UHID declarations are supplied by the surrounding build.

/* HID Report Descriptor (the kernel parses this as a 3-button mouse with wheel and LEDs). */
static mut rdesc: &[u8] = &[
    0x05, 0x01, 0x09, 0x02, 0xa1, 0x01, 0x09, 0x01, 0xa1, 0x00,
    0x85, 0x01, 0x05, 0x09, 0x19, 0x01, 0x29, 0x03, 0x15, 0x00,
    0x25, 0x01, 0x95, 0x03, 0x75, 0x01, 0x81, 0x02, 0x95, 0x01,
    0x75, 0x05, 0x81, 0x01, 0x05, 0x01, 0x09, 0x30, 0x09, 0x31,
    0x09, 0x38, 0x15, 0x81, 0x25, 0x7f, 0x75, 0x08, 0x95, 0x03,
    0x81, 0x06, 0xc0, 0xc0, 0x05, 0x01, 0x09, 0x06, 0xa1, 0x01,
    0x85, 0x02, 0x05, 0x08, 0x19, 0x01, 0x29, 0x03, 0x15, 0x00,
    0x25, 0x01, 0x95, 0x03, 0x75, 0x01, 0x91, 0x02, 0x95, 0x01,
    0x75, 0x05, 0x91, 0x01, 0xc0,
];

// The following declarations intentionally depend on the corresponding Linux/libc definitions.
extern "C" {
    fn write(fd: i32, buf: *const core::ffi::c_void, count: usize) -> isize;
    fn read(fd: i32, buf: *mut core::ffi::c_void, count: usize) -> isize;
    fn close(fd: i32) -> i32;
    fn open(path: *const i8, flags: i32, ...) -> i32;
    fn poll(fds: *mut PollFd, nfds: usize, timeout: i32) -> i32;
    fn tcgetattr(fd: i32, termios: *mut Termios) -> i32;
    fn tcsetattr(fd: i32, action: i32, termios: *const Termios) -> i32;
    fn fprintf(stream: *mut core::ffi::c_void, format: *const i8, ...) -> i32;
    fn strcmp(a: *const i8, b: *const i8) -> i32;
}

#[repr(C)]
struct PollFd { fd: i32, events: i16, revents: i16 }
#[repr(C)] struct Termios { _opaque: [u8; 256] }

const STDIN_FILENO: i32 = 0;
const O_RDWR: i32 = 0x2;
const O_CLOEXEC: i32 = 0x80000;
const POLLIN: i16 = 0x001;
const POLLHUP: i16 = 0x010;
const TCSANOW: i32 = 0;
const ICANON: u32 = 0x0002;
const VMIN: usize = 6;
const UHID_START: u32 = 1;
const UHID_STOP: u32 = 2;
const UHID_OPEN: u32 = 3;
const UHID_CLOSE: u32 = 4;
const UHID_OUTPUT: u32 = 6;
const UHID_OUTPUT_EV: u32 = 7;
const UHID_INPUT: u32 = 10;
const UHID_OUTPUT_REPORT: u8 = 0;
const BUS_USB: u16 = 0x03;
const EFAULT: i32 = 14;
const ECANCELED: i32 = 125;

#[repr(C)]
struct UhidEvent { data: [u8; 438] }

static mut btn1_down: bool = false;
static mut btn2_down: bool = false;
static mut btn3_down: bool = false;
static mut abs_hor: i8 = 0;
static mut abs_ver: i8 = 0;
static mut wheel: i8 = 0;

unsafe fn uhid_write(fd: i32, ev: *const UhidEvent) -> i32 {
    let ret = write(fd, ev.cast(), core::mem::size_of::<UhidEvent>());
    if ret < 0 { return -1; }
    if ret as usize != core::mem::size_of::<UhidEvent>() { return -EFAULT; }
    0
}

unsafe fn create(fd: i32) -> i32 {
    let mut ev: UhidEvent = core::mem::zeroed();
    // Field layout is provided by linux/uhid.h; populate the UHID_CREATE union member.
    let p = (&mut ev as *mut UhidEvent).cast::<u8>();
    (p as *mut u32).write(1);
    rdesc.as_ptr().copy_to(p.add(4), rdesc.len());
    uhid_write(fd, &ev)
}

unsafe fn destroy(fd: i32) {
    let mut ev: UhidEvent = core::mem::zeroed();
    ((&mut ev as *mut UhidEvent).cast::<u32>()).write(4);
    uhid_write(fd, &ev);
}

unsafe fn handle_output(_ev: *mut UhidEvent) {
    // LED output parsing depends on the linux/uhid.h union layout.
}

unsafe fn event(fd: i32) -> i32 {
    let mut ev: UhidEvent = core::mem::zeroed();
    let ret = read(fd, (&mut ev).cast(), core::mem::size_of::<UhidEvent>());
    if ret <= 0 { return -EFAULT; }
    let ty = (ev.data.as_ptr() as *const u32).read();
    match ty {
        UHID_START => {}, UHID_STOP => {}, UHID_OPEN => {}, UHID_CLOSE => {},
        UHID_OUTPUT => handle_output(&mut ev), UHID_OUTPUT_EV => {}, _ => {}
    }
    0
}

unsafe fn send_event(fd: i32) -> i32 {
    let mut ev: UhidEvent = core::mem::zeroed();
    ev.data[0..4].copy_from_slice(&UHID_INPUT.to_ne_bytes());
    ev.data[4] = 5;
    ev.data[5] = (btn1_down as u8) | ((btn2_down as u8) << 1) | ((btn3_down as u8) << 2);
    ev.data[6] = abs_hor as u8;
    ev.data[7] = abs_ver as u8;
    ev.data[8] = wheel as u8;
    uhid_write(fd, &ev)
}

unsafe fn keyboard(fd: i32) -> i32 {
    let mut buf = [0u8; 128];
    let ret = read(STDIN_FILENO, buf.as_mut_ptr().cast(), buf.len());
    if ret <= 0 { return -EFAULT; }
    for c in &buf[..ret as usize] {
        match *c as char {
            '1' => { btn1_down = !btn1_down; if send_event(fd) != 0 { return -1; } },
            '2' => { btn2_down = !btn2_down; if send_event(fd) != 0 { return -1; } },
            '3' => { btn3_down = !btn3_down; if send_event(fd) != 0 { return -1; } },
            'a' | 'd' => { abs_hor = if *c == b'a' { -20 } else { 20 }; let r = send_event(fd); abs_hor = 0; if r != 0 { return r; } },
            'w' | 's' => { abs_ver = if *c == b'w' { -20 } else { 20 }; let r = send_event(fd); abs_ver = 0; if r != 0 { return r; } },
            'r' | 'f' => { wheel = if *c == b'r' { 1 } else { -1 }; let r = send_event(fd); wheel = 0; if r != 0 { return r; } },
            'q' => return -ECANCELED,
            _ => {}
        }
    }
    0
}

fn main() {
    // The executable entry point follows the C implementation; libc and Linux
    // structure layouts are supplied by the surrounding build.
    unsafe {
        let args: Vec<*const i8> = std::env::args_os()
            .map(|arg| std::ffi::CString::new(arg.to_string_lossy().as_bytes()).unwrap().into_raw() as *const i8)
            .collect();
        let default_path = std::ffi::CString::new("/dev/uhid").unwrap();
        let path = if args.len() >= 2 { args[1] } else { default_path.as_ptr() };
        let fd = open(path, O_RDWR | O_CLOEXEC);
        if fd < 0 { return; }
        if create(fd) != 0 { close(fd); return; }
        let mut pfds = [
            PollFd { fd: STDIN_FILENO, events: POLLIN, revents: 0 },
            PollFd { fd, events: POLLIN, revents: 0 },
        ];
        loop {
            if poll(pfds.as_mut_ptr(), 2, -1) < 0 { break; }
            if pfds[0].revents & POLLHUP != 0 || pfds[1].revents & POLLHUP != 0 { break; }
            if pfds[0].revents & POLLIN != 0 && keyboard(fd) != 0 { break; }
            if pfds[1].revents & POLLIN != 0 && event(fd) != 0 { break; }
        }
        destroy(fd);
        close(fd);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
