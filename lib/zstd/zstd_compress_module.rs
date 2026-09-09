// SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause
/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license found in the
 * LICENSE file in the root directory of this source tree and the GPLv2 found
 * in the COPYING file in the root directory of this source tree.
 */

// C headers and local zstd headers are supplied by the surrounding translation.

#[inline]
unsafe fn zstd_forward_if_err(ret: usize) -> Option<usize> {
    if ZSTD_isError(ret) != 0 {
        Some(ret)
    } else {
        None
    }
}

unsafe fn zstd_cctx_init(
    cctx: *mut zstd_cctx,
    parameters: *const zstd_parameters,
    pledged_src_size: u64,
) -> usize {
    macro_rules! forward {
        ($expr:expr) => {
            {
                let ret = $expr;
                if let Some(err) = zstd_forward_if_err(ret) {
                    return err;
                }
            }
        };
    }
    forward!(ZSTD_CCtx_reset(cctx, ZSTD_reset_session_and_parameters));
    forward!(ZSTD_CCtx_setPledgedSrcSize(cctx, pledged_src_size));
    forward!(ZSTD_CCtx_setParameter(cctx, ZSTD_c_windowLog, (*parameters).cParams.windowLog));
    forward!(ZSTD_CCtx_setParameter(cctx, ZSTD_c_hashLog, (*parameters).cParams.hashLog));
    forward!(ZSTD_CCtx_setParameter(cctx, ZSTD_c_chainLog, (*parameters).cParams.chainLog));
    forward!(ZSTD_CCtx_setParameter(cctx, ZSTD_c_searchLog, (*parameters).cParams.searchLog));
    forward!(ZSTD_CCtx_setParameter(cctx, ZSTD_c_minMatch, (*parameters).cParams.minMatch));
    forward!(ZSTD_CCtx_setParameter(cctx, ZSTD_c_targetLength, (*parameters).cParams.targetLength));
    forward!(ZSTD_CCtx_setParameter(cctx, ZSTD_c_strategy, (*parameters).cParams.strategy));
    forward!(ZSTD_CCtx_setParameter(cctx, ZSTD_c_contentSizeFlag, (*parameters).fParams.contentSizeFlag));
    forward!(ZSTD_CCtx_setParameter(cctx, ZSTD_c_checksumFlag, (*parameters).fParams.checksumFlag));
    forward!(ZSTD_CCtx_setParameter(cctx, ZSTD_c_dictIDFlag, (!(*parameters).fParams.noDictIDFlag) as i32));
    0
}

pub unsafe extern "C" fn zstd_min_clevel() -> i32 { ZSTD_minCLevel() }
pub unsafe extern "C" fn zstd_max_clevel() -> i32 { ZSTD_maxCLevel() }
pub unsafe extern "C" fn zstd_default_clevel() -> i32 { ZSTD_defaultCLevel() }
pub unsafe extern "C" fn zstd_compress_bound(src_size: usize) -> usize { ZSTD_compressBound(src_size) }
pub unsafe extern "C" fn zstd_get_params(level: i32, estimated_src_size: u64) -> zstd_parameters {
    ZSTD_getParams(level, estimated_src_size, 0)
}
pub unsafe extern "C" fn zstd_get_cparams(level: i32, estimated_src_size: u64, dict_size: usize) -> zstd_compression_parameters {
    ZSTD_getCParams(level, estimated_src_size, dict_size)
}
pub unsafe extern "C" fn zstd_cctx_set_param(cctx: *mut zstd_cctx, param: ZSTD_cParameter, value: i32) -> usize {
    ZSTD_CCtx_setParameter(cctx, param, value)
}
pub unsafe extern "C" fn zstd_cctx_workspace_bound(cparams: *const zstd_compression_parameters) -> usize {
    ZSTD_estimateCCtxSize_usingCParams(*cparams)
}

unsafe extern "C" fn dummy_external_sequence_producer(
    _sequenceProducerState: *mut core::ffi::c_void, _outSeqs: *mut ZSTD_Sequence,
    _outSeqsCapacity: usize, _src: *const core::ffi::c_void, _srcSize: usize,
    _dict: *const core::ffi::c_void, _dictSize: usize, _compressionLevel: i32,
    _windowSize: usize,
) -> usize { ZSTD_SEQUENCE_PRODUCER_ERROR }

unsafe fn init_cctx_params_from_compress_params(
    cctx_params: *mut ZSTD_CCtx_params,
    compress_params: *const zstd_compression_parameters,
) {
    let mut zstd_params: ZSTD_parameters = core::mem::zeroed();
    zstd_params.cParams = *compress_params;
    ZSTD_CCtxParams_init_advanced(cctx_params, zstd_params);
}

pub unsafe extern "C" fn zstd_cctx_workspace_bound_with_ext_seq_prod(compress_params: *const zstd_compression_parameters) -> usize {
    let mut cctx_params: ZSTD_CCtx_params = core::mem::zeroed();
    init_cctx_params_from_compress_params(&mut cctx_params, compress_params);
    ZSTD_CCtxParams_registerSequenceProducer(&mut cctx_params, core::ptr::null_mut(), dummy_external_sequence_producer);
    ZSTD_estimateCCtxSize_usingCCtxParams(&cctx_params)
}

