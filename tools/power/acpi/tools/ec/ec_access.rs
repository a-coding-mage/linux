// SPDX-License-Identifier: GPL-2.0-only
/*
 * ec_access.c
 *
 * Copyright (C) 2010 SUSE Linux Products GmbH
 * Author:
 *      Thomas Renninger <trenn@suse.de>
 */

// C dependencies originally included:
// <fcntl.h>, <err.h>, <stdio.h>, <stdlib.h>, <libgen.h>, <unistd.h>,
// <getopt.h>, <stdint.h>, <sys/types.h>, <sys/stat.h>

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

const EC_SPACE_SIZE: usize = 256;
const SYSFS_PATH: &[u8] = b"/sys/kernel/debug/ec/ec0/io\0";

const EXIT_FAILURE: c_int = 1;
const EXIT_SUCCESS: c_int = 0;
const O_RDONLY: c_int = 0;
const O_WRONLY: c_int = 1;
const SEEK_SET: c_int = 0;

/*
 * TBD/Enhancements:
 *   - Provide param for accessing different ECs (not supported by kernel yet)
 */

static mut READ_MODE: c_int = -1;
static mut SLEEP_TIME: c_int = 0;
static mut WRITE_BYTE_OFFSET: c_int = -1;
static mut READ_BYTE_OFFSET: c_int = -1;
static mut WRITE_VALUE: u8 = (-1i32) as u8;

unsafe extern "C" {
    static mut optarg: *mut c_char;

    fn atoi(nptr: *const c_char) -> c_int;
    fn basename(path: *mut c_char) -> *mut c_char;
    fn close(fd: c_int) -> c_int;
    fn err(eval: c_int, fmt: *const c_char, ...) -> !;
    fn exit(status: c_int) -> !;
    fn fprintf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn getopt(argc: c_int, argv: *const *mut c_char, optstring: *const c_char) -> c_int;
    fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn puts(s: *const c_char) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn sleep(seconds: c_uint) -> c_uint;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;

    static mut stderr: *mut FILE;
}

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

#[allow(non_camel_case_types)]
type size_t = usize;
#[allow(non_camel_case_types)]
type ssize_t = isize;
#[allow(non_camel_case_types)]
type off_t = c_long;

unsafe fn usage(progname: *mut c_char, exit_status: c_int) -> ! {
    printf(c"Usage:\n".as_ptr());
    printf(c"1) %s -r [-s sleep]\n".as_ptr(), basename(progname));
    printf(c"2) %s -b byte_offset\n".as_ptr(), basename(progname));
    printf(c"3) %s -w byte_offset -v value\n\n".as_ptr(), basename(progname));

    puts(c"\t-r [-s sleep]      : Dump EC registers".as_ptr());
    puts(c"\t                     If sleep is given, sleep x seconds,".as_ptr());
    puts(c"\t                     re-read EC registers and show changes".as_ptr());
    puts(c"\t-b offset          : Read value at byte_offset (in hex)".as_ptr());
    puts(c"\t-w offset -v value : Write value at byte_offset".as_ptr());
    puts(c"\t-h                 : Print this help\n\n".as_ptr());
    puts(c"Offsets and values are in hexadecimal number system.".as_ptr());
    puts(c"The offset and value must be between 0 and 0xff.".as_ptr());
    exit(exit_status);
}

