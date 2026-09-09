/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: __QCOM_SMEM_H__

pub const QCOM_SMEM_HOST_ANY: i32 = -1;

unsafe extern "C" {
    pub fn qcom_smem_is_available() -> bool;
    pub fn qcom_smem_alloc(host: u32, item: u32, size: usize) -> i32;
    pub fn qcom_smem_get(host: u32, item: u32, size: *mut usize) -> *mut core::ffi::c_void;

    pub fn qcom_smem_get_free_space(host: u32) -> i32;

    pub fn qcom_smem_virt_to_phys(p: *mut core::ffi::c_void) -> phys_addr_t;

    pub fn qcom_smem_get_soc_id(id: *mut u32) -> i32;
    pub fn qcom_smem_get_feature_code(code: *mut u32) -> i32;

    pub fn qcom_smem_bust_hwspin_lock_by_host(host: u32) -> i32;

    pub fn qcom_smem_dram_get_hbb() -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
