// SPDX-License-Identifier: GPL-2.0
// test ir decoder
//
// Copyright (C) 2018 Sean Young <sean@mess.org>

// When sending LIRC_MODE_SCANCODE, the IR will be encoded. rc-loopback
// will send this IR to the receiver side, where we try to read the decoded
// IR. Decoding happens in a separate kernel thread, so we will need to
// wait until that is scheduled, hence we use poll to check for read
// readiness.

// C dependencies translated as external declarations:
// linux/lirc.h, errno.h, stdio.h, stdlib.h, stdbool.h, string.h, unistd.h,
// poll.h, time.h, sys/types.h, sys/ioctl.h, dirent.h, sys/stat.h, fcntl.h,
// and "kselftest.h".

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

const TEST_SCANCODES: c_int = 10;
const SYSFS_PATH_MAX: usize = 256;
const DNAME_PATH_MAX: usize = 256;

/*
 * Support ancient lirc.h which does not have these values. Can be removed
 * once RHEL 8 is no longer a relevant testing platform.
 *
 * Original C condition:
 * #if RC_PROTO_MAX < 26
 * #define RC_PROTO_RCMM12 24
 * #define RC_PROTO_RCMM24 25
 * #define RC_PROTO_RCMM32 26
 * #endif
 */
const RC_PROTO_RCMM12: rc_proto = 24;
const RC_PROTO_RCMM24: rc_proto = 25;
const RC_PROTO_RCMM32: rc_proto = 26;

type rc_proto = c_uint;

const RC_PROTO_RC5: rc_proto = 2;
const RC_PROTO_RC5X_20: rc_proto = 3;
const RC_PROTO_RC5_SZ: rc_proto = 4;
const RC_PROTO_JVC: rc_proto = 5;
const RC_PROTO_SONY12: rc_proto = 6;
const RC_PROTO_SONY15: rc_proto = 7;
const RC_PROTO_SONY20: rc_proto = 8;
const RC_PROTO_NEC: rc_proto = 9;
const RC_PROTO_NECX: rc_proto = 10;
const RC_PROTO_NEC32: rc_proto = 11;
const RC_PROTO_SANYO: rc_proto = 12;
const RC_PROTO_RC6_0: rc_proto = 15;
const RC_PROTO_RC6_6A_20: rc_proto = 16;
const RC_PROTO_RC6_6A_24: rc_proto = 17;
const RC_PROTO_RC6_6A_32: rc_proto = 18;
const RC_PROTO_RC6_MCE: rc_proto = 19;
const RC_PROTO_SHARP: rc_proto = 20;
const RC_PROTO_IMON: rc_proto = 23;

#[repr(C)]
struct Protocol {
    proto: rc_proto,
    name: *const c_char,
    mask: c_uint,
    decoder: *const c_char,
}

unsafe impl Sync for Protocol {}

static PROTOCOLS: [Protocol; 21] = [
    Protocol { proto: RC_PROTO_RC5, name: c"rc-5".as_ptr(), mask: 0x1f7f, decoder: c"rc-5".as_ptr() },
    Protocol { proto: RC_PROTO_RC5X_20, name: c"rc-5x-20".as_ptr(), mask: 0x1f7f3f, decoder: c"rc-5".as_ptr() },
    Protocol { proto: RC_PROTO_RC5_SZ, name: c"rc-5-sz".as_ptr(), mask: 0x2fff, decoder: c"rc-5-sz".as_ptr() },
    Protocol { proto: RC_PROTO_JVC, name: c"jvc".as_ptr(), mask: 0xffff, decoder: c"jvc".as_ptr() },
    Protocol { proto: RC_PROTO_SONY12, name: c"sony-12".as_ptr(), mask: 0x1f007f, decoder: c"sony".as_ptr() },
    Protocol { proto: RC_PROTO_SONY15, name: c"sony-15".as_ptr(), mask: 0xff007f, decoder: c"sony".as_ptr() },
    Protocol { proto: RC_PROTO_SONY20, name: c"sony-20".as_ptr(), mask: 0x1fff7f, decoder: c"sony".as_ptr() },
    Protocol { proto: RC_PROTO_NEC, name: c"nec".as_ptr(), mask: 0xffff, decoder: c"nec".as_ptr() },
    Protocol { proto: RC_PROTO_NECX, name: c"nec-x".as_ptr(), mask: 0xffffff, decoder: c"nec".as_ptr() },
    Protocol { proto: RC_PROTO_NEC32, name: c"nec-32".as_ptr(), mask: 0xffffffff, decoder: c"nec".as_ptr() },
    Protocol { proto: RC_PROTO_SANYO, name: c"sanyo".as_ptr(), mask: 0x1fffff, decoder: c"sanyo".as_ptr() },
    Protocol { proto: RC_PROTO_RC6_0, name: c"rc-6-0".as_ptr(), mask: 0xffff, decoder: c"rc-6".as_ptr() },
    Protocol { proto: RC_PROTO_RC6_6A_20, name: c"rc-6-6a-20".as_ptr(), mask: 0xfffff, decoder: c"rc-6".as_ptr() },
    Protocol { proto: RC_PROTO_RC6_6A_24, name: c"rc-6-6a-24".as_ptr(), mask: 0xffffff, decoder: c"rc-6".as_ptr() },
    Protocol { proto: RC_PROTO_RC6_6A_32, name: c"rc-6-6a-32".as_ptr(), mask: 0xffffffff, decoder: c"rc-6".as_ptr() },
    Protocol { proto: RC_PROTO_RC6_MCE, name: c"rc-6-mce".as_ptr(), mask: 0x00007fff, decoder: c"rc-6".as_ptr() },
    Protocol { proto: RC_PROTO_SHARP, name: c"sharp".as_ptr(), mask: 0x1fff, decoder: c"sharp".as_ptr() },
    Protocol { proto: RC_PROTO_IMON, name: c"imon".as_ptr(), mask: 0x7fffffff, decoder: c"imon".as_ptr() },
    Protocol { proto: RC_PROTO_RCMM12, name: c"rcmm-12".as_ptr(), mask: 0x00000fff, decoder: c"rc-mm".as_ptr() },
    Protocol { proto: RC_PROTO_RCMM24, name: c"rcmm-24".as_ptr(), mask: 0x00ffffff, decoder: c"rc-mm".as_ptr() },
    Protocol { proto: RC_PROTO_RCMM32, name: c"rcmm-32".as_ptr(), mask: 0xffffffff, decoder: c"rc-mm".as_ptr() },
];

