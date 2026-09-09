// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2022 Benjamin Tissoires
 *
 * This is a pure HID-BPF example, and should be considered as such:
 * on the Etekcity Scroll 6E, the X and Y axes will be swapped and
 * inverted. On any other device... Not sure what this will do.
 *
 * This C main file is generic though. To adapt the code and test, users
 * must amend only the .bpf.c file, which this program will load any
 * eBPF program it finds.
 */

// C headers and generated skeleton declarations are supplied by external dependencies.

use std::os::raw::{c_char, c_int, c_void};

#[repr(C)]
pub struct hid_mouse__struct_ops {
    pub mouse_invert: *mut hid_mouse_mouse_invert,
}

#[repr(C)]
pub struct hid_mouse__maps {
    pub mouse_invert: *mut bpf_map,
}

#[repr(C)]
pub struct hid_mouse {
    pub struct_ops: hid_mouse__struct_ops,
    pub maps: hid_mouse__maps,
}

#[repr(C)]
pub struct hid_mouse_mouse_invert {
    pub hid_id: c_int,
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn snprintf(s: *mut c_char, n: usize, fmt: *const c_char, ...) -> c_int;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn basename(path: *mut c_char) -> *mut c_char;
    fn strtol(s: *const c_char, end: *mut *mut c_char, base: c_int) -> isize;
    fn hid_mouse__open() -> *mut hid_mouse;
    fn hid_mouse__load(skel: *mut hid_mouse) -> c_int;
    fn hid_mouse__destroy(skel: *mut hid_mouse);
    fn bpf_map__attach_struct_ops(map: *mut bpf_map) -> *mut bpf_link;
}

static mut RUNNING: bool = true;

unsafe extern "C" fn int_exit(_sig: c_int) {
    RUNNING = false;
    std::process::exit(0);
}

unsafe fn usage(prog: *const c_char) {
    // fprintf(stderr, "%s: %s /sys/bus/hid/devices/0BUS:0VID:0PID:00ID\n\n", __func__, prog);
    eprintln!("usage: {:?}: {:?} /sys/bus/hid/devices/0BUS:0VID:0PID:00ID\n", b"usage\0".as_ptr(), prog);
    eprintln!("This program will upload and attach a HID-BPF program to the given device.");
    eprintln!("On the Etekcity Scroll 6E, the X and Y axis will be inverted, but on any other");
    eprintln!("device, chances are high that the device will not be working anymore\n");
    eprintln!("consider this as a demo and adapt the eBPF program to your needs");
    eprintln!("Hit Ctrl-C to unbind the program and reset the device");
}

unsafe fn get_hid_id(path: *const c_char) -> c_int {
    let mut uevent = [0i8; 1024];
    memset(uevent.as_mut_ptr() as *mut c_void, 0, uevent.len());
    snprintf(uevent.as_mut_ptr(), uevent.len() - 1, b"%s/uevent\0".as_ptr() as *const c_char, path);

    let fd = open(uevent.as_ptr(), 0o0 | 0o4000);
    if fd < 0 {
        return -2;
    }
    close(fd);

    let dir = basename(path as *mut c_char);
    let str_id = dir.add(std::mem::size_of::<[u8; 16]>());
    strtol(str_id, std::ptr::null_mut(), 16) as c_int
}

pub unsafe fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let optstr = b"\0";
    let mut opt: c_int;
    let hid_id: c_int;
    let sysfs_path: *const c_char;
    let skel: *mut hid_mouse;
    let link: *mut bpf_link;
    let err: c_int;

    // getopt(argc, argv, optstr)
    opt = -1;
    while opt != -1 {
        match opt {
            _ => {
                usage(*argv);
                return 1;
            }
        }
    }

    sysfs_path = *argv.add(1);
    if sysfs_path.is_null() {
        eprintln!("sysfs");
        return 1;
    }

    skel = hid_mouse__open();
    if skel.is_null() {
        eprintln!("main  hid_mouse.c:0");
        return -1;
    }

    hid_id = get_hid_id(sysfs_path);
    if hid_id < 0 {
        eprintln!("can not open HID device");
        return 1;
    }
    (*(*skel).struct_ops.mouse_invert).hid_id = hid_id;

    err = hid_mouse__load(skel);
    if err < 0 {
        eprintln!("can not load HID-BPF program");
        return 1;
    }

    link = bpf_map__attach_struct_ops((*skel).maps.mouse_invert);
    if link.is_null() {
        eprintln!("can not attach HID-BPF program");
        return 1;
    }

    // signal(SIGINT, int_exit); signal(SIGTERM, int_exit);
    while RUNNING {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    hid_mouse__destroy(skel);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
