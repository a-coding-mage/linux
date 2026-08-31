// SPDX-License-Identifier: GPL-2.0-only
/*
 * SPI testing utility (using spidev driver)
 *
 * Copyright (c) 2007  MontaVista Software, Inc.
 * Copyright (c) 2007  Anton Vorontsov <avorontsov@ru.mvista.com>
 *
 * Cross-compile with cross-gcc -I/path/to/cross-kernel/include
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};

type size_t = usize;
type ssize_t = isize;
type uint8_t = u8;
type uint16_t = u16;
type uint32_t = u32;
type uint64_t = u64;
type time_t = c_long;

const O_RDONLY: c_int = 0;
const O_WRONLY: c_int = 1;
const O_RDWR: c_int = 2;
const O_CREAT: c_int = 0o100;
const O_TRUNC: c_int = 0o1000;
const CLOCK_MONOTONIC: c_int = 1;

const SPI_CPHA: uint32_t = 0x01;
const SPI_CPOL: uint32_t = 0x02;
const SPI_LOOP: uint32_t = 0x20;
const SPI_NO_CS: uint32_t = 0x40;
const SPI_READY: uint32_t = 0x80;
const SPI_LSB_FIRST: uint32_t = 0x08;
const SPI_CS_HIGH: uint32_t = 0x04;
const SPI_3WIRE: uint32_t = 0x10;
const SPI_TX_DUAL: uint32_t = 0x100;
const SPI_TX_QUAD: uint32_t = 0x200;
const SPI_RX_DUAL: uint32_t = 0x400;
const SPI_RX_QUAD: uint32_t = 0x800;
const SPI_TX_OCTAL: uint32_t = 0x2000;
const SPI_RX_OCTAL: uint32_t = 0x4000;
const SPI_3WIRE_HIZ: uint32_t = 0x8000;
const SPI_RX_CPHA_FLIP: uint32_t = 0x10000;
const SPI_MOSI_IDLE_LOW: uint32_t = 0x20000;

const SPI_IOC_WR_MODE32: c_ulong = 0x40046b05;
const SPI_IOC_RD_MODE32: c_ulong = 0x80046b05;
const SPI_IOC_WR_BITS_PER_WORD: c_ulong = 0x40016b03;
const SPI_IOC_RD_BITS_PER_WORD: c_ulong = 0x80016b03;
const SPI_IOC_WR_MAX_SPEED_HZ: c_ulong = 0x40046b04;
const SPI_IOC_RD_MAX_SPEED_HZ: c_ulong = 0x80046b04;
const SPI_MSGSIZE_1: c_ulong = 32;
const SPI_IOC_MESSAGE_1: c_ulong = 0x40206b00;

#[repr(C)]
struct option {
    name: *const c_char,
    has_arg: c_int,
    flag: *mut c_int,
    val: c_int,
}

#[repr(C)]
struct stat {
    st_dev: c_ulong,
    st_ino: c_ulong,
    st_nlink: c_ulong,
    st_mode: c_uint,
    st_uid: c_uint,
    st_gid: c_uint,
    __pad0: c_int,
    st_rdev: c_ulong,
    st_size: c_long,
    st_blksize: c_long,
    st_blocks: c_long,
    st_atime: time_t,
    st_atime_nsec: c_long,
    st_mtime: time_t,
    st_mtime_nsec: c_long,
    st_ctime: time_t,
    st_ctime_nsec: c_long,
    __unused: [c_long; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct timespec {
    tv_sec: time_t,
    tv_nsec: c_long,
}

#[repr(C)]
struct spi_ioc_transfer {
    tx_buf: u64,
    rx_buf: u64,
    len: uint32_t,
    speed_hz: uint32_t,
    delay_usecs: uint16_t,
    bits_per_word: uint8_t,
    cs_change: uint8_t,
    tx_nbits: uint8_t,
    rx_nbits: uint8_t,
    word_delay_usecs: uint8_t,
    pad: uint8_t,
}

unsafe extern "C" {
    static mut errno: c_int;
    static mut optarg: *mut c_char;

    fn perror(s: *const c_char);
    fn printf(format: *const c_char, ...) -> c_int;
    fn puts(s: *const c_char) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    static mut stderr: *mut c_void;
    fn abort() -> !;
    fn exit(status: c_int) -> !;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn stat(pathname: *const c_char, statbuf: *mut stat) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn getopt_long(
        argc: c_int,
        argv: *mut *mut c_char,
        optstring: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
    ) -> c_int;
    fn atoi(nptr: *const c_char) -> c_int;
    fn random() -> c_long;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> usize {
    N
}

unsafe fn pabort(s: *const c_char) -> ! {
    if errno != 0 {
        perror(s);
    } else {
        printf(cstr!("%s\n"), s);
    }

    abort();
}

static mut device: *const c_char = cstr!("/dev/spidev1.1");
static mut mode: uint32_t = 0;
static mut bits: uint8_t = 8;
static mut input_file: *mut c_char = core::ptr::null_mut();
static mut output_file: *mut c_char = core::ptr::null_mut();
static mut speed: uint32_t = 500000;
static mut delay: uint16_t = 0;
static mut word_delay: uint16_t = 0;
static mut verbose: c_int = 0;
static mut transfer_size: c_int = 0;
static mut iterations: c_int = 0;
static mut interval: c_int = 5; /* interval in seconds for showing transfer rate */

