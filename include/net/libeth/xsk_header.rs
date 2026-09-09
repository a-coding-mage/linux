/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (C) 2025 Intel Corporation */

// C dependencies: <net/libeth/xdp.h>, <net/xdp_sock_drv.h>

#[inline]
pub unsafe fn libeth_xsk_tx_queue_head(bq: *mut libeth_xdp_tx_bulk, xdp: *mut libeth_xdp_buff) -> bool {
    (*bq).bulk[(*bq).count as usize] = libeth_xdp_tx_frame { xsk: xdp, opts: __libeth_xdp_tx_len((*xdp).base.data_end.offset_from((*xdp).data) as u32, LIBETH_XDP_TX_FIRST) };
    (*bq).count += 1;
    if !xdp_buff_has_frags(&(*xdp).base) { return false; }
    (*bq).bulk[((*bq).count - 1) as usize].flags |= LIBETH_XDP_TX_MULTI;
    true
}

#[inline]
pub unsafe fn libeth_xsk_tx_queue_frag(bq: *mut libeth_xdp_tx_bulk, frag: *mut libeth_xdp_buff) {
    (*bq).bulk[(*bq).count as usize] = libeth_xdp_tx_frame { xsk: frag, opts: __libeth_xdp_tx_len((*frag).base.data_end.offset_from((*frag).data) as u32) };
    (*bq).count += 1;
}

#[inline]
pub unsafe fn libeth_xsk_tx_queue_bulk(bq: *mut libeth_xdp_tx_bulk, mut xdp: *mut libeth_xdp_buff, flush_bulk: Option<unsafe extern "C" fn(*mut libeth_xdp_tx_bulk, u32) -> bool>) -> bool {
    let mut ret = true;
    if (*bq).count == LIBETH_XDP_TX_BULK && !(flush_bulk.unwrap())(bq, LIBETH_XDP_TX_XSK) { libeth_xsk_buff_free_slow(xdp); return false; }
    if !libeth_xsk_tx_queue_head(bq, xdp) { (*bq).bulk[((*bq).count - 1) as usize].flags |= LIBETH_XDP_TX_LAST; return true; }
    let head = xdp;
    loop {
        xdp = container_of(xsk_buff_get_frag(&(*head).base));
        if xdp.is_null() { break; }
        if (*bq).count == LIBETH_XDP_TX_BULK && !(flush_bulk.unwrap())(bq, LIBETH_XDP_TX_XSK) { ret = false; break; }
        libeth_xsk_tx_queue_frag(bq, xdp);
    }
    (*bq).bulk[((*bq).count - 1) as usize].flags |= LIBETH_XDP_TX_LAST;
    ret
}

