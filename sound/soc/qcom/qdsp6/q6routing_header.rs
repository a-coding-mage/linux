// SPDX-License-Identifier: GPL-2.0

unsafe extern "C" {
    pub fn q6routing_stream_open(
        fedai_id: core::ffi::c_int,
        perf_mode: core::ffi::c_int,
        stream_id: core::ffi::c_int,
        stream_type: core::ffi::c_int,
    ) -> core::ffi::c_int;
    pub fn q6routing_stream_close(fedai_id: core::ffi::c_int, stream_type: core::ffi::c_int);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