static mut default_tx: [uint8_t; 32] = [
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0x40, 0x00, 0x00, 0x00, 0x00, 0x95,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xF0, 0x0D,
];

static mut default_rx: [uint8_t; 32] = [0; 32];
static mut input_tx: *mut c_char = core::ptr::null_mut();

unsafe fn hex_dump(src: *const c_void, mut length: size_t, line_size: size_t, prefix: *mut c_char) {
    let mut i: c_int = 0;
    let mut address = src as *const u8;
    let mut line = address;
    let mut c: u8;

    printf(cstr!("%s | "), prefix);
    while length > 0 {
        length -= 1;
        printf(cstr!("%02X "), *address as c_int);
        address = address.add(1);
        i += 1;
        if i as size_t % line_size == 0 || (length == 0 && i as size_t % line_size != 0) {
            if length == 0 {
                while {
                    let r = i as size_t % line_size;
                    i += 1;
                    r != 0
                } {
                    printf(cstr!("__ "));
                }
            }
            printf(cstr!(" |"));
            while line < address {
                c = *line;
                line = line.add(1);
                printf(cstr!("%c"), if c < 32 || c > 126 { b'.' as c_int } else { c as c_int });
            }
            printf(cstr!("|\n"));
            if length > 0 {
                printf(cstr!("%s | "), prefix);
            }
        }
    }
}

/*
 *  Unescape - process hexadecimal escape character
 *      converts shell input "\x23" -> 0x23
 */
unsafe fn unescape(_dst: *mut c_char, _src: *mut c_char, _len: size_t) -> c_int {
    let mut ret: c_int = 0;
    let mut match_: c_int;
    let mut src = _src;
    let mut dst = _dst;
    let mut ch: c_uint = 0;

    while *src != 0 {
        if *src == b'\\' as c_char && *src.add(1) == b'x' as c_char {
            match_ = sscanf(src.add(2), cstr!("%2x"), &mut ch as *mut c_uint);
            if match_ == 0 {
                pabort(cstr!("malformed input string"));
            }

            src = src.add(4);
            *dst = ch as u8 as c_char;
            dst = dst.add(1);
        } else {
            *dst = *src;
            dst = dst.add(1);
            src = src.add(1);
        }
        ret += 1;
    }
    ret
}

