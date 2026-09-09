/* SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause */
/* Kernel-style Rust translation of linux/zstd.h. */

use core::ffi::{c_char, c_void};

/* Dependencies: linux/types.h, linux/zstd_errors.h, linux/zstd_lib.h. */

pub type zstd_error_code = ZSTD_ErrorCode;
pub type zstd_custom_mem = ZSTD_customMem;
pub type zstd_dict_load_method = ZSTD_dictLoadMethod_e;
pub type zstd_dict_content_type = ZSTD_dictContentType_e;
pub type zstd_strategy = ZSTD_strategy;
pub type zstd_compression_parameters = ZSTD_compressionParameters;
pub type zstd_frame_parameters = ZSTD_frameParameters;
pub type zstd_parameters = ZSTD_parameters;
pub type zstd_cctx = ZSTD_CCtx;
pub type zstd_cparameter = ZSTD_cParameter;
pub type zstd_cdict = ZSTD_CDict;
pub type zstd_dctx = ZSTD_DCtx;
pub type zstd_ddict = ZSTD_DDict;
pub type zstd_in_buffer = ZSTD_inBuffer;
pub type zstd_out_buffer = ZSTD_outBuffer;
pub type zstd_cstream = ZSTD_CStream;
pub type zstd_dstream = ZSTD_DStream;
pub type zstd_sequence_producer_f = ZSTD_sequenceProducer_F;
pub type zstd_frame_header = ZSTD_FrameHeader;
pub type zstd_sequence = ZSTD_Sequence;

extern "C" {
    pub fn zstd_compress_bound(src_size: usize) -> usize;
    pub fn zstd_is_error(code: usize) -> u32;
    pub fn zstd_get_error_code(code: usize) -> zstd_error_code;
    pub fn zstd_get_error_name(code: usize) -> *const c_char;
    pub fn zstd_min_clevel() -> i32;
    pub fn zstd_max_clevel() -> i32;
    pub fn zstd_default_clevel() -> i32;

    pub fn zstd_get_params(level: i32, estimated_src_size: u64) -> zstd_parameters;
    pub fn zstd_get_cparams(level: i32, estimated_src_size: u64, dict_size: usize)
        -> zstd_compression_parameters;
    pub fn zstd_cctx_set_param(cctx: *mut zstd_cctx, param: zstd_cparameter, value: i32) -> usize;

    pub fn zstd_cctx_workspace_bound(parameters: *const zstd_compression_parameters) -> usize;
    pub fn zstd_cctx_workspace_bound_with_ext_seq_prod(
        parameters: *const zstd_compression_parameters,
    ) -> usize;
    pub fn zstd_init_cctx(workspace: *mut c_void, workspace_size: usize) -> *mut zstd_cctx;
    pub fn zstd_compress_cctx(
        cctx: *mut zstd_cctx,
        dst: *mut c_void,
        dst_capacity: usize,
        src: *const c_void,
        src_size: usize,
        parameters: *const zstd_parameters,
    ) -> usize;
    pub fn zstd_create_cctx_advanced(custom_mem: zstd_custom_mem) -> *mut zstd_cctx;
    pub fn zstd_free_cctx(cctx: *mut zstd_cctx) -> usize;
    pub fn zstd_create_cdict_byreference(
        dict: *const c_void,
        dict_size: usize,
        cparams: zstd_compression_parameters,
        custom_mem: zstd_custom_mem,
    ) -> *mut zstd_cdict;
    pub fn zstd_free_cdict(cdict: *mut zstd_cdict) -> usize;
    pub fn zstd_compress_using_cdict(
        cctx: *mut zstd_cctx,
        dst: *mut c_void,
        dst_capacity: usize,
        src: *const c_void,
        src_size: usize,
        cdict: *const zstd_cdict,
    ) -> usize;

    pub fn zstd_dctx_workspace_bound() -> usize;
    pub fn zstd_init_dctx(workspace: *mut c_void, workspace_size: usize) -> *mut zstd_dctx;
    pub fn zstd_decompress_dctx(
        dctx: *mut zstd_dctx,
        dst: *mut c_void,
        dst_capacity: usize,
        src: *const c_void,
        src_size: usize,
    ) -> usize;
    pub fn zstd_create_ddict_byreference(
        dict: *const c_void,
        dict_size: usize,
        custom_mem: zstd_custom_mem,
    ) -> *mut zstd_ddict;
    pub fn zstd_free_ddict(ddict: *mut zstd_ddict) -> usize;
    pub fn zstd_create_dctx_advanced(custom_mem: zstd_custom_mem) -> *mut zstd_dctx;
    pub fn zstd_free_dctx(dctx: *mut zstd_dctx) -> usize;
    pub fn zstd_decompress_using_ddict(
        dctx: *mut zstd_dctx,
        dst: *mut c_void,
        dst_capacity: usize,
        src: *const c_void,
        src_size: usize,
        ddict: *const zstd_ddict,
    ) -> usize;

    pub fn zstd_cstream_workspace_bound(cparams: *const zstd_compression_parameters) -> usize;
    pub fn zstd_cstream_workspace_bound_with_ext_seq_prod(
        cparams: *const zstd_compression_parameters,
    ) -> usize;
    pub fn zstd_init_cstream(
        parameters: *const zstd_parameters,
        pledged_src_size: u64,
        workspace: *mut c_void,
        workspace_size: usize,
    ) -> *mut zstd_cstream;
    pub fn zstd_reset_cstream(cstream: *mut zstd_cstream, pledged_src_size: u64) -> usize;
    pub fn zstd_compress_stream(cstream: *mut zstd_cstream, output: *mut zstd_out_buffer, input: *mut zstd_in_buffer) -> usize;
    pub fn zstd_flush_stream(cstream: *mut zstd_cstream, output: *mut zstd_out_buffer) -> usize;
    pub fn zstd_end_stream(cstream: *mut zstd_cstream, output: *mut zstd_out_buffer) -> usize;

    pub fn zstd_dstream_workspace_bound(max_window_size: usize) -> usize;
    pub fn zstd_init_dstream(max_window_size: usize, workspace: *mut c_void, workspace_size: usize) -> *mut zstd_dstream;
    pub fn zstd_reset_dstream(dstream: *mut zstd_dstream) -> usize;
    pub fn zstd_decompress_stream(dstream: *mut zstd_dstream, output: *mut zstd_out_buffer, input: *mut zstd_in_buffer) -> usize;

    pub fn zstd_find_frame_compressed_size(src: *const c_void, src_size: usize) -> usize;
    pub fn zstd_register_sequence_producer(
        cctx: *mut zstd_cctx,
        sequence_producer_state: *mut c_void,
        sequence_producer: zstd_sequence_producer_f,
    );
    pub fn zstd_get_frame_header(params: *mut zstd_frame_header, src: *const c_void, src_size: usize) -> usize;
    pub fn zstd_compress_sequences_and_literals(
        cctx: *mut zstd_cctx,
        dst: *mut c_void,
        dst_capacity: usize,
        in_seqs: *const zstd_sequence,
        in_seqs_size: usize,
        literals: *const c_void,
        lit_size: usize,
        lit_capacity: usize,
        decompressed_size: usize,
    ) -> usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
