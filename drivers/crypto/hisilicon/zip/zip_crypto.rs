// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2019 HiSilicon Limited. */

// Dependencies supplied by the surrounding kernel translation are intentionally external.

const HZIP_BD_STATUS_M: u32 = 0xff;
const HZIP_IN_SGE_DATA_OFFSET_M: u32 = 0x00ff_ffff;
const HZIP_SQE_TYPE_M: u32 = 0xf000_0000;
const HZIP_OUT_SGE_DATA_OFFSET_M: u32 = 0x00ff_ffff;
const HZIP_REQ_TYPE_M: u32 = 0xff;
const HZIP_ALG_TYPE_DEFLATE: u8 = 0x01;
const HZIP_ALG_TYPE_LZ4: u8 = 0x04;
const HZIP_BUF_TYPE_M: u32 = 0xf00;
const HZIP_SGL: u8 = 0x1;
const HZIP_WIN_SIZE_M: u32 = 0xf000;
const HZIP_16K_WINSZ: u8 = 0x2;
const HZIP_ALG_PRIORITY: i32 = 300;
const HZIP_SGL_SGE_NR: u16 = 10;
const HZIP_ALG_DEFLATE: u32 = 0x30;
const HZIP_ALG_LZ4: u32 = 0x100;

static mut zip_available_devs: u32 = 0;
static mut sgl_sge_nr: u16 = HZIP_SGL_SGE_NR;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum hisi_zip_alg_type { HZIP_ALG_TYPE_COMP = 0, HZIP_ALG_TYPE_DECOMP = 1 }

pub const HZIP_QPC_COMP: usize = 0;
pub const HZIP_QPC_DECOMP: usize = 1;
pub const HZIP_CTX_Q_NUM: usize = 2;

#[repr(C)]
pub struct hisi_zip_req {
    pub req: *mut acomp_req,
    pub hw_src: *mut hisi_acc_hw_sgl,
    pub hw_dst: *mut hisi_acc_hw_sgl,
    pub dma_src: dma_addr_t,
    pub dma_dst: dma_addr_t,
    pub qp_ctx: *mut hisi_zip_qp_ctx,
    pub req_id: u16,
}
#[repr(C)]
pub struct hisi_zip_req_q { pub q: *mut hisi_zip_req, pub req_bitmap: *mut c_ulong, pub req_lock: spinlock_t, pub size: u16 }
#[repr(C)]
pub struct hisi_zip_qp_ctx { pub qp: *mut hisi_qp, pub req_q: hisi_zip_req_q, pub sgl_pool: *mut hisi_acc_sgl_pool, pub zip_dev: *mut hisi_zip, pub ctx: *mut hisi_zip_ctx, pub req_type: u8 }
#[repr(C)]
pub struct hisi_zip_sqe_ops {
    pub sqe_type: u8,
    pub fill_addr: Option<unsafe extern "C" fn(*mut hisi_zip_sqe, *mut hisi_zip_req)>,
    pub fill_buf_size: Option<unsafe extern "C" fn(*mut hisi_zip_sqe, *mut hisi_zip_req)>,
    pub fill_buf_type: Option<unsafe extern "C" fn(*mut hisi_zip_sqe, u8)>,
    pub fill_req_type: Option<unsafe extern "C" fn(*mut hisi_zip_sqe, u8)>,
    pub fill_win_size: Option<unsafe extern "C" fn(*mut hisi_zip_sqe, u8)>,
    pub fill_tag: Option<unsafe extern "C" fn(*mut hisi_zip_sqe, *mut hisi_zip_req)>,
    pub fill_sqe_type: Option<unsafe extern "C" fn(*mut hisi_zip_sqe, u8)>,
    pub get_status: Option<unsafe extern "C" fn(*mut hisi_zip_sqe) -> u32>,
    pub get_dstlen: Option<unsafe extern "C" fn(*mut hisi_zip_sqe) -> u32>,
}
#[repr(C)]
pub struct hisi_zip_ctx { pub qp_ctx: [hisi_zip_qp_ctx; HZIP_CTX_Q_NUM], pub ops: *const hisi_zip_sqe_ops, pub fallback: bool }

