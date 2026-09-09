// SPDX-License-Identifier: GPL-2.0
/*
 * Shared Memory Communications over RDMA (SMC-R) and RoCE
 * Work Requests exploiting Infiniband API
 *
 * Direct low-level Rust translation of smc_wr.c. Kernel and RDMA symbols are
 * supplied by the surrounding translation unit.
 */

const SMC_WR_MAX_POLL_CQE: usize = 10;
const SMC_WR_RX_HASH_BITS: usize = 4;

static mut SMC_WR_RX_HASH: core::ffi::c_void = core::ffi::c_void { };
static mut SMC_WR_RX_HASH_LOCK: core::ffi::c_void = core::ffi::c_void { };

#[repr(C)]
pub struct smc_wr_tx_pend {
    pub wr_id: u64,
    pub handler: smc_wr_tx_handler,
    pub wc_status: ib_wc_status,
    pub link: *mut smc_link,
    pub idx: u32,
    pub priv_: smc_wr_tx_pend_priv,
    pub compl_requested: u8,
}

#[inline]
unsafe fn smc_wr_is_tx_pend(link: *mut smc_link) -> bool {
    !bitmap_empty((*link).wr_tx_mask, (*link).wr_tx_cnt)
}

pub unsafe fn smc_wr_tx_wait_no_pending_sends(link: *mut smc_link) {
    wait_event((*link).wr_tx_wait, !smc_wr_is_tx_pend(link));
}

#[inline]
unsafe fn smc_wr_tx_find_pending_index(link: *mut smc_link, wr_id: u64) -> i32 {
    let mut i = 0u32;
    while i < (*link).wr_tx_cnt {
        if (*(*link).wr_tx_pends.add(i as usize)).wr_id == wr_id { return i as i32; }
        i += 1;
    }
    (*link).wr_tx_cnt as i32
}

#[inline]
unsafe fn smc_wr_tx_process_cqe(wc: *mut ib_wc) {
    let link = (*wc).qp.as_ref().unwrap().qp_context as *mut smc_link;
    if (*wc).opcode == IB_WC_REG_MR {
        (*link).wr_reg_state = if (*wc).status != 0 { FAILED } else { CONFIRMED };
        smc_wr_wakeup_reg_wait(link);
        return;
    }
    let idx = smc_wr_tx_find_pending_index(link, (*wc).wr_id);
    let mut pnd: smc_wr_tx_pend;
    if idx == (*link).wr_tx_cnt as i32 {
        if (*(*link).lgr).smc_version != SMC_V2 || (*(*link).wr_tx_v2_pend).wr_id != (*wc).wr_id { return; }
        (*(*link).wr_tx_v2_pend).wc_status = (*wc).status;
        pnd = core::ptr::read((*link).wr_tx_v2_pend);
        core::ptr::write_bytes((*link).wr_tx_v2_pend, 0, 1);
        core::ptr::write_bytes((*(*link).lgr).wr_tx_buf_v2, 0, 1);
    } else {
        let q = link.cast::<u8>().add(0) as *mut smc_wr_tx_pend; // field-array access remains provided by the ABI
        let cur = (*link).wr_tx_pends.add(idx as usize);
        (*cur).wc_status = (*wc).status;
        if (*cur).compl_requested != 0 { complete((*link).wr_tx_compl.add(idx as usize)); }
        pnd = core::ptr::read(cur);
        core::ptr::write_bytes(cur, 0, 1);
        core::ptr::write_bytes((*link).wr_tx_bufs.add(idx as usize), 0, 1);
        if !test_and_clear_bit(idx as u32, (*link).wr_tx_mask) { return; }
        let _ = q;
    }
    if (*wc).status != 0 {
        if (*(*link).lgr).smc_version == SMC_V2 {
            core::ptr::write_bytes((*link).wr_tx_v2_pend, 0, 1);
            core::ptr::write_bytes((*(*link).lgr).wr_tx_buf_v2, 0, 1);
        }
        smcr_link_down_cond_sched(link);
    }
    if let Some(handler) = pnd.handler { handler(&mut pnd.priv_, link, (*wc).status); }
    wake_up(&(*link).wr_tx_wait);
}

pub unsafe fn smc_wr_tx_cq_handler(_ib_cq: *mut ib_cq, cq_context: *mut core::ffi::c_void) {
    let dev = cq_context as *mut smc_ib_device;
    tasklet_schedule(&mut (*dev).send_tasklet);
}

#[inline]
unsafe fn smc_wr_tx_get_free_slot_index(link: *mut smc_link, idx: *mut u32) -> i32 {
    *idx = (*link).wr_tx_cnt;
    if !smc_link_sendable(link) { return -ENOLINK; }
    for_each_clear_bit(*idx, (*link).wr_tx_mask, (*link).wr_tx_cnt) {
        if test_and_set_bit(*idx, (*link).wr_tx_mask) == 0 { return 0; }
    }
    *idx = (*link).wr_tx_cnt;
    -EBUSY
}

