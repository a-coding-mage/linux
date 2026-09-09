// SPDX-License-Identifier: GPL-2.0

/*
 * Important notes about in-place decompression
 *
 * At least on x86, the kernel is decompressed in place: the compressed data
 * is placed to the end of the output buffer, and the decompressor overwrites
 * most of the compressed data. There must be enough safety margin to
 * guarantee that the write position is always behind the read position.
 *
 * The safety margin for ZSTD with a 128 KB block size is calculated below.
 * Note that the margin with ZSTD is bigger than with GZIP or XZ!
 *
 * The worst case for in-place decompression is that the beginning of
 * the file is compressed extremely well, and the rest of the file is
 * uncompressible. Thus, we must look for worst-case expansion when the
 * compressor is encoding uncompressible data.
 *
 * The structure of the .zst file in case of a compressed kernel is as follows.
 * Maximum sizes (as bytes) of the fields are in parenthesis.
 *
 *    Frame Header: (18)
 *    Blocks: (N)
 *    Checksum: (4)
 *
 * The frame header and checksum overhead is at most 22 bytes.
 *
 * ZSTD stores the data in blocks. Each block has a header whose size is a 3
 * bytes. After the block header, there is up to 128 KB of payload. The
 * maximum uncompressed size of the payload is 128 KB. The minimum
 * uncompressed size of the payload is never less than the payload size
 * (excluding the block header).
 *
 * The assumption, that the uncompressed size of the payload is never
 * smaller than the payload itself, is valid only when talking about the
 * payload as a whole. It is possible that the payload has parts where the
 * decompressor consumes more input than it produces output. Calculating the
 * worst case for this would be tricky. Instead of trying to do that,
 * let's simply make sure that the decompressor never overwrites any bytes
 * of the payload which it is currently reading.
 *
 * Now we have enough information to calculate the safety margin. We need
 *   - 22 bytes for the .zst file format headers;
 *   - 3 bytes per every 128 KiB of uncompressed size (one block header per
 *     block); and
 *   - 128 KiB (biggest possible zstd block size) to make sure that the
 *     decompressor never overwrites anything from the block it is currently
 *     reading.
 *
 * We get the following formula:
 *
 *    safety_margin = 22 + uncompressed_size * 3 / 131072 + 131072
 *                 <= 22 + (uncompressed_size >> 15) + 131072
 */

/* External definitions supplied by the surrounding decompression sources. */
type u8 = u8;
type zstd_error_code = i32;
#[repr(C)] pub struct zstd_dctx { _private: [u8; 0] }
#[repr(C)] pub struct zstd_dstream { _private: [u8; 0] }
#[repr(C)] pub struct zstd_frame_header { pub windowSize: usize }
#[repr(C)] pub struct zstd_in_buffer { pub src: *const u8, pub size: usize, pub pos: usize }
#[repr(C)] pub struct zstd_out_buffer { pub dst: *mut u8, pub size: usize, pub pos: usize }

extern "C" {
    fn zstd_get_error_code(ret: usize) -> zstd_error_code;
    fn zstd_is_error(ret: usize) -> usize;
    fn zstd_dctx_workspace_bound() -> usize;
    fn zstd_init_dctx(wksp: *mut core::ffi::c_void, wksp_size: usize) -> *mut zstd_dctx;
    fn zstd_find_frame_compressed_size(src: *const u8, src_size: isize) -> usize;
    fn zstd_decompress_dctx(dctx: *mut zstd_dctx, dst: *mut u8, dst_size: isize, src: *const u8, src_size: isize) -> usize;
    fn zstd_get_frame_header(header: *mut zstd_frame_header, src: *const u8, src_size: usize) -> usize;
    fn zstd_dstream_workspace_bound(window_size: usize) -> usize;
    fn zstd_init_dstream(window_size: usize, wksp: *mut core::ffi::c_void, wksp_size: usize) -> *mut zstd_dstream;
    fn zstd_decompress_stream(dstream: *mut zstd_dstream, output: *mut zstd_out_buffer, input: *mut zstd_in_buffer) -> usize;
    fn large_malloc(size: usize) -> *mut core::ffi::c_void;
    fn large_free(ptr: *mut core::ffi::c_void);
}