#[inline]
pub unsafe fn libeth_xsk_tx_fill_buf(frm: libeth_xdp_tx_frame, i: u32, sq: *const libeth_xdpsq, _priv: u64) -> libeth_xdp_tx_desc {
    let xdp = frm.xsk;
    let desc = libeth_xdp_tx_desc { addr: xsk_buff_xdp_get_dma(&(*xdp).base), opts: frm.opts, ..Default::default() };
    xsk_buff_raw_dma_sync_for_device((*sq).pool, desc.addr, desc.len);
    let sqe = &mut (*sq).sqes[i as usize]; sqe.xsk = xdp;
    if desc.flags & LIBETH_XDP_TX_FIRST == 0 { sqe.r#type = LIBETH_SQE_XSK_TX_FRAG; return desc; }
    sqe.r#type = LIBETH_SQE_XSK_TX;
    libeth_xdp_tx_fill_stats(sqe, &desc, xdp_get_shared_info_from_buff(&(*xdp).base));
    desc
}

#[inline]
pub unsafe fn libeth_xsktmo_req_csum(_csum_start: u16, _csum_offset: u16, priv_: *mut core::ffi::c_void) {
    (*(priv_ as *mut libeth_xdp_tx_desc)).flags |= LIBETH_XDP_TX_CSUM;
}

pub static __libeth_xsktmo: xsk_tx_metadata_ops = xsk_tx_metadata_ops { tmo_request_checksum: Some(libeth_xsktmo_req_csum) };

#[inline]
pub unsafe fn __libeth_xsk_xmit_fill_buf_md(xdesc: *const xdp_desc, sq: *const libeth_xdpsq, priv_: u64) -> libeth_xdp_tx_desc {
    let tmo = libeth_xdp_priv_to_ptr(priv_);
    let ctx = xsk_buff_raw_get_ctx((*sq).pool, (*xdesc).addr, (*xdesc).options);
    let mut desc = libeth_xdp_tx_desc { addr: ctx.dma, opts: __libeth_xdp_tx_len((*xdesc).len), ..Default::default() };
    let ops = if tmo == &__libeth_xsktmo { &__libeth_xsktmo } else { tmo };
    xsk_tx_metadata_request((*sq).pool, &ctx.meta, ops, &mut desc); desc
}

#[inline]
pub unsafe fn __libeth_xsk_xmit_fill_buf(xdesc: *const xdp_desc, sq: *const libeth_xdpsq) -> libeth_xdp_tx_desc { libeth_xdp_tx_desc { addr: xsk_buff_raw_get_dma((*sq).pool, (*xdesc).addr), opts: __libeth_xdp_tx_len((*xdesc).len), ..Default::default() } }

#[inline]
pub unsafe fn libeth_xsk_xmit_fill_buf(frm: libeth_xdp_tx_frame, _i: u32, sq: *const libeth_xdpsq, priv_: u64) -> libeth_xdp_tx_desc {
    let mut desc = if priv_ != 0 { __libeth_xsk_xmit_fill_buf_md(&frm.desc, sq, priv_) } else { __libeth_xsk_xmit_fill_buf(&frm.desc, sq) };
    if xsk_is_eop_desc(&frm.desc) { desc.flags |= LIBETH_XDP_TX_LAST; }
    xsk_buff_raw_dma_sync_for_device((*sq).pool, desc.addr, desc.len); desc
}

extern "C" {
    pub fn libeth_xsk_buff_add_frag(head: *mut libeth_xdp_buff, xdp: *mut libeth_xdp_buff) -> *mut libeth_xdp_buff;
    pub fn libeth_xsk_buff_stats_frags(rs: *mut libeth_rq_napi_stats, xdp: *const libeth_xdp_buff);
    pub fn __libeth_xsk_run_prog_slow(xdp: *mut libeth_xdp_buff, bq: *const libeth_xdp_tx_bulk, act: xdp_action, ret: i32) -> u32;
    pub fn libeth_xskfq_create(fq: *mut libeth_xskfq) -> i32;
    pub fn libeth_xskfq_destroy(fq: *mut libeth_xskfq);
    pub fn libeth_xsk_init_wakeup(csd: *mut call_single_data_t, napi: *mut napi_struct);
    pub fn libeth_xsk_wakeup(csd: *mut call_single_data_t, qid: u32);
    pub fn libeth_xsk_setup_pool(dev: *mut net_device, qid: u32, enable: bool) -> i32;
}

#[repr(C)]
pub struct libeth_xskfq { pub fp: libeth_xskfq_fp, pub pending: u32, pub thresh: u32, pub buf_len: u32, pub truesize: u32, pub nid: i32 }
#[repr(C)]
pub struct libeth_xskfq_fp { pub pool: *mut xsk_buff_pool, pub fqes: *mut *mut libeth_xdp_buff, pub descs: *mut core::ffi::c_void, pub ntu: u32, pub count: u32 }

#[inline]
pub unsafe fn libeth_xsk_process_buff(mut head: *mut libeth_xdp_buff, xdp: *mut libeth_xdp_buff, len: u32) -> *mut libeth_xdp_buff {
    if len == 0 { libeth_xsk_buff_free_slow(xdp); return head; }
    xsk_buff_set_size(&mut (*xdp).base, len); xsk_buff_dma_sync_for_cpu(&mut (*xdp).base);
    if !head.is_null() { return libeth_xsk_buff_add_frag(head, xdp); }
    prefetch((*xdp).data); xdp
}

#[inline]
pub unsafe fn __libeth_xsk_run_prog(xdp: *mut libeth_xdp_buff, bq: *const libeth_xdp_tx_bulk) -> u32 {
    let mut ret = 0; let act = bpf_prog_run_xdp((*bq).prog, &mut (*xdp).base);
    if act != XDP_REDIRECT { return __libeth_xsk_run_prog_slow(xdp, bq, act, ret); }
    ret = xdp_do_redirect((*bq).dev, &mut (*xdp).base, (*bq).prog);
    if ret != 0 { return __libeth_xsk_run_prog_slow(xdp, bq, act, ret); } LIBETH_XDP_REDIRECT
}

#[inline]
pub unsafe fn __libeth_xsk_run_pass(xdp: *mut libeth_xdp_buff, bq: *mut libeth_xdp_tx_bulk, napi: *mut napi_struct, rs: *mut libeth_rq_napi_stats, md: *const core::ffi::c_void, prep: Option<unsafe extern "C" fn(*mut libeth_xdp_buff, *const core::ffi::c_void)>, run: unsafe extern "C" fn(*mut libeth_xdp_buff, *mut libeth_xdp_tx_bulk) -> u32, populate: unsafe extern "C" fn(*mut sk_buff, *const libeth_xdp_buff, *mut libeth_rq_napi_stats) -> bool) -> bool {
    (*rs).bytes += (*xdp).base.data_end.offset_from((*xdp).data) as u64; (*rs).packets += 1;
    if xdp_buff_has_frags(&(*xdp).base) { libeth_xsk_buff_stats_frags(rs, xdp); }
    if let Some(f) = prep { if !md.is_null() { f(xdp, md); } }
    let act = run(xdp, bq); if act == LIBETH_XDP_REDIRECT { return true; } if act != LIBETH_XDP_PASS { return act != LIBETH_XDP_ABORTED; }
    let skb = xdp_build_skb_from_zc(&mut (*xdp).base); if skb.is_null() { libeth_xsk_buff_free_slow(xdp); return true; }
    if !populate(skb, xdp, rs) { napi_consume_skb(skb, true); return true; } napi_gro_receive(napi, skb); true
}

#[inline]
pub unsafe fn libeth_xskfqe_alloc(fq: *mut libeth_xskfq_fp, n: u32, fill: unsafe extern "C" fn(*const libeth_xskfq_fp, u32)) -> u32 {
    let mut this = (*fq).count - (*fq).ntu; if this > n { this = n; } let mut done = 0;
    loop { let xskb = (*fq).fqes.add((*fq).ntu) as *mut *mut xdp_buff; let ret = xsk_buff_alloc_batch((*fq).pool, xskb, this);
        for i in 0..ret { fill(fq, (*fq).ntu + i); } done += ret; (*fq).ntu += ret;
        if (*fq).ntu < (*fq).count || ret < this { break; } (*fq).ntu = 0; if this >= n { break; } this = n - this; }
    done
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
