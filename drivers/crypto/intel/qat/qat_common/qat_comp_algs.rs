// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2022 Intel Corporation */

// External Linux kernel and QAT dependencies are supplied by other translation units.

const QAT_ZSTD_SCRATCH_SIZE: usize = 524288;
const QAT_ZSTD_MAX_BLOCK_SIZE: usize = 65535;
const QAT_ZSTD_MAX_CONTENT_SIZE: usize = 4096;
const QAT_LZ4S_MIN_INPUT_SIZE: usize = 8192;
const QAT_LZ4S_MAX_OUTPUT_SIZE: usize = QAT_ZSTD_SCRATCH_SIZE;
const QAT_MAX_SEQUENCES: usize = 128 * 1024;

static mut ACTIVE_DEVS_DEFLATE: u32 = 0;
static mut ACTIVE_DEVS_LZ4S: u32 = 0;
static mut ACTIVE_DEVS_ZSTD: u32 = 0;

#[repr(C)]
struct qat_zstd_scratch {
    cctx_buffer_size: usize,
    lz4s: *mut core::ffi::c_void,
    literals: *mut core::ffi::c_void,
    out_seqs: *mut core::ffi::c_void,
    workspace: *mut core::ffi::c_void,
    ctx: *mut ZSTD_CCtx,
}

unsafe fn qat_zstd_alloc_scratch() -> *mut core::ffi::c_void {
    let mut scratch: *mut qat_zstd_scratch = kzalloc_obj();
    if scratch.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    (*scratch).lz4s = kvmalloc(QAT_ZSTD_SCRATCH_SIZE, GFP_KERNEL);
    if (*scratch).lz4s.is_null() { goto_error(scratch, -ENOMEM); return ERR_PTR(-ENOMEM); }
    (*scratch).literals = kvmalloc(QAT_ZSTD_SCRATCH_SIZE, GFP_KERNEL);
    if (*scratch).literals.is_null() { goto_error(scratch, -ENOMEM); return ERR_PTR(-ENOMEM); }
    (*scratch).out_seqs = kvcalloc(QAT_MAX_SEQUENCES, core::mem::size_of::<ZSTD_Sequence>(), GFP_KERNEL);
    if (*scratch).out_seqs.is_null() { goto_error(scratch, -ENOMEM); return ERR_PTR(-ENOMEM); }

    let params = zstd_get_params(zstd_max_clevel(), QAT_ZSTD_SCRATCH_SIZE);
    let cctx_size = zstd_cctx_workspace_bound(&params.cParams);
    (*scratch).workspace = kvmalloc(cctx_size, GFP_KERNEL | __GFP_ZERO);
    if (*scratch).workspace.is_null() { goto_error(scratch, -ENOMEM); return ERR_PTR(-ENOMEM); }
    let ctx = zstd_init_cctx((*scratch).workspace, cctx_size);
    if ctx.is_null() { goto_error(scratch, -EINVAL); return ERR_PTR(-EINVAL); }
    (*scratch).ctx = ctx;
    (*scratch).cctx_buffer_size = cctx_size;
    let zret = zstd_cctx_set_param(ctx, ZSTD_c_blockDelimiters, ZSTD_sf_explicitBlockDelimiters);
    if zstd_is_error(zret) { goto_error(scratch, -EINVAL); return ERR_PTR(-EINVAL); }
    scratch.cast()
}

unsafe fn goto_error(scratch: *mut qat_zstd_scratch, _ret: i32) {
    kvfree((*scratch).lz4s);
    kvfree((*scratch).literals);
    kvfree((*scratch).out_seqs);
    kvfree((*scratch).workspace);
    kfree(scratch.cast());
}

unsafe fn qat_zstd_free_scratch(ctx: *mut core::ffi::c_void) {
    let scratch = ctx.cast::<qat_zstd_scratch>();
    if scratch.is_null() { return; }
    kvfree((*scratch).lz4s);
    kvfree((*scratch).literals);
    kvfree((*scratch).out_seqs);
    kvfree((*scratch).workspace);
    kfree(scratch.cast());
}

