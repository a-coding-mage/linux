// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2022 Benjamin Tissoires
 *
 * This program will morph the Microsoft Surface Dial into a mouse,
 * and depending on the chosen resolution enable or not the haptic feedback:
 * - a resolution (-r) of 3600 will report 3600 "ticks" in one full rotation
 *   without haptic feedback
 * - any other resolution will report N "ticks" in a full rotation with haptic
 *   feedback
 *
 * A good default for low resolution haptic scrolling is 72 (1 "tick" every 5
 * degrees), and set to 3600 for smooth scrolling.
 */

// C headers and the generated hid_surface_dial.skel.h provide the external
// types and functions referenced below.

use core::ffi::{c_char, c_int, c_void};

static mut RUNNING: bool = true;

#[repr(C)]
struct HapticSyscallArgs {
    hid: u32,
    retval: c_int,
}

unsafe extern "C" {
    static mut stderr: *mut c_void;
    static mut optarg: *mut c_char;
    static mut optind: c_int;
    fn exit(status: c_int) -> !;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn basename(path: *mut c_char) -> *mut c_char;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> i64;
    fn bpf_program__fd(prog: *mut c_void) -> c_int;
    fn bpf_prog_test_run_opts(fd: c_int, opts: *mut c_void) -> c_int;
    fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;
    fn signal(sig: c_int, handler: unsafe extern "C" fn(c_int)) -> *mut c_void;
    fn sleep(seconds: u32) -> u32;
    fn perror(s: *const c_char);
}

#[repr(C)]
struct BpfLink;

#[repr(C)]
struct HapticProg {
    set_haptic: *mut c_void,
}

#[repr(C)]
struct SurfaceDialOps {
    hid_id: c_int,
}

#[repr(C)]
struct SurfaceDialData {
    resolution: c_int,
    physical: c_int,
}

#[repr(C)]
struct HIdSurfaceDial {
    progs: HapticProg,
    struct_ops: SurfaceDialStructOps,
    data: *mut SurfaceDialData,
    maps: SurfaceDialMaps,
}

#[repr(C)]
struct SurfaceDialStructOps {
    surface_dial: *mut SurfaceDialOps,
}

#[repr(C)]
struct SurfaceDialMaps {
    surface_dial: *mut c_void,
}

unsafe extern "C" {
    fn hid_surface_dial__open() -> *mut HIdSurfaceDial;
    fn hid_surface_dial__load(skel: *mut HIdSurfaceDial) -> c_int;
    fn hid_surface_dial__destroy(skel: *mut HIdSurfaceDial);
    fn bpf_map__attach_struct_ops(map: *mut c_void) -> *mut BpfLink;
}

unsafe extern "C" fn int_exit(_sig: c_int) {
    RUNNING = false;
    exit(0);
}

unsafe fn usage(prog: *const c_char) {
    fprintf(stderr, c"%s: %s [OPTIONS] /sys/bus/hid/devices/0BUS:0VID:0PID:00ID\n\n  OPTIONS:\n    -r N\t set the given resolution to the device (number of ticks per 360°)\n\n".as_ptr(), c"usage".as_ptr(), prog);
    fprintf(stderr, c"This program will morph the Microsoft Surface Dial into a mouse,\nand depending on the chosen resolution enable or not the haptic feedback:\n- a resolution (-r) of 3600 will report 3600 'ticks' in one full rotation\n  without haptic feedback\n- any other resolution will report N 'ticks' in a full rotation with haptic\n  feedback\n\nA good default for low resolution haptic scrolling is 72 (1 'tick' every 5\ndegrees), and set to 3600 for smooth scrolling.\n".as_ptr());
}

unsafe fn get_hid_id(path: *const c_char) -> c_int {
    let mut uevent = [0 as c_char; 1024];
    memset(uevent.as_mut_ptr() as *mut c_void, 0, uevent.len());
    snprintf(uevent.as_mut_ptr(), uevent.len() - 1, c"%s/uevent".as_ptr(), path);

    let fd = open(uevent.as_ptr(), 0x0000 | 0x0800);
    if fd < 0 {
        return -2;
    }
    close(fd);

    let dir = basename(path as *mut c_char);
    let str_id = dir.add(core::mem::size_of_val(&c"0003:0001:0A37."));
    strtol(str_id, core::ptr::null_mut(), 16) as c_int
}

unsafe fn set_haptic(skel: *mut HIdSurfaceDial, hid_id: c_int) -> c_int {
    let mut args = HapticSyscallArgs { hid: hid_id as u32, retval: -1 };
    let haptic_fd = bpf_program__fd((*skel).progs.set_haptic);
    if haptic_fd < 0 {
        return 1;
    }

    let mut tattr = (args, core::mem::size_of::<HapticSyscallArgs>());
    let err = bpf_prog_test_run_opts(haptic_fd, &mut tattr as *mut _ as *mut c_void);
    if err != 0 { return 1; }
    0
}

pub unsafe fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut skel: *mut HIdSurfaceDial;
    let mut resolution: c_int = 72;
    let optstr = c"r:";

    loop {
        let opt = getopt(argc, argv, optstr.as_ptr());
        if opt == -1 { break; }
        match opt as u8 as char {
            'r' => {
                let mut endp: *mut c_char = core::ptr::null_mut();
                let mut l: i64 = -1;
                if !optarg.is_null() {
                    l = strtol(optarg, &mut endp, 10);
                    if !endp.is_null() && *endp != 0 { l = -1; }
                }
                if l < 0 {
                    fprintf(stderr, c"invalid r option %s - expecting a number\n".as_ptr(), if optarg.is_null() { c"".as_ptr() } else { optarg });
                    exit(1);
                }
                resolution = l as c_int;
            }
            _ => { usage(*argv); return 1; }
        }
    }

    if optind == argc { usage(basename(*argv)); return 1; }
    let sysfs_path = *argv.add(optind as usize);
    if sysfs_path.is_null() { perror(c"sysfs".as_ptr()); return 1; }

    skel = hid_surface_dial__open();
    if skel.is_null() { return -1; }
    let hid_id = get_hid_id(sysfs_path);
    if hid_id < 0 { return 1; }

    (*(*skel).struct_ops.surface_dial).hid_id = hid_id;
    if hid_surface_dial__load(skel) < 0 { return 1; }
    (*(*skel).data).resolution = resolution;
    (*(*skel).data).physical = resolution / 72;
    if bpf_map__attach_struct_ops((*skel).maps.surface_dial).is_null() { return 1; }

    signal(2, int_exit);
    signal(15, int_exit);
    set_haptic(skel, hid_id);
    while RUNNING { sleep(1); }
    hid_surface_dial__destroy(skel);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
