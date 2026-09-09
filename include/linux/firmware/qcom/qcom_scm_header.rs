/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (c) 2010-2015, 2018-2019 The Linux Foundation. All rights reserved.
 * Copyright (C) 2015 Linaro Ltd.
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct qcom_scm_hdcp_req {
    pub addr: u32,
    pub val: u32,
}

#[repr(C)]
pub struct qcom_scm_vmperm {
    pub vmid: i32,
    pub perm: i32,
}

#[repr(C)]
pub struct qcom_scm_pas_context {
    pub dev: *mut device,
    pub pas_id: u32,
    pub mem_phys: phys_addr_t,
    pub mem_size: usize,
    pub ptr: *mut core::ffi::c_void,
    pub phys: dma_addr_t,
    pub size: isize,
    pub use_tzmem: bool,
}

#[repr(u32)]
pub enum qcom_scm_ocmem_client {
    QCOM_SCM_OCMEM_UNUSED_ID = 0x0,
    QCOM_SCM_OCMEM_GRAPHICS_ID,
    QCOM_SCM_OCMEM_VIDEO_ID,
    QCOM_SCM_OCMEM_LP_AUDIO_ID,
    QCOM_SCM_OCMEM_SENSORS_ID,
    QCOM_SCM_OCMEM_OTHER_OS_ID,
    QCOM_SCM_OCMEM_DEBUG_ID,
}

#[repr(u32)]
pub enum qcom_scm_sec_dev_id {
    QCOM_SCM_MDSS_DEV_ID = 1,
    QCOM_SCM_OCMEM_DEV_ID = 5,
    QCOM_SCM_PCIE0_DEV_ID = 11,
    QCOM_SCM_PCIE1_DEV_ID = 12,
    QCOM_SCM_GFX_DEV_ID = 18,
    QCOM_SCM_UFS_DEV_ID = 19,
    QCOM_SCM_ICE_DEV_ID = 20,
}

#[repr(u32)]
pub enum qcom_scm_ice_cipher {
    QCOM_SCM_ICE_CIPHER_AES_128_XTS = 0,
    QCOM_SCM_ICE_CIPHER_AES_128_CBC = 1,
    QCOM_SCM_ICE_CIPHER_AES_256_XTS = 3,
    QCOM_SCM_ICE_CIPHER_AES_256_CBC = 4,
}

pub const QCOM_SCM_CPU_PWR_DOWN_L2_ON: u32 = 0x0;
pub const QCOM_SCM_CPU_PWR_DOWN_L2_OFF: u32 = 0x1;
pub const QCOM_SCM_HDCP_MAX_REQ_CNT: u32 = 5;
pub const QCOM_SCM_PERM_READ: u32 = 0x4;
pub const QCOM_SCM_PERM_WRITE: u32 = 0x2;
pub const QCOM_SCM_PERM_EXEC: u32 = 0x1;
pub const QCOM_SCM_PERM_RW: u32 = QCOM_SCM_PERM_READ | QCOM_SCM_PERM_WRITE;
pub const QCOM_SCM_PERM_RWX: u32 = QCOM_SCM_PERM_RW | QCOM_SCM_PERM_EXEC;
pub const QCOM_SCM_GPU_ALWAYS_EN_REQ: u32 = 1 << 0;
pub const QCOM_SCM_GPU_BCL_EN_REQ: u32 = 1 << 1;
pub const QCOM_SCM_GPU_CLX_EN_REQ: u32 = 1 << 2;
pub const QCOM_SCM_GPU_TSENSE_EN_REQ: u32 = 1 << 3;

#[inline]
pub const fn QCOM_SCM_VERSION(major: u32, minor: u32) -> u32 {
    (major << 16) | (minor & 0xFF)
}

