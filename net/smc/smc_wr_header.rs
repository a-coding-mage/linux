/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Shared Memory Communications over RDMA (SMC-R) and RoCE
 *
 * Work Requests exploiting Infiniband API
 *
 * Copyright IBM Corp. 2016
 *
 * Author(s):  Steffen Maier <maier@linux.vnet.ibm.com>
 */

// Dependencies supplied by the surrounding kernel/RDMA translation.

pub const SMC_WR_TX_WAIT_FREE_SLOT_TIME: u64 = 10 * HZ;
pub const SMC_WR_TX_SIZE: usize = 44; // actual size of wr_send data (<=SMC_WR_BUF_SIZE)
pub const SMC_WR_TX_PEND_PRIV_SIZE: usize = 32;

#[repr(C)]
pub struct smc_wr_tx_pend_priv {
    pub priv_: [u8; SMC_WR_TX_PEND_PRIV_SIZE],
}

pub type smc_wr_tx_handler = unsafe extern "C" fn(
    *mut smc_wr_tx_pend_priv,
    *mut smc_link,
    ib_wc_status,
);
pub type smc_wr_tx_filter = unsafe extern "C" fn(*mut smc_wr_tx_pend_priv, usize) -> bool;
pub type smc_wr_tx_dismisser = unsafe extern "C" fn(*mut smc_wr_tx_pend_priv);

#[repr(C)]
pub struct smc_wr_rx_handler {
    pub list: hlist_node, /* hash table collision resolution */
    pub handler: Option<unsafe extern "C" fn(*mut ib_wc, *mut core::ffi::c_void)>,
    pub type_: u8,
}

// Only used by RDMA write WRs.
// All other WRs (CDC/LLC) use smc_wr_tx_send handling WR_ID implicitly
#[inline]
pub unsafe fn smc_wr_tx_get_next_wr_id(link: *mut smc_link) -> i64 {
    atomic_long_inc_return(&mut (*link).wr_tx_id)
}

#[inline]
pub unsafe fn smc_wr_tx_set_wr_id(wr_tx_id: *mut atomic_long_t, val: i64) {
    atomic_long_set(wr_tx_id, val);
}

#[inline]
pub unsafe fn smc_wr_tx_link_hold(link: *mut smc_link) -> bool {
    if !smc_link_sendable(link) {
        return false;
    }
    percpu_ref_get(&mut (*link).wr_tx_refs);
    true
}

#[inline]
pub unsafe fn smc_wr_tx_link_put(link: *mut smc_link) {
    percpu_ref_put(&mut (*link).wr_tx_refs);
}

#[inline]
pub unsafe fn smc_wr_drain_cq(lnk: *mut smc_link) {
    wait_event(
        &mut (*lnk).wr_rx_empty_wait,
        (*lnk).wr_rx_id_compl == (*lnk).wr_rx_id,
    );
}

#[inline]
pub unsafe fn smc_wr_wakeup_tx_wait(lnk: *mut smc_link) {
    wake_up_all(&mut (*lnk).wr_tx_wait);
}

#[inline]
pub unsafe fn smc_wr_wakeup_reg_wait(lnk: *mut smc_link) {
    wake_up(&mut (*lnk).wr_reg_wait);
}

/* post a new receive work request to fill a completed old work request entry */
#[inline]
pub unsafe fn smc_wr_rx_post(link: *mut smc_link) -> i32 {
    let wr_id: u64;
    let mut temp_wr_id: u64;
    let index: u32;

    (*link).wr_rx_id += 1; // tasklet context, thus not atomic
    wr_id = (*link).wr_rx_id;
    temp_wr_id = wr_id;
    index = do_div(&mut temp_wr_id, (*link).wr_rx_cnt);
    (*link).wr_rx_ibs[index as usize].wr_id = wr_id;
    ib_post_recv(
        (*link).roce_qp,
        &mut (*link).wr_rx_ibs[index as usize],
        core::ptr::null_mut(),
    )
}

extern "C" {
    pub fn smc_wr_create_link(lnk: *mut smc_link) -> i32;
    pub fn smc_wr_alloc_link_mem(lnk: *mut smc_link) -> i32;
    pub fn smc_wr_alloc_lgr_mem(lgr: *mut smc_link_group) -> i32;
    pub fn smc_wr_free_link(lnk: *mut smc_link);
    pub fn smc_wr_free_link_mem(lnk: *mut smc_link);
    pub fn smc_wr_free_lgr_mem(lgr: *mut smc_link_group);
    pub fn smc_wr_remember_qp_attr(lnk: *mut smc_link);
    pub fn smc_wr_remove_dev(smcibdev: *mut smc_ib_device);
    pub fn smc_wr_add_dev(smcibdev: *mut smc_ib_device);

    pub fn smc_wr_tx_get_free_slot(
        link: *mut smc_link, handler: smc_wr_tx_handler,
        wr_buf: *mut *mut smc_wr_buf, wrs: *mut *mut smc_rdma_wr,
        wr_pend_priv: *mut *mut smc_wr_tx_pend_priv,
    ) -> i32;
    pub fn smc_wr_tx_get_v2_slot(
        link: *mut smc_link, handler: smc_wr_tx_handler,
        wr_buf: *mut *mut smc_wr_v2_buf,
        wr_pend_priv: *mut *mut smc_wr_tx_pend_priv,
    ) -> i32;
    pub fn smc_wr_tx_put_slot(link: *mut smc_link, wr_pend_priv: *mut smc_wr_tx_pend_priv) -> i32;
    pub fn smc_wr_tx_send(link: *mut smc_link, wr_pend_priv: *mut smc_wr_tx_pend_priv) -> i32;
    pub fn smc_wr_tx_v2_send(link: *mut smc_link, priv_: *mut smc_wr_tx_pend_priv, len: i32) -> i32;
    pub fn smc_wr_tx_send_wait(link: *mut smc_link, priv_: *mut smc_wr_tx_pend_priv, timeout: usize) -> i32;
    pub fn smc_wr_tx_cq_handler(ib_cq: *mut ib_cq, cq_context: *mut core::ffi::c_void);
    pub fn smc_wr_tx_wait_no_pending_sends(link: *mut smc_link);
    pub fn smc_wr_rx_register_handler(handler: *mut smc_wr_rx_handler) -> i32;
    pub fn smc_wr_rx_post_init(link: *mut smc_link) -> i32;
    pub fn smc_wr_rx_cq_handler(ib_cq: *mut ib_cq, cq_context: *mut core::ffi::c_void);
    pub fn smc_wr_reg_send(link: *mut smc_link, mr: *mut ib_mr) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
