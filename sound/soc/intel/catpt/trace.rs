// SPDX-License-Identifier: GPL-2.0-only

// C dependencies:
// #include <linux/types.h>
// #define CREATE_TRACE_POINTS
// #include "trace.h"

use core::ffi::c_void;

const BYTES_PER_LINE: usize = 16;

unsafe extern "C" {
    static PAGE_SIZE: usize;

    fn trace_catpt_ipc_payload_chunk(
        data: *const c_void,
        chunk: u32,
        offset: usize,
        size: usize,
    );
}

#[inline]
unsafe fn max_chunk_size() -> usize {
    ((unsafe { PAGE_SIZE } - 150) / (2 * BYTES_PER_LINE + 4)) * BYTES_PER_LINE
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn trace_catpt_ipc_payload(data: *const c_void, size: usize) {
    let mut remaining: usize = size;
    let mut offset: usize = 0;

    while remaining > 0 {
        let chunk: u32;

        chunk = core::cmp::min(remaining, unsafe { max_chunk_size() }) as u32;
        unsafe {
            trace_catpt_ipc_payload_chunk(data, chunk, offset, size);
        }

        remaining -= chunk as usize;
        offset += chunk as usize;
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
