/*
 * Copyright 2020 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 */

// Dependency supplied by the corresponding tracing module:
// #include "amdgpu_dm_trace.h"

macro_rules! TRACE_DC_PIPE_STATE {
    ($dc:expr, $max_pipes:expr) => {{
        for index in 0..($max_pipes) {
            let pipe_ctx = unsafe {
                &(*$dc).current_state.res_ctx.pipe_ctx[index as usize]
            };
            if !pipe_ctx.plane_state.is_null() {
                trace_amdgpu_dm_dc_pipe_state(
                    pipe_ctx.pipe_idx,
                    pipe_ctx.plane_state,
                    pipe_ctx.stream,
                    &pipe_ctx.plane_res,
                    pipe_ctx.update_flags.raw,
                );
            }
        }
    }};
}

macro_rules! TRACE_DCE_CLOCK_STATE {
    ($dce_clocks:expr) => {{
        trace_amdgpu_dm_dce_clocks_state($dce_clocks)
    }};
}

macro_rules! TRACE_DCN_CLOCK_STATE {
    ($dcn_clocks:expr) => {{
        trace_amdgpu_dm_dc_clocks_state($dcn_clocks)
    }};
}

macro_rules! TRACE_DCN_FPU {
    ($begin:expr, $function:expr, $line:expr, $ref_count:expr) => {{
        trace_dcn_fpu($begin, $function, $line, $ref_count)
    }};
}

macro_rules! TRACE_OPTC_LOCK_UNLOCK_STATE {
    ($optc:expr, $inst:expr, $lock:expr) => {{
        trace_dcn_optc_lock_unlock_state($optc, $inst, $lock, module_path!(), line!())
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
