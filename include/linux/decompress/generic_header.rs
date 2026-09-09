/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_uchar, c_void};

pub type DecompressFn = Option<
    unsafe extern "C" fn(
        inbuf: *mut c_uchar,
        len: c_long,
        fill: Option<unsafe extern "C" fn(*mut c_void, c_ulong) -> c_long>,
        flush: Option<unsafe extern "C" fn(*mut c_void, c_ulong) -> c_long>,
        outbuf: *mut c_uchar,
        posp: *mut c_long,
        error: Option<unsafe extern "C" fn(*mut c_char)>,
    ) -> c_int,
>;

/* inbuf   - input buffer
 * len     - len of pre-read data in inbuf
 * fill    - function to fill inbuf when empty
 * flush   - function to write out outbuf
 * outbuf  - output buffer
 * posp    - if non-null, input position (number of bytes read) will be
 *           returned here
 *
 * If len != 0, inbuf should contain all the necessary input data, and fill
 * should be NULL
 * If len = 0, inbuf can be NULL, in which case the decompressor will allocate
 * the input buffer.  If inbuf != NULL it must be at least XXX_IOBUF_SIZE bytes.
 * fill will be called (repeatedly...) to read data, at most XXX_IOBUF_SIZE
 * bytes should be read per call.  Replace XXX with the appropriate decompressor
 * name, i.e. LZMA_IOBUF_SIZE.
 *
 * If flush = NULL, outbuf must be large enough to buffer all the expected
 * output.  If flush != NULL, the output buffer will be allocated by the
 * decompressor (outbuf = NULL), and the flush function will be called to
 * flush the output buffer at the appropriate time (decompressor and stream
 * dependent).
 */

/* Utility routine to detect the decompression method */
unsafe extern "C" {
    pub fn decompress_method(inbuf: *const c_uchar, len: c_long, name: *mut *const c_char)
        -> DecompressFn;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