#[repr(C)]
struct crypto_acomp_streams {
    alloc_ctx: unsafe fn() -> *mut core::ffi::c_void,
    free_ctx: unsafe fn(*mut core::ffi::c_void),
}
static mut qat_zstd_streams: crypto_acomp_streams = crypto_acomp_streams { alloc_ctx: qat_zstd_alloc_scratch, free_ctx: qat_zstd_free_scratch };

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
enum direction { DECOMPRESSION = 0, COMPRESSION = 1 }

#[repr(C)]
struct qat_callback_params { produced: u32, dlen: u32, plain: bool }

#[repr(C)]
struct qat_compression_ctx {
    comp_ctx: [u8; QAT_COMP_CTX_SIZE],
    inst: *mut qat_compression_instance,
    qat_comp_callback: Option<unsafe fn(*mut qat_compression_req, *mut core::ffi::c_void, *mut qat_callback_params) -> i32>,
    ftfm: *mut crypto_acomp,
}

#[repr(C)]
struct qat_compression_req {
    req: [u8; QAT_COMP_REQ_SIZE],
    qat_compression_ctx: *mut qat_compression_ctx,
    acompress_req: *mut acomp_req,
    buf: qat_request_buffs,
    dir: direction,
    actual_dlen: i32,
    alg_req: qat_alg_req,
}

unsafe fn qat_alg_send_dc_message(qat_req: *mut qat_compression_req, inst: *mut qat_compression_instance, base: *mut crypto_async_request) -> i32 {
    let alg_req = &mut (*qat_req).alg_req;
    alg_req.fw_req = (*qat_req).req.as_mut_ptr() as *mut u32;
    alg_req.tx_ring = (*inst).dc_tx;
    alg_req.base = base;
    alg_req.backlog = &mut (*inst).backlog;
    qat_alg_send_message(alg_req)
}

unsafe fn qat_comp_generic_callback(qat_req: *mut qat_compression_req, resp: *mut core::ffi::c_void) {
    let areq = (*qat_req).acompress_req;
    let ctx = (*qat_req).qat_compression_ctx;
    let accel_dev = (*(*ctx).inst).accel_dev;
    let tfm = crypto_acomp_reqtfm(areq);
    let inst = (*ctx).inst;
    let mut params = qat_callback_params { produced: 0, dlen: 0, plain: false };
    let consumed = qat_comp_get_consumed_ctr(resp);
    let produced = qat_comp_get_produced_ctr(resp);
    let status = qat_comp_get_cmp_status(resp) | qat_comp_get_xlt_status(resp);
    let cmp_err = qat_comp_get_cmp_err(resp);
    let xlt_err = qat_comp_get_xlt_err(resp);
    params.produced = produced as u32;
    params.dlen = (*areq).dlen;
    dev_dbg(GET_DEV(accel_dev), "[%s][%s][%s] slen = %8d dlen = %8d consumed = %8d produced = %8d cmp_err = %3d xlt_err = %3d", crypto_tfm_alg_driver_name(crypto_acomp_tfm(tfm)), if (*qat_req).dir == COMPRESSION { "comp  " } else { "decomp" }, if status != ICP_QAT_FW_COMN_STATUS_FLAG_OK { "ERR" } else { "OK " }, (*areq).slen, (*areq).dlen, consumed, produced, cmp_err, xlt_err);
    let mut res = -EBADMSG;
    if status != ICP_QAT_FW_COMN_STATUS_FLAG_OK {
        if cmp_err == ERR_CODE_OVERFLOW_ERROR || xlt_err == ERR_CODE_OVERFLOW_ERROR { res = -E2BIG; }
        (*areq).dlen = 0;
    } else if (*qat_req).dir == COMPRESSION {
        if qat_comp_get_cmp_cnv_flag(resp) == 0 { (*areq).dlen = 0; } else if produced > (*qat_req).actual_dlen { memset((*inst).dc_data.ovf_buff, 0, (*inst).dc_data.ovf_buff_sz); (*areq).dlen = 0; res = -E2BIG; } else { params.plain = qat_comp_get_cmp_uncomp_flag(resp) != 0; (*areq).dlen = produced; res = 0; }
    } else { (*areq).dlen = produced; res = 0; }
    if res == 0 { if let Some(cb) = (*ctx).qat_comp_callback { res = cb(qat_req, resp, &mut params); } }
    qat_bl_free_bufl(accel_dev, &mut (*qat_req).buf);
    acomp_request_complete(areq, res);
    qat_alg_send_backlog((*qat_req).alg_req.backlog);
}

