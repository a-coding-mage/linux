/* SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only) */
/* Copyright(c) 2014 - 2020 Intel Corporation */

// C dependencies supplied by the surrounding translation unit:
// crypto/aes.h, linux/list.h, linux/slab.h, adf_accel_devices.h,
// icp_qat_fw_la.h, qat_algs_send.h, and qat_bl.h.

#[repr(C)]
pub struct qat_crypto_instance {
    pub sym_tx: *mut adf_etr_ring_data,
    pub sym_rx: *mut adf_etr_ring_data,
    pub pke_tx: *mut adf_etr_ring_data,
    pub pke_rx: *mut adf_etr_ring_data,
    pub accel_dev: *mut adf_accel_dev,
    pub list: list_head,
    pub state: ::core::ffi::c_ulong,
    pub id: ::core::ffi::c_int,
    pub refctr: atomic_t,
    pub backlog: qat_instance_backlog,
}

// C forward declaration: struct qat_crypto_request;

#[repr(C)]
pub union qat_crypto_request_ctx {
    pub aead_ctx: *mut qat_alg_aead_ctx,
    pub skcipher_ctx: *mut qat_alg_skcipher_ctx,
}

#[repr(C)]
pub union qat_crypto_request_req {
    pub aead_req: *mut aead_request,
    pub skcipher_req: *mut skcipher_request,
}

#[repr(C)]
pub union qat_crypto_request_iv {
    pub iv_hi: __be64,
    pub iv_lo: __be64,
    pub iv: [u8; AES_BLOCK_SIZE],
}

#[repr(C)]
pub struct qat_crypto_request {
    pub req: icp_qat_fw_la_bulk_req,
    pub ctx: qat_crypto_request_ctx,
    pub request: qat_crypto_request_req,
    pub buf: qat_request_buffs,
    pub cb: Option<unsafe extern "C" fn(
        resp: *mut icp_qat_fw_la_resp,
        req: *mut qat_crypto_request,
    )>,
    pub iv: qat_crypto_request_iv,
    pub encryption: bool,
    pub alg_req: qat_alg_req,
}

#[inline]
pub unsafe fn adf_hw_dev_has_crypto(accel_dev: *mut adf_accel_dev) -> bool {
    let hw_device = (*accel_dev).hw_device;
    let mask: u32 = !(*hw_device).accel_capabilities_mask;

    if mask & ADF_ACCEL_CAPABILITIES_CRYPTO_SYMMETRIC != 0 {
        return false;
    }
    if mask & ADF_ACCEL_CAPABILITIES_CRYPTO_ASYMMETRIC != 0 {
        return false;
    }
    if mask & ADF_ACCEL_CAPABILITIES_AUTHENTICATION != 0 {
        return false;
    }

    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
