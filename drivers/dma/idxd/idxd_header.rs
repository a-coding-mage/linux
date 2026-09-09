/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright(c) 2019 Intel Corporation. All rights rsvd. */
// Translated from idxd.h. Linux header dependencies are intentionally external.

pub const IDXD_DRIVER_VERSION: &str = "1.00";

extern "C" {
    pub fn perfmon_pmu_init(*mut idxd_device) -> i32; pub fn perfmon_pmu_remove(*mut idxd_device); pub fn perfmon_counter_overflow(*mut idxd_device);
    pub fn idxd_misc_thread(i32, *mut core::ffi::c_void) -> i32; pub fn idxd_wq_thread(i32, *mut core::ffi::c_void) -> i32;
    pub static mut tc_override: bool;
}

pub enum idxd_wq {}
pub enum idxd_dev_opaque {}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum idxd_dev_type { IDXD_DEV_NONE = -1, IDXD_DEV_DSA = 0, IDXD_DEV_IAX, IDXD_DEV_WQ, IDXD_DEV_GROUP, IDXD_DEV_ENGINE, IDXD_DEV_CDEV, IDXD_DEV_CDEV_FILE, IDXD_DEV_MAX_TYPE }

#[repr(C)]
pub struct idxd_dev { pub conf_dev: device, pub type_: idxd_dev_type }

pub const IDXD_REG_TIMEOUT: u32 = 50;
pub const IDXD_DRAIN_TIMEOUT: u32 = 5000;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum idxd_type { IDXD_TYPE_UNKNOWN = -1, IDXD_TYPE_DSA = 0, IDXD_TYPE_IAX, IDXD_TYPE_MAX }
pub const IDXD_NAME_SIZE: usize = 128;
pub const IDXD_PMU_EVENT_MAX: usize = 64;
pub const IDXD_ENQCMDS_RETRIES: u32 = 32;
pub const IDXD_ENQCMDS_MAX_RETRIES: u32 = 64;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum idxd_complete_type { IDXD_COMPLETE_NORMAL = 0, IDXD_COMPLETE_ABORT, IDXD_COMPLETE_DEV_FAIL }
pub enum idxd_desc {}

#[repr(C)]
pub struct idxd_device_driver {
    pub name: *const core::ffi::c_char,
    pub type_: *mut idxd_dev_type,
    pub probe: Option<unsafe extern "C" fn(*mut idxd_dev) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut idxd_dev)>,
    pub desc_complete: Option<unsafe extern "C" fn(*mut idxd_desc, idxd_complete_type, bool, *mut core::ffi::c_void, *mut u32)>,
    pub drv: device_driver,
}
extern "C" { pub static mut dsa_drv: idxd_device_driver; pub static mut idxd_drv: idxd_device_driver; pub static mut idxd_dmaengine_drv: idxd_device_driver; pub static mut idxd_user_drv: idxd_device_driver; }

pub const INVALID_INT_HANDLE: i32 = -1;
#[repr(C)] pub struct idxd_irq_entry { pub id: i32, pub vector: i32, pub pending_llist: llist_head, pub work_list: list_head, pub list_lock: spinlock_t, pub int_handle: i32, pub pasid: ioasid_t }
#[repr(C)] pub struct idxd_group { pub idxd_dev: idxd_dev, pub idxd: *mut idxd_device, pub grpcfg: grpcfg, pub id: i32, pub num_engines: i32, pub num_wqs: i32, pub use_rdbuf_limit: bool, pub rdbufs_allowed: u8, pub rdbufs_reserved: u8, pub tc_a: i32, pub tc_b: i32, pub desc_progress_limit: i32, pub batch_progress_limit: i32 }
#[repr(C)] pub struct idxd_pmu { pub idxd: *mut idxd_device, pub event_list: [*mut perf_event; IDXD_PMU_EVENT_MAX], pub n_events: i32, pub used_mask: [usize; 1], pub pmu: pmu, pub name: [core::ffi::c_char; IDXD_NAME_SIZE], pub n_counters: i32, pub counter_width: i32, pub n_event_categories: i32, pub per_counter_caps_supported: bool, pub supported_event_categories: usize, pub supported_filters: usize, pub n_filters: i32 }
pub const IDXD_MAX_PRIORITY: u32 = 0xf;
pub const COUNTER_FAULTS: u32 = 0; pub const COUNTER_FAULT_FAILS: u32 = 1; pub const COUNTER_MAX: u32 = 2;
#[repr(C)] pub enum idxd_wq_state { IDXD_WQ_DISABLED = 0, IDXD_WQ_ENABLED }
#[repr(C)] pub enum idxd_wq_flag { WQ_FLAG_DEDICATED = 0, WQ_FLAG_BLOCK_ON_FAULT, WQ_FLAG_ATS_DISABLE, WQ_FLAG_PRS_DISABLE }
#[repr(C)] pub enum idxd_wq_type { IDXD_WQT_NONE = 0, IDXD_WQT_KERNEL, IDXD_WQT_USER }
#[repr(C)] pub struct idxd_cdev { pub wq: *mut idxd_wq, pub cdev: cdev, pub idxd_dev: idxd_dev, pub minor: i32 }
pub const DRIVER_NAME_SIZE: usize = 128; pub const WQ_NAME_SIZE: usize = 1024; pub const WQ_TYPE_SIZE: usize = 10; pub const WQ_DEFAULT_QUEUE_DEPTH: u32 = 16; pub const WQ_DEFAULT_MAX_XFER: usize = SZ_2M; pub const WQ_DEFAULT_MAX_BATCH: u32 = 32;
#[repr(C)] pub enum idxd_op_type { IDXD_OP_BLOCK = 0, IDXD_OP_NONBLOCK = 1 }
#[repr(C)] pub struct idxd_dma_chan { pub chan: dma_chan, pub wq: *mut idxd_wq }

