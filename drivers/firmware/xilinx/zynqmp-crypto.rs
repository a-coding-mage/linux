// SPDX-License-Identifier: GPL-2.0
/*
 * Firmware layer for XilSecure APIs.
 *
 * Copyright (C) 2014-2022 Xilinx, Inc.
 * Copyright (C) 2022-2025 Advanced Micro Devices, Inc.
 */

// Dependencies supplied by <linux/firmware/xlnx-zynqmp.h> and <linux/module.h>.

use core::ffi::c_void;

const PAYLOAD_ARG_CNT: usize = 4;

unsafe extern "C" {
    fn zynqmp_pm_invoke_fn(id: u32, ret_payload: *mut u32, count: u32, ... ) -> i32;
    fn zynqmp_pm_get_family_info(code: *mut u32) -> i32;
    fn zynqmp_pm_feature(feature_id: u32) -> i32;
}

#[repr(C)]
pub struct xlnx_feature {
    pub family: u32,
    pub feature_id: u32,
    pub data: *mut c_void,
}

unsafe extern "C" {
    static PM_SECURE_AES: u32;
    static PM_SECURE_SHA: u32;
    static XSECURE_API_AES_WRITE_KEY: u32;
    static XSECURE_API_AES_KEY_ZERO: u32;
    static XSECURE_API_AES_OP_INIT: u32;
    static XSECURE_API_AES_UPDATE_AAD: u32;
    static XSECURE_API_AES_ENCRYPT_UPDATE: u32;
    static XSECURE_API_AES_ENCRYPT_FINAL: u32;
    static XSECURE_API_AES_DECRYPT_UPDATE: u32;
    static XSECURE_API_AES_DECRYPT_FINAL: u32;
    static XSECURE_API_AES_INIT: u32;
}

#[inline]
fn lower_32_bits(value: u64) -> u32 { value as u32 }

#[inline]
fn upper_32_bits(value: u64) -> u32 { (value >> 32) as u32 }

#[inline]
fn err_ptr(error: i32) -> *mut c_void { error as isize as *mut c_void }

pub unsafe fn zynqmp_pm_aes_engine(address: u64, out: *mut u32) -> i32 {
    let mut ret_payload = [0u32; PAYLOAD_ARG_CNT];
    let ret: i32;

    if out.is_null() {
        return -22;
    }

    ret = zynqmp_pm_invoke_fn(
        PM_SECURE_AES, ret_payload.as_mut_ptr(), 2,
        upper_32_bits(address), lower_32_bits(address));
    *out = ret_payload[1];

    ret
}

pub unsafe fn zynqmp_pm_sha_hash(address: u64, size: u32, flags: u32) -> i32 {
    let lower_addr = lower_32_bits(address);
    let upper_addr = upper_32_bits(address);

    zynqmp_pm_invoke_fn(PM_SECURE_SHA, core::ptr::null_mut(), 4,
                        upper_addr, lower_addr, size, flags)
}

pub unsafe fn xlnx_get_crypto_dev_data(feature_map: *mut xlnx_feature) -> *mut c_void {
    let mut feature: *mut xlnx_feature;
    let mut pm_family_code = 0u32;
    let ret = zynqmp_pm_get_family_info(&mut pm_family_code);
    if ret < 0 {
        return err_ptr(ret);
    }

    feature = feature_map;
    while (*feature).family != 0 {
        if (*feature).family == pm_family_code {
            let ret = zynqmp_pm_feature((*feature).feature_id);
            if ret < 0 {
                return err_ptr(ret);
            }
            return (*feature).data;
        }
        feature = feature.add(1);
    }
    err_ptr(-19)
}

pub unsafe fn versal_pm_aes_key_write(keylen: u32, keysrc: u32, keyaddr: u64) -> i32 {
    zynqmp_pm_invoke_fn(XSECURE_API_AES_WRITE_KEY, core::ptr::null_mut(), 4,
                        keylen, keysrc, lower_32_bits(keyaddr), upper_32_bits(keyaddr))
}

pub unsafe fn versal_pm_aes_key_zero(keysrc: u32) -> i32 {
    zynqmp_pm_invoke_fn(XSECURE_API_AES_KEY_ZERO, core::ptr::null_mut(), 1, keysrc)
}

pub unsafe fn versal_pm_aes_op_init(hw_req: u64) -> i32 {
    zynqmp_pm_invoke_fn(XSECURE_API_AES_OP_INIT, core::ptr::null_mut(), 2,
                        lower_32_bits(hw_req), upper_32_bits(hw_req))
}

pub unsafe fn versal_pm_aes_update_aad(aad_addr: u64, aad_len: u32) -> i32 {
    zynqmp_pm_invoke_fn(XSECURE_API_AES_UPDATE_AAD, core::ptr::null_mut(), 3,
                        lower_32_bits(aad_addr), upper_32_bits(aad_addr), aad_len)
}

pub unsafe fn versal_pm_aes_enc_update(in_params: u64, in_addr: u64) -> i32 {
    zynqmp_pm_invoke_fn(XSECURE_API_AES_ENCRYPT_UPDATE, core::ptr::null_mut(), 4,
                        lower_32_bits(in_params), upper_32_bits(in_params),
                        lower_32_bits(in_addr), upper_32_bits(in_addr))
}

pub unsafe fn versal_pm_aes_enc_final(gcm_addr: u64) -> i32 {
    zynqmp_pm_invoke_fn(XSECURE_API_AES_ENCRYPT_FINAL, core::ptr::null_mut(), 2,
                        lower_32_bits(gcm_addr), upper_32_bits(gcm_addr))
}

pub unsafe fn versal_pm_aes_dec_update(in_params: u64, in_addr: u64) -> i32 {
    zynqmp_pm_invoke_fn(XSECURE_API_AES_DECRYPT_UPDATE, core::ptr::null_mut(), 4,
                        lower_32_bits(in_params), upper_32_bits(in_params),
                        lower_32_bits(in_addr), upper_32_bits(in_addr))
}

pub unsafe fn versal_pm_aes_dec_final(gcm_addr: u64) -> i32 {
    zynqmp_pm_invoke_fn(XSECURE_API_AES_DECRYPT_FINAL, core::ptr::null_mut(), 2,
                        lower_32_bits(gcm_addr), upper_32_bits(gcm_addr))
}

pub unsafe fn versal_pm_aes_init() -> i32 {
    zynqmp_pm_invoke_fn(XSECURE_API_AES_INIT, core::ptr::null_mut(), 0)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
