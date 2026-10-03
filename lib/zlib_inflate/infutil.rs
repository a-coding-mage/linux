/* Rust translation of lib/zlib_inflate/infutil.c and the Adler routine from
 * include/linux/zutil.h. Copyright (C) 1995-1998 Jean-loup Gailly.
 * Conditions of distribution and use: include/linux/zlib.h.
 * This is an altered source version.
 */

use core::ffi::{c_uint, c_ulong};

// Keep the native unsigned-long accumulators, NMAX reduction boundary, and
// sixteen-byte unroll. This is Rust algorithm code, not an inline C helper.
pub(crate) unsafe fn zlib_adler32(adler: c_ulong, mut buf: *const u8, mut len: c_uint) -> c_ulong {
    unsafe {
        let mut s1 = adler & 0xffff;
        let mut s2 = (adler >> 16) & 0xffff;
        if buf.is_null() {
            return 1;
        }
        while len != 0 {
            let mut k = len.min(5552);
            len -= k;
            while k >= 16 {
                macro_rules! do1 {
                    ($i:expr) => {{
                        s1 = s1.wrapping_add(*buf.add($i) as c_ulong);
                        s2 = s2.wrapping_add(s1);
                    }};
                }
                do1!(0);
                do1!(1);
                do1!(2);
                do1!(3);
                do1!(4);
                do1!(5);
                do1!(6);
                do1!(7);
                do1!(8);
                do1!(9);
                do1!(10);
                do1!(11);
                do1!(12);
                do1!(13);
                do1!(14);
                do1!(15);
                buf = buf.add(16);
                k -= 16;
            }
            while k != 0 {
                s1 = s1.wrapping_add(*buf as c_ulong);
                buf = buf.add(1);
                s2 = s2.wrapping_add(s1);
                k -= 1;
            }
            s1 %= 65521;
            s2 %= 65521;
        }
        (s2 << 16) | s1
    }
}

// infutil.c is not textually included in decompress_inflate.c's PREBOOT row.
// Non-PREBOOT integration supplies the native kmalloc/kfree ABI shim and
// native GFP_KERNEL/errno bindings; no kernel allocator is linked into boot.
#[cfg(not(PREBOOT))]
mod blob {
    use crate::bindings::*;
    use crate::zlib_inflate::{
        zlib_inflate, zlib_inflateEnd, zlib_inflateInit2, zlib_inflate_workspacesize,
    };
    use core::ffi::{c_int, c_uint, c_ulong, c_void};
    use core::mem;

    extern "C" {
        fn lupos_zlib_kmalloc(size: usize, flags: gfp_t) -> *mut c_void;
        fn lupos_zlib_kfree(pointer: *mut c_void);
    }

    #[no_mangle]
    pub(crate) unsafe extern "C" fn zlib_inflate_blob(
        gunzip_buf: *mut c_void,
        sz: c_uint,
        buf: *const c_void,
        len: c_uint,
    ) -> c_int {
        unsafe {
            let mut rc = -(ENOMEM as c_int);
            let strm = lupos_zlib_kmalloc(mem::size_of::<z_stream>(), GFP_KERNEL as gfp_t)
                .cast::<z_stream>();
            if strm.is_null() {
                return rc;
            }
            (*strm).workspace =
                lupos_zlib_kmalloc(zlib_inflate_workspacesize() as usize, GFP_KERNEL as gfp_t);
            if (*strm).workspace.is_null() {
                lupos_zlib_kfree(strm.cast());
                return rc;
            }
            (*strm).next_in = buf.cast();
            (*strm).avail_in = len as c_ulong;
            (*strm).next_out = gunzip_buf.cast();
            (*strm).avail_out = sz as c_ulong;
            rc = zlib_inflateInit2(strm, -(MAX_WBITS as c_int));
            if rc == Z_OK as c_int {
                rc = zlib_inflate(strm, Z_FINISH as c_int);
                if rc == Z_STREAM_END as c_int {
                    rc = (sz as c_ulong).wrapping_sub((*strm).avail_out) as c_int;
                } else {
                    rc = -(EINVAL as c_int);
                }
                zlib_inflateEnd(strm);
            } else {
                rc = -(EINVAL as c_int);
            }
            lupos_zlib_kfree((*strm).workspace);
            lupos_zlib_kfree(strm.cast());
            rc
        }
    }
}

#[cfg(not(PREBOOT))]
pub(crate) use blob::zlib_inflate_blob;
