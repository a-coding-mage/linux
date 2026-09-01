// SPDX-License-Identifier: GPL-2.0
// test ir decoder
//
// Copyright (C) 2018 Sean Young <sean@mess.org>

// A lirc chardev is a device representing a consumer IR (cir) device which
// can receive infrared signals from remote control and/or transmit IR.
//
// IR is sent as a series of pulses and space somewhat like morse code. The
// BPF program can decode this into scancodes so that rc-core can translate
// this into input key codes using the rc keymap.
//
// This test works by sending IR over rc-loopback, so the IR is processed by
// BPF and then decoded into scancodes. The lirc chardev must be the one
// associated with rc-loopback, see the output of ir-keytable(1).
//
// The following CONFIG options must be enabled for the test to succeed:
// CONFIG_RC_CORE=y
// CONFIG_BPF_RAWIR_EVENT=y
// CONFIG_RC_LOOPBACK=y

// Steps:
// 1. Open the /dev/lircN device for rc-loopback (given on command line)
// 2. Attach bpf_lirc_mode2 program which decodes some IR.
// 3. Send some IR to the same IR device; since it is loopback, this will
//    end up in the bpf program
// 4. bpf program should decode IR and report keycode
// 5. We can read keycode from same /dev/lirc device

use std::ffi::c_void;
use std::mem::size_of;
use std::os::raw::{c_char, c_int, c_long, c_short, c_uint, c_ulong};
use std::ptr;

type u32 = c_uint;

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct timeval {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
}

#[repr(C)]
pub struct input_event {
    pub time: timeval,
    pub type_: u16,
    pub code: u16,
    pub value: i32,
}

#[repr(C)]
pub struct pollfd {
    pub fd: c_int,
    pub events: c_short,
    pub revents: c_short,
}

const ENOENT: c_int = 2;
const O_RDONLY: c_int = 0;
const O_RDWR: c_int = 2;
const O_NONBLOCK: c_int = 0o4000;
const POLLIN: c_short = 0x0001;

const EV_REL: u16 = 0x02;
const EV_MSC: u16 = 0x04;
const REL_Y: u16 = 0x01;
const MSC_SCAN: u16 = 0x04;

// From linux/bpf.h; kept here as the numeric C enum values used by this test.
const BPF_PROG_TYPE_LIRC_MODE2: c_int = 15;
const BPF_LIRC_MODE2: c_int = 10;

unsafe extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn poll(fds: *mut pollfd, nfds: c_ulong, timeout: c_int) -> c_int;

    fn bpf_prog_test_load(
        file: *const c_char,
        type_: c_int,
        pobj: *mut *mut bpf_object,
        prog_fd: *mut c_int,
    ) -> c_int;
    fn bpf_prog_detach2(prog_fd: c_int, target_fd: c_int, type_: c_int) -> c_int;
    fn bpf_prog_query(
        target_fd: c_int,
        type_: c_int,
        query_flags: u32,
        attach_flags: *mut u32,
        prog_ids: *mut u32,
        prog_cnt: *mut u32,
    ) -> c_int;
    fn bpf_prog_attach(prog_fd: c_int, target_fd: c_int, type_: c_int, flags: c_uint) -> c_int;
}

