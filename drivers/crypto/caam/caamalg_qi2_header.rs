/* SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause) */
/*
 * Copyright 2015-2016 Freescale Semiconductor Inc.
 * Copyright 2017-2018 NXP
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

pub const DPAA2_CAAM_STORE_SIZE: usize = 16;
/* NAPI weight *must* be a multiple of the store size. */
pub const DPAA2_CAAM_NAPI_WEIGHT: usize = 512;

/* The congestion entrance threshold was chosen so that on LS2088
 * we support the maximum throughput for the available memory
 */
pub const DPAA2_SEC_CONG_ENTRY_THRESH: usize = 128 * 1024 * 1024;
pub const DPAA2_SEC_CONG_EXIT_THRESH: usize =
    DPAA2_SEC_CONG_ENTRY_THRESH * 9 / 10;

/**
 * dpaa2_caam_priv - driver private data
 * @dpseci_id: DPSECI object unique ID
 * @major_ver: DPSECI major version
 * @minor_ver: DPSECI minor version
 * @dpseci_attr: DPSECI attributes
 * @sec_attr: SEC engine attributes
 * @rx_queue_attr: array of Rx queue attributes
 * @tx_queue_attr: array of Tx queue attributes
 * @cscn_mem: pointer to memory region containing the congestion SCN
 *	it’s size is larger than to accommodate alignment
 * @cscn_dma: dma address used by the QMAN to write CSCN messages
 * @dev: device associated with the DPSECI object
 * @mc_io: pointer to MC portal’s I/O object
 * @domain: IOMMU domain
 * @ppriv: per CPU pointers to privata data
 * @clean_mask: CPU mask of CPUs that have allocated netdevs
 */
#[repr(C)]
pub struct dpaa2_caam_priv {
    pub dpsec_id: core::ffi::c_int,
    pub major_ver: u16,
    pub minor_ver: u16,
    pub dpseci_attr: dpseci_attr,
    pub sec_attr: dpseci_sec_attr,
    pub rx_queue_attr: [dpseci_rx_queue_attr; DPSECI_MAX_QUEUE_NUM],
    pub tx_queue_attr: [dpseci_tx_queue_attr; DPSECI_MAX_QUEUE_NUM],
    pub num_pairs: core::ffi::c_int,
    pub cscn_mem: *mut core::ffi::c_void,
    pub cscn_dma: dma_addr_t,
    pub dev: *mut device,
    pub mc_io: *mut fsl_mc_io,
    pub domain: *mut iommu_domain,
    pub ppriv: *mut dpaa2_caam_priv_per_cpu,
    pub dfs_root: *mut dentry,
    pub clean_mask: cpumask_var_t,
}

/** Per CPU private data. */
#[repr(C)]
pub struct dpaa2_caam_priv_per_cpu {
    pub napi: napi_struct,
    pub net_dev: *mut net_device,
    pub req_fqid: core::ffi::c_int,
    pub rsp_fqid: core::ffi::c_int,
    pub prio: core::ffi::c_int,
    pub nctx: dpaa2_io_notification_ctx,
    pub store: *mut dpaa2_io_store,
    pub priv_: *mut dpaa2_caam_priv,
    pub dpio: *mut dpaa2_io,
}

/* Length of a single buffer in the QI driver memory cache */
pub const CAAM_QI_MEMCACHE_SIZE: usize = 512;

#[repr(C)]
pub struct aead_edesc {
    pub src_nents: core::ffi::c_int,
    pub dst_nents: core::ffi::c_int,
    pub iv_dma: dma_addr_t,
    pub qm_sg_bytes: core::ffi::c_int,
    pub qm_sg_dma: dma_addr_t,
    pub assoclen: core::ffi::c_uint,
    pub assoclen_dma: dma_addr_t,
    pub sgt: [dpaa2_sg_entry; 0],
}

#[repr(C)]
pub struct skcipher_edesc {
    pub src_nents: core::ffi::c_int,
    pub dst_nents: core::ffi::c_int,
    pub iv_dma: dma_addr_t,
    pub qm_sg_bytes: core::ffi::c_int,
    pub qm_sg_dma: dma_addr_t,
    pub sgt: [dpaa2_sg_entry; 0],
}

#[repr(C)]
pub struct ahash_edesc {
    pub qm_sg_dma: dma_addr_t,
    pub src_nents: core::ffi::c_int,
    pub qm_sg_bytes: core::ffi::c_int,
    pub sgt: [dpaa2_sg_entry; 0],
}

/** caam_flc - Flow Context (FLC) */
#[repr(C)]
pub struct caam_flc {
    pub flc: [u32; 16],
    pub sh_desc: [u32; MAX_SDLEN],
}

#[repr(C)]
pub enum optype {
    ENCRYPT = 0,
    DECRYPT,
    NUM_OP,
}

#[repr(C)]
pub struct caam_request {
    pub fd_flt: [dpaa2_fl_entry; 2],
    pub fd_flt_dma: dma_addr_t,
    pub flc: *mut caam_flc,
    pub flc_dma: dma_addr_t,
    pub cbk: Option<unsafe extern "C" fn(ctx: *mut core::ffi::c_void, err: u32)>,
    pub ctx: *mut core::ffi::c_void,
    pub edesc: *mut core::ffi::c_void,
    pub fallback_req: skcipher_request,
}

extern "C" {
    pub fn dpaa2_caam_enqueue(dev: *mut device, req: *mut caam_request) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
