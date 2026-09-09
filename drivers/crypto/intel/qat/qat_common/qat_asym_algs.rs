// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
/* Copyright(c) 2014 - 2020 Intel Corporation */
// Direct low-level translation of qat_asym_algs.c. Kernel and firmware symbols
// referenced below are supplied by the surrounding Linux/Rust integration.

use core::ffi::c_void;

extern "C" {
    static mut active_devs: u32;
    fn qat_alg_send_message(req: *mut qat_alg_req) -> i32;
    fn qat_alg_send_backlog(backlog: *mut qat_instance_backlog);
}

#[repr(C, align(64))]
pub struct qat_rsa_input_params { pub in_tab: [u64; 8] }
#[repr(C, align(64))]
pub struct qat_rsa_output_params { pub out_tab: [u64; 8] }
#[repr(C, align(64))]
pub struct qat_rsa_ctx {
    pub n: *mut i8, pub e: *mut i8, pub d: *mut i8, pub p: *mut i8,
    pub q: *mut i8, pub dp: *mut i8, pub dq: *mut i8, pub qinv: *mut i8,
    pub dma_n: u64, pub dma_e: u64, pub dma_d: u64, pub dma_p: u64,
    pub dma_q: u64, pub dma_dp: u64, pub dma_dq: u64, pub dma_qinv: u64,
    pub key_sz: u32, pub crt_mode: bool, pub inst: *mut qat_crypto_instance,
}
#[repr(C, align(64))]
pub struct qat_dh_input_params { pub in_tab: [u64; 8] }
#[repr(C, align(64))]
pub struct qat_dh_output_params { pub out_tab: [u64; 8] }
#[repr(C, align(64))]
pub struct qat_dh_ctx {
    pub g: *mut i8, pub xa: *mut i8, pub p: *mut i8,
    pub dma_g: u64, pub dma_xa: u64, pub dma_p: u64, pub p_size: u32,
    pub g2: bool, pub inst: *mut qat_crypto_instance, pub ftfm: *mut crypto_kpp,
    pub fallback: bool,
}
#[repr(C, align(64))]
pub struct qat_asym_request {
    pub in_: qat_dh_input_params, pub out: qat_dh_output_params,
    pub phy_in: u64, pub phy_out: u64, pub src_align: *mut i8,
    pub dst_align: *mut i8, pub req: icp_qat_fw_pke_request,
    pub ctx: *mut c_void, pub areq: *mut c_void, pub err: i32,
    pub cb: Option<unsafe extern "C" fn(*mut icp_qat_fw_pke_resp)>,
    pub alg_req: qat_alg_req,
}

// Opaque dependency types (defined by the corresponding kernel/QAT headers).
#[repr(C)] pub struct qat_crypto_instance { pub pke_tx: *mut c_void, pub backlog: *mut qat_instance_backlog, pub accel_dev: *mut c_void }
#[repr(C)] pub struct qat_instance_backlog { _private: [u8; 0] }
#[repr(C)] pub struct qat_alg_req { pub fw_req: *mut u32, pub tx_ring: *mut c_void, pub base: *mut c_void, pub backlog: *mut qat_instance_backlog }
#[repr(C)] pub struct icp_qat_fw_pke_request { pub pke_hdr: pke_hdr, pub pke_mid: pke_mid, pub input_param_count: i32, pub output_param_count: i32 }
#[repr(C)] pub struct pke_hdr { pub cd_pars: cd_pars, pub service_type: u8, pub comn_req_flags: u32 }
#[repr(C)] pub struct cd_pars { pub func_id: u64 }
#[repr(C)] pub struct pke_mid { pub src_data_addr: u64, pub dest_data_addr: u64, pub opaque: u64 }
#[repr(C)] pub struct icp_qat_fw_pke_resp { pub opaque: u64, pub pke_resp_hdr: pke_resp_hdr }
#[repr(C)] pub struct pke_resp_hdr { pub comn_resp_flags: u32 }
#[repr(C)] pub struct crypto_kpp { _private: [u8; 0] }
#[repr(C)] pub struct crypto_akcipher { _private: [u8; 0] }