const ZSTD_WINDOWLOG_MAX: usize = 27;
const ZSTD_WINDOWSIZE_MAX: usize = 1usize << ZSTD_WINDOWLOG_MAX;
const ZSTD_IOBUF_SIZE: usize = 1usize << 17;
const ZSTD_error_memory_allocation: zstd_error_code = 1;
const ZSTD_error_prefix_unknown: zstd_error_code = 10;
const ZSTD_error_dstSize_tooSmall: zstd_error_code = 70;
const ZSTD_error_corruption_detected: zstd_error_code = 7;
const ZSTD_error_checksum_wrong: zstd_error_code = 9;

unsafe fn handle_zstd_error(ret: usize, error: Option<unsafe extern "C" fn(*mut i8)>) -> i32 {
    let err = zstd_get_error_code(ret);
    if zstd_is_error(ret) == 0 { return 0; }
    match err {
        ZSTD_error_memory_allocation => error.unwrap()(b"ZSTD decompressor ran out of memory\0".as_ptr() as *mut i8),
        ZSTD_error_prefix_unknown => error.unwrap()(b"Input is not in the ZSTD format (wrong magic bytes)\0".as_ptr() as *mut i8),
        ZSTD_error_dstSize_tooSmall | ZSTD_error_corruption_detected | ZSTD_error_checksum_wrong => error.unwrap()(b"ZSTD-compressed data is corrupt\0".as_ptr() as *mut i8),
        _ => error.unwrap()(b"ZSTD-compressed data is probably corrupt\0".as_ptr() as *mut i8),
    }
    -1
}

unsafe fn decompress_single(in_buf: *const u8, mut in_len: isize, out_buf: *mut u8, out_len: isize, in_pos: *mut isize, error: Option<unsafe extern "C" fn(*mut i8)>) -> i32 {
    let wksp_size = zstd_dctx_workspace_bound();
    let wksp = large_malloc(wksp_size);
    let dctx = zstd_init_dctx(wksp, wksp_size);
    let mut err: i32;
    let mut ret: usize;
    if dctx.is_null() { error.unwrap()(b"Out of memory while allocating zstd_dctx\0".as_ptr() as *mut i8); err = -1; if !wksp.is_null() { large_free(wksp); } return err; }
    ret = zstd_find_frame_compressed_size(in_buf, in_len);
    err = handle_zstd_error(ret, error); if err != 0 { if !wksp.is_null() { large_free(wksp); } return err; }
    in_len = ret as isize;
    ret = zstd_decompress_dctx(dctx, out_buf, out_len, in_buf, in_len);
    err = handle_zstd_error(ret, error); if err != 0 { if !wksp.is_null() { large_free(wksp); } return err; }
    if !in_pos.is_null() { *in_pos = in_len; }
    err = 0;
    if !wksp.is_null() { large_free(wksp); }
    err
}

