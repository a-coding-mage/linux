/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Firmware layer for XilSECURE APIs.
 *
 * Copyright (C) 2014-2022 Xilinx, Inc.
 * Copyright (C) 2022-2025 Advanced Micro Devices, Inc.
 */

/// Feature data.
#[repr(C)]
pub struct xlnx_feature {
    pub family: u32,
    pub feature_id: u32,
    pub data: *mut core::ffi::c_void,
}

/* xilSecure API commands module id + api id */
pub const XSECURE_API_AES_INIT: u32 = 0x509;
pub const XSECURE_API_AES_OP_INIT: u32 = 0x50a;
pub const XSECURE_API_AES_UPDATE_AAD: u32 = 0x50b;
pub const XSECURE_API_AES_ENCRYPT_UPDATE: u32 = 0x50c;
pub const XSECURE_API_AES_ENCRYPT_FINAL: u32 = 0x50d;
pub const XSECURE_API_AES_DECRYPT_UPDATE: u32 = 0x50e;
pub const XSECURE_API_AES_DECRYPT_FINAL: u32 = 0x50f;
pub const XSECURE_API_AES_KEY_ZERO: u32 = 0x510;
pub const XSECURE_API_AES_WRITE_KEY: u32 = 0x511;

/* Corresponds to: IS_REACHABLE(CONFIG_ZYNQMP_FIRMWARE). */
#[cfg(feature = "zynqmp_firmware")]
extern "C" {
    pub fn zynqmp_pm_aes_engine(address: u64, out: *mut u32) -> i32;
    pub fn zynqmp_pm_sha_hash(address: u64, size: u32, flags: u32) -> i32;
    pub fn xlnx_get_crypto_dev_data(feature_map: *mut xlnx_feature)
        -> *mut core::ffi::c_void;
    pub fn versal_pm_aes_key_write(keylen: u32, keysrc: u32, keyaddr: u64) -> i32;
    pub fn versal_pm_aes_key_zero(keysrc: u32) -> i32;
    pub fn versal_pm_aes_op_init(hw_req: u64) -> i32;
    pub fn versal_pm_aes_update_aad(aad_addr: u64, aad_len: u32) -> i32;
    pub fn versal_pm_aes_enc_update(in_params: u64, in_addr: u64) -> i32;
    pub fn versal_pm_aes_dec_update(in_params: u64, in_addr: u64) -> i32;
    pub fn versal_pm_aes_dec_final(gcm_addr: u64) -> i32;
    pub fn versal_pm_aes_enc_final(gcm_addr: u64) -> i32;
    pub fn versal_pm_aes_init() -> i32;
}

#[cfg(not(feature = "zynqmp_firmware"))]
pub unsafe fn zynqmp_pm_aes_engine(_address: u64, _out: *mut u32) -> i32 { -19 }
#[cfg(not(feature = "zynqmp_firmware"))]
pub unsafe fn zynqmp_pm_sha_hash(_address: u64, _size: u32, _flags: u32) -> i32 { -19 }
#[cfg(not(feature = "zynqmp_firmware"))]
pub unsafe fn xlnx_get_crypto_dev_data(_feature_map: *mut xlnx_feature) -> *mut core::ffi::c_void {
    (-19isize) as *mut core::ffi::c_void
}
#[cfg(not(feature = "zynqmp_firmware"))]
pub unsafe fn versal_pm_aes_key_write(_keylen: u32, _keysrc: u32, _keyaddr: u64) -> i32 { -19 }
#[cfg(not(feature = "zynqmp_firmware"))]
pub unsafe fn versal_pm_aes_key_zero(_keysrc: u32) -> i32 { -19 }
#[cfg(not(feature = "zynqmp_firmware"))]
pub unsafe fn versal_pm_aes_op_init(_hw_req: u64) -> i32 { -19 }
#[cfg(not(feature = "zynqmp_firmware"))]
pub unsafe fn versal_pm_aes_update_aad(_aad_addr: u64, _aad_len: u32) -> i32 { -19 }
#[cfg(not(feature = "zynqmp_firmware"))]
pub unsafe fn versal_pm_aes_enc_update(_in_params: u64, _in_addr: u64) -> i32 { -19 }
#[cfg(not(feature = "zynqmp_firmware"))]
pub unsafe fn versal_pm_aes_dec_update(_in_params: u64, _in_addr: u64) -> i32 { -19 }
#[cfg(not(feature = "zynqmp_firmware"))]
pub unsafe fn versal_pm_aes_enc_final(_gcm_addr: u64) -> i32 { -19 }
#[cfg(not(feature = "zynqmp_firmware"))]
pub unsafe fn versal_pm_aes_dec_final(_gcm_addr: u64) -> i32 { -19 }
#[cfg(not(feature = "zynqmp_firmware"))]
pub unsafe fn versal_pm_aes_init() -> i32 { -19 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