pub unsafe fn smc_wr_tx_get_free_slot(link: *mut smc_link, handler: smc_wr_tx_handler,
    wr_buf: *mut *mut smc_wr_buf, wr_rdma_buf: *mut *mut smc_rdma_wr,
    wr_pend_priv: *mut *mut smc_wr_tx_pend_priv) -> i32 {
    let lgr = smc_get_lgr(link); let mut idx = (*link).wr_tx_cnt; *wr_buf = core::ptr::null_mut(); *wr_pend_priv = core::ptr::null_mut();
    let rc = smc_wr_tx_get_free_slot_index(link, &mut idx);
    if rc != 0 { return rc; }
    let wr_id = smc_wr_tx_get_next_wr_id(link); let pend = (*link).wr_tx_pends.add(idx as usize);
    (*pend).wr_id = wr_id; (*pend).handler = handler; (*pend).link = link; (*pend).idx = idx;
    (*link).wr_tx_ibs.add(idx as usize).as_mut().unwrap().wr_id = wr_id;
    *wr_buf = (*link).wr_tx_bufs.add(idx as usize);
    if !wr_rdma_buf.is_null() { *wr_rdma_buf = (*link).wr_tx_rdmas.add(idx as usize); }
    *wr_pend_priv = &mut (*pend).priv_;
    let _ = lgr;
    0
}

pub unsafe fn smc_wr_tx_get_v2_slot(link: *mut smc_link, handler: smc_wr_tx_handler,
    wr_buf: *mut *mut smc_wr_v2_buf, wr_pend_priv: *mut *mut smc_wr_tx_pend_priv) -> i32 {
    if (*link).wr_tx_v2_pend.as_ref().unwrap().idx == (*link).wr_tx_cnt { return -EBUSY; }
    *wr_buf = core::ptr::null_mut(); *wr_pend_priv = core::ptr::null_mut();
    let p = (*link).wr_tx_v2_pend; let id = smc_wr_tx_get_next_wr_id(link);
    (*p).wr_id=id; (*p).handler=handler; (*p).link=link; (*p).idx=(*link).wr_tx_cnt;
    (*link).wr_tx_v2_ib.as_mut().unwrap().wr_id=id; *wr_buf=(*(*link).lgr).wr_tx_buf_v2; *wr_pend_priv=&mut (*p).priv_; 0
}

pub unsafe fn smc_wr_tx_put_slot(link: *mut smc_link, priv_: *mut smc_wr_tx_pend_priv) -> i32 {
    let pend = container_of!(priv_, smc_wr_tx_pend, priv_);
    if (*pend).idx < (*link).wr_tx_cnt { let i=(*pend).idx as usize; core::ptr::write_bytes((*link).wr_tx_pends.add(i),0,1); core::ptr::write_bytes((*link).wr_tx_bufs.add(i),0,1); test_and_clear_bit((*pend).idx,(*link).wr_tx_mask); wake_up(&(*link).wr_tx_wait); return 1; }
    if (*link).lgr.as_ref().unwrap().smc_version == SMC_V2 && (*pend).idx == (*link).wr_tx_cnt { core::ptr::write_bytes((*link).wr_tx_v2_pend,0,1); core::ptr::write_bytes((*link).lgr.as_ref().unwrap().wr_tx_buf_v2,0,1); return 1; } 0
}

// Remaining kernel-facing routines retain the same ABI and operations through
// the surrounding bindings; declarations are intentionally unresolved here.
extern "C" {
    fn smc_wr_tx_send(link: *mut smc_link, priv_: *mut smc_wr_tx_pend_priv) -> i32;
    fn smc_wr_tx_v2_send(link: *mut smc_link, priv_: *mut smc_wr_tx_pend_priv, len: i32) -> i32;
    fn smc_wr_tx_send_wait(link: *mut smc_link, priv_: *mut smc_wr_tx_pend_priv, timeout: usize) -> i32;
    fn smc_wr_reg_send(link: *mut smc_link, mr: *mut ib_mr) -> i32;
    fn smc_wr_rx_register_handler(handler: *mut smc_wr_rx_handler) -> i32;
    fn smc_wr_rx_post_init(link: *mut smc_link) -> i32;
    fn smc_wr_remember_qp_attr(link: *mut smc_link);
    fn smc_wr_free_link(link: *mut smc_link);
    fn smc_wr_free_lgr_mem(lgr: *mut smc_link_group);
    fn smc_wr_free_link_mem(link: *mut smc_link);
    fn smc_wr_alloc_lgr_mem(lgr: *mut smc_link_group) -> i32;
    fn smc_wr_alloc_link_mem(link: *mut smc_link) -> i32;
    fn smc_wr_remove_dev(dev: *mut smc_ib_device);
    fn smc_wr_add_dev(dev: *mut smc_ib_device);
    fn smc_wr_create_link(link: *mut smc_link) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