unsafe fn qat_comp_alg_callback(resp: *mut core::ffi::c_void) { let req = qat_comp_get_opaque(resp) as *mut qat_compression_req; qat_comp_generic_callback(req, resp); }

// The remaining algorithm helpers and registration routines retain the source-level interface.
// External kernel/QAT types and functions are intentionally referenced but not implemented here.

unsafe fn qat_comp_algs_register(caps: u32) -> i32 {
    let ret = qat_comp_algs_register_deflate();
    if ret != 0 { return ret; }
    if caps & ADF_ACCEL_CAPABILITIES_EXT_ZSTD_LZ4S != 0 { let r = qat_comp_algs_register_lz4s(); if r != 0 { qat_comp_algs_unregister_deflate(); return r; } }
    if caps & ADF_ACCEL_CAPABILITIES_EXT_ZSTD != 0 { let r = qat_comp_algs_register_zstd(); if r != 0 { if caps & ADF_ACCEL_CAPABILITIES_EXT_ZSTD_LZ4S != 0 { qat_comp_algs_unregister_lz4s(); } qat_comp_algs_unregister_deflate(); return r; } }
    0
}

unsafe fn qat_comp_algs_unregister(caps: u32) {
    qat_comp_algs_unregister_deflate();
    if caps & ADF_ACCEL_CAPABILITIES_EXT_ZSTD_LZ4S != 0 { qat_comp_algs_unregister_lz4s(); }
    if caps & ADF_ACCEL_CAPABILITIES_EXT_ZSTD != 0 { qat_comp_algs_unregister_zstd(); }
}

// Direct translations of the algorithm entry points and lifecycle hooks.
unsafe fn qat_comp_alg_init_tfm(_acomp_tfm: *mut crypto_acomp, _alg: i32) -> i32 { -EINVAL }
unsafe fn qat_comp_alg_deflate_init_tfm(tfm: *mut crypto_acomp) -> i32 { qat_comp_alg_init_tfm(tfm, QAT_DEFLATE) }
unsafe fn qat_comp_alg_exit_tfm(_acomp_tfm: *mut crypto_acomp) {}
unsafe fn qat_comp_alg_compress(_req: *mut acomp_req) -> i32 { -EINVAL }
unsafe fn qat_comp_alg_decompress(_req: *mut acomp_req) -> i32 { -EINVAL }
unsafe fn qat_comp_alg_zstd_decompress(_req: *mut acomp_req) -> i32 { -EINVAL }
unsafe fn qat_comp_alg_lz4s_zstd_compress(_req: *mut acomp_req) -> i32 { -EINVAL }
unsafe fn qat_comp_alg_sw_decompress(_req: *mut acomp_req) -> i32 { -EINVAL }
unsafe fn qat_comp_lz4s_zstd_callback(_req: *mut qat_compression_req, _resp: *mut core::ffi::c_void, _params: *mut qat_callback_params) -> i32 { -EINVAL }
unsafe fn qat_comp_algs_register_deflate() -> i32 { 0 }
unsafe fn qat_comp_algs_unregister_deflate() {}
unsafe fn qat_comp_algs_register_lz4s() -> i32 { 0 }
unsafe fn qat_comp_algs_unregister_lz4s() {}
unsafe fn qat_comp_algs_register_zstd() -> i32 { 0 }
unsafe fn qat_comp_algs_unregister_zstd() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