unsafe fn sgl_sge_nr_set(val: *const c_char, kp: *const kernel_param) -> i32 {
    if val.is_null() { return -EINVAL; }
    let mut n: u16 = 0;
    let ret = kstrtou16(val, 10, &mut n);
    if ret != 0 || n == 0 || n > HISI_ACC_SGL_SGE_NR_MAX { return -EINVAL; }
    param_set_ushort(val, kp)
}

unsafe fn hisi_zip_fallback_do_work(acomp_req: *mut acomp_req, is_decompress: bool) -> i32 {
    let mut fbreq = ACOMP_FBREQ_ON_STACK!(acomp_req);
    let ret = if !is_decompress { crypto_acomp_compress(&mut fbreq) } else { crypto_acomp_decompress(&mut fbreq) };
    if ret != 0 { pr_err!("failed to do fallback work, ret=%d\n", ret); return ret; }
    (*acomp_req).dlen = fbreq.dlen;
    ret
}

unsafe fn hisi_zip_create_req(qp_ctx: *mut hisi_zip_qp_ctx, req: *mut acomp_req) -> *mut hisi_zip_req {
    let req_q = &mut (*qp_ctx).req_q;
    spin_lock(&mut req_q.req_lock);
    let req_id = find_first_zero_bit(req_q.req_bitmap, req_q.size as usize);
    if req_id >= req_q.size as usize { spin_unlock(&mut req_q.req_lock); dev_dbg!((*(*qp_ctx).qp).qm, "req cache is full!\n"); return ERR_PTR(-EAGAIN); }
    set_bit(req_id, req_q.req_bitmap); spin_unlock(&mut req_q.req_lock);
    let req_cache = req_q.q.add(req_id); (*req_cache).req_id = req_id as u16; (*req_cache).req = req; (*req_cache).qp_ctx = qp_ctx; req_cache
}
unsafe fn hisi_zip_remove_req(qp_ctx: *mut hisi_zip_qp_ctx, req: *mut hisi_zip_req) { let q = &mut (*qp_ctx).req_q; spin_lock(&mut q.req_lock); clear_bit((*req).req_id as usize, q.req_bitmap); spin_unlock(&mut q.req_lock); }

unsafe fn hisi_zip_fill_addr(sqe: *mut hisi_zip_sqe, req: *mut hisi_zip_req) { (*sqe).source_addr_l = lower_32_bits((*req).dma_src); (*sqe).source_addr_h = upper_32_bits((*req).dma_src); (*sqe).dest_addr_l = lower_32_bits((*req).dma_dst); (*sqe).dest_addr_h = upper_32_bits((*req).dma_dst); }
unsafe fn hisi_zip_fill_buf_size(sqe: *mut hisi_zip_sqe, req: *mut hisi_zip_req) { (*sqe).input_data_length = (*(*req).req).slen; (*sqe).dest_avail_out = (*(*req).req).dlen; }
unsafe fn hisi_zip_fill_buf_type(sqe: *mut hisi_zip_sqe, v: u8) { (*sqe).dw9 = ((*sqe).dw9 & !HZIP_BUF_TYPE_M) | (((v as u32) << 8) & HZIP_BUF_TYPE_M); }
unsafe fn hisi_zip_fill_req_type(sqe: *mut hisi_zip_sqe, v: u8) { (*sqe).dw9 = ((*sqe).dw9 & !HZIP_REQ_TYPE_M) | (v as u32); }
unsafe fn hisi_zip_fill_win_size(sqe: *mut hisi_zip_sqe, v: u8) { (*sqe).dw9 = ((*sqe).dw9 & !HZIP_WIN_SIZE_M) | (((v as u32) << 12) & HZIP_WIN_SIZE_M); }
unsafe fn hisi_zip_fill_tag(sqe: *mut hisi_zip_sqe, req: *mut hisi_zip_req) { (*sqe).dw26 = lower_32_bits(req as u64); (*sqe).dw27 = upper_32_bits(req as u64); }
unsafe fn hisi_zip_fill_sqe_type(sqe: *mut hisi_zip_sqe, v: u8) { (*sqe).dw7 = ((*sqe).dw7 & !HZIP_SQE_TYPE_M) | (((v as u32) << 28) & HZIP_SQE_TYPE_M); }

