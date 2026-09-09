/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the C header BOOT_UV_H.

unsafe extern "C" {
    pub fn adjust_to_uv_max(limit: core::ffi::c_ulong) -> core::ffi::c_ulong;
    pub fn sanitize_prot_virt_host();
    pub fn uv_query_info();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