unsafe fn __unzstd(mut in_buf: *mut u8, mut in_len: isize, fill: Option<unsafe extern "C" fn(*mut core::ffi::c_void, usize) -> isize>, flush: Option<unsafe extern "C" fn(*mut u8, usize) -> isize>, mut out_buf: *mut u8, mut out_len: isize, in_pos: *mut isize, error: Option<unsafe extern "C" fn(*mut i8)>) -> i32 {
    let mut input = zstd_in_buffer { src: core::ptr::null(), pos: 0, size: 0 };
    let mut output = zstd_out_buffer { dst: core::ptr::null_mut(), pos: 0, size: 0 };
    let mut header = zstd_frame_header { windowSize: 0 };
    let mut in_allocated: *mut core::ffi::c_void = core::ptr::null_mut();
    let mut out_allocated: *mut core::ffi::c_void = core::ptr::null_mut();
    let mut wksp: *mut core::ffi::c_void = core::ptr::null_mut();
    if out_len == 0 { out_len = usize::MAX.wrapping_sub(out_buf as usize) as isize; }
    if fill.is_none() && flush.is_none() { return decompress_single(in_buf, in_len, out_buf, out_len, in_pos, error); }
    if in_buf.is_null() { in_allocated = large_malloc(ZSTD_IOBUF_SIZE); if in_allocated.is_null() { error.unwrap()(b"Out of memory while allocating input buffer\0".as_ptr() as *mut i8); return -1; } in_buf = in_allocated as *mut u8; in_len = 0; }
    if let Some(f) = fill { in_len = f(in_buf as *mut core::ffi::c_void, ZSTD_IOBUF_SIZE); }
    if in_len < 0 { error.unwrap()(b"ZSTD-compressed data is truncated\0".as_ptr() as *mut i8); return -1; }
    input.src = in_buf; input.pos = 0; input.size = in_len as usize;
    if flush.is_some() { out_allocated = large_malloc(ZSTD_IOBUF_SIZE); if out_allocated.is_null() { error.unwrap()(b"Out of memory while allocating output buffer\0".as_ptr() as *mut i8); return -1; } out_buf = out_allocated as *mut u8; out_len = ZSTD_IOBUF_SIZE as isize; }
    output.dst = out_buf; output.pos = 0; output.size = out_len as usize;
    let ret = zstd_get_frame_header(&mut header, input.src, input.size); let err = handle_zstd_error(ret, error); if err != 0 { return err; }
    if ret != 0 { error.unwrap()(b"ZSTD-compressed data has an incomplete frame header\0".as_ptr() as *mut i8); return -1; }
    if header.windowSize > ZSTD_WINDOWSIZE_MAX { error.unwrap()(b"ZSTD-compressed data has too large a window size\0".as_ptr() as *mut i8); return -1; }
    let wksp_size = zstd_dstream_workspace_bound(header.windowSize); wksp = large_malloc(wksp_size); let dstream = zstd_init_dstream(header.windowSize, wksp, wksp_size);
    if dstream.is_null() { error.unwrap()(b"Out of memory while allocating ZSTD_DStream\0".as_ptr() as *mut i8); return -1; }
    if !in_pos.is_null() { *in_pos = 0; }
    let mut ret;
    loop { if input.pos == input.size { if !in_pos.is_null() { *in_pos += input.pos as isize; } in_len = fill.map(|f| f(in_buf as *mut core::ffi::c_void, ZSTD_IOBUF_SIZE)).unwrap_or(-1); if in_len < 0 { error.unwrap()(b"ZSTD-compressed data is truncated\0".as_ptr() as *mut i8); return -1; } input.pos = 0; input.size = in_len as usize; }
        ret = zstd_decompress_stream(dstream, &mut output, &mut input); let err = handle_zstd_error(ret, error); if err != 0 { return err; }
        if flush.is_some() && output.pos > 0 { if output.pos as isize != flush.unwrap()(output.dst, output.pos) { error.unwrap()(b"Failed to flush()\0".as_ptr() as *mut i8); return -1; } output.pos = 0; }
        if ret == 0 { break; }
    }
    if !in_pos.is_null() { *in_pos += input.pos as isize; }
    if !in_allocated.is_null() { large_free(in_allocated); } if !out_allocated.is_null() { large_free(out_allocated); } if !wksp.is_null() { large_free(wksp); } 0
}

#[cfg(not(feature = "unzstd_preboot"))]
pub unsafe fn unzstd(buf: *mut u8, len: isize, fill: Option<unsafe extern "C" fn(*mut core::ffi::c_void, usize) -> isize>, flush: Option<unsafe extern "C" fn(*mut u8, usize) -> isize>, out_buf: *mut u8, pos: *mut isize, error: Option<unsafe extern "C" fn(*mut i8)>) -> i32 { __unzstd(buf, len, fill, flush, out_buf, 0, pos, error) }

#[cfg(feature = "unzstd_preboot")]
pub unsafe fn __decompress(buf: *mut u8, len: isize, fill: Option<unsafe extern "C" fn(*mut core::ffi::c_void, usize) -> isize>, flush: Option<unsafe extern "C" fn(*mut u8, usize) -> isize>, out_buf: *mut u8, out_len: isize, pos: *mut isize, error: Option<unsafe extern "C" fn(*mut i8)>) -> i32 { __unzstd(buf, len, fill, flush, out_buf, out_len, pos, error) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