extern "C" {
    pub fn qcom_scm_is_available() -> bool;
    pub fn qcom_scm_set_cold_boot_addr(entry: *mut core::ffi::c_void) -> i32;
    pub fn qcom_scm_set_warm_boot_addr(entry: *mut core::ffi::c_void) -> i32;
    pub fn qcom_scm_cpu_power_down(flags: u32);
    pub fn qcom_scm_set_remote_state(state: u32, id: u32) -> i32;
    pub fn devm_qcom_scm_pas_context_alloc(dev: *mut device, pas_id: u32, mem_phys: phys_addr_t, mem_size: usize) -> *mut qcom_scm_pas_context;
    pub fn qcom_scm_pas_init_image(pas_id: u32, metadata: *const core::ffi::c_void, size: usize, ctx: *mut qcom_scm_pas_context) -> i32;
    pub fn qcom_scm_pas_metadata_release(ctx: *mut qcom_scm_pas_context);
    pub fn qcom_scm_pas_mem_setup(pas_id: u32, addr: phys_addr_t, size: phys_addr_t) -> i32;
    pub fn qcom_scm_pas_auth_and_reset(pas_id: u32) -> i32;
    pub fn qcom_scm_pas_shutdown(pas_id: u32) -> i32;
    pub fn qcom_scm_pas_supported(pas_id: u32) -> bool;
    pub fn qcom_scm_pas_get_rsc_table(ctx: *mut qcom_scm_pas_context, input_rt: *mut core::ffi::c_void, input_rt_size: usize, output_rt_size: *mut usize) -> *mut resource_table;
    pub fn qcom_scm_pas_prepare_and_auth_reset(ctx: *mut qcom_scm_pas_context) -> i32;
    pub fn qcom_scm_io_readl(addr: phys_addr_t, val: *mut u32) -> i32;
    pub fn qcom_scm_io_writel(addr: phys_addr_t, val: u32) -> i32;
    pub fn qcom_scm_restore_sec_cfg_available() -> bool;
    pub fn qcom_scm_restore_sec_cfg(device_id: u32, spare: u32) -> i32;
    pub fn qcom_scm_set_gpu_smmu_aperture(context_bank: u32) -> i32;
    pub fn qcom_scm_set_gpu_smmu_aperture_is_available() -> bool;
    pub fn qcom_scm_iommu_secure_ptbl_size(spare: u32, size: *mut usize) -> i32;
    pub fn qcom_scm_iommu_secure_ptbl_init(addr: u64, size: u32, spare: u32) -> i32;
    pub fn qcom_scm_iommu_set_cp_pool_size(spare: u32, size: u32) -> i32;
    pub fn qcom_scm_mem_protect_video_var(cp_start: u32, cp_size: u32, cp_nonpixel_start: u32, cp_nonpixel_size: u32) -> i32;
    pub fn qcom_scm_assign_mem(mem_addr: phys_addr_t, mem_sz: usize, src: *mut u64, newvm: *const qcom_scm_vmperm, dest_cnt: u32) -> i32;
    pub fn qcom_scm_ocmem_lock_available() -> bool;
    pub fn qcom_scm_ocmem_lock(id: qcom_scm_ocmem_client, offset: u32, size: u32, mode: u32) -> i32;
    pub fn qcom_scm_ocmem_unlock(id: qcom_scm_ocmem_client, offset: u32, size: u32) -> i32;
    pub fn qcom_scm_ice_available() -> bool;
    pub fn qcom_scm_ice_invalidate_key(index: u32) -> i32;
    pub fn qcom_scm_ice_set_key(index: u32, key: *const u8, key_size: u32, cipher: qcom_scm_ice_cipher, data_unit_size: u32) -> i32;
    pub fn qcom_scm_has_wrapped_key_support() -> bool;
    pub fn qcom_scm_derive_sw_secret(eph_key: *const u8, eph_key_size: usize, sw_secret: *mut u8, sw_secret_size: usize) -> i32;
    pub fn qcom_scm_generate_ice_key(lt_key: *mut u8, lt_key_size: usize) -> i32;
    pub fn qcom_scm_prepare_ice_key(lt_key: *const u8, lt_key_size: usize, eph_key: *mut u8, eph_key_size: usize) -> i32;
    pub fn qcom_scm_import_ice_key(raw_key: *const u8, raw_key_size: usize, lt_key: *mut u8, lt_key_size: usize) -> i32;
    pub fn qcom_scm_hdcp_available() -> bool;
    pub fn qcom_scm_hdcp_req(req: *mut qcom_scm_hdcp_req, req_cnt: u32, resp: *mut u32) -> i32;
    pub fn qcom_scm_iommu_set_pt_format(sec_id: u32, ctx_num: u32, pt_fmt: u32) -> i32;
    pub fn qcom_scm_qsmmu500_wait_safe_toggle(en: bool) -> i32;
    pub fn qcom_scm_lmh_dcvsh(payload_fn: u32, payload_reg: u32, payload_val: u32, limit_node: u64, node_id: u32, version: u64) -> i32;
    pub fn qcom_scm_lmh_profile_change(profile_id: u32) -> i32;
    pub fn qcom_scm_lmh_dcvsh_available() -> bool;
    pub fn qcom_scm_gpu_init_regs(gpu_req: u32) -> i32;
    pub fn qcom_scm_shm_bridge_create(pfn_and_ns_perm_flags: u64, ipfn_and_s_perm_flags: u64, size_and_flags: u64, ns_vmids: u64, handle: *mut u64) -> i32;
    pub fn qcom_scm_shm_bridge_delete(handle: u64) -> i32;
}

#[cfg(feature = "CONFIG_QCOM_QSEECOM")]
extern "C" {
    pub fn qcom_scm_qseecom_app_get_id(app_name: *const core::ffi::c_char, app_id: *mut u32) -> i32;
    pub fn qcom_scm_qseecom_app_send(app_id: u32, req: *mut core::ffi::c_void, req_size: usize, rsp: *mut core::ffi::c_void, rsp_size: usize) -> i32;
}

#[cfg(not(feature = "CONFIG_QCOM_QSEECOM"))]
pub unsafe fn qcom_scm_qseecom_app_get_id(_app_name: *const core::ffi::c_char, _app_id: *mut u32) -> i32 { -22 }

#[cfg(not(feature = "CONFIG_QCOM_QSEECOM"))]
pub unsafe fn qcom_scm_qseecom_app_send(_app_id: u32, _req: *mut core::ffi::c_void, _req_size: usize, _rsp: *mut core::ffi::c_void, _rsp_size: usize) -> i32 { -22 }

extern "C" {
    pub fn qcom_scm_qtee_invoke_smc(inbuf: phys_addr_t, inbuf_size: usize, outbuf: phys_addr_t, outbuf_size: usize, result: *mut u64, response_type: *mut u64) -> i32;
    pub fn qcom_scm_qtee_callback_response(buf: phys_addr_t, buf_size: usize, result: *mut u64, response_type: *mut u64) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
