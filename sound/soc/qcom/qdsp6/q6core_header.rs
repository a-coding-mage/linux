// SPDX-License-Identifier: GPL-2.0

#[repr(C)]
pub struct q6core_svc_api_info {
    pub service_id: u32,
    pub api_version: u32,
    pub api_branch_version: u32,
}

unsafe extern "C" {
    pub fn q6core_is_adsp_ready() -> bool;
    pub fn q6core_get_svc_api_info(svc_id: i32, ainfo: *mut q6core_svc_api_info) -> i32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
