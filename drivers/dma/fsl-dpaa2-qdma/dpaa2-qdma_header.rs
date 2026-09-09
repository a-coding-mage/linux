/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright 2019 NXP */

pub const DPAA2_QDMA_STORE_SIZE: usize = 16;
pub const NUM_CH: usize = 8;
pub const DPAA2_QDMA_DEFAULT_PRIORITY: u32 = 0;

#[repr(C, packed)]
pub struct dpaa2_qdma_sd_d_sdf {
    pub ssd: u32, /* source stride distance */
    pub sss: u32, /* source stride size */
    pub rsv1: u32,
}

#[repr(C, packed)]
pub struct dpaa2_qdma_sd_d_ddf {
    pub dsd: u32, /* Destination stride distance */
    pub dss: u32, /* Destination stride size */
    pub rsv2: u32,
}

#[repr(C)]
pub union dpaa2_qdma_sd_d_df {
    pub sdf: dpaa2_qdma_sd_d_sdf,
    pub ddf: dpaa2_qdma_sd_d_ddf,
}

#[repr(C, packed)]
pub struct dpaa2_qdma_sd_d {
    pub rsv: u32,
    pub df: dpaa2_qdma_sd_d_df,
    pub rbpcmd: u32, /* Route-by-port command */
    pub cmd: u32,
}

/* Source descriptor command read transaction type for RBP=0: */
/* coherent copy of cacheable memory */
pub const QDMA_SD_CMD_RDTTYPE_COHERENT: u32 = 0xb << 28;
/* Destination descriptor command write transaction type for RBP=0: */
/* coherent copy of cacheable memory */
pub const QDMA_DD_CMD_WRTTYPE_COHERENT: u32 = 0x6 << 28;
pub const LX2160_QDMA_DD_CMD_WRTTYPE_COHERENT: u32 = 0xb << 28;

pub const QMAN_FD_FMT_ENABLE: u32 = 1 << 0; /* frame list table enable */
pub const QMAN_FD_BMT_ENABLE: u32 = 1 << 15; /* bypass memory translation */
pub const QMAN_FD_BMT_DISABLE: u32 = 0; /* bypass memory translation */
pub const QMAN_FD_SL_DISABLE: u32 = 0; /* short lengthe disabled */
pub const QMAN_FD_SL_ENABLE: u32 = 1 << 14; /* short lengthe enabled */

pub const QDMA_FINAL_BIT_DISABLE: u32 = 0; /* final bit disable */
pub const QDMA_FINAL_BIT_ENABLE: u32 = 1 << 31; /* final bit enable */

pub const QDMA_FD_SHORT_FORMAT: u32 = 1 << 11; /* short format */
pub const QDMA_FD_LONG_FORMAT: u32 = 0; /* long format */
pub const QDMA_SER_DISABLE: u32 = 8; /* no notification */
pub const QDMA_SER_CTX: u32 = 1 << 8; /* notification by FQD_CTX[fqid] */
pub const QDMA_SER_DEST: u32 = 2 << 8; /* notification by destination desc */
pub const QDMA_SER_BOTH: u32 = 3 << 8; /* source and dest notification */
pub const QDMA_FD_SPF_ENALBE: u32 = 1 << 30; /* source prefetch enable */

pub const QMAN_FD_VA_ENABLE: u32 = 1 << 14; /* Address used is virtual address */
pub const QMAN_FD_VA_DISABLE: u32 = 0; /* Address used is a real address */
/* Flow Context: 49bit physical address */
pub const QMAN_FD_CBMT_ENABLE: u32 = 1 << 15;
pub const QMAN_FD_CBMT_DISABLE: u32 = 0; /* Flow Context: 64bit virtual address */
pub const QMAN_FD_SC_DISABLE: u32 = 0; /* stashing control */

