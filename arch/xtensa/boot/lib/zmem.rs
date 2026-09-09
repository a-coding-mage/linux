// SPDX-License-Identifier: GPL-2.0
// Dependency declarations supplied by linux/zlib.h are expected externally.

use core::ffi::{c_int, c_uint, c_void};

extern "C" {
    static mut avail_ram: *mut c_void;
    static mut end_avail: *mut c_void;

    fn zlib_inflate_workspacesize() -> c_uint;
    fn zlib_inflateInit2(stream: *mut z_stream, window_bits: c_int) -> c_int;
    fn zlib_inflate(stream: *mut z_stream, flush: c_int) -> c_int;
    fn zlib_inflateEnd(stream: *mut z_stream) -> c_int;
}

// Supplied by linux/zlib.h.
#[repr(C)]
pub struct z_stream {
    pub next_in: *mut u8,
    pub avail_in: c_uint,
    pub total_in: c_uint,
    pub next_out: *mut u8,
    pub avail_out: c_uint,
    pub total_out: c_uint,
    pub msg: *mut u8,
    pub state: *mut c_void,
    pub zalloc: Option<unsafe extern "C" fn(*mut c_void, c_uint, c_uint) -> *mut c_void>,
    pub zfree: Option<unsafe extern "C" fn(*mut c_void, *mut c_void)>,
    pub opaque: *mut c_void,
    pub data_type: c_int,
    pub adler: c_uint,
    pub reserved: c_uint,
    pub workspace: *mut c_void,
}

const HEAD_CRC: u8 = 2;
const EXTRA_FIELD: u8 = 4;
const ORIG_NAME: u8 = 8;
const COMMENT: u8 = 0x10;
const RESERVED: u8 = 0xe0;

const DEFLATED: u8 = 8;
const MAX_WBITS: c_int = 15;
const Z_OK: c_int = 0;
const Z_STREAM_END: c_int = 1;
const Z_FINISH: c_int = 4;

unsafe fn exit() -> ! {
    loop {}
}

unsafe fn zalloc(size: c_uint) -> *mut c_void {
    let p = avail_ram;

    let size = (size.wrapping_add(7)) & !7;
    avail_ram = (avail_ram as *mut u8).add(size as usize) as *mut c_void;
    if (avail_ram as *mut u8) > (end_avail as *mut u8) {
        //puts("oops... out of memory\n");
        //pause();
        exit();
    }
    p
}

pub unsafe fn gunzip(dst: *mut c_void, dstlen: c_int, src: *mut u8, lenp: *mut c_int) {
    let mut s: z_stream = core::mem::zeroed();
    let r: c_int;
    let mut i: usize;
    let flags: u8;

    /* skip header */
    i = 10;
    flags = *src.add(3);
    if *src.add(2) != DEFLATED || (flags & RESERVED) != 0 {
        //puts("bad gzipped data\n");
        exit();
    }
    if (flags & EXTRA_FIELD) != 0 {
        i = 12 + (*src.add(10) as usize) + ((*src.add(11) as usize) << 8);
    }
    if (flags & ORIG_NAME) != 0 {
        while *src.add(i) != 0 {
            i += 1;
        }
        i += 1;
    }
    if (flags & COMMENT) != 0 {
        while *src.add(i) != 0 {
            i += 1;
        }
        i += 1;
    }
    if (flags & HEAD_CRC) != 0 {
        i += 2;
    }
    if i >= *lenp as usize {
        //puts("gunzip: ran out of data in header\n");
        exit();
    }

    s.workspace = zalloc(zlib_inflate_workspacesize());
    r = zlib_inflateInit2(&mut s, -MAX_WBITS);
    if r != Z_OK {
        //puts("inflateInit2 returned "); puthex(r); puts("\n");
        exit();
    }
    s.next_in = src.add(i);
    s.avail_in = (*lenp as usize - i) as c_uint;
    s.next_out = dst as *mut u8;
    s.avail_out = dstlen as c_uint;
    r = zlib_inflate(&mut s, Z_FINISH);
    if r != Z_OK && r != Z_STREAM_END {
        //puts("inflate returned "); puthex(r); puts("\n");
        exit();
    }
    *lenp = s.next_out.offset_from(dst as *mut u8) as c_int;
    zlib_inflateEnd(&mut s);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