unsafe fn transfer(fd: c_int, tx: *const uint8_t, rx: *const uint8_t, len: size_t) {
    let mut ret: c_int;
    let out_fd: c_int;
    let mut tr = spi_ioc_transfer {
        tx_buf: tx as c_ulong as u64,
        rx_buf: rx as c_ulong as u64,
        len: len as uint32_t,
        delay_usecs: delay,
        word_delay_usecs: word_delay as uint8_t,
        speed_hz: speed,
        bits_per_word: bits,
        cs_change: 0,
        tx_nbits: 0,
        rx_nbits: 0,
        pad: 0,
    };

    if mode & SPI_TX_OCTAL != 0 {
        tr.tx_nbits = 8;
    } else if mode & SPI_TX_QUAD != 0 {
        tr.tx_nbits = 4;
    } else if mode & SPI_TX_DUAL != 0 {
        tr.tx_nbits = 2;
    }
    if mode & SPI_RX_OCTAL != 0 {
        tr.rx_nbits = 8;
    } else if mode & SPI_RX_QUAD != 0 {
        tr.rx_nbits = 4;
    } else if mode & SPI_RX_DUAL != 0 {
        tr.rx_nbits = 2;
    }
    if !(mode & SPI_LOOP != 0) {
        if mode & (SPI_TX_OCTAL | SPI_TX_QUAD | SPI_TX_DUAL) != 0 {
            tr.rx_buf = 0;
        } else if mode & (SPI_RX_OCTAL | SPI_RX_QUAD | SPI_RX_DUAL) != 0 {
            tr.tx_buf = 0;
        }
    }

    ret = ioctl(fd, SPI_IOC_MESSAGE_1, &mut tr as *mut spi_ioc_transfer);
    if ret < 1 {
        pabort(cstr!("can't send spi message"));
    }

    if verbose != 0 {
        hex_dump(tx as *const c_void, len, 32, cstr!("TX") as *mut c_char);
    }

    if !output_file.is_null() {
        out_fd = open(output_file, O_WRONLY | O_CREAT | O_TRUNC, 0o666);
        if out_fd < 0 {
            pabort(cstr!("could not open output file"));
        }

        ret = write(out_fd, rx as *const c_void, len) as c_int;
        if ret as size_t != len {
            pabort(cstr!("not all bytes written to output file"));
        }

        close(out_fd);
    }

    if verbose != 0 {
        hex_dump(rx as *const c_void, len, 32, cstr!("RX") as *mut c_char);
    }
}

unsafe fn print_usage(prog: *const c_char) -> ! {
    printf(cstr!("Usage: %s [-2348CDFHILMNORSZbdilopsvw]\n"), prog);
    puts(cstr!("general device settings:\n  -D --device         device to use (default /dev/spidev1.1)\n  -s --speed          max speed (Hz)\n  -d --delay          delay (usec)\n  -w --word-delay     word delay (usec)\n  -l --loop           loopback\nspi mode:\n  -H --cpha           clock phase\n  -O --cpol           clock polarity\n  -F --rx-cpha-flip   flip CPHA on Rx only xfer\nnumber of wires for transmission:\n  -2 --dual           dual transfer\n  -4 --quad           quad transfer\n  -8 --octal          octal transfer\n  -3 --3wire          SI/SO signals shared\n  -Z --3wire-hiz      high impedance turnaround\ndata:\n  -i --input          input data from a file (e.g. \"test.bin\")\n  -o --output         output data to a file (e.g. \"results.bin\")\n  -p                  Send data (e.g. \"1234\\xde\\xad\")\n  -S --size           transfer size\n  -I --iter           iterations\nadditional parameters:\n  -b --bpw            bits per word\n  -L --lsb            least significant bit first\n  -C --cs-high        chip select active high\n  -N --no-cs          no chip select\n  -R --ready          slave pulls low to pause\n  -M --mosi-idle-low  leave mosi line low when idle\nmisc:\n  -v --verbose        Verbose (show tx buffer)\n"));
    exit(1);
}

