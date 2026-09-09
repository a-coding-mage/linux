/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2022-2024, Advanced Micro Devices, Inc.
 */

// Dependencies supplied by the surrounding kernel/DRM translation are intentionally external.

#[macro_export]
macro_rules! XDNA_INFO { ($xdna:expr, $($args:tt)*) => { drm_info!(&mut ($xdna).ddev, $($args)*) }; }
#[macro_export]
macro_rules! XDNA_WARN { ($xdna:expr, $($args:tt)*) => { drm_warn!(&mut ($xdna).ddev, concat!("{}: ", $($args)*), core::module_path!()) }; }
#[macro_export]
macro_rules! XDNA_ERR { ($xdna:expr, $($args:tt)*) => { drm_err!(&mut ($xdna).ddev, concat!("{}: ", $($args)*), core::module_path!()) }; }
#[macro_export]
macro_rules! XDNA_DBG { ($xdna:expr, $($args:tt)*) => { drm_dbg!(&mut ($xdna).ddev, $($args)*) }; }
#[macro_export]
macro_rules! XDNA_INFO_ONCE { ($xdna:expr, $($args:tt)*) => { drm_info_once!(&mut ($xdna).ddev, $($args)*) }; }

#[macro_export]
macro_rules! XDNA_MBZ_DBG {
    ($xdna:expr, $ptr:expr, $sz:expr) => {{
        let mut __ret: i32 = 0;
        let __ptr = $ptr as *const u8;
        for __i in 0..($sz as usize) {
            if unsafe { *__ptr.add(__i) } != 0 {
                XDNA_DBG!($xdna, "MBZ check failed");
                __ret = -EINVAL;
                break;
            }
        }
        __ret
    }};
}

#[macro_export]
macro_rules! to_xdna_dev {
    ($drm_dev:expr) => { container_of!($drm_dev, amdxdna_dev, ddev) as *mut amdxdna_dev };
}

extern "C" {
    pub static amdxdna_drm_drv: drm_driver;
}

#[repr(C)] pub struct amdxdna_client;
#[repr(C)] pub struct amdxdna_dev;
#[repr(C)] pub struct amdxdna_drm_get_info;
#[repr(C)] pub struct amdxdna_drm_set_state;
#[repr(C)] pub struct amdxdna_gem_obj;
#[repr(C)] pub struct amdxdna_hwctx;
#[repr(C)] pub struct amdxdna_sched_job;

/* struct amdxdna_dev_ops - Device hardware operation callbacks */
#[repr(C)]
pub struct amdxdna_dev_ops {
    pub init: Option<unsafe extern "C" fn(*mut amdxdna_dev) -> i32>,
    pub fini: Option<unsafe extern "C" fn(*mut amdxdna_dev)>,
    pub resume: Option<unsafe extern "C" fn(*mut amdxdna_dev) -> i32>,
    pub suspend: Option<unsafe extern "C" fn(*mut amdxdna_dev) -> i32>,
    pub sriov_configure: Option<unsafe extern "C" fn(*mut amdxdna_dev, i32) -> i32>,
    pub mmap: Option<unsafe extern "C" fn(*mut amdxdna_client, *mut vm_area_struct) -> i32>,
    pub hwctx_init: Option<unsafe extern "C" fn(*mut amdxdna_hwctx) -> i32>,
    pub hwctx_fini: Option<unsafe extern "C" fn(*mut amdxdna_hwctx)>,
    pub hwctx_config: Option<unsafe extern "C" fn(*mut amdxdna_hwctx, u32, u64, *mut core::ffi::c_void, u32) -> i32>,
    pub hwctx_sync_debug_bo: Option<unsafe extern "C" fn(*mut amdxdna_hwctx, u32) -> i32>,
    pub hwctx_heap_expand: Option<unsafe extern "C" fn(*mut amdxdna_hwctx, *mut amdxdna_gem_obj) -> i32>,
    pub hmm_invalidate: Option<unsafe extern "C" fn(*mut amdxdna_gem_obj, usize)>,
    pub cmd_submit: Option<unsafe extern "C" fn(*mut amdxdna_hwctx, *mut amdxdna_sched_job, *mut u64) -> i32>,
    pub cmd_wait: Option<unsafe extern "C" fn(*mut amdxdna_hwctx, u64, u32) -> i32>,
    pub get_aie_info: Option<unsafe extern "C" fn(*mut amdxdna_client, *mut amdxdna_drm_get_info) -> i32>,
    pub set_aie_state: Option<unsafe extern "C" fn(*mut amdxdna_client, *mut amdxdna_drm_set_state) -> i32>,
    pub get_array: Option<unsafe extern "C" fn(*mut amdxdna_client, *mut amdxdna_drm_get_array) -> i32>,
    pub get_dev_revision: Option<unsafe extern "C" fn(*mut amdxdna_dev, *mut u32) -> i32>,
}

