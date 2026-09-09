/* SPDX-License-Identifier: GPL-2.0 */
/*
 * CAAM/SEC 4.x driver backend
 * Private/internal definitions between modules
 *
 * Copyright 2008-2011 Freescale Semiconductor, Inc.
 * Copyright 2019, 2023 NXP
 */

/* Dependencies supplied by the surrounding kernel translation. */

/* Currently comes from Kconfig param as a ^2 (driver-required). */
pub const JOBR_DEPTH: usize = 1usize << CONFIG_CRYPTO_DEV_FSL_CAAM_RINGSIZE;

/* Maximum size for crypto-engine software queue based on Job Ring size and
 * a threshold reserved for non-crypto-API requests. */
pub const THRESHOLD: usize = 15;
pub const CRYPTO_ENGINE_MAX_QLEN: usize = JOBR_DEPTH - THRESHOLD;

/* Kconfig params for interrupt coalescing if selected (else zero).
 * The CONFIG_* conditions are retained as build-time configuration intent. */
#[cfg(CONFIG_CRYPTO_DEV_FSL_CAAM_INTC)]
pub const JOBR_INTC: u32 = JRCFG_ICEN;
#[cfg(not(CONFIG_CRYPTO_DEV_FSL_CAAM_INTC))]
pub const JOBR_INTC: u32 = 0;
#[cfg(CONFIG_CRYPTO_DEV_FSL_CAAM_INTC)]
pub const JOBR_INTC_TIME_THLD: u32 = CONFIG_CRYPTO_DEV_FSL_CAAM_INTC_TIME_THLD;
#[cfg(not(CONFIG_CRYPTO_DEV_FSL_CAAM_INTC))]
pub const JOBR_INTC_TIME_THLD: u32 = 0;
#[cfg(CONFIG_CRYPTO_DEV_FSL_CAAM_INTC)]
pub const JOBR_INTC_COUNT_THLD: u32 = CONFIG_CRYPTO_DEV_FSL_CAAM_INTC_COUNT_THLD;
#[cfg(not(CONFIG_CRYPTO_DEV_FSL_CAAM_INTC))]
pub const JOBR_INTC_COUNT_THLD: u32 = 0;

/* Storage for tracking each in-process entry moving across a ring. */
#[repr(C)]
pub struct caam_jrentry_info {
    pub callbk: Option<unsafe extern "C" fn(dev: *mut device, desc: *mut u32, status: u32, arg: *mut core::ffi::c_void)>,
    pub cbkarg: *mut core::ffi::c_void,
    pub desc_addr_virt: *mut u32,
    pub desc_addr_dma: dma_addr_t,
    pub desc_size: u32,
}

#[repr(C)]
pub struct caam_jr_state {
    pub inpbusaddr: dma_addr_t,
    pub outbusaddr: dma_addr_t,
}

#[repr(C)]
pub struct caam_jr_dequeue_params {
    pub dev: *mut device,
    pub enable_itr: core::ffi::c_int,
}

/* Private sub-storage for a single JobR. */
#[repr(C)]
pub struct caam_drv_private_jr {
    pub list_node: list_head,
    pub dev: *mut device,
    pub ridx: core::ffi::c_int,
    pub rregs: *mut caam_job_ring,
    pub irqtask: tasklet_struct,
    pub tasklet_params: caam_jr_dequeue_params,
    pub irq: core::ffi::c_int,
    pub hwrng: bool,
    pub tfm_count: atomic_t,
    pub entinfo: *mut caam_jrentry_info,
    pub inplock: spinlock_t,
    pub inpring_avail: u32,
    pub head: core::ffi::c_int,
    pub inpring: *mut core::ffi::c_void,
    pub out_ring_read_index: core::ffi::c_int,
    pub tail: core::ffi::c_int,
    pub outring: *mut core::ffi::c_void,
    pub engine: *mut crypto_engine,
    pub state: caam_jr_state,
}

#[repr(C)]
pub struct caam_ctl_state {
    pub deco_mid: [masterid; 16],
    pub jr_mid: [masterid; 4],
    pub mcr: u32,
    pub scfgr: u32,
}