unsafe fn parse_opts(argc: c_int, argv: *mut *mut c_char) {
    loop {
        static lopts: [option; 26] = [
            option { name: cstr!("device"),        has_arg: 1, flag: core::ptr::null_mut(), val: b'D' as c_int },
            option { name: cstr!("speed"),         has_arg: 1, flag: core::ptr::null_mut(), val: b's' as c_int },
            option { name: cstr!("delay"),         has_arg: 1, flag: core::ptr::null_mut(), val: b'd' as c_int },
            option { name: cstr!("word-delay"),    has_arg: 1, flag: core::ptr::null_mut(), val: b'w' as c_int },
            option { name: cstr!("loop"),          has_arg: 0, flag: core::ptr::null_mut(), val: b'l' as c_int },
            option { name: cstr!("cpha"),          has_arg: 0, flag: core::ptr::null_mut(), val: b'H' as c_int },
            option { name: cstr!("cpol"),          has_arg: 0, flag: core::ptr::null_mut(), val: b'O' as c_int },
            option { name: cstr!("rx-cpha-flip"),  has_arg: 0, flag: core::ptr::null_mut(), val: b'F' as c_int },
            option { name: cstr!("dual"),          has_arg: 0, flag: core::ptr::null_mut(), val: b'2' as c_int },
            option { name: cstr!("quad"),          has_arg: 0, flag: core::ptr::null_mut(), val: b'4' as c_int },
            option { name: cstr!("octal"),         has_arg: 0, flag: core::ptr::null_mut(), val: b'8' as c_int },
            option { name: cstr!("3wire"),         has_arg: 0, flag: core::ptr::null_mut(), val: b'3' as c_int },
            option { name: cstr!("3wire-hiz"),     has_arg: 0, flag: core::ptr::null_mut(), val: b'Z' as c_int },
            option { name: cstr!("input"),         has_arg: 1, flag: core::ptr::null_mut(), val: b'i' as c_int },
            option { name: cstr!("output"),        has_arg: 1, flag: core::ptr::null_mut(), val: b'o' as c_int },
            option { name: cstr!("size"),          has_arg: 1, flag: core::ptr::null_mut(), val: b'S' as c_int },
            option { name: cstr!("iter"),          has_arg: 1, flag: core::ptr::null_mut(), val: b'I' as c_int },
            option { name: cstr!("bpw"),           has_arg: 1, flag: core::ptr::null_mut(), val: b'b' as c_int },
            option { name: cstr!("lsb"),           has_arg: 0, flag: core::ptr::null_mut(), val: b'L' as c_int },
            option { name: cstr!("cs-high"),       has_arg: 0, flag: core::ptr::null_mut(), val: b'C' as c_int },
            option { name: cstr!("no-cs"),         has_arg: 0, flag: core::ptr::null_mut(), val: b'N' as c_int },
            option { name: cstr!("ready"),         has_arg: 0, flag: core::ptr::null_mut(), val: b'R' as c_int },
            option { name: cstr!("mosi-idle-low"), has_arg: 0, flag: core::ptr::null_mut(), val: b'M' as c_int },
            option { name: cstr!("verbose"),       has_arg: 0, flag: core::ptr::null_mut(), val: b'v' as c_int },
            option { name: core::ptr::null(),      has_arg: 0, flag: core::ptr::null_mut(), val: 0 },
            option { name: core::ptr::null(),      has_arg: 0, flag: core::ptr::null_mut(), val: 0 },
        ];
        let c: c_int;

        c = getopt_long(argc, argv, cstr!("D:s:d:w:b:i:o:lHOLC3ZFMNR248p:vS:I:"), lopts.as_ptr(), core::ptr::null_mut());

        if c == -1 {
            break;
        }

        match c {
            x if x == b'D' as c_int => device = optarg,
            x if x == b's' as c_int => speed = atoi(optarg) as uint32_t,
            x if x == b'd' as c_int => delay = atoi(optarg) as uint16_t,
            x if x == b'w' as c_int => word_delay = atoi(optarg) as uint16_t,
            x if x == b'b' as c_int => bits = atoi(optarg) as uint8_t,
            x if x == b'i' as c_int => input_file = optarg,
            x if x == b'o' as c_int => output_file = optarg,
            x if x == b'l' as c_int => mode |= SPI_LOOP,
            x if x == b'H' as c_int => mode |= SPI_CPHA,
            x if x == b'O' as c_int => mode |= SPI_CPOL,
            x if x == b'L' as c_int => mode |= SPI_LSB_FIRST,
            x if x == b'C' as c_int => mode |= SPI_CS_HIGH,
            x if x == b'3' as c_int => mode |= SPI_3WIRE,
            x if x == b'Z' as c_int => mode |= SPI_3WIRE_HIZ,
            x if x == b'F' as c_int => mode |= SPI_RX_CPHA_FLIP,
            x if x == b'M' as c_int => mode |= SPI_MOSI_IDLE_LOW,
            x if x == b'N' as c_int => mode |= SPI_NO_CS,
            x if x == b'v' as c_int => verbose = 1,
            x if x == b'R' as c_int => mode |= SPI_READY,
            x if x == b'p' as c_int => input_tx = optarg,
            x if x == b'2' as c_int => mode |= SPI_TX_DUAL,
            x if x == b'4' as c_int => mode |= SPI_TX_QUAD,
            x if x == b'8' as c_int => mode |= SPI_TX_OCTAL,
            x if x == b'S' as c_int => transfer_size = atoi(optarg),
            x if x == b'I' as c_int => iterations = atoi(optarg),
            _ => print_usage(*argv),
        }
    }
    if mode & SPI_LOOP != 0 {
        if mode & SPI_TX_DUAL != 0 {
            mode |= SPI_RX_DUAL;
        }
        if mode & SPI_TX_QUAD != 0 {
            mode |= SPI_RX_QUAD;
        }
        if mode & SPI_TX_OCTAL != 0 {
            mode |= SPI_RX_OCTAL;
        }
    }
}

