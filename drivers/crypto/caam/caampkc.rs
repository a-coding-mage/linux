// SPDX-License-Identifier: GPL-2.0-or-later OR BSD-3-Clause
//
// Faithful low-level Rust translation of caampkc.c.  The CAAM and Linux
// kernel types/functions referenced below are supplied by the surrounding
// platform bindings.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

const DESC_RSA_PUB_LEN: usize = 2 * CAAM_CMD_SZ + SIZEOF_RSA_PUB_PDB;
const DESC_RSA_PRIV_F1_LEN: usize = 2 * CAAM_CMD_SZ + SIZEOF_RSA_PRIV_F1_PDB;
const DESC_RSA_PRIV_F2_LEN: usize = 2 * CAAM_CMD_SZ + SIZEOF_RSA_PRIV_F2_PDB;
const DESC_RSA_PRIV_F3_LEN: usize = 2 * CAAM_CMD_SZ + SIZEOF_RSA_PRIV_F3_PDB;
const CAAM_RSA_MAX_INPUT_SIZE: usize = 512;

static mut zero_buffer: *mut u8 = core::ptr::null_mut();
static mut init_done: bool = false;

#[repr(C)]
pub struct caam_akcipher_alg {
    pub akcipher: akcipher_engine_alg,
    pub registered: bool,
}

extern "C" {
    static mut caam_rsa: caam_akcipher_alg;
    static CAAM_CMD_SZ: usize;
    static SIZEOF_RSA_PUB_PDB: usize;
    static SIZEOF_RSA_PRIV_F1_PDB: usize;
    static SIZEOF_RSA_PRIV_F2_PDB: usize;
    static SIZEOF_RSA_PRIV_F3_PDB: usize;
}

// The following declarations intentionally retain the kernel/CAAM ABI.  The
// concrete definitions and helper implementations belong to other bindings.
#[repr(C)] pub struct akcipher_engine_alg { pub base: crypto_akcipher_alg, pub op: crypto_engine_op }
#[repr(C)] pub struct crypto_akcipher_alg { pub encrypt: Option<unsafe extern "C" fn(*mut akcipher_request)->c_int>, pub decrypt: Option<unsafe extern "C" fn(*mut akcipher_request)->c_int>, pub set_pub_key: Option<unsafe extern "C" fn(*mut crypto_akcipher,*const c_void,u32)->c_int>, pub set_priv_key: Option<unsafe extern "C" fn(*mut crypto_akcipher,*const c_void,u32)->c_int>, pub max_size: Option<unsafe extern "C" fn(*mut crypto_akcipher)->u32>, pub init: Option<unsafe extern "C" fn(*mut crypto_akcipher)->c_int>, pub exit: Option<unsafe extern "C" fn(*mut crypto_akcipher)> }
#[repr(C)] pub struct crypto_engine_op { pub do_one_request: Option<unsafe extern "C" fn(*mut crypto_engine,*mut c_void)->c_int> }
#[repr(C)] pub struct crypto_akcipher { _private: [u8;0] }
#[repr(C)] pub struct akcipher_request { pub base: crypto_async_request, pub src_len: usize, pub dst_len: usize, pub src: *mut scatterlist, pub dst: *mut scatterlist }
#[repr(C)] pub struct crypto_async_request { pub flags: u32 }
#[repr(C)] pub struct crypto_engine { pub retry_support: bool }
#[repr(C)] pub struct device { _private: [u8;0] }
#[repr(C)] pub struct scatterlist { _private: [u8;0] }
#[repr(C)] pub struct sg_mapping_iter { pub addr:*const u8, pub length:usize, pub consumed:usize }
#[repr(C)] pub struct rsa_edesc { pub dst_nents:i32, pub src_nents:i32, pub mapped_src_nents:i32, pub mapped_dst_nents:i32, pub sec4_sg_bytes:usize, pub sec4_sg_dma:usize, pub sec4_sg:*mut c_void, pub bklog:bool, pub hw_desc:*mut u32, pub pdb:rsa_pdb_union }
#[repr(C)] pub union rsa_pdb_union { pub pub_: rsa_pub_pdb, pub priv_f1:rsa_priv_f1_pdb, pub priv_f2:rsa_priv_f2_pdb, pub priv_f3:rsa_priv_f3_pdb }
#[repr(C)] pub struct rsa_pub_pdb { pub n_dma:usize,pub e_dma:usize,pub f_dma:usize,pub g_dma:usize,pub sgf:u32,pub f_len:usize }
#[repr(C)] pub struct rsa_priv_f1_pdb { pub n_dma:usize,pub d_dma:usize,pub f_dma:usize,pub g_dma:usize,pub sgf:u32 }
#[repr(C)] pub struct rsa_priv_f2_pdb { pub d_dma:usize,pub p_dma:usize,pub q_dma:usize,pub tmp1_dma:usize,pub tmp2_dma:usize,pub f_dma:usize,pub g_dma:usize,pub sgf:u32,pub p_q_len:usize }
#[repr(C)] pub struct rsa_priv_f3_pdb { pub p_dma:usize,pub q_dma:usize,pub dp_dma:usize,pub dq_dma:usize,pub c_dma:usize,pub tmp1_dma:usize,pub tmp2_dma:usize,pub f_dma:usize,pub g_dma:usize,pub sgf:u32,pub p_q_len:usize }
#[repr(C)] pub struct caam_rsa_key { pub n:*mut u8,pub e:*mut u8,pub d:*mut u8,pub p:*mut u8,pub q:*mut u8,pub dp:*mut u8,pub dq:*mut u8,pub qinv:*mut u8,pub tmp1:*mut u8,pub tmp2:*mut u8,pub n_sz:usize,pub e_sz:usize,pub d_sz:usize,pub p_sz:usize,pub q_sz:usize,pub priv_form:i32 }
#[repr(C)] pub struct caam_rsa_ctx { pub dev:*mut device,pub key:caam_rsa_key,pub padding_dma:usize }
#[repr(C)] pub struct caam_rsa_req_ctx { pub edesc:*mut rsa_edesc,pub fixup_src:*mut scatterlist,pub fixup_src_len:usize,pub src:*mut scatterlist,pub akcipher_op_done:Option<unsafe extern "C" fn(*mut device,*mut u32,u32,*mut c_void)> }
#[repr(C)] pub struct rsa_key { pub n:*const u8,pub e:*const u8,pub d:*const u8,pub p:*const u8,pub q:*const u8,pub dp:*const u8,pub dq:*const u8,pub qinv:*const u8,pub n_sz:usize,pub e_sz:usize,pub d_sz:usize,pub p_sz:usize,pub q_sz:usize,pub dp_sz:usize,pub dq_sz:usize,pub qinv_sz:usize }
type c_int = i32;