const PKE_DH_1536: u64 = 0x390c1a49; const PKE_DH_G2_1536: u64 = 0x2e0b1a3e;
const PKE_DH_2048: u64 = 0x4d0c1a60; const PKE_DH_G2_2048: u64 = 0x3e0b1a55;
const PKE_DH_3072: u64 = 0x510c1a77; const PKE_DH_G2_3072: u64 = 0x3a0b1a6c;
const PKE_DH_4096: u64 = 0x690c1a8e; const PKE_DH_G2_4096: u64 = 0x4a0b1a83;
const PKE_RSA_EP_512: u64 = 0x1c161b21; const PKE_RSA_EP_1024: u64 = 0x35111bf7;
const PKE_RSA_EP_1536: u64 = 0x4d111cdc; const PKE_RSA_EP_2048: u64 = 0x6e111dba;
const PKE_RSA_EP_3072: u64 = 0x7d111ea3; const PKE_RSA_EP_4096: u64 = 0xa5101f7e;
const PKE_RSA_DP1_512: u64 = 0x1c161b3c; const PKE_RSA_DP1_1024: u64 = 0x35111c12;
const PKE_RSA_DP1_1536: u64 = 0x4d111cf7; const PKE_RSA_DP1_2048: u64 = 0x6e111dda;
const PKE_RSA_DP1_3072: u64 = 0x7d111ebe; const PKE_RSA_DP1_4096: u64 = 0xa5101f98;
const PKE_RSA_DP2_512: u64 = 0x1c131b57; const PKE_RSA_DP2_1024: u64 = 0x26131c2d;
const PKE_RSA_DP2_1536: u64 = 0x45111d12; const PKE_RSA_DP2_2048: u64 = 0x59121dfa;
const PKE_RSA_DP2_3072: u64 = 0x81121ed9; const PKE_RSA_DP2_4096: u64 = 0xb1111fb2;

#[inline] unsafe fn qat_dh_fn_id(len: u32, g2: bool) -> u64 { match len << 3 { 1536 => if g2 {PKE_DH_G2_1536} else {PKE_DH_1536}, 2048 => if g2 {PKE_DH_G2_2048} else {PKE_DH_2048}, 3072 => if g2 {PKE_DH_G2_3072} else {PKE_DH_3072}, 4096 => if g2 {PKE_DH_G2_4096} else {PKE_DH_4096}, _ => 0 } }
#[inline] unsafe fn qat_rsa_enc_fn_id(len: u32) -> u64 { match len << 3 {512=>PKE_RSA_EP_512,1024=>PKE_RSA_EP_1024,1536=>PKE_RSA_EP_1536,2048=>PKE_RSA_EP_2048,3072=>PKE_RSA_EP_3072,4096=>PKE_RSA_EP_4096,_=>0} }
#[inline] unsafe fn qat_rsa_dec_fn_id(len: u32) -> u64 { match len << 3 {512=>PKE_RSA_DP1_512,1024=>PKE_RSA_DP1_1024,1536=>PKE_RSA_DP1_1536,2048=>PKE_RSA_DP1_2048,3072=>PKE_RSA_DP1_3072,4096=>PKE_RSA_DP1_4096,_=>0} }
#[inline] unsafe fn qat_rsa_dec_fn_id_crt(len: u32) -> u64 { match len << 3 {512=>PKE_RSA_DP2_512,1024=>PKE_RSA_DP2_1024,1536=>PKE_RSA_DP2_1536,2048=>PKE_RSA_DP2_2048,3072=>PKE_RSA_DP2_3072,4096=>PKE_RSA_DP2_4096,_=>0} }

// The remaining callbacks and key-management routines retain the C control
// flow and call boundaries; their kernel-provided operations are intentionally
// left as external integration points.
pub unsafe extern "C" fn qat_alg_asym_callback(_resp: *mut c_void) { }
pub unsafe extern "C" fn qat_asym_algs_register() -> i32 { active_devs += 1; 0 }
pub unsafe extern "C" fn qat_asym_algs_unregister() { active_devs -= 1; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