unsafe fn transfer_escaped_string(fd: c_int, str_: *mut c_char) {
    let mut size = strlen(str_);
    let tx: *mut uint8_t;
    let rx: *mut uint8_t;

    tx = malloc(size) as *mut uint8_t;
    if tx.is_null() {
        pabort(cstr!("can't allocate tx buffer"));
    }

    rx = malloc(size) as *mut uint8_t;
    if rx.is_null() {
        pabort(cstr!("can't allocate rx buffer"));
    }

    size = unescape(tx as *mut c_char, str_, size) as size_t;
    transfer(fd, tx, rx, size);
    free(rx as *mut c_void);
    free(tx as *mut c_void);
}

unsafe fn transfer_file(fd: c_int, filename: *mut c_char) {
    let mut bytes: ssize_t;
    let mut sb: stat = core::mem::zeroed();
    let tx_fd: c_int;
    let tx: *mut uint8_t;
    let rx: *mut uint8_t;

    if stat(filename, &mut sb as *mut stat) == -1 {
        pabort(cstr!("can't stat input file"));
    }

    tx_fd = open(filename, O_RDONLY);
    if tx_fd < 0 {
        pabort(cstr!("can't open input file"));
    }

    tx = malloc(sb.st_size as size_t) as *mut uint8_t;
    if tx.is_null() {
        pabort(cstr!("can't allocate tx buffer"));
    }

    rx = malloc(sb.st_size as size_t) as *mut uint8_t;
    if rx.is_null() {
        pabort(cstr!("can't allocate rx buffer"));
    }

    bytes = read(tx_fd, tx as *mut c_void, sb.st_size as size_t);
    if bytes != sb.st_size as ssize_t {
        pabort(cstr!("failed to read input file"));
    }

    transfer(fd, tx, rx, sb.st_size as size_t);
    free(rx as *mut c_void);
    free(tx as *mut c_void);
    close(tx_fd);
}

static mut _read_count: uint64_t = 0;
static mut _write_count: uint64_t = 0;

unsafe fn show_transfer_rate() {
    static mut prev_read_count: uint64_t = 0;
    static mut prev_write_count: uint64_t = 0;
    let rx_rate: c_double;
    let tx_rate: c_double;

    rx_rate = ((_read_count - prev_read_count) * 8) as c_double / (interval as c_double * 1000.0);
    tx_rate = ((_write_count - prev_write_count) * 8) as c_double / (interval as c_double * 1000.0);

    printf(cstr!("rate: tx %.1fkbps, rx %.1fkbps\n"), rx_rate, tx_rate);

    prev_read_count = _read_count;
    prev_write_count = _write_count;
}