#[repr(C)]
pub struct amdxdna_fw_feature_tbl { pub features: u64, pub major: u32, pub max_minor: u32, pub min_minor: u32 }

/* struct amdxdna_dev_info - Device hardware information
 * Record device static information, like reg, mbox, PSP, SMU bar index */
#[repr(C)]
pub struct amdxdna_dev_info {
    pub reg_bar: i32, pub mbox_bar: i32, pub sram_bar: i32, pub psp_bar: i32,
    pub smu_bar: i32, pub doorbell_bar: i32, pub device_type: i32, pub first_col: i32,
    pub dev_mem_buf_shift: u32, pub dev_mem_base: u64, pub dev_mem_size: usize,
    pub default_vbnv: *const i8, pub rev_vbnv_tbl: *const amdxdna_rev_vbnv,
    pub dev_heap_max_size: usize, pub dev_priv: *const amdxdna_dev_priv,
    pub fw_feature_tbl: *const amdxdna_fw_feature_tbl, pub ops: *const amdxdna_dev_ops,
}

#[repr(C)] pub struct amdxdna_fw_ver { pub major: u32, pub minor: u32, pub sub: u32, pub build: u32 }
#[repr(C)] pub struct amdxdna_carveout;

#[repr(C)]
pub struct amdxdna_dev {
    pub ddev: drm_device, pub dev_handle: *mut amdxdna_dev_hdl,
    pub dev_info: *const amdxdna_dev_info, pub xrs_hdl: *mut core::ffi::c_void,
    pub dev_lock: mutex, pub client_list: list_head, pub client_lock: mutex,
    pub fw_ver: amdxdna_fw_ver, pub notifier_lock: rw_semaphore,
    pub notifier_wq: *mut workqueue_struct, pub group: *mut iommu_group,
    pub domain: *mut iommu_domain, pub iovad: iova_domain, pub vbnv: *const i8,
    pub carveout: *mut amdxdna_carveout,
}

#[repr(C)] pub struct amdxdna_device_id { pub device: u16, pub revision: u8, pub dev_info: *const amdxdna_dev_info }

#[repr(C)]
pub struct amdxdna_client {
    pub node: list_head, pub pid: pid_t, pub hwctx_srcu: srcu_struct, pub hwctx_xa: xarray,
    pub next_hwctxid: u32, pub xdna: *mut amdxdna_dev, pub filp: *mut drm_file,
    pub mm_lock: mutex, pub dev_heap_xa: xarray, pub dev_heap_mm: drm_mm,
    pub dev_heap_nid: u32, pub total_heap_size: usize, pub sva: *mut iommu_sva,
    pub pasid: i32, pub mm: *mut mm_struct, pub heap_usage: usize,
    pub total_bo_usage: usize, pub total_int_bo_usage: usize,
}

#[macro_export]
macro_rules! amdxdna_for_each_hwctx { ($client:expr, $hwctx_id:expr, $entry:expr) => { xa_for_each!(&($client).hwctx_xa, $hwctx_id, $entry) }; }

extern "C" {
    pub static dev_npu1_info: amdxdna_dev_info;
    pub static dev_npu3_pf_info: amdxdna_dev_info;
    pub static dev_npu3_vf_info: amdxdna_dev_info;
    pub static dev_npu4_info: amdxdna_dev_info;
    pub static dev_npu5_info: amdxdna_dev_info;
    pub static dev_npu6_info: amdxdna_dev_info;
    pub fn amdxdna_sysfs_init(xdna: *mut amdxdna_dev) -> i32;
    pub fn amdxdna_sysfs_fini(xdna: *mut amdxdna_dev);
    pub fn amdxdna_iommu_init(xdna: *mut amdxdna_dev) -> i32;
    pub fn amdxdna_iommu_fini(xdna: *mut amdxdna_dev);
    pub fn amdxdna_iommu_alloc(xdna: *mut amdxdna_dev, size: usize, dma_addr: *mut dma_addr_t) -> *mut core::ffi::c_void;
    pub fn amdxdna_iommu_free(xdna: *mut amdxdna_dev, size: usize, cpu_addr: *mut core::ffi::c_void, dma_addr: dma_addr_t);
    pub fn amdxdna_dma_map_bo(xdna: *mut amdxdna_dev, abo: *mut amdxdna_gem_obj) -> i32;
    pub fn amdxdna_dma_unmap_bo(xdna: *mut amdxdna_dev, abo: *mut amdxdna_gem_obj);
}

#[inline]
pub unsafe fn amdxdna_iova_on(xdna: *mut amdxdna_dev) -> bool { !(*xdna).domain.is_null() }

#[inline]
pub unsafe fn amdxdna_pasid_on(client: *mut amdxdna_client) -> bool { (*client).pasid != IOMMU_PASID_INVALID }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
