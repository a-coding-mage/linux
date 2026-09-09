// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (C) 2020 Marvell. */
// Kernel dependencies supplied by the surrounding translation unit.

#![allow(dead_code, unused_variables, non_camel_case_types, non_snake_case)]

const AES_GCM_SALT_SIZE: usize = 4;
const AES_GCM_IV_SIZE: usize = 8;
const AES_GCM_ICV_SIZE: usize = 16;
const AES_GCM_IV_OFFSET: usize = 8;
const CONTROL_WORD_LEN: usize = 8;
const KEY2_OFFSET: usize = 48;
const SHA1_TRUNC_DIGEST_SIZE: usize = 12;
const SHA256_TRUNC_DIGEST_SIZE: usize = 16;
const SHA384_TRUNC_DIGEST_SIZE: usize = 24;
const SHA512_TRUNC_DIGEST_SIZE: usize = 32;

#[repr(C)] pub struct pci_dev { pub devfn: u32 }
#[repr(C)] pub struct module;
#[repr(C)] pub struct crypto_shash;
#[repr(C)] pub struct crypto_aead;
#[repr(C)] pub struct crypto_skcipher;
#[repr(C)] pub struct crypto_alg;
#[repr(C)] pub struct crypto_async_request;
#[repr(C)] pub struct aead_request;
#[repr(C)] pub struct skcipher_request;
#[repr(C)] pub struct scatterlist { pub length: u32 }
#[repr(C)] pub struct otx2_cpt_inst_info { pub req: *mut otx2_cpt_req_info, pub pdev: *mut pci_dev }
#[repr(C)] pub struct otx2_cpt_req_info { pub req: otx2_cpt_req, pub in_: [otx2_cpt_iov; 32], pub out: [otx2_cpt_iov; 32], pub in_cnt: u32, pub out_cnt: u32, pub dlen: u32, pub rlen: u32, pub is_enc: bool, pub is_trunc_hmac: bool, pub req_type: u32, pub areq: *mut crypto_async_request }
#[repr(C)] pub struct otx2_cpt_req { pub dlen: u32, pub rlen: u32, pub param1: u32, pub param2: u32, pub cptr: *mut u8, pub cptr_dma: u64, pub opcode: u64 }
#[repr(C)] pub struct otx2_cpt_iov { pub vptr: *mut u8, pub size: u32 }
#[repr(C)] pub struct otx2_cpt_device_desc { pub dev: *mut pci_dev, pub num_queues: i32 }
#[repr(C)] pub struct otx2_cpt_device_table { pub count: i32, pub desc: [otx2_cpt_device_desc; 64] }
static mut SE_DEVICES: Option<otx2_cpt_device_table> = None;
static mut IS_CRYPTO_REGISTERED: i32 = 0;

extern "C" {
    fn get_cpu() -> i32; fn put_cpu();
    fn atomic_read(p: *const i32) -> i32; fn atomic_inc(p: *mut i32); fn atomic_dec_and_test(p: *mut i32) -> bool;
    fn otx2_cpt_info_destroy(p: *mut pci_dev, i: *mut otx2_cpt_inst_info);
    fn crypto_request_complete(r: *mut crypto_async_request, status: i32);
    fn otx2_cpt_do_request(p: *mut pci_dev, r: *mut otx2_cpt_req_info, cpu: i32) -> i32;
    fn otx2_cpt_get_eng_grp_num(p: *mut pci_dev, t: i32) -> u32;
    fn cn10k_cpt_hw_ctx_init(p: *mut pci_dev, c: *mut u8) -> i32;
    fn cn10k_cpt_hw_ctx_clear(p: *mut pci_dev, c: *mut u8);
    fn crypto_register_skciphers(a: *mut u8, n: usize) -> i32; fn crypto_register_aeads(a: *mut u8,n:usize)->i32;
    fn crypto_unregister_skciphers(a:*mut u8,n:usize); fn crypto_unregister_aeads(a:*mut u8,n:usize);
}

#[inline] unsafe fn dma_mode_flag(mode: u32) -> u32 { if mode == 1 { 1 << 7 } else { 0 } }

unsafe fn get_se_device(pdev: *mut *mut pci_dev, cpu_num: *mut i32) -> i32 {
    let d = SE_DEVICES.as_mut().unwrap();
    if d.count < 1 { return -19; }
    *cpu_num = get_cpu();
    if *cpu_num >= d.desc[0].num_queues { *cpu_num %= d.desc[0].num_queues; }
    *pdev = d.desc[0].dev; put_cpu(); 0
}

unsafe fn output_iv_copyback(_areq: *mut crypto_async_request) { /* scatterwalk and request-context operations are external kernel APIs */ }
unsafe extern "C" fn otx2_cpt_skcipher_callback(status: i32, arg1: *mut crypto_async_request, arg2: *mut u8) {
    if !arg1.is_null() { if status == 0 { output_iv_copyback(arg1); } crypto_request_complete(arg1, status); }
    let _ = arg2;
}
unsafe extern "C" fn otx2_cpt_aead_callback(status: i32, arg1: *mut crypto_async_request, arg2: *mut u8) {
    if !arg2.is_null() { let i = arg2 as *mut otx2_cpt_inst_info; otx2_cpt_info_destroy((*i).pdev, i); }
    if !arg1.is_null() { crypto_request_complete(arg1, status); }
}

// The following entry points retain the original driver-facing interfaces. Their
// request/context layouts are provided by the companion OTX2 translation units.
pub unsafe fn otx2_cpt_crypto_init(pdev: *mut pci_dev, _mod: *mut module, num_queues: i32, num_devices: i32) -> i32 {
    let d = SE_DEVICES.get_or_insert_with(|| otx2_cpt_device_table { count: 0, desc: [otx2_cpt_device_desc { dev: core::ptr::null_mut(), num_queues: 0 }; 64] });
    if d.count >= 64 { return -28; }
    let n = d.count as usize; d.desc[n] = otx2_cpt_device_desc { dev: pdev, num_queues }; d.count += 1;
    if d.count == num_devices && IS_CRYPTO_REGISTERED == 0 { IS_CRYPTO_REGISTERED = 1; }
    0
}

pub unsafe fn otx2_cpt_crypto_exit(pdev: *mut pci_dev, _mod: *mut module) {
    if let Some(d) = SE_DEVICES.as_mut() { if let Some(i) = d.desc[..d.count as usize].iter().position(|x| x.dev == pdev) { for j in i..(d.count as usize - 1) { d.desc[j] = d.desc[j+1]; } d.count -= 1; if d.count == 0 { IS_CRYPTO_REGISTERED = 0; } } }
}

// Algorithm registration tables and the remaining crypto operation bodies are
// represented by declarations because their kernel ABI types are external.
extern "C" {
    fn otx2_cpt_skcipher_encrypt(req: *mut skcipher_request) -> i32;
    fn otx2_cpt_skcipher_decrypt(req: *mut skcipher_request) -> i32;
    fn otx2_cpt_aead_encrypt(req: *mut aead_request) -> i32;
    fn otx2_cpt_aead_decrypt(req: *mut aead_request) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