unsafe fn parse_opts(argc: c_int, argv: *mut *mut c_char) {
    let mut c: c_int;

    loop {
        c = getopt(argc, argv, c"rs:b:w:v:h".as_ptr());
        if c == -1 {
            break;
        }

        match c as u8 as char {
            'r' => {
                if READ_MODE != -1 {
                    usage(*argv.add(0), EXIT_FAILURE);
                }
                READ_MODE = 1;
            }
            's' => {
                if READ_MODE != -1 && READ_MODE != 1 {
                    usage(*argv.add(0), EXIT_FAILURE);
                }

                SLEEP_TIME = atoi(optarg);
                if SLEEP_TIME <= 0 {
                    SLEEP_TIME = 0;
                    usage(*argv.add(0), EXIT_FAILURE);
                    printf(c"Bad sleep time: %s\n".as_ptr(), optarg);
                }
            }
            'b' => {
                if READ_MODE != -1 {
                    usage(*argv.add(0), EXIT_FAILURE);
                }
                READ_MODE = 1;
                READ_BYTE_OFFSET = strtoul(optarg, core::ptr::null_mut(), 16) as c_int;
            }
            'w' => {
                if READ_MODE != -1 {
                    usage(*argv.add(0), EXIT_FAILURE);
                }
                READ_MODE = 0;
                WRITE_BYTE_OFFSET = strtoul(optarg, core::ptr::null_mut(), 16) as c_int;
            }
            'v' => {
                WRITE_VALUE = strtoul(optarg, core::ptr::null_mut(), 16) as u8;
            }
            'h' => {
                usage(*argv.add(0), EXIT_SUCCESS);
            }
            _ => {
                fprintf(stderr, c"Unknown option!\n".as_ptr());
                usage(*argv.add(0), EXIT_FAILURE);
            }
        }
    }
    if READ_MODE == 0 {
        if WRITE_BYTE_OFFSET < 0 || WRITE_BYTE_OFFSET >= EC_SPACE_SIZE as c_int {
            fprintf(
                stderr,
                c"Wrong byte offset 0x%.2x, valid: [0-0x%.2x]\n".as_ptr(),
                WRITE_BYTE_OFFSET,
                EC_SPACE_SIZE as c_int - 1,
            );
            usage(*argv.add(0), EXIT_FAILURE);
        }
        if (WRITE_VALUE as c_int) < 0 || (WRITE_VALUE as c_int) >= 255 {
            fprintf(
                stderr,
                c"Wrong byte offset 0x%.2x, valid:[0-0xff]\n".as_ptr(),
                WRITE_BYTE_OFFSET,
            );
            usage(*argv.add(0), EXIT_FAILURE);
        }
    }
    if READ_MODE == 1 && READ_BYTE_OFFSET != -1 {
        if READ_BYTE_OFFSET < -1 || READ_BYTE_OFFSET >= EC_SPACE_SIZE as c_int {
            fprintf(
                stderr,
                c"Wrong byte offset 0x%.2x, valid: [0-0x%.2x]\n".as_ptr(),
                READ_BYTE_OFFSET,
                EC_SPACE_SIZE as c_int - 1,
            );
            usage(*argv.add(0), EXIT_FAILURE);
        }
    }
    /* Add additional parameter checks here */
}

unsafe fn dump_ec(fd: c_int) {
    let mut buf = [0 as c_char; EC_SPACE_SIZE];
    let mut buf2 = [0 as c_char; EC_SPACE_SIZE];
    let mut byte_off: c_int;
    let mut bytes_read: c_int;

    bytes_read = read(fd, buf.as_mut_ptr() as *mut c_void, EC_SPACE_SIZE) as c_int;

    if bytes_read == -1 {
        err(
            EXIT_FAILURE,
            c"Could not read from %s\n".as_ptr(),
            SYSFS_PATH.as_ptr() as *const c_char,
        );
    }

    if bytes_read != EC_SPACE_SIZE as c_int {
        fprintf(stderr, c"Could only read %d bytes\n".as_ptr(), bytes_read);
    }

    printf(c"     00  01  02  03  04  05  06  07  08  09  0A  0B  0C  0D  0E  0F".as_ptr());
    byte_off = 0;
    while byte_off < bytes_read {
        if (byte_off % 16) == 0 {
            printf(c"\n%.2X: ".as_ptr(), byte_off);
        }
        printf(c" %.2x ".as_ptr(), buf[byte_off as usize] as u8 as c_int);
        byte_off += 1;
    }
    printf(c"\n".as_ptr());

    if SLEEP_TIME == 0 {
        return;
    }

    printf(c"\n".as_ptr());
    lseek(fd, 0, SEEK_SET);
    sleep(SLEEP_TIME as c_uint);

    bytes_read = read(fd, buf2.as_mut_ptr() as *mut c_void, EC_SPACE_SIZE) as c_int;

    if bytes_read == -1 {
        err(
            EXIT_FAILURE,
            c"Could not read from %s\n".as_ptr(),
            SYSFS_PATH.as_ptr() as *const c_char,
        );
    }

    if bytes_read != EC_SPACE_SIZE as c_int {
        fprintf(stderr, c"Could only read %d bytes\n".as_ptr(), bytes_read);
    }

    printf(c"     00  01  02  03  04  05  06  07  08  09  0A  0B  0C  0D  0E  0F".as_ptr());
    byte_off = 0;
    while byte_off < bytes_read {
        if (byte_off % 16) == 0 {
            printf(c"\n%.2X: ".as_ptr(), byte_off);
        }

        if buf[byte_off as usize] == buf2[byte_off as usize] {
            printf(c" %.2x ".as_ptr(), buf2[byte_off as usize] as u8 as c_int);
        } else {
            printf(c"*%.2x ".as_ptr(), buf2[byte_off as usize] as u8 as c_int);
        }
        byte_off += 1;
    }
    printf(c"\n".as_ptr());
}