#[repr(C)] pub struct idxd_wq {
    pub portal: *mut core::ffi::c_void, pub portal_offset: u32, pub enqcmds_retries: u32, pub wq_active: percpu_ref, pub wq_dead: completion, pub wq_resurrect: completion, pub idxd_dev: idxd_dev, pub idxd_cdev: *mut idxd_cdev, pub err_queue: wait_queue_head_t, pub wq: *mut workqueue_struct, pub idxd: *mut idxd_device, pub id: i32, pub ie: idxd_irq_entry, pub type_: idxd_wq_type, pub group: *mut idxd_group, pub client_count: i32, pub wq_lock: mutex, pub size: u32, pub threshold: u32, pub priority: u32, pub state: idxd_wq_state, pub flags: usize, pub wqcfg: *mut wqcfg, pub opcap_bmap: *mut usize, pub hw_descs: *mut *mut dsa_hw_desc, pub num_descs: i32, pub completion: wq_completions, pub compls_addr: dma_addr_t, pub compls_size: i32, pub descs: *mut *mut idxd_desc, pub sbq: sbitmap_queue, pub idxd_chan: *mut idxd_dma_chan, pub name: [core::ffi::c_char; WQ_NAME_SIZE + 1], pub max_xfer_bytes: u64, pub max_batch_size: u32, pub max_sgl_size: u32, pub uc_lock: mutex, pub upasid_xa: xarray, pub driver_name: [core::ffi::c_char; DRIVER_NAME_SIZE + 1]
}
#[repr(C)] pub union wq_completions { pub compls: *mut dsa_completion_record, pub iax_compls: *mut iax_completion_record }
#[repr(C)] pub struct idxd_engine { pub idxd_dev: idxd_dev, pub id: i32, pub group: *mut idxd_group, pub idxd: *mut idxd_device }
#[repr(C)] pub struct idxd_hw { pub version: u32, pub gen_cap: gen_cap_reg, pub wq_cap: wq_cap_reg, pub group_cap: group_cap_reg, pub engine_cap: engine_cap_reg, pub opcap: opcap, pub cmd_cap: u32, pub iaa_cap: iaa_cap_reg, pub dsacap0: dsacap0_reg, pub dsacap1: dsacap1_reg, pub dsacap2: dsacap2_reg }
#[repr(C)] pub enum idxd_device_state { IDXD_DEV_HALTED = -1, IDXD_DEV_DISABLED = 0, IDXD_DEV_ENABLED }
#[repr(C)] pub enum idxd_device_flag { IDXD_FLAG_CONFIGURABLE = 0, IDXD_FLAG_CMD_RUNNING, IDXD_FLAG_PASID_ENABLED, IDXD_FLAG_USER_PASID_ENABLED }
#[repr(C)] pub struct idxd_dma_dev { pub idxd: *mut idxd_device, pub dma: dma_device }
pub type load_device_defaults_fn_t = unsafe extern "C" fn(*mut idxd_device) -> i32;
#[repr(C)] pub struct idxd_driver_data { pub name_prefix: *const core::ffi::c_char, pub type_: idxd_type, pub dev_type: *const device_type, pub compl_size: i32, pub align: i32, pub evl_cr_off: i32, pub cr_status_off: i32, pub cr_result_off: i32, pub user_submission_safe: bool, pub load_device_defaults: Option<load_device_defaults_fn_t> }
#[repr(C)] pub struct idxd_evl { pub lock: mutex, pub log: *mut core::ffi::c_void, pub dma: dma_addr_t, pub log_size: u32, pub size: u16, pub bmap: *mut usize, pub batch_fail: [bool; IDXD_MAX_BATCH_IDENT] }
#[repr(C)] pub struct idxd_evl_fault { pub work: work_struct, pub wq: *mut idxd_wq, pub status: u8, pub entry: [__evl_entry; 0] }