pub unsafe extern "C" fn zstd_cstream_workspace_bound_with_ext_seq_prod(compress_params: *const zstd_compression_parameters) -> usize {
    let mut cctx_params: ZSTD_CCtx_params = core::mem::zeroed();
    init_cctx_params_from_compress_params(&mut cctx_params, compress_params);
    ZSTD_CCtxParams_registerSequenceProducer(&mut cctx_params, core::ptr::null_mut(), dummy_external_sequence_producer);
    ZSTD_estimateCStreamSize_usingCCtxParams(&cctx_params)
}

pub unsafe extern "C" fn zstd_init_cctx(workspace: *mut core::ffi::c_void, workspace_size: usize) -> *mut zstd_cctx {
    if workspace.is_null() { return core::ptr::null_mut(); }
    ZSTD_initStaticCCtx(workspace, workspace_size)
}
pub unsafe extern "C" fn zstd_create_cctx_advanced(custom_mem: zstd_custom_mem) -> *mut zstd_cctx { ZSTD_createCCtx_advanced(custom_mem) }
pub unsafe extern "C" fn zstd_free_cctx(cctx: *mut zstd_cctx) -> usize { ZSTD_freeCCtx(cctx) }
pub unsafe extern "C" fn zstd_create_cdict_byreference(dict: *const core::ffi::c_void, dict_size: usize, cparams: zstd_compression_parameters, custom_mem: zstd_custom_mem) -> *mut zstd_cdict {
    ZSTD_createCDict_advanced(dict, dict_size, ZSTD_dlm_byRef, ZSTD_dct_auto, cparams, custom_mem)
}
pub unsafe extern "C" fn zstd_free_cdict(cdict: *mut zstd_cdict) -> usize { ZSTD_freeCDict(cdict) }

pub unsafe extern "C" fn zstd_compress_cctx(cctx: *mut zstd_cctx, dst: *mut core::ffi::c_void, dst_capacity: usize, src: *const core::ffi::c_void, src_size: usize, parameters: *const zstd_parameters) -> usize {
    let ret = zstd_cctx_init(cctx, parameters, src_size as u64);
    if let Some(err) = zstd_forward_if_err(ret) { return err; }
    ZSTD_compress2(cctx, dst, dst_capacity, src, src_size)
}
pub unsafe extern "C" fn zstd_compress_using_cdict(cctx: *mut zstd_cctx, dst: *mut core::ffi::c_void, dst_capacity: usize, src: *const core::ffi::c_void, src_size: usize, cdict: *const ZSTD_CDict) -> usize {
    ZSTD_compress_usingCDict(cctx, dst, dst_capacity, src, src_size, cdict)
}
pub unsafe extern "C" fn zstd_cstream_workspace_bound(cparams: *const zstd_compression_parameters) -> usize { ZSTD_estimateCStreamSize_usingCParams(*cparams) }

pub unsafe extern "C" fn zstd_init_cstream(parameters: *const zstd_parameters, mut pledged_src_size: u64, workspace: *mut core::ffi::c_void, workspace_size: usize) -> *mut zstd_cstream {
    if workspace.is_null() { return core::ptr::null_mut(); }
    let cstream = ZSTD_initStaticCStream(workspace, workspace_size);
    if cstream.is_null() { return core::ptr::null_mut(); }
    if pledged_src_size == 0 { pledged_src_size = ZSTD_CONTENTSIZE_UNKNOWN; }
    if ZSTD_isError(zstd_cctx_init(cstream, parameters, pledged_src_size)) != 0 { return core::ptr::null_mut(); }
    cstream
}

pub unsafe extern "C" fn zstd_reset_cstream(cstream: *mut zstd_cstream, mut pledged_src_size: u64) -> usize {
    if pledged_src_size == 0 { pledged_src_size = ZSTD_CONTENTSIZE_UNKNOWN; }
    let ret = ZSTD_CCtx_reset(cstream, ZSTD_reset_session_only);
    if let Some(err) = zstd_forward_if_err(ret) { return err; }
    let ret = ZSTD_CCtx_setPledgedSrcSize(cstream, pledged_src_size);
    if let Some(err) = zstd_forward_if_err(ret) { return err; }
    0
}
pub unsafe extern "C" fn zstd_compress_stream(cstream: *mut zstd_cstream, output: *mut zstd_out_buffer, input: *mut zstd_in_buffer) -> usize { ZSTD_compressStream(cstream, output, input) }
pub unsafe extern "C" fn zstd_flush_stream(cstream: *mut zstd_cstream, output: *mut zstd_out_buffer) -> usize { ZSTD_flushStream(cstream, output) }
pub unsafe extern "C" fn zstd_end_stream(cstream: *mut zstd_cstream, output: *mut zstd_out_buffer) -> usize { ZSTD_endStream(cstream, output) }
pub unsafe extern "C" fn zstd_register_sequence_producer(cctx: *mut zstd_cctx, sequence_producer_state: *mut core::ffi::c_void, sequence_producer: zstd_sequence_producer_f) { ZSTD_registerSequenceProducer(cctx, sequence_producer_state, sequence_producer); }
pub unsafe extern "C" fn zstd_compress_sequences_and_literals(cctx: *mut zstd_cctx, dst: *mut core::ffi::c_void, dst_capacity: usize, in_seqs: *const zstd_sequence, in_seqs_size: usize, literals: *const core::ffi::c_void, lit_size: usize, lit_capacity: usize, decompressed_size: usize) -> usize { ZSTD_compressSequencesAndLiterals(cctx, dst, dst_capacity, in_seqs, in_seqs_size, literals, lit_size, lit_capacity, decompressed_size) }

// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_DESCRIPTION("Zstd Compressor");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