unsafe fn read_ec_val(fd: c_int, byte_offset: c_int) {
    let mut buf: u8 = 0;
    let mut error: c_int;

    error = lseek(fd, byte_offset as off_t, SEEK_SET) as c_int;
    if error != byte_offset {
        err(
            EXIT_FAILURE,
            c"Cannot set offset to 0x%.2x".as_ptr(),
            byte_offset,
        );
    }

    error = read(fd, &mut buf as *mut u8 as *mut c_void, 1) as c_int;
    if error != 1 {
        err(
            EXIT_FAILURE,
            c"Could not read byte 0x%.2x from %s\n".as_ptr(),
            byte_offset,
            SYSFS_PATH.as_ptr() as *const c_char,
        );
    }
    printf(c"0x%.2x\n".as_ptr(), buf as c_int);
    return;
}

unsafe fn write_ec_val(fd: c_int, byte_offset: c_int, value: u8) {
    let mut error: c_int;

    error = lseek(fd, byte_offset as off_t, SEEK_SET) as c_int;
    if error != byte_offset {
        err(
            EXIT_FAILURE,
            c"Cannot set offset to 0x%.2x".as_ptr(),
            byte_offset,
        );
    }

    error = write(fd, &value as *const u8 as *const c_void, 1) as c_int;
    if error != 1 {
        err(
            EXIT_FAILURE,
            c"Cannot write value 0x%.2x to offset 0x%.2x".as_ptr(),
            value as c_int,
            byte_offset,
        );
    }
}

unsafe fn c_main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut file_mode: c_int = O_RDONLY;
    let fd: c_int;

    parse_opts(argc, argv);

    if READ_MODE == 0 {
        file_mode = O_WRONLY;
    } else if READ_MODE == 1 {
        file_mode = O_RDONLY;
    } else {
        usage(*argv.add(0), EXIT_FAILURE);
    }

    fd = open(SYSFS_PATH.as_ptr() as *const c_char, file_mode);
    if fd == -1 {
        err(
            EXIT_FAILURE,
            c"%s".as_ptr(),
            SYSFS_PATH.as_ptr() as *const c_char,
        );
    }

    if READ_MODE != 0 {
        if READ_BYTE_OFFSET == -1 {
            dump_ec(fd);
        } else if READ_BYTE_OFFSET < 0 || READ_BYTE_OFFSET >= EC_SPACE_SIZE as c_int {
            usage(*argv.add(0), EXIT_FAILURE);
        } else {
            read_ec_val(fd, READ_BYTE_OFFSET);
        }
    } else {
        write_ec_val(fd, WRITE_BYTE_OFFSET, WRITE_VALUE);
    }
    close(fd);

    exit(EXIT_SUCCESS);
}

fn main() {
    unsafe {
        unsafe extern "C" {
            static mut __libc_argc: c_int;
            static mut __libc_argv: *mut *mut c_char;
        }

        c_main(__libc_argc, __libc_argv);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