#[repr(C)] pub struct idxd_device { pub idxd_dev: idxd_dev, pub data: *mut idxd_driver_data, pub list: list_head, pub hw: idxd_hw, pub state: idxd_device_state, pub flags: usize, pub id: i32, pub major: i32, pub cmd_status: u32, pub ie: idxd_irq_entry, pub pdev: *mut pci_dev, pub reg_base: *mut core::ffi::c_void, pub dev_lock: spinlock_t, pub cmd_lock: spinlock_t, pub cmd_done: *mut completion, pub groups: *mut *mut idxd_group, pub wqs: *mut *mut idxd_wq, pub engines: *mut *mut idxd_engine, pub sva: *mut iommu_sva, pub pasid: u32, pub num_groups: i32, pub irq_cnt: i32, pub request_int_handles: bool, pub msix_perm_offset: u32, pub wqcfg_offset: u32, pub grpcfg_offset: u32, pub perfmon_offset: u32, pub max_xfer_bytes: u64, pub max_batch_size: u32, pub max_sgl_size: u32, pub max_groups: i32, pub max_engines: i32, pub max_rdbufs: i32, pub max_wqs: i32, pub max_wq_size: i32, pub rdbuf_limit: i32, pub nr_rdbufs: i32, pub wqcfg_size: u32, pub wq_enable_map: *mut usize, pub sw_err: sw_err_reg, pub cmd_waitq: wait_queue_head_t, pub idxd_dma: *mut idxd_dma_dev, pub wq: *mut workqueue_struct, pub work: work_struct, pub idxd_pmu: *mut idxd_pmu, pub opcap_bmap: *mut usize, pub evl: *mut idxd_evl, pub evl_cache: *mut kmem_cache, pub dbgfs_dir: *mut dentry, pub dbgfs_evl_file: *mut dentry, pub user_submission_safe: bool, pub idxd_saved: *mut idxd_saved_states }
#[repr(C)] pub struct idxd_saved_states { pub saved_idxd: idxd_device, pub saved_evl: idxd_evl, pub saved_engines: *mut *mut idxd_engine, pub saved_wqs: *mut *mut idxd_wq, pub saved_groups: *mut *mut idxd_group, pub saved_wq_enable_map: *mut usize }

pub unsafe fn evl_ent_size(idxd: *mut idxd_device) -> u32 { if (*idxd).hw.gen_cap.evl_support != 0 { 32 * (1u32 << (*idxd).hw.gen_cap.evl_support) } else { 0 } }
pub unsafe fn evl_size(idxd: *mut idxd_device) -> u32 { (*(*idxd).evl).size as u32 * evl_ent_size(idxd) }
#[repr(C)] pub struct crypto_ctx { pub req: *mut acomp_req, pub tfm: *mut crypto_tfm, pub src_addr: dma_addr_t, pub dst_addr: dma_addr_t, pub compress: bool }
#[repr(C)] pub union idxd_desc_payload { pub txd: dma_async_tx_descriptor, pub crypto: crypto_ctx }
#[repr(C)] pub struct idxd_desc_full { pub hw: idxd_desc_hw, pub desc_dma: dma_addr_t, pub completion: idxd_desc_completion, pub compl_dma: dma_addr_t, pub payload: idxd_desc_payload, pub llnode: llist_node, pub list: list_head, pub id: i32, pub cpu: i32, pub wq: *mut idxd_wq }
#[repr(C)] pub union idxd_desc_hw { pub hw: *mut dsa_hw_desc, pub iax_hw: *mut iax_hw_desc }
#[repr(C)] pub union idxd_desc_completion { pub completion: *mut dsa_completion_record, pub iax_completion: *mut iax_completion_record }
#[repr(C)] pub enum idxd_completion_status { IDXD_COMP_DESC_ABORT = 0xff }