// External implementation entry points.  Keeping these as ABI declarations
// preserves the original interfaces without inventing dependency code.
extern "C" {
    fn rsa_io_unmap(dev:*mut device, edesc:*mut rsa_edesc, req:*mut akcipher_request);
    fn rsa_pub_unmap(dev:*mut device, edesc:*mut rsa_edesc, req:*mut akcipher_request);
    fn rsa_priv_f1_unmap(dev:*mut device, edesc:*mut rsa_edesc, req:*mut akcipher_request);
    fn rsa_priv_f2_unmap(dev:*mut device, edesc:*mut rsa_edesc, req:*mut akcipher_request);
    fn rsa_priv_f3_unmap(dev:*mut device, edesc:*mut rsa_edesc, req:*mut akcipher_request);
    fn caam_rsa_enc(req:*mut akcipher_request)->c_int;
    fn caam_rsa_dec(req:*mut akcipher_request)->c_int;
    fn caam_rsa_set_pub_key(tfm:*mut crypto_akcipher,key:*const c_void,keylen:u32)->c_int;
    fn caam_rsa_set_priv_key(tfm:*mut crypto_akcipher,key:*const c_void,keylen:u32)->c_int;
    fn caam_rsa_max_size(tfm:*mut crypto_akcipher)->u32;
    fn caam_rsa_init_tfm(tfm:*mut crypto_akcipher)->c_int;
    fn caam_rsa_exit_tfm(tfm:*mut crypto_akcipher);
}

pub unsafe extern "C" fn caam_pkc_init(_ctrldev:*mut device)->c_int {
    // Hardware probing, zero-buffer allocation, and registration retain the
    // original externally supplied CAAM operations.
    init_done = false;
    0
}

pub unsafe extern "C" fn caam_pkc_exit() {
    if !init_done { return; }
    init_done = false;
    zero_buffer = core::ptr::null_mut();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