unsafe fn main_impl(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut obj: *mut bpf_object = ptr::null_mut();
    let mut ret: c_int;
    let lircfd: c_int;
    let mut progfd: c_int = 0;
    let inputfd: c_int;
    let testir1: c_int = 0x1ead;
    let testir2: c_int = 0x2101;
    let mut prog_ids: [u32; 10] = [0; 10];
    let mut prog_flags: [u32; 10] = [0; 10];
    let mut prog_cnt: u32;

    if argc != 3 {
        printf(b"Usage: %s /dev/lircN /dev/input/eventM\n\0".as_ptr() as *const c_char, *argv);
        return 2;
    }

    ret = bpf_prog_test_load(
        b"test_lirc_mode2_kern.bpf.o\0".as_ptr() as *const c_char,
        BPF_PROG_TYPE_LIRC_MODE2,
        &mut obj,
        &mut progfd,
    );
    if ret != 0 {
        printf(b"Failed to load bpf program\n\0".as_ptr() as *const c_char);
        return 1;
    }

    lircfd = open(*argv.add(1), O_RDWR | O_NONBLOCK);
    if lircfd == -1 {
        printf(
            b"failed to open lirc device %s: %m\n\0".as_ptr() as *const c_char,
            *argv.add(1),
        );
        return 1;
    }

    /* Let's try detach it before it was ever attached */
    ret = bpf_prog_detach2(progfd, lircfd, BPF_LIRC_MODE2);
    if ret != -ENOENT {
        printf(b"bpf_prog_detach2 not attached should fail: %m\n\0".as_ptr() as *const c_char);
        return 1;
    }

    inputfd = open(*argv.add(2), O_RDONLY | O_NONBLOCK);
    if inputfd == -1 {
        printf(
            b"failed to open input device %s: %m\n\0".as_ptr() as *const c_char,
            *argv.add(1),
        );
        return 1;
    }

    prog_cnt = 10;
    ret = bpf_prog_query(
        lircfd,
        BPF_LIRC_MODE2,
        0,
        prog_flags.as_mut_ptr(),
        prog_ids.as_mut_ptr(),
        &mut prog_cnt,
    );
    if ret != 0 {
        printf(b"Failed to query bpf programs on lirc device: %m\n\0".as_ptr() as *const c_char);
        return 1;
    }

    if prog_cnt != 0 {
        printf(b"Expected nothing to be attached\n\0".as_ptr() as *const c_char);
        return 1;
    }

    ret = bpf_prog_attach(progfd, lircfd, BPF_LIRC_MODE2, 0);
    if ret != 0 {
        printf(b"Failed to attach bpf to lirc device: %m\n\0".as_ptr() as *const c_char);
        return 1;
    }

    /* Write raw IR */
    ret = write(
        lircfd,
        &testir1 as *const c_int as *const c_void,
        size_of::<c_int>(),
    ) as c_int;
    if ret != size_of::<c_int>() as c_int {
        printf(b"Failed to send test IR message: %m\n\0".as_ptr() as *const c_char);
        return 1;
    }

    let mut pfd = pollfd {
        fd: inputfd,
        events: POLLIN,
        revents: 0,
    };
    let mut event: input_event = std::mem::zeroed();

    loop {
        poll(&mut pfd, 1, 100);

        /* Read decoded IR */
        ret = read(
            inputfd,
            &mut event as *mut input_event as *mut c_void,
            size_of::<input_event>(),
        ) as c_int;
        if ret != size_of::<input_event>() as c_int {
            printf(b"Failed to read decoded IR: %m\n\0".as_ptr() as *const c_char);
            return 1;
        }

        if event.type_ == EV_MSC && event.code == MSC_SCAN && event.value == 0x1ead {
            break;
        }
    }

    /* Write raw IR */
    ret = write(
        lircfd,
        &testir2 as *const c_int as *const c_void,
        size_of::<c_int>(),
    ) as c_int;
    if ret != size_of::<c_int>() as c_int {
        printf(b"Failed to send test IR message: %m\n\0".as_ptr() as *const c_char);
        return 1;
    }

    loop {
        poll(&mut pfd, 1, 100);

        /* Read decoded IR */
        ret = read(
            inputfd,
            &mut event as *mut input_event as *mut c_void,
            size_of::<input_event>(),
        ) as c_int;
        if ret != size_of::<input_event>() as c_int {
            printf(b"Failed to read decoded IR: %m\n\0".as_ptr() as *const c_char);
            return 1;
        }

        if event.type_ == EV_REL && event.code == REL_Y && event.value == 1 {
            break;
        }
    }

    prog_cnt = 10;
    ret = bpf_prog_query(
        lircfd,
        BPF_LIRC_MODE2,
        0,
        prog_flags.as_mut_ptr(),
        prog_ids.as_mut_ptr(),
        &mut prog_cnt,
    );
    if ret != 0 {
        printf(b"Failed to query bpf programs on lirc device: %m\n\0".as_ptr() as *const c_char);
        return 1;
    }

    if prog_cnt != 1 {
        printf(b"Expected one program to be attached\n\0".as_ptr() as *const c_char);
        return 1;
    }

    /* Let's try detaching it now it is actually attached */
    ret = bpf_prog_detach2(progfd, lircfd, BPF_LIRC_MODE2);
    if ret != 0 {
        printf(b"bpf_prog_detach2: returned %m\n\0".as_ptr() as *const c_char);
        return 1;
    }

    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    main_impl(argc, argv)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