pub unsafe fn idxd_set_max_batch_size(t: i32, d: *mut idxd_device, n: u32) { (*d).max_batch_size = if t == IDXD_TYPE_IAX as i32 { 0 } else { n }; }
pub unsafe fn idxd_wq_set_max_batch_size(t: i32, w: *mut idxd_wq, n: u32) { (*w).max_batch_size = if t == IDXD_TYPE_IAX as i32 { 0 } else { n }; }
pub unsafe fn idxd_wq_set_init_max_sgl_size(d: *mut idxd_device, w: *mut idxd_wq) { if (*d).data != core::ptr::null_mut() && (*(*d).data).type_ == idxd_type::IDXD_TYPE_DSA && (*d).hw.version >= DEVICE_VERSION_3 && (*d).hw.dsacap0.sgl_formats != 0 { (*w).max_sgl_size = 1u32 << (*d).hw.dsacap0.max_sgl_shift; } }
pub unsafe fn idxd_wqcfg_set_max_batch_shift(t: i32, c: *mut wqcfg, n: u32) { (*c).max_batch_shift = if t == IDXD_TYPE_IAX as i32 { 0 } else { n }; }

#[repr(C)] pub enum idxd_portal_prot { IDXD_PORTAL_UNLIMITED = 0, IDXD_PORTAL_LIMITED }
#[repr(C)] pub enum idxd_interrupt_type { IDXD_IRQ_MSIX = 0, IDXD_IRQ_IMS }
pub unsafe fn idxd_get_wq_portal_offset(p: idxd_portal_prot) -> i32 { p as i32 * 0x1000 }
pub unsafe fn idxd_get_wq_portal_full_offset(id: i32, p: idxd_portal_prot) -> i32 { ((id * 4) << PAGE_SHIFT) + idxd_get_wq_portal_offset(p) }
pub const IDXD_PORTAL_MASK: usize = PAGE_SIZE - 1;

extern "C" { pub static support_enqcmd: bool; pub static mut idxd_ida: ida; pub static dsa_bus_type: bus_type; }
pub unsafe fn is_dsa_dev(d: *mut idxd_dev) -> bool { (*d).type_ == idxd_dev_type::IDXD_DEV_DSA }
pub unsafe fn is_iax_dev(d: *mut idxd_dev) -> bool { (*d).type_ == idxd_dev_type::IDXD_DEV_IAX }
pub unsafe fn is_idxd_dev(d: *mut idxd_dev) -> bool { is_dsa_dev(d) || is_iax_dev(d) }
pub unsafe fn is_idxd_wq_dev(d: *mut idxd_dev) -> bool { (*d).type_ == idxd_dev_type::IDXD_DEV_WQ }
pub unsafe fn is_idxd_wq_user(w: *mut idxd_wq) -> bool { (*w).type_ == idxd_wq_type::IDXD_WQT_USER }
pub unsafe fn is_idxd_wq_kernel(w: *mut idxd_wq) -> bool { (*w).type_ == idxd_wq_type::IDXD_WQT_KERNEL }
pub unsafe fn wq_dedicated(_w: *mut idxd_wq) -> bool { true }
pub unsafe fn wq_shared(w: *mut idxd_wq) -> bool { !wq_dedicated(w) }
pub unsafe fn device_pasid_enabled(_d: *mut idxd_device) -> bool { true }
pub unsafe fn device_user_pasid_enabled(_d: *mut idxd_device) -> bool { true }
pub unsafe fn wq_pasid_enabled(w: *mut idxd_wq) -> bool { (is_idxd_wq_kernel(w) && device_pasid_enabled((*w).idxd)) || (is_idxd_wq_user(w) && device_user_pasid_enabled((*w).idxd)) }
pub unsafe fn wq_shared_supported(w: *mut idxd_wq) -> bool { support_enqcmd && wq_pasid_enabled(w) }

