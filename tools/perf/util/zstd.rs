// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_int, c_long, c_void};

pub type size_t = usize;
pub type ssize_t = isize;

#[repr(C)]
pub struct zstd_data {
    pub comp_level: c_int,
    pub dstream: *mut ZSTD_DStream,
    pub cstream: *mut ZSTD_CStream,
}

#[repr(C)]
pub struct ZSTD_DStream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ZSTD_CStream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ZSTD_inBuffer {
    pub src: *const c_void,
    pub size: size_t,
    pub pos: size_t,
}

#[repr(C)]
pub struct ZSTD_outBuffer {
    pub dst: *mut c_void,
    pub size: size_t,
    pub pos: size_t,
}

unsafe extern "C" {
    fn ZSTD_freeDStream(zds: *mut ZSTD_DStream) -> size_t;
    fn ZSTD_freeCStream(zcs: *mut ZSTD_CStream) -> size_t;
    fn ZSTD_createCStream() -> *mut ZSTD_CStream;
    fn ZSTD_initCStream(zcs: *mut ZSTD_CStream, compressionLevel: c_int) -> size_t;
    fn ZSTD_isError(code: size_t) -> c_int;
    fn ZSTD_getErrorName(code: size_t) -> *const c_char;
    fn ZSTD_compressStream(
        zcs: *mut ZSTD_CStream,
        output: *mut ZSTD_outBuffer,
        input: *mut ZSTD_inBuffer,
    ) -> size_t;
    fn ZSTD_flushStream(zcs: *mut ZSTD_CStream, output: *mut ZSTD_outBuffer) -> size_t;
    fn ZSTD_createDStream() -> *mut ZSTD_DStream;
    fn ZSTD_initDStream(zds: *mut ZSTD_DStream) -> size_t;
    fn ZSTD_decompressStream(
        zds: *mut ZSTD_DStream,
        output: *mut ZSTD_outBuffer,
        input: *mut ZSTD_inBuffer,
    ) -> size_t;

    fn pr_err(fmt: *const c_char, ...);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zstd_init(data: *mut zstd_data, level: c_int) -> c_int {
    unsafe {
        (*data).comp_level = level;
        (*data).dstream = core::ptr::null_mut();
        (*data).cstream = core::ptr::null_mut();
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zstd_fini(data: *mut zstd_data) -> c_int {
    unsafe {
        if !(*data).dstream.is_null() {
            ZSTD_freeDStream((*data).dstream);
            (*data).dstream = core::ptr::null_mut();
        }

        if !(*data).cstream.is_null() {
            ZSTD_freeCStream((*data).cstream);
            (*data).cstream = core::ptr::null_mut();
        }
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zstd_compress_stream_to_records(
    data: *mut zstd_data,
    mut dst: *mut c_void,
    mut dst_size: size_t,
    src: *mut c_void,
    src_size: size_t,
    max_record_size: size_t,
    process_header: unsafe extern "C" fn(
        record: *mut c_void,
        dst_size: size_t,
        data_size: size_t,
    ) -> ssize_t,
) -> ssize_t {
    let mut ret: size_t;
    let mut compressed: size_t = 0;
    let mut size: ssize_t;
    let mut input = ZSTD_inBuffer {
        src,
        size: src_size,
        pos: 0,
    };
    let mut output: ZSTD_outBuffer;
    let mut record: *mut c_void;

    unsafe {
        if (*data).cstream.is_null() {
            (*data).cstream = ZSTD_createCStream();
            if (*data).cstream.is_null() {
                pr_err(c"Couldn't create compression stream.\n".as_ptr());
                return -1;
            }

            ret = ZSTD_initCStream((*data).cstream, (*data).comp_level);
            if ZSTD_isError(ret) != 0 {
                pr_err(
                    c"Failed to initialize compression stream: %s\n".as_ptr(),
                    ZSTD_getErrorName(ret),
                );
                return -1;
            }
        }

        while input.pos < input.size {
            record = dst;
            size = process_header(record, dst_size, 0);
            /* Output buffer full - cannot fit even the record header */
            if size < 0 {
                return reset(data);
            }
            compressed = compressed.wrapping_add(size as size_t);
            dst = (dst as *mut u8).add(size as usize) as *mut c_void;
            dst_size = dst_size.wrapping_sub(size as size_t);
            output = ZSTD_outBuffer {
                dst,
                size: if dst_size > max_record_size {
                    max_record_size
                } else {
                    dst_size
                },
                pos: 0,
            };
            ret = ZSTD_compressStream((*data).cstream, &mut output, &mut input);
            ZSTD_flushStream((*data).cstream, &mut output);
            if ZSTD_isError(ret) != 0 {
                pr_err(
                    c"failed to compress %ld bytes: %s\n".as_ptr(),
                    src_size as c_long,
                    ZSTD_getErrorName(ret),
                );
                return reset(data);
            }
            compressed = compressed.wrapping_add(output.pos);
            dst = (dst as *mut u8).add(output.pos) as *mut c_void;
            dst_size = dst_size.wrapping_sub(output.pos);
            /*
             * No progress: ZSTD couldn't emit any bytes into the
             * remaining output buffer.  Calling process_header
             * with output.pos=0 would re-trigger header initialization,
             * double-subtracting the header size from dst_size and
             * underflowing the unsigned counter.
             */
            if output.pos == 0 {
                return reset(data);
            }
            size = process_header(record, dst_size, output.pos);
            if size < 0 {
                return reset(data);
            }
            compressed = compressed.wrapping_add(size as size_t);
            dst = (dst as *mut u8).add(size as usize) as *mut c_void;
            dst_size = dst_size.wrapping_sub(size as size_t);
        }
    }

    compressed as ssize_t
}

unsafe fn reset(data: *mut zstd_data) -> ssize_t {
    let ret: size_t;

    unsafe {
        /* Reset so the context is usable if the caller retries */
        ret = ZSTD_initCStream((*data).cstream, (*data).comp_level);
        if ZSTD_isError(ret) != 0 {
            pr_err(
                c"failed to reset compression context: %s\n".as_ptr(),
                ZSTD_getErrorName(ret),
            );
        }
    }
    -1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zstd_decompress_stream(
    data: *mut zstd_data,
    src: *mut c_void,
    src_size: size_t,
    dst: *mut c_void,
    dst_size: size_t,
) -> size_t {
    let mut ret: size_t;
    let mut input = ZSTD_inBuffer {
        src,
        size: src_size,
        pos: 0,
    };
    let mut output = ZSTD_outBuffer {
        dst,
        size: dst_size,
        pos: 0,
    };

    unsafe {
        if (*data).dstream.is_null() {
            (*data).dstream = ZSTD_createDStream();
            if (*data).dstream.is_null() {
                pr_err(c"Couldn't create decompression stream.\n".as_ptr());
                return 0;
            }

            ret = ZSTD_initDStream((*data).dstream);
            if ZSTD_isError(ret) != 0 {
                pr_err(
                    c"Failed to initialize decompression stream: %s\n".as_ptr(),
                    ZSTD_getErrorName(ret),
                );
                return 0;
            }
        }
        while input.pos < input.size {
            let prev_in: size_t = input.pos;
            let prev_out: size_t = output.pos;

            ret = ZSTD_decompressStream((*data).dstream, &mut output, &mut input);
            if ZSTD_isError(ret) != 0 {
                pr_err(
                    c"failed to decompress (B): %zd -> %zd, dst_size %zd : %s\n".as_ptr(),
                    src_size,
                    output.pos,
                    dst_size,
                    ZSTD_getErrorName(ret),
                );
                return 0;
            }
            /*
             * Neither stream advanced - decompression is stuck.
             * Return 0 (error) rather than partial output: perf
             * uses ZSTD_flushStream (not ZSTD_endStream), so the
             * stream is continuous across compressed events.
             * Discarding unconsumed input would desynchronize the
             * decompressor, causing the next call to produce
             * garbage that could be misinterpreted as valid events.
             */
            if input.pos == prev_in && output.pos == prev_out {
                return 0;
            }
        }
    }

    output.pos
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
