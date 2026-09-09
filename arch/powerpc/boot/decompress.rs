// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Wrapper around the kernel's pre-boot decompression library.
 *
 * Copyright (C) IBM Corporation 2016.
 */

// C dependencies supplied by the surrounding build.

/* The decompressor sources may be conditionally included by the build. */

extern "C" {
    fn __decompress(
        inbuf: *mut core::ffi::c_void,
        input_size: usize,
        fill: *mut core::ffi::c_void,
        flush: unsafe extern "C" fn(*mut core::ffi::c_void, usize) -> isize,
        outbuf: *mut core::ffi::c_void,
        output_size: usize,
        header: *mut core::ffi::c_void,
        error: unsafe extern "C" fn(*mut i8),
    ) -> i32;
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    fn printf(format: *const i8, ...);
}

/* globals for tracking the state of the decompression */
static mut DECOMPRESSED_BYTES: usize = 0;
static mut LIMIT: usize = 0;
static mut SKIP: usize = 0;
static mut OUTPUT_BUFFER: *mut i8 = core::ptr::null_mut();

/*
 * flush() is called by __decompress() when the decompressor's scratch buffer is
 * full.
 */
unsafe extern "C" fn flush(v: *mut core::ffi::c_void, buffer_size: usize) -> isize {
    let end = DECOMPRESSED_BYTES.wrapping_add(buffer_size);
    let mut size = buffer_size;
    let mut offset = 0usize;
    let mut input = v as *mut i8;
    let out: *mut i8;

    /*
     * if we hit our decompression limit, we need to fake an error to abort
     * the in-progress decompression.
     */
    if DECOMPRESSED_BYTES >= LIMIT {
        return -1;
    }

    /* skip this entire block */
    if end <= SKIP {
        DECOMPRESSED_BYTES = DECOMPRESSED_BYTES.wrapping_add(buffer_size);
        return buffer_size as isize;
    }

    /* skip some data at the start, but keep the rest of the block */
    if DECOMPRESSED_BYTES < SKIP && end > SKIP {
        offset = SKIP - DECOMPRESSED_BYTES;
        input = input.add(offset);
        size -= offset;
        DECOMPRESSED_BYTES = DECOMPRESSED_BYTES.wrapping_add(offset);
    }

    out = OUTPUT_BUFFER.add(DECOMPRESSED_BYTES - SKIP);
    size = core::cmp::min(DECOMPRESSED_BYTES.wrapping_add(size), LIMIT)
        .wrapping_sub(DECOMPRESSED_BYTES);

    memcpy(out as *mut core::ffi::c_void, input as *const core::ffi::c_void, size);
    DECOMPRESSED_BYTES = DECOMPRESSED_BYTES.wrapping_add(size);

    buffer_size as isize
}

unsafe extern "C" fn print_err(s: *mut i8) {
    /* suppress the "error" when we terminate the decompressor */
    if DECOMPRESSED_BYTES >= LIMIT {
        return;
    }

    printf(b"Decompression error: '%s'\n\r\0".as_ptr() as *const i8, s);
}

/**
 * partial_decompress - decompresses part or all of a compressed buffer
 * @inbuf:       input buffer
 * @input_size:  length of the input buffer
 * @outbuf:      output buffer
 * @output_size: length of the output buffer
 * @_skip:       number of output bytes to ignore
 *
 * This function takes compressed data from inbuf, decompresses and write it to
 * outbuf. Once output_size bytes are written to the output buffer, or the
 * stream is exhausted the function will return the number of bytes that were
 * decompressed. Otherwise it will return whatever error code the decompressor
 * reported (NB: This is specific to each decompressor type).
 *
 * The skip functionality is mainly there so the program and discover
 * the size of the compressed image so that it can ask firmware (if present)
 * for an appropriately sized buffer.
 */
#[no_mangle]
pub unsafe extern "C" fn partial_decompress(
    inbuf: *mut core::ffi::c_void,
    input_size: usize,
    outbuf: *mut core::ffi::c_void,
    mut output_size: usize,
    skip_bytes: usize,
) -> isize {
    let ret: i32;

    /*
     * The skipped bytes needs to be included in the size of data we want to
     * decompress.
     */
    output_size = output_size.wrapping_add(skip_bytes);

    DECOMPRESSED_BYTES = 0;
    OUTPUT_BUFFER = outbuf as *mut i8;
    LIMIT = output_size;
    SKIP = skip_bytes;

    ret = __decompress(
        inbuf,
        input_size,
        core::ptr::null_mut(),
        flush,
        outbuf,
        output_size,
        core::ptr::null_mut(),
        print_err,
    );

    /*
     * If decompression was aborted due to an actual error rather than
     * a fake error that we used to abort, then we should report it.
     */
    if DECOMPRESSED_BYTES < LIMIT {
        return ret as isize;
    }

    DECOMPRESSED_BYTES.wrapping_sub(SKIP) as isize
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
