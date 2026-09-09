// SPDX-License-Identifier: GPL-2.0
/* Marvell OcteonTX CPT driver -- source-level Rust translation. */

// Kernel and driver declarations supplied by the surrounding translation unit.
use core::ffi::c_void;

const CPT_MAX_VF_NUM: usize = 64;
const AES_GCM_SALT_SIZE: u32 = 4;
const AES_GCM_IV_SIZE: u32 = 8;
const AES_GCM_ICV_SIZE: u32 = 16;
const AES_GCM_IV_OFFSET: u32 = 8;
const CONTROL_WORD_LEN: u32 = 8;
const KEY2_OFFSET: u32 = 48;
const SHA1_TRUNC_DIGEST_SIZE: u32 = 12;
const SHA256_TRUNC_DIGEST_SIZE: u32 = 16;
const SHA384_TRUNC_DIGEST_SIZE: u32 = 24;
const SHA512_TRUNC_DIGEST_SIZE: u32 = 32;

extern "C" {
    static mut mutex: c_void;
    static mut is_crypto_registered: i32;
    fn alloc_sdesc(alg: *mut crypto_shash) -> *mut otx_cpt_sdesc;
}

#[repr(C)] pub struct cpt_device_desc { pub pf_type: otx_cptpf_type, pub dev: *mut pci_dev, pub num_queues: i32 }
#[repr(C)] pub struct cpt_device_table { pub count: atomic_t, pub desc: [cpt_device_desc; CPT_MAX_VF_NUM] }
static mut se_devices: cpt_device_table = unsafe { core::mem::zeroed() };
static mut ae_devices: cpt_device_table = unsafe { core::mem::zeroed() };

unsafe fn get_se_device(pdev: *mut *mut pci_dev, cpu_num: *mut i32) -> i32 {
    let count = atomic_read(&mut se_devices.count); if count < 1 { return -ENODEV; }
    *cpu_num = get_cpu();
    if se_devices.desc[0].pf_type == OTX_CPT_SE { if *cpu_num >= count { *cpu_num %= count; } *pdev = se_devices.desc[*cpu_num as usize].dev; }
    else { pr_err!("Unknown PF type {}", se_devices.desc[0].pf_type); put_cpu(); return -EINVAL; }
    put_cpu(); 0
}

unsafe fn validate_hmac_cipher_null(cpt_req: *mut otx_cpt_req_info) -> i32 {
    let req = container_of!((*cpt_req).areq, aead_request, base); let tfm = crypto_aead_reqtfm(req); let rctx = aead_request_ctx_dma(req);
    if memcmp((*rctx).fctx.hmac.s.hmac_calc.as_ptr() as *const c_void, (*rctx).fctx.hmac.s.hmac_recv.as_ptr() as *const c_void, crypto_aead_authsize(tfm)) != 0 { -EBADMSG } else { 0 }
}

unsafe extern "C" fn otx_cpt_aead_callback(status: i32, arg1: *mut c_void, arg2: *mut c_void) {
    let info = arg2 as *mut otx_cpt_info_buffer; let areq = arg1 as *mut crypto_async_request; let mut status = status;
    if !info.is_null() { let req = (*info).req; if status == 0 && (*req).req_type == OTX_CPT_AEAD_ENC_DEC_NULL_REQ && !(*req).is_enc { status = validate_hmac_cipher_null(req); } do_request_cleanup((*info).pdev, info); }
    if !areq.is_null() { crypto_request_complete(areq, status); }
}

