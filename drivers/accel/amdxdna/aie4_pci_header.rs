/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2026, Advanced Micro Devices, Inc.
 */

// Dependencies supplied by the surrounding kernel and driver translation.
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct cert_comp {
    pub ndev: *mut amdxdna_dev_hdl,
    pub msix_idx: u32,
    pub irq: c_int,
    pub kref: kref,
    pub waitq: wait_queue_head_t,
}

#[repr(C)]
pub struct amdxdna_hwctx_priv {
    pub umq_bo: *mut amdxdna_gem_obj,
    pub umq_read_index: *mut u64,
    pub umq_write_index: *mut u64,
    pub cert_comp: *mut cert_comp,
    pub hw_ctx_id: u32,
}

#[repr(C)]
pub struct amdxdna_dev_priv {
    pub npufw_path: *const c_char,
    pub certfw_path: *const c_char,
    pub mbox_bar: u32,
    pub mbox_rbuf_bar: u32,
    pub mbox_info_off: u64,
    pub doorbell_off: u32,
    pub psp_regs_off: [aie_bar_off_pair; PSP_MAX_REGS],
    pub smu_regs_off: [aie_bar_off_pair; SMU_MAX_REGS],
}

#[repr(C)]
pub struct amdxdna_dev_hdl {
    pub aie: aie_device,
    pub priv_: *const amdxdna_dev_priv,
    pub mbox_base: *mut c_void,
    pub rbuf_base: *mut c_void,
    pub mbox: *mut mailbox,
    pub partition_id: u32,
    pub cert_comp_xa: xarray, // device level indexed by msix id
    pub cert_comp_lock: mutex, // protects cert_comp operations
    pub work_buf: *mut c_void,
    pub work_buf_addr: dma_addr_t,
    pub work_buf_size: u32,
}

/* aie4_message.c */
extern "C" {
    pub fn aie4_query_aie_metadata(
        ndev: *mut amdxdna_dev_hdl,
        metadata: *mut amdxdna_drm_query_aie_metadata,
    ) -> c_int;
    pub fn aie4_suspend_fw(ndev: *mut amdxdna_dev_hdl) -> c_int;
    pub fn aie4_attach_work_buffer(ndev: *mut amdxdna_dev_hdl) -> c_int;
}

/* aie4_ctx.c */
extern "C" {
    pub fn aie4_hwctx_init(hwctx: *mut amdxdna_hwctx) -> c_int;
    pub fn aie4_hwctx_fini(hwctx: *mut amdxdna_hwctx);
    pub fn aie4_cmd_wait(hwctx: *mut amdxdna_hwctx, seq: u64, timeout: u32) -> c_int;
    pub fn aie4_hwctx_valid_doorbell(client: *mut amdxdna_client, vm_pgoff: u32) -> c_int;
}

/* aie4_sriov.c */
// IS_ENABLED(CONFIG_PCI_IOV) selects the following declarations at build time.
extern "C" {
    pub fn aie4_sriov_configure(xdna: *mut amdxdna_dev, num_vfs: c_int) -> c_int;
    pub fn aie4_sriov_stop(ndev: *mut amdxdna_dev_hdl) -> c_int;
}

// When CONFIG_PCI_IOV is disabled, aie4_sriov_configure is NULL and stop returns 0.
pub const AIE4_SRIOV_CONFIGURE_DISABLED: Option<unsafe extern "C" fn(*mut amdxdna_dev, c_int) -> c_int> = None;

#[inline]
pub unsafe fn aie4_sriov_stop_disabled(_ndev: *mut amdxdna_dev_hdl) -> c_int {
    0
}

extern "C" {
    pub static aie4_pf_ops: amdxdna_dev_ops;
    pub static aie4_vf_ops: amdxdna_dev_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