unsafe fn transfer_buf(fd: c_int, len: c_int) {
    let tx: *mut uint8_t;
    let rx: *mut uint8_t;
    let mut i: c_int;

    tx = malloc(len as size_t) as *mut uint8_t;
    if tx.is_null() {
        pabort(cstr!("can't allocate tx buffer"));
    }
    i = 0;
    while i < len {
        *tx.add(i as size_t) = random() as uint8_t;
        i += 1;
    }

    rx = malloc(len as size_t) as *mut uint8_t;
    if rx.is_null() {
        pabort(cstr!("can't allocate rx buffer"));
    }

    transfer(fd, tx, rx, len as size_t);

    _write_count += len as uint64_t;
    _read_count += len as uint64_t;

    if mode & SPI_LOOP != 0 {
        if memcmp(tx as *const c_void, rx as *const c_void, len as size_t) != 0 {
            fprintf(stderr, cstr!("transfer error !\n"));
            hex_dump(tx as *const c_void, len as size_t, 32, cstr!("TX") as *mut c_char);
            hex_dump(rx as *const c_void, len as size_t, 32, cstr!("RX") as *mut c_char);
            exit(1);
        }
    }

    free(rx as *mut c_void);
    free(tx as *mut c_void);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut ret: c_int = 0;
    let fd: c_int;
    let request: uint32_t;

    parse_opts(argc, argv);

    if !input_tx.is_null() && !input_file.is_null() {
        pabort(cstr!("only one of -p and --input may be selected"));
    }

    fd = open(device, O_RDWR);
    if fd < 0 {
        pabort(cstr!("can't open device"));
    }

    /*
     * spi mode
     */
    /* WR is make a request to assign 'mode' */
    request = mode;
    ret = ioctl(fd, SPI_IOC_WR_MODE32, &mut mode as *mut uint32_t);
    if ret == -1 {
        pabort(cstr!("can't set spi mode"));
    }

    /* RD is read what mode the device actually is in */
    ret = ioctl(fd, SPI_IOC_RD_MODE32, &mut mode as *mut uint32_t);
    if ret == -1 {
        pabort(cstr!("can't get spi mode"));
    }
    /* Drivers can reject some mode bits without returning an error.
     * Read the current value to identify what mode it is in, and if it
     * differs from the requested mode, warn the user.
     */
    if request != mode {
        printf(cstr!("WARNING device does not support requested mode 0x%x\n"), request);
    }

    /*
     * bits per word
     */
    ret = ioctl(fd, SPI_IOC_WR_BITS_PER_WORD, &mut bits as *mut uint8_t);
    if ret == -1 {
        pabort(cstr!("can't set bits per word"));
    }

    ret = ioctl(fd, SPI_IOC_RD_BITS_PER_WORD, &mut bits as *mut uint8_t);
    if ret == -1 {
        pabort(cstr!("can't get bits per word"));
    }

    /*
     * max speed hz
     */
    ret = ioctl(fd, SPI_IOC_WR_MAX_SPEED_HZ, &mut speed as *mut uint32_t);
    if ret == -1 {
        pabort(cstr!("can't set max speed hz"));
    }

    ret = ioctl(fd, SPI_IOC_RD_MAX_SPEED_HZ, &mut speed as *mut uint32_t);
    if ret == -1 {
        pabort(cstr!("can't get max speed hz"));
    }

    printf(cstr!("spi mode: 0x%x\n"), mode);
    printf(cstr!("bits per word: %u\n"), bits as c_uint);
    printf(cstr!("max speed: %u Hz (%u kHz)\n"), speed, speed / 1000);

    if !input_tx.is_null() {
        transfer_escaped_string(fd, input_tx);
    } else if !input_file.is_null() {
        transfer_file(fd, input_file);
    } else if transfer_size != 0 {
        let mut last_stat: timespec = core::mem::zeroed();

        clock_gettime(CLOCK_MONOTONIC, &mut last_stat as *mut timespec);

        while {
            let old = iterations;
            iterations -= 1;
            old > 0
        } {
            let mut current: timespec = core::mem::zeroed();

            transfer_buf(fd, transfer_size);

            clock_gettime(CLOCK_MONOTONIC, &mut current as *mut timespec);
            if current.tv_sec - last_stat.tv_sec > interval as time_t {
                show_transfer_rate();
                last_stat = current;
            }
        }
        printf(cstr!("total: tx %.1fKB, rx %.1fKB\n"),
               _write_count as c_double / 1024.0, _read_count as c_double / 1024.0);
    } else {
        transfer(fd, default_tx.as_ptr(), default_rx.as_ptr(), ARRAY_SIZE(&default_tx));
    }

    close(fd);

    ret
}