// External declarations from the Linux kernel and other repository headers.
extern "C" {
    pub fn idxd_free_desc(*mut idxd_wq, *mut idxd_desc);
    pub fn idxd_dma_complete_txd(*mut idxd_desc, idxd_complete_type, bool, *mut core::ffi::c_void, *mut u32);
    pub fn idxd_register_devices(*mut idxd_device) -> i32; pub fn idxd_unregister_devices(*mut idxd_device);
    pub fn idxd_wqs_quiesce(*mut idxd_device); pub fn idxd_queue_int_handle_resubmit(*mut idxd_desc) -> bool;
    pub fn idxd_submit_desc(*mut idxd_wq, *mut idxd_desc) -> i32; pub fn idxd_alloc_desc(*mut idxd_wq, idxd_op_type) -> *mut idxd_desc;
    pub fn idxd_enqcmds(*mut idxd_wq, *mut core::ffi::c_void, *const core::ffi::c_void) -> i32;
    pub fn idxd_register_dma_device(*mut idxd_device) -> i32; pub fn idxd_unregister_dma_device(*mut idxd_device);
    pub fn idxd_cdev_register() -> i32; pub fn idxd_cdev_remove(); pub fn idxd_cdev_get_major(*mut idxd_device) -> i32;
    pub fn idxd_device_drv_probe(*mut idxd_dev) -> i32; pub fn idxd_device_drv_remove(*mut idxd_dev);
    pub fn idxd_mask_error_interrupts(*mut idxd_device); pub fn idxd_unmask_error_interrupts(*mut idxd_device);
    pub fn idxd_pci_probe_alloc(*mut idxd_device, *mut pci_dev, *const pci_device_id) -> i32;
    pub fn idxd_drv_enable_wq(*mut idxd_wq) -> i32; pub fn idxd_drv_disable_wq(*mut idxd_wq);
    pub fn idxd_device_init_reset(*mut idxd_device) -> i32; pub fn idxd_device_enable(*mut idxd_device) -> i32;
    pub fn idxd_device_disable(*mut idxd_device) -> i32; pub fn idxd_device_reset(*mut idxd_device);
    pub fn idxd_device_clear_state(*mut idxd_device); pub fn idxd_device_config(*mut idxd_device) -> i32;
    pub fn idxd_device_drain_pasid(*mut idxd_device, i32); pub fn idxd_device_load_config(*mut idxd_device) -> i32;
    pub fn idxd_device_request_int_handle(*mut idxd_device, i32, *mut i32, idxd_interrupt_type) -> i32;
    pub fn idxd_device_release_int_handle(*mut idxd_device, i32, idxd_interrupt_type) -> i32;
    pub fn idxd_wqs_unmap_portal(*mut idxd_device); pub fn idxd_wq_alloc_resources(*mut idxd_wq) -> i32;
    pub fn idxd_wq_free_resources(*mut idxd_wq); pub fn idxd_wq_enable(*mut idxd_wq) -> i32;
    pub fn idxd_wq_disable(*mut idxd_wq, bool) -> i32; pub fn idxd_wq_drain(*mut idxd_wq);
    pub fn idxd_wq_reset(*mut idxd_wq); pub fn idxd_wq_map_portal(*mut idxd_wq) -> i32;
    pub fn idxd_wq_unmap_portal(*mut idxd_wq); pub fn idxd_wq_set_pasid(*mut idxd_wq, i32) -> i32;
    pub fn idxd_wq_disable_pasid(*mut idxd_wq) -> i32; pub fn __idxd_wq_quiesce(*mut idxd_wq);
    pub fn idxd_wq_quiesce(*mut idxd_wq); pub fn idxd_wq_init_percpu_ref(*mut idxd_wq) -> i32;
    pub fn idxd_wq_free_irq(*mut idxd_wq); pub fn idxd_wq_request_irq(*mut idxd_wq) -> i32;
    pub fn idxd_wq_flush_descs(*mut idxd_wq); pub fn idxd_dma_complete_txd(*mut idxd_desc, idxd_complete_type, bool, *mut core::ffi::c_void, *mut u32);
    pub fn idxd_copy_cr(*mut idxd_wq, ioasid_t, usize, *mut core::ffi::c_void, i32) -> i32;
    pub fn idxd_user_counter_increment(*mut idxd_wq, u32, i32);
    pub fn idxd_device_init_debugfs(*mut idxd_device) -> i32; pub fn idxd_device_remove_debugfs(*mut idxd_device);
    pub fn idxd_init_debugfs() -> i32; pub fn idxd_remove_debugfs();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