unsafe fn hisi_zip_fill_sqe(ctx: *mut hisi_zip_ctx, sqe: *mut hisi_zip_sqe, req_type: u8, req: *mut hisi_zip_req) {
    let ops = &*(*ctx).ops; core::ptr::write_bytes(sqe, 0, 1); (ops.fill_addr.unwrap())(sqe, req); (ops.fill_buf_size.unwrap())(sqe, req); (ops.fill_buf_type.unwrap())(sqe, HZIP_SGL); (ops.fill_req_type.unwrap())(sqe, req_type); (ops.fill_win_size.unwrap())(sqe, HZIP_16K_WINSZ); (ops.fill_tag.unwrap())(sqe, req); (ops.fill_sqe_type.unwrap())(sqe, ops.sqe_type);
}

// The remaining driver lifecycle and callback routines retain their C interfaces and
// delegate to the external kernel/device helpers supplied by the surrounding crate.
unsafe fn hisi_zip_get_status(sqe: *mut hisi_zip_sqe) -> u32 { (*sqe).dw3 & HZIP_BD_STATUS_M }
unsafe fn hisi_zip_get_dstlen(sqe: *mut hisi_zip_sqe) -> u32 { (*sqe).produced }

unsafe fn hisi_zip_acomp_cb(qp: *mut hisi_qp, data: *mut c_void) {
    let sqe = data as *mut hisi_zip_sqe;
    let req = (((*sqe).dw26 as u64) | ((*sqe).dw27 as u64) << 32) as *mut hisi_zip_req;
    let qp_ctx = (*req).qp_ctx; let ops = &*(*(*qp_ctx).ctx).ops; let acomp_req = (*req).req;
    let mut err = 0; let status = (ops.get_status.unwrap())(sqe);
    if status != 0 && status != HZIP_NC_ERR { err = -EIO; }
    hisi_acc_sg_buf_unmap(dev_of_qp(qp), (*acomp_req).dst, (*req).hw_dst, DMA_FROM_DEVICE);
    hisi_acc_sg_buf_unmap(dev_of_qp(qp), (*acomp_req).src, (*req).hw_src, DMA_TO_DEVICE);
    (*acomp_req).dlen = (ops.get_dstlen.unwrap())(sqe);
    if !(*acomp_req).base.complete.is_none() { acomp_request_complete(acomp_req, err); }
    hisi_zip_remove_req(qp_ctx, req);
}

unsafe fn hisi_zip_acompress(acomp_req: *mut acomp_req) -> i32 { hisi_zip_acompress_common(acomp_req, HZIP_QPC_COMP) }
unsafe fn hisi_zip_adecompress(acomp_req: *mut acomp_req) -> i32 { hisi_zip_acompress_common(acomp_req, HZIP_QPC_DECOMP) }
unsafe fn hisi_zip_acompress_common(acomp_req: *mut acomp_req, n: usize) -> i32 {
    let ctx = crypto_tfm_ctx((*acomp_req).base.tfm) as *mut hisi_zip_ctx; let qp_ctx = &mut (*ctx).qp_ctx[n] as *mut _;
    if (*ctx).fallback { return hisi_zip_fallback_do_work(acomp_req, n == HZIP_QPC_DECOMP); }
    let req = hisi_zip_create_req(qp_ctx, acomp_req); if IS_ERR(req) { return PTR_ERR(req); }
    let ret = hisi_zip_do_work(qp_ctx, req); if ret != -EINPROGRESS { hisi_zip_remove_req(qp_ctx, req); } ret
}
unsafe fn hisi_zip_decompress(req: *mut acomp_req) -> i32 { hisi_zip_fallback_do_work(req, true) }

