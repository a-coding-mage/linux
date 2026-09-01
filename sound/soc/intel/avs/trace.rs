// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2021-2022 Intel Corporation
//
// Author: Cezary Rojewski <cezary.rojewski@intel.com>
//         Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//

use core::ffi::c_void;

// C source includes <linux/types.h>, defines CREATE_TRACE_POINTS, and includes
// "trace.h" to provide the trace_avs_ipc_msg_payload tracepoint.

const BYTES_PER_LINE: usize = 16;

unsafe extern "C" {
    static PAGE_SIZE: usize;

    fn trace_avs_ipc_msg_payload(data: *const c_void, size: u32, offset: usize, total: usize);
}

unsafe fn max_chunk_size() -> usize {
    ((PAGE_SIZE - 150) / (2 * BYTES_PER_LINE + 4)) * BYTES_PER_LINE
}

#[no_mangle]
pub unsafe extern "C" fn trace_avs_msg_payload(data: *const c_void, size: usize) {
    let mut remaining: usize = size;
    let mut offset: usize = 0;

    while remaining > 0 {
        let chunk: u32;

        chunk = core::cmp::min(remaining, max_chunk_size()) as u32;
        trace_avs_ipc_msg_payload(data, chunk, offset, size);

        remaining -= chunk as usize;
        offset += chunk as usize;
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
