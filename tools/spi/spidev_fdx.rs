// SPDX-License-Identifier: GPL-2.0
//
// Translated from C. Original dependencies included:
// stdio.h, unistd.h, stdlib.h, fcntl.h, string.h, sys/ioctl.h,
// sys/types.h, sys/stat.h, linux/types.h, linux/spi/spidev.h.

use core::ffi::{c_char, c_int, c_ulong, c_void};

type __u8 = u8;
type __u32 = u32;

// Provided by linux/spi/spidev.h in the original source.
#[repr(C)]
pub struct spi_ioc_transfer {
    pub tx_buf: u64,
    pub rx_buf: u64,
    pub len: u32,
    pub speed_hz: u32,
    pub delay_usecs: u16,
    pub bits_per_word: u8,
    pub cs_change: u8,
    pub tx_nbits: u8,
    pub rx_nbits: u8,
    pub word_delay_usecs: u8,
    pub pad: u8,
}

unsafe extern "C" {
    static mut optarg: *mut c_char;
    static mut optind: c_int;
    static mut stderr: *mut FILE;

    fn atoi(nptr: *const c_char) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn getopt(argc: c_int, argv: *const *mut c_char, optstring: *const c_char) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn perror(s: *const c_char);
    fn printf(format: *const c_char, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

const EOF: c_int = -1;
const O_RDWR: c_int = 0o2;

// ioctl request constants and macro supplied by linux/spi/spidev.h.
const SPI_IOC_RD_MODE32: c_ulong = 0x8004_6b05;
const SPI_IOC_RD_LSB_FIRST: c_ulong = 0x8001_6b02;
const SPI_IOC_RD_BITS_PER_WORD: c_ulong = 0x8001_6b03;
const SPI_IOC_RD_MAX_SPEED_HZ: c_ulong = 0x8004_6b04;

const IOC_NRBITS: c_ulong = 8;
const IOC_TYPEBITS: c_ulong = 8;
const IOC_SIZEBITS: c_ulong = 14;
const IOC_NRSHIFT: c_ulong = 0;
const IOC_TYPESHIFT: c_ulong = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: c_ulong = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: c_ulong = IOC_SIZESHIFT + IOC_SIZEBITS;
const IOC_WRITE: c_ulong = 1;

const fn _IOC(dir: c_ulong, type_: c_ulong, nr: c_ulong, size: c_ulong) -> c_ulong {
    (dir << IOC_DIRSHIFT) | (type_ << IOC_TYPESHIFT) | (nr << IOC_NRSHIFT) | (size << IOC_SIZESHIFT)
}

const fn SPI_MSGSIZE(n: c_ulong) -> c_ulong {
    let size = n * core::mem::size_of::<spi_ioc_transfer>() as c_ulong;
    if size < (1 << IOC_SIZEBITS) {
        size
    } else {
        0
    }
}

const fn SPI_IOC_MESSAGE(n: c_ulong) -> c_ulong {
    _IOC(IOC_WRITE, b'k' as c_ulong, 0, SPI_MSGSIZE(n))
}

static mut verbose: c_int = 0;

unsafe fn do_read(fd: c_int, mut len: c_int) {
    let mut buf = [0u8; 32];
    let mut bp: *mut u8;
    let mut status: c_int;

    /* read at least 2 bytes, no more than 32 */
    if len < 2 {
        len = 2;
    } else if (len as usize) > core::mem::size_of_val(&buf) {
        len = core::mem::size_of_val(&buf) as c_int;
    }
    memset(
        buf.as_mut_ptr() as *mut c_void,
        0,
        core::mem::size_of_val(&buf),
    );

    status = read(fd, buf.as_mut_ptr() as *mut c_void, len as usize) as c_int;
    if status < 0 {
        perror(b"read\0".as_ptr() as *const c_char);
        return;
    }
    if status != len {
        fprintf(stderr, b"short read\n\0".as_ptr() as *const c_char);
        return;
    }

    printf(
        b"read(%2d, %2d): %02x %02x,\0".as_ptr() as *const c_char,
        len,
        status,
        buf[0] as c_int,
        buf[1] as c_int,
    );
    status -= 2;
    bp = buf.as_mut_ptr().add(2);
    while {
        let old = status;
        status -= 1;
        old > 0
    } {
        printf(b" %02x\0".as_ptr() as *const c_char, *bp as c_int);
        bp = bp.add(1);
    }
    printf(b"\n\0".as_ptr() as *const c_char);
}

unsafe fn do_msg(fd: c_int, mut len: c_int) {
    let mut xfer: [spi_ioc_transfer; 2] = core::mem::zeroed();
    let mut buf = [0u8; 32];
    let mut bp: *mut u8;
    let mut status: c_int;

    memset(
        xfer.as_mut_ptr() as *mut c_void,
        0,
        core::mem::size_of_val(&xfer),
    );
    memset(
        buf.as_mut_ptr() as *mut c_void,
        0,
        core::mem::size_of_val(&buf),
    );

    if (len as usize) > core::mem::size_of_val(&buf) {
        len = core::mem::size_of_val(&buf) as c_int;
    }

    buf[0] = 0xaa;
    xfer[0].tx_buf = buf.as_mut_ptr() as c_ulong as u64;
    xfer[0].len = 1;

    xfer[1].rx_buf = buf.as_mut_ptr() as c_ulong as u64;
    xfer[1].len = len as u32;

    status = ioctl(fd, SPI_IOC_MESSAGE(2), xfer.as_mut_ptr());
    if status < 0 {
        perror(b"SPI_IOC_MESSAGE\0".as_ptr() as *const c_char);
        return;
    }

    printf(
        b"response(%2d, %2d): \0".as_ptr() as *const c_char,
        len,
        status,
    );
    bp = buf.as_mut_ptr();
    while len != 0 {
        printf(b" %02x\0".as_ptr() as *const c_char, *bp as c_int);
        bp = bp.add(1);
        len -= 1;
    }
    printf(b"\n\0".as_ptr() as *const c_char);
}

unsafe fn dumpstat(name: *const c_char, fd: c_int) {
    let mut lsb: __u8 = 0;
    let mut bits: __u8 = 0;
    let mut mode: __u32 = 0;
    let mut speed: __u32 = 0;

    if ioctl(fd, SPI_IOC_RD_MODE32, &mut mode) < 0 {
        perror(b"SPI rd_mode\0".as_ptr() as *const c_char);
        return;
    }
    if ioctl(fd, SPI_IOC_RD_LSB_FIRST, &mut lsb) < 0 {
        perror(b"SPI rd_lsb_fist\0".as_ptr() as *const c_char);
        return;
    }
    if ioctl(fd, SPI_IOC_RD_BITS_PER_WORD, &mut bits) < 0 {
        perror(b"SPI bits_per_word\0".as_ptr() as *const c_char);
        return;
    }
    if ioctl(fd, SPI_IOC_RD_MAX_SPEED_HZ, &mut speed) < 0 {
        perror(b"SPI max_speed_hz\0".as_ptr() as *const c_char);
        return;
    }

    printf(
        b"%s: spi mode 0x%x, %d bits %sper word, %u Hz max\n\0".as_ptr() as *const c_char,
        name,
        mode,
        bits as c_int,
        if lsb != 0 {
            b"(lsb first) \0".as_ptr() as *const c_char
        } else {
            b"\0".as_ptr() as *const c_char
        },
        speed,
    );
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut c: c_int;
    let mut readcount: c_int = 0;
    let mut msglen: c_int = 0;
    let fd: c_int;
    let name: *const c_char;

    loop {
        c = getopt(argc, argv, b"hm:r:v\0".as_ptr() as *const c_char);
        if c == EOF {
            break;
        }
        match c {
            x if x == b'm' as c_int => {
                msglen = atoi(optarg);
                if msglen < 0 {
                    return usage(argv);
                }
                continue;
            }
            x if x == b'r' as c_int => {
                readcount = atoi(optarg);
                if readcount < 0 {
                    return usage(argv);
                }
                continue;
            }
            x if x == b'v' as c_int => {
                verbose += 1;
                continue;
            }
            x if x == b'h' as c_int || x == b'?' as c_int => {
                return usage(argv);
            }
            _ => {}
        }
    }

    if (optind + 1) != argc {
        return usage(argv);
    }
    name = *argv.add(optind as usize);

    fd = open(name, O_RDWR);
    if fd < 0 {
        perror(b"open\0".as_ptr() as *const c_char);
        return 1;
    }

    dumpstat(name, fd);

    if msglen != 0 {
        do_msg(fd, msglen);
    }

    if readcount != 0 {
        do_read(fd, readcount);
    }

    close(fd);
    0
}

unsafe fn usage(argv: *mut *mut c_char) -> c_int {
    fprintf(
        stderr,
        b"usage: %s [-h] [-m N] [-r N] /dev/spidevB.D\n\0".as_ptr() as *const c_char,
        *argv,
    );
    1
}