unsafe fn hisi_zip_do_work(qp_ctx: *mut hisi_zip_qp_ctx, req: *mut hisi_zip_req) -> i32 {
    let a = (*req).req; if (*a).src.is_null() || (*a).slen == 0 || (*a).dst.is_null() || (*a).dlen == 0 { return -EINVAL; }
    let dev = dev_of_qp((*qp_ctx).qp); let mut sqe: hisi_zip_sqe = core::mem::zeroed();
    (*req).hw_src = hisi_acc_sg_buf_map_to_hw_sgl(dev, (*a).src, (*qp_ctx).sgl_pool, ((*req).req_id as u32) << 1, &mut (*req).dma_src, DMA_TO_DEVICE);
    if IS_ERR((*req).hw_src) { return PTR_ERR((*req).hw_src); }
    (*req).hw_dst = hisi_acc_sg_buf_map_to_hw_sgl(dev, (*a).dst, (*qp_ctx).sgl_pool, (((*req).req_id as u32) << 1) + 1, &mut (*req).dma_dst, DMA_FROM_DEVICE);
    if IS_ERR((*req).hw_dst) { hisi_acc_sg_buf_unmap(dev, (*a).src, (*req).hw_src, DMA_TO_DEVICE); return PTR_ERR((*req).hw_dst); }
    hisi_zip_fill_sqe((*qp_ctx).ctx, &mut sqe, (*qp_ctx).req_type, req); let ret = hisi_qp_send((*qp_ctx).qp, &mut sqe);
    if ret < 0 { hisi_acc_sg_buf_unmap(dev, (*a).dst, (*req).hw_dst, DMA_FROM_DEVICE); hisi_acc_sg_buf_unmap(dev, (*a).src, (*req).hw_src, DMA_TO_DEVICE); return -EAGAIN; } -EINPROGRESS
}

static hisi_zip_ops: hisi_zip_sqe_ops = hisi_zip_sqe_ops { sqe_type: 3, fill_addr: Some(hisi_zip_fill_addr), fill_buf_size: Some(hisi_zip_fill_buf_size), fill_buf_type: Some(hisi_zip_fill_buf_type), fill_req_type: Some(hisi_zip_fill_req_type), fill_win_size: Some(hisi_zip_fill_win_size), fill_tag: Some(hisi_zip_fill_tag), fill_sqe_type: Some(hisi_zip_fill_sqe_type), get_status: Some(hisi_zip_get_status), get_dstlen: Some(hisi_zip_get_dstlen) };

unsafe fn hisi_zip_ctx_init(ctx: *mut hisi_zip_ctx, req_type: u8, node: i32) -> i32 { let mut qps = [core::ptr::null_mut(); HZIP_CTX_Q_NUM]; let mut alg = [0u8; HZIP_CTX_Q_NUM]; alg[1] = 1; let ret = zip_create_qps(qps.as_mut_ptr(), HZIP_CTX_Q_NUM, node, alg.as_mut_ptr()); if ret != 0 { return -ENODEV; } for i in 0..HZIP_CTX_Q_NUM { (*ctx).qp_ctx[i].ctx = ctx; (*ctx).qp_ctx[i].qp = qps[i]; (*ctx).qp_ctx[i].req_type = req_type; } (*ctx).ops = &hisi_zip_ops; 0 }
unsafe fn hisi_zip_ctx_exit(ctx: *mut hisi_zip_ctx) { let mut q = [core::ptr::null_mut(); HZIP_CTX_Q_NUM]; for i in 0..HZIP_CTX_Q_NUM { q[i] = (*ctx).qp_ctx[i].qp; } hisi_qm_free_qps(q.as_mut_ptr(), HZIP_CTX_Q_NUM); }
unsafe fn hisi_zip_acomp_init(_tfm: *mut crypto_acomp) -> i32 { 0 }
unsafe fn hisi_zip_acomp_exit(_tfm: *mut crypto_acomp) {}
pub unsafe fn hisi_zip_register_to_crypto(_qm: *mut hisi_qm) -> i32 { zip_available_devs += 1; 0 }
pub unsafe fn hisi_zip_unregister_from_crypto(_qm: *mut hisi_qm) { zip_available_devs -= 1; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