#[repr(C)]
struct DIR {
    _private: [u8; 0],
}

#[repr(C)]
struct dirent {
    d_ino: c_ulong,
    d_off: c_long,
    d_reclen: u16,
    d_type: u8,
    d_name: [c_char; 256],
}

#[repr(C)]
struct pollfd {
    fd: c_int,
    events: i16,
    revents: i16,
}

#[repr(C)]
struct lirc_scancode {
    timestamp: u64,
    flags: u16,
    rc_proto: rc_proto,
    keycode: u32,
    scancode: u64,
}

unsafe extern "C" {
    static mut errno: c_int;

    static LIRC_MODE_SCANCODE: c_uint;
    static LIRC_SET_REC_MODE: c_ulong;
    static LIRC_SET_SEND_MODE: c_ulong;

    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn opendir(name: *const c_char) -> *mut DIR;
    fn readdir(dirp: *mut DIR) -> *mut dirent;
    fn closedir(dirp: *mut DIR) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn srand(seed: c_uint);
    fn rand() -> c_int;
    fn time(tloc: *mut c_long) -> c_long;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn strlen(s: *const c_char) -> usize;
    fn poll(fds: *mut pollfd, nfds: c_ulong, timeout: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;

    fn ksft_exit_fail_msg(msg: *const c_char, ...) -> !;
    fn ksft_exit_skip(msg: *const c_char, ...) -> !;
    fn ksft_test_result_error(msg: *const c_char, ...);
    fn ksft_inc_pass_cnt();
    fn ksft_get_fail_cnt() -> c_int;
    fn ksft_exit_fail() -> !;
    fn ksft_exit_pass() -> !;
}

const O_RDWR: c_int = 0o2;
const O_NONBLOCK: c_int = 0o4000;
const O_WRONLY: c_int = 0o1;
const EINTR: c_int = 4;
const POLLIN: i16 = 0x0001;

pub unsafe fn lirc_open(rc: *const c_char) -> c_int {
    let mut dent: *mut dirent;
    let mut buf = [0 as c_char; SYSFS_PATH_MAX + DNAME_PATH_MAX];
    let d: *mut DIR;
    let fd: c_int;

    snprintf(buf.as_mut_ptr(), buf.len(), c"/sys/class/rc/%s".as_ptr(), rc);

    d = opendir(buf.as_ptr());
    if d.is_null() {
        ksft_exit_fail_msg(c"cannot open %s: %m\n".as_ptr(), buf.as_ptr());
    }

    loop {
        dent = readdir(d);
        if dent.is_null() {
            break;
        }
        if strncmp((*dent).d_name.as_ptr(), c"lirc".as_ptr(), 4) == 0 {
            snprintf(buf.as_mut_ptr(), buf.len(), c"/dev/%s".as_ptr(), (*dent).d_name.as_ptr());
            break;
        }
    }

    if dent.is_null() {
        ksft_exit_skip(c"cannot find lirc device for %s\n".as_ptr(), rc);
    }

    closedir(d);

    fd = open(buf.as_ptr(), O_RDWR | O_NONBLOCK);
    if fd == -1 {
        ksft_exit_fail_msg(c"cannot open: %s: %m\n".as_ptr(), buf.as_ptr());
    }

    fd
}

pub unsafe fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut mode: c_uint;
    let mut buf = [0 as c_char; 100];
    let rlircfd: c_int;
    let wlircfd: c_int;
    let protocolfd: c_int;
    let mut i: c_int;
    let mut n: c_int;

    srand(time(core::ptr::null_mut()) as c_uint);

    if argc != 3 {
        ksft_exit_fail_msg(c"Usage: %s <write rcN> <read rcN>\n".as_ptr(), *argv.add(0));
    }

    rlircfd = lirc_open(*argv.add(2));
    mode = LIRC_MODE_SCANCODE;
    if ioctl(rlircfd, LIRC_SET_REC_MODE, &mut mode as *mut c_uint) != 0 {
        ksft_exit_fail_msg(c"failed to set scancode rec mode %s: %m\n".as_ptr(), *argv.add(2));
    }

    wlircfd = lirc_open(*argv.add(1));
    if ioctl(wlircfd, LIRC_SET_SEND_MODE, &mut mode as *mut c_uint) != 0 {
        ksft_exit_fail_msg(c"failed to set scancode send mode %s: %m\n".as_ptr(), *argv.add(1));
    }

    snprintf(buf.as_mut_ptr(), buf.len(), c"/sys/class/rc/%s/protocols".as_ptr(), *argv.add(2));
    protocolfd = open(buf.as_ptr(), O_WRONLY);
    if protocolfd == -1 {
        ksft_exit_fail_msg(c"failed to open %s: %m\n".as_ptr(), buf.as_ptr());
    }

    printf(c"Sending IR on %s and receiving IR on %s.\n".as_ptr(), *argv.add(1), *argv.add(2));

    i = 0;
    while (i as usize) < PROTOCOLS.len() {
        let protocol = &PROTOCOLS[i as usize];
        if write(
            protocolfd,
            protocol.decoder as *const c_void,
            strlen(protocol.decoder),
        ) == -1
        {
            ksft_exit_fail_msg(c"failed to set write decoder\n".as_ptr());
        }

        printf(
            c"Testing protocol %s for decoder %s (%d/%d)...\n".as_ptr(),
            protocol.name,
            protocol.decoder,
            i + 1,
            PROTOCOLS.len() as c_int,
        );

        n = 0;
        while n < TEST_SCANCODES {
            let mut scancode: c_uint = (rand() as c_uint) & protocol.mask;
            let rc_proto: c_uint = protocol.proto;

            if rc_proto == RC_PROTO_RC6_MCE {
                scancode |= 0x800f0000;
            }

            if rc_proto == RC_PROTO_NECX
                && ((((scancode >> 16) ^ !(scancode >> 8)) & 0xff) == 0)
            {
                n += 1;
                continue;
            }

            if rc_proto == RC_PROTO_NEC32 && ((((scancode >> 8) ^ !scancode) & 0xff) == 0) {
                n += 1;
                continue;
            }

            if rc_proto == RC_PROTO_RCMM32
                && (scancode & 0x000c0000) != 0x000c0000
                && (scancode & 0x00008000) != 0
            {
                n += 1;
                continue;
            }

            let lsc = lirc_scancode {
                timestamp: 0,
                flags: 0,
                rc_proto,
                keycode: 0,
                scancode: scancode as u64,
            };

            printf(c"Testing scancode:%x\n".as_ptr(), scancode);

            while write(
                wlircfd,
                &lsc as *const lirc_scancode as *const c_void,
                core::mem::size_of::<lirc_scancode>(),
            ) < 0
            {
                if errno == EINTR {
                    continue;
                }

                ksft_exit_fail_msg(c"failed to send ir: %m\n".as_ptr());
            }

            let mut pfd = pollfd {
                fd: rlircfd,
                events: POLLIN,
                revents: 0,
            };
            let mut lsc2 = core::mem::MaybeUninit::<lirc_scancode>::uninit();

            poll(&mut pfd as *mut pollfd, 1, 1000);

            let mut decoded = true;

            while read(
                rlircfd,
                lsc2.as_mut_ptr() as *mut c_void,
                core::mem::size_of::<lirc_scancode>(),
            ) < 0
            {
                if errno == EINTR {
                    continue;
                }

                ksft_test_result_error(c"no scancode decoded: %m\n".as_ptr());
                decoded = false;
                break;
            }

            if !decoded {
                n += 1;
                continue;
            }

            let lsc2 = lsc2.assume_init();

            if lsc.rc_proto != lsc2.rc_proto {
                ksft_test_result_error(
                    c"decoded protocol is different: %d\n".as_ptr(),
                    lsc2.rc_proto,
                );
            } else if lsc.scancode != lsc2.scancode {
                ksft_test_result_error(
                    c"decoded scancode is different: %llx\n".as_ptr(),
                    lsc2.scancode,
                );
            } else {
                ksft_inc_pass_cnt();
            }

            n += 1;
        }

        printf(c"OK\n".as_ptr());
        i += 1;
    }

    close(rlircfd);
    close(wlircfd);
    close(protocolfd);

    if ksft_get_fail_cnt() > 0 {
        ksft_exit_fail();
    } else {
        ksft_exit_pass();
    }

    0
}