/* Driver-private storage for a single CAAM block instance. */
#[repr(C)]
pub struct caam_drv_private {
    pub ctrl: *mut caam_ctrl,
    pub deco: *mut caam_deco,
    pub assure: *mut caam_assurance,
    pub qi: *mut caam_queue_if,
    pub jr: [*mut caam_job_ring; 4],
    pub domain: *mut iommu_domain,
    pub total_jobrs: u8,
    pub qi_present: u8,
    pub blob_present: u8,
    pub mc_en: u8,
    pub optee_en: u8,
    pub no_page0: u8,
    pub pr_support: bool,
    pub secvio_irq: core::ffi::c_int,
    pub virt_en: core::ffi::c_int,
    pub era: core::ffi::c_int,
    pub rng4_sh_init: u32,
    pub clks: *mut clk_bulk_data,
    pub num_clks: core::ffi::c_int,
    #[cfg(CONFIG_DEBUG_FS)]
    pub ctl: *mut dentry,
    #[cfg(CONFIG_DEBUG_FS)]
    pub ctl_kek_wrap: debugfs_blob_wrapper,
    #[cfg(CONFIG_DEBUG_FS)]
    pub ctl_tkek_wrap: debugfs_blob_wrapper,
    #[cfg(CONFIG_DEBUG_FS)]
    pub ctl_tdsk_wrap: debugfs_blob_wrapper,
    pub caam_off_during_pm: core::ffi::c_int,
    pub state: caam_ctl_state,
}

#[cfg(CONFIG_CRYPTO_DEV_FSL_CAAM_CRYPTO_API)]
extern "C" {
    pub fn caam_algapi_init(dev: *mut device) -> core::ffi::c_int;
    pub fn caam_algapi_exit();
}
#[cfg(not(CONFIG_CRYPTO_DEV_FSL_CAAM_CRYPTO_API))]
pub unsafe fn caam_algapi_init(_dev: *mut device) -> core::ffi::c_int { 0 }
#[cfg(not(CONFIG_CRYPTO_DEV_FSL_CAAM_CRYPTO_API))]
pub unsafe fn caam_algapi_exit() {}

#[cfg(CONFIG_CRYPTO_DEV_FSL_CAAM_AHASH_API)]
extern "C" {
    pub fn caam_algapi_hash_init(dev: *mut device) -> core::ffi::c_int;
    pub fn caam_algapi_hash_exit();
}
#[cfg(not(CONFIG_CRYPTO_DEV_FSL_CAAM_AHASH_API))]
pub unsafe fn caam_algapi_hash_init(_dev: *mut device) -> core::ffi::c_int { 0 }
#[cfg(not(CONFIG_CRYPTO_DEV_FSL_CAAM_AHASH_API))]
pub unsafe fn caam_algapi_hash_exit() {}

#[cfg(CONFIG_CRYPTO_DEV_FSL_CAAM_PKC_API)]
extern "C" {
    pub fn caam_pkc_init(dev: *mut device) -> core::ffi::c_int;
    pub fn caam_pkc_exit();
}
#[cfg(not(CONFIG_CRYPTO_DEV_FSL_CAAM_PKC_API))]
pub unsafe fn caam_pkc_init(_dev: *mut device) -> core::ffi::c_int { 0 }
#[cfg(not(CONFIG_CRYPTO_DEV_FSL_CAAM_PKC_API))]
pub unsafe fn caam_pkc_exit() {}

#[cfg(CONFIG_CRYPTO_DEV_FSL_CAAM_RNG_API)]
extern "C" {
    pub fn caam_rng_init(dev: *mut device) -> core::ffi::c_int;
    pub fn caam_rng_exit(dev: *mut device);
}
#[cfg(not(CONFIG_CRYPTO_DEV_FSL_CAAM_RNG_API))]
pub unsafe fn caam_rng_init(_dev: *mut device) -> core::ffi::c_int { 0 }
#[cfg(not(CONFIG_CRYPTO_DEV_FSL_CAAM_RNG_API))]
pub unsafe fn caam_rng_exit(_dev: *mut device) {}

#[cfg(CONFIG_CRYPTO_DEV_FSL_CAAM_CRYPTO_API_QI)]
extern "C" {
    pub fn caam_qi_algapi_init(dev: *mut device) -> core::ffi::c_int;
    pub fn caam_qi_algapi_exit();
}
#[cfg(not(CONFIG_CRYPTO_DEV_FSL_CAAM_CRYPTO_API_QI))]
pub unsafe fn caam_qi_algapi_init(_dev: *mut device) -> core::ffi::c_int { 0 }
#[cfg(not(CONFIG_CRYPTO_DEV_FSL_CAAM_CRYPTO_API_QI))]
pub unsafe fn caam_qi_algapi_exit() {}

pub unsafe fn caam_get_dma_mask(dev: *mut device) -> u64 {
    let nprop = (*dev).of_node;
    if caam_ptr_sz != core::mem::size_of::<u64>() {
        return DMA_BIT_MASK(32);
    }
    if caam_dpaa2 {
        return DMA_BIT_MASK(49);
    }
    if of_device_is_compatible(nprop, c"fsl,sec-v5.0-job-ring".as_ptr()) != 0
        || of_device_is_compatible(nprop, c"fsl,sec-v5.0".as_ptr()) != 0
    {
        return DMA_BIT_MASK(40);
    }
    DMA_BIT_MASK(36)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