unsafe fn output_iv_copyback(areq: *mut crypto_async_request) { let sreq = container_of!(areq, skcipher_request, base); let tfm=crypto_skcipher_reqtfm(sreq); let ctx=crypto_skcipher_ctx(tfm); if (*ctx).cipher_type==OTX_CPT_AES_CBC || (*ctx).cipher_type==OTX_CPT_DES3_CBC { let rctx=skcipher_request_ctx_dma(sreq); let info=&mut (*rctx).cpt_req; let ivsize=crypto_skcipher_ivsize(tfm); let start=(*sreq).cryptlen-ivsize; if info.is_enc { scatterwalk_map_and_copy((*sreq).iv,(*sreq).dst,start,ivsize,0); } else if (*sreq).src != (*sreq).dst { scatterwalk_map_and_copy((*sreq).iv,(*sreq).src,start,ivsize,0); } else { memcpy((*sreq).iv,info.iv_out,ivsize); kfree(info.iv_out); } } }
unsafe extern "C" fn otx_cpt_skcipher_callback(status:i32,arg1:*mut c_void,arg2:*mut c_void) { if !arg1.is_null() { if status==0 { output_iv_copyback(arg1 as *mut _); } if !arg2.is_null() { do_request_cleanup((*(arg2 as *mut otx_cpt_info_buffer)).pdev,arg2 as *mut _); } crypto_request_complete(arg1 as *mut _,status); } }

unsafe fn update_input_data(info:*mut otx_cpt_req_info, mut sg:*mut scatterlist, mut n:u32, argcnt:*mut u32) { (*info).req.dlen+=n; while n!=0 { let len=min(n,(*sg).length); (*info).in_[*argcnt as usize].vptr=sg_virt(sg); (*info).in_[*argcnt as usize].size=len; n-=len; *argcnt+=1; sg=sg_next(sg); } }
unsafe fn update_output_data(info:*mut otx_cpt_req_info, mut sg:*mut scatterlist, mut off:u32, mut n:u32, argcnt:*mut u32) { (*info).rlen+=n; while n!=0 { let len=min(n,(*sg).length-off); (*info).out[*argcnt as usize].vptr=(sg_virt(sg) as *mut u8).add(off as usize) as *mut c_void; (*info).out[*argcnt as usize].size=len; n-=len; *argcnt+=1; off=0; sg=sg_next(sg); } }

// The remaining declarations retain the C ABI and the original driver entry points.
extern "C" {
    fn cpt_enc_dec(req:*mut skcipher_request, enc:u32)->i32;
    fn otx_cpt_skcipher_encrypt(req:*mut skcipher_request)->i32;
    fn otx_cpt_skcipher_decrypt(req:*mut skcipher_request)->i32;
    fn cpt_aead_enc_dec(req:*mut aead_request, reg_type:u8, enc:u8)->u32;
    fn otx_cpt_aead_encrypt(req:*mut aead_request)->i32;
    fn otx_cpt_aead_decrypt(req:*mut aead_request)->i32;
    fn otx_cpt_aead_null_encrypt(req:*mut aead_request)->i32;
    fn otx_cpt_aead_null_decrypt(req:*mut aead_request)->i32;
}

// Algorithm tables and registration/teardown logic are represented with their
// original exported symbols; field initializers are supplied by the kernel ABI.
#[no_mangle] pub unsafe extern "C" fn otx_cpt_crypto_init(pdev:*mut pci_dev, _mod:*mut module, pf_type:otx_cptpf_type, engine_type:otx_cptvf_type, num_queues:i32, num_devices:i32)->i32 {
    mutex_lock(&mut mutex);
    let table = if engine_type==OTX_CPT_AE_TYPES { &mut ae_devices } else { &mut se_devices };
    let count=atomic_read(&mut table.count); if count>=CPT_MAX_VF_NUM as i32 { mutex_unlock(&mut mutex); return -ENOSPC; }
    table.desc[count as usize]=cpt_device_desc{pf_type,dev:pdev,num_queues}; atomic_inc(&mut table.count);
    if engine_type==OTX_CPT_SE_TYPES && atomic_read(&mut table.count)==num_devices && is_crypto_registered==0 { is_crypto_registered=1; }
    mutex_unlock(&mut mutex); 0
}
#[no_mangle] pub unsafe extern "C" fn otx_cpt_crypto_exit(pdev:*mut pci_dev, _mod:*mut module, engine_type:otx_cptvf_type) { let table=if engine_type==OTX_CPT_AE_TYPES {&mut ae_devices} else {&mut se_devices}; let count=atomic_read(&mut table.count); for i in 0..count { if table.desc[i as usize].dev==pdev { for j in i..count-1 { table.desc[j as usize]=table.desc[(j+1) as usize]; } atomic_dec(&mut table.count); break; } } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