pub const QDMA_FL_FMT_SBF: u32 = 0x0; /* Single buffer frame */
pub const QDMA_FL_FMT_SGE: u32 = 0x2; /* Scatter gather frame */
pub const QDMA_FL_BMT_ENABLE: u32 = 1 << 15; /* enable bypass memory translation */
pub const QDMA_FL_BMT_DISABLE: u32 = 0x0; /* enable bypass memory translation */
pub const QDMA_FL_SL_LONG: u32 = 0x0; /* long length */
pub const QDMA_FL_SL_SHORT: u32 = 0x1; /* short length */
pub const QDMA_FL_F: u32 = 0x1; /* last frame list bit */

/* Description of Frame list table structure */
#[repr(C)]
pub struct dpaa2_qdma_chan {
    pub qdma: *mut dpaa2_qdma_engine,
    pub vchan: virt_dma_chan,
    pub vdesc: virt_dma_desc,
    pub status: dma_status,
    pub fqid: u32,
    pub queue_lock: spinlock_t,
    pub fd_pool: *mut dma_pool,
    pub fl_pool: *mut dma_pool,
    pub sdd_pool: *mut dma_pool,
    pub comp_used: list_head,
    pub comp_free: list_head,
}

#[repr(C)]
pub struct dpaa2_qdma_comp {
    pub fd_bus_addr: dma_addr_t,
    pub fl_bus_addr: dma_addr_t,
    pub desc_bus_addr: dma_addr_t,
    pub fd_virt_addr: *mut dpaa2_fd,
    pub fl_virt_addr: *mut dpaa2_fl_entry,
    pub desc_virt_addr: *mut dpaa2_qdma_sd_d,
    pub qchan: *mut dpaa2_qdma_chan,
    pub vdesc: virt_dma_desc,
    pub list: list_head,
}

#[repr(C)]
pub struct dpaa2_qdma_engine {
    pub dma_dev: dma_device,
    pub n_chans: u32,
    pub chans: [dpaa2_qdma_chan; NUM_CH],
    pub qdma_wrtype_fixup: i32,
    pub desc_allocated: i32,
    pub priv_: *mut dpaa2_qdma_priv,
}

/* dpaa2_qdma_priv - driver private data */
#[repr(C)]
pub struct dpaa2_qdma_priv {
    pub dpqdma_id: i32,
    pub iommu_domain: *mut iommu_domain,
    pub dpdmai_attr: dpdmai_attr,
    pub dev: *mut device,
    pub mc_io: *mut fsl_mc_io,
    pub dpdmai_dev: *mut fsl_mc_device,
    pub num_pairs: u8,
    pub dpaa2_qdma: *mut dpaa2_qdma_engine,
    pub ppriv: *mut dpaa2_qdma_priv_per_prio,
    pub rx_queue_attr: [dpdmai_rx_queue_attr; DPDMAI_MAX_QUEUE_NUM],
    pub tx_queue_attr: [dpdmai_tx_queue_attr; DPDMAI_MAX_QUEUE_NUM],
}

#[repr(C)]
pub struct dpaa2_qdma_priv_per_prio {
    pub req_fqid: i32,
    pub rsp_fqid: i32,
    pub prio: i32,
    pub store: *mut dpaa2_io_store,
    pub nctx: dpaa2_io_notification_ctx,
    pub priv_: *mut dpaa2_qdma_priv,
}

pub static mut soc_fixup_tuning: [soc_device_attribute; 2] = [
    soc_device_attribute { family: "QorIQ LX2160A" },
    soc_device_attribute { family: core::ptr::null() },
];

pub const FD_POOL_SIZE: usize = core::mem::size_of::<dpaa2_fd>()
    + core::mem::size_of::<dpaa2_fl_entry>() * 3
    + core::mem::size_of::<dpaa2_qdma_sd_d>() * 2;

extern "C" {
    pub fn dpaa2_dpdmai_free_channels(dpaa2_qdma: *mut dpaa2_qdma_engine);
    pub fn dpaa2_dpdmai_free_comp(qchan: *mut dpaa2_qdma_chan, head: *mut list_head);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
