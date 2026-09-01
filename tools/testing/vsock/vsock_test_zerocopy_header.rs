/* SPDX-License-Identifier: GPL-2.0-only */

/* Dependency from C header: #include "util.h" */
pub enum test_opts {}

unsafe extern "C" {
    pub fn test_stream_msgzcopy_client(opts: *const test_opts);
    pub fn test_stream_msgzcopy_server(opts: *const test_opts);

    pub fn test_seqpacket_msgzcopy_client(opts: *const test_opts);
    pub fn test_seqpacket_msgzcopy_server(opts: *const test_opts);

    pub fn test_stream_msgzcopy_empty_errq_client(opts: *const test_opts);
    pub fn test_stream_msgzcopy_empty_errq_server(opts: *const test_opts);

    pub fn test_stream_msgzcopy_mangle_client(opts: *const test_opts);
    pub fn test_stream_msgzcopy_mangle_server(opts: *const test_opts);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
