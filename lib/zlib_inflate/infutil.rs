use core::ffi::c_void;

/* Dependencies supplied by the zlib/kernel environment. */
pub type u8 = core::ffi::c_uchar;
pub type u32 = core::ffi::c_uint;

#[repr(C)]
pub struct z_stream_s {
    pub next_in: *const u8,
    pub avail_in: u32,
    pub next_out: *mut u8,
    pub avail_out: u32,
    pub workspace: *mut c_void,
}

extern "C" {
    fn kmalloc(size: usize, flags: u32) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn zlib_inflate_workspacesize() -> usize;
    fn zlib_inflateInit2(strm: *mut z_stream_s, window_bits: i32) -> i32;
    fn zlib_inflate(strm: *mut z_stream_s, flush: i32) -> i32;
    fn zlib_inflateEnd(strm: *mut z_stream_s) -> i32;
}

/* Values supplied by linux/zutil.h and linux/errno.h. */
const GFP_KERNEL: u32 = 0;
const MAX_WBITS: i32 = 15;
const Z_OK: i32 = 0;
const Z_STREAM_END: i32 = 1;
const Z_FINISH: i32 = 4;
const ENOMEM: i32 = 12;
const EINVAL: i32 = 22;

/* Utility function: initialize zlib, unpack binary blob, clean up zlib,
 * return len or negative error code.
 */
pub unsafe fn zlib_inflate_blob(
    gunzip_buf: *mut c_void,
    sz: u32,
    buf: *const c_void,
    len: u32,
) -> i32 {
    let zbuf = buf as *const u8;
    let strm: *mut z_stream_s;
    let mut rc: i32;

    rc = -ENOMEM;
    strm = kmalloc(core::mem::size_of::<z_stream_s>(), GFP_KERNEL) as *mut z_stream_s;
    if strm.is_null() {
        return rc;
    }
    (*strm).workspace = kmalloc(zlib_inflate_workspacesize(), GFP_KERNEL);
    if (*strm).workspace.is_null() {
        kfree(strm as *mut c_void);
        return rc;
    }

    /* gzip header (1f,8b,08... 10 bytes total + possible asciz filename)
     * expected to be stripped from input
     */
    (*strm).next_in = zbuf;
    (*strm).avail_in = len;
    (*strm).next_out = gunzip_buf as *mut u8;
    (*strm).avail_out = sz;

    rc = zlib_inflateInit2(strm, -MAX_WBITS);
    if rc == Z_OK {
        rc = zlib_inflate(strm, Z_FINISH);
        /* after Z_FINISH, only Z_STREAM_END is "we unpacked it all" */
        if rc == Z_STREAM_END {
            rc = (sz - (*strm).avail_out) as i32;
        } else {
            rc = -EINVAL;
        }
        zlib_inflateEnd(strm);
    } else {
        rc = -EINVAL;
    }

    kfree((*strm).workspace);
    kfree(strm as *mut c_void);
    rc /* returns Z_OK (0) if successful */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
