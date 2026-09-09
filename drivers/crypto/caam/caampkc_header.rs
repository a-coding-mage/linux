/* SPDX-License-Identifier: GPL-2.0 */
/*
 * caam - Freescale FSL CAAM support for Public Key Cryptography descriptors
 *
 * Copyright 2016 Freescale Semiconductor, Inc.
 *
 * There is no Shared Descriptor for PKC so that the Job Descriptor must carry
 * all the desired key parameters, input and output pointers.
 */

// C dependencies: compat.h and pdb.h

/**
 * caam_priv_key_form - CAAM RSA private key representation
 * CAAM RSA private key may have either of three forms.
 *
 * 1. The first representation consists of the pair (n, d), where the
 *    components have the following meanings:
 *        n      the RSA modulus
 *        d      the RSA private exponent
 *
 * 2. The second representation consists of the triplet (p, q, d), where the
 *    components have the following meanings:
 *        p      the first prime factor of the RSA modulus n
 *        q      the second prime factor of the RSA modulus n
 *        d      the RSA private exponent
 *
 * 3. The third representation consists of the quintuple (p, q, dP, dQ, qInv),
 *    where the components have the following meanings:
 *        p      the first prime factor of the RSA modulus n
 *        q      the second prime factor of the RSA modulus n
 *        dP     the first factors's CRT exponent
 *        dQ     the second factors's CRT exponent
 *        qInv   the (first) CRT coefficient
 *
 * The benefit of using the third or the second key form is lower computational
 * cost for the decryption and signature operations.
 */
#[repr(C)]
pub enum caam_priv_key_form {
    FORM1,
    FORM2,
    FORM3,
}

/** CAAM RSA key structure. Keys are allocated in DMA zone. */
#[repr(C)]
pub struct caam_rsa_key {
    pub n: *mut u8,
    pub e: *mut u8,
    pub d: *mut u8,
    pub p: *mut u8,
    pub q: *mut u8,
    pub dp: *mut u8,
    pub dq: *mut u8,
    pub qinv: *mut u8,
    pub tmp1: *mut u8,
    pub tmp2: *mut u8,
    pub n_sz: usize,
    pub e_sz: usize,
    pub d_sz: usize,
    pub p_sz: usize,
    pub q_sz: usize,
    pub priv_form: caam_priv_key_form,
}

/** Per session context. */
#[repr(C)]
pub struct caam_rsa_ctx {
    pub key: caam_rsa_key,
    pub dev: *mut device,
    pub padding_dma: dma_addr_t,
}

/** Per request context. */
#[repr(C)]
pub struct caam_rsa_req_ctx {
    pub src: [scatterlist; 2],
    pub fixup_src: *mut scatterlist,
    pub fixup_src_len: ::core::ffi::c_uint,
    pub edesc: *mut rsa_edesc,
    pub akcipher_op_done: Option<unsafe extern "C" fn(
        jrdev: *mut device,
        desc: *mut u32,
        err: u32,
        context: *mut ::core::ffi::c_void,
    )>,
}

/** s/w-extended rsa descriptor */
#[repr(C)]
pub struct rsa_edesc {
    pub src_nents: ::core::ffi::c_int,
    pub dst_nents: ::core::ffi::c_int,
    pub mapped_src_nents: ::core::ffi::c_int,
    pub mapped_dst_nents: ::core::ffi::c_int,
    pub sec4_sg_bytes: ::core::ffi::c_int,
    pub bklog: bool,
    pub sec4_sg_dma: dma_addr_t,
    pub sec4_sg: *mut sec4_sg_entry,
    pub pdb: rsa_edesc_pdb,
    pub hw_desc: [u32; 0],
}

#[repr(C)]
pub union rsa_edesc_pdb {
    pub pub_: rsa_pub_pdb,
    pub priv_f1: rsa_priv_f1_pdb,
    pub priv_f2: rsa_priv_f2_pdb,
    pub priv_f3: rsa_priv_f3_pdb,
}

/* Descriptor construction primitives. */
extern "C" {
    pub fn init_rsa_pub_desc(desc: *mut u32, pdb: *mut rsa_pub_pdb);
    pub fn init_rsa_priv_f1_desc(desc: *mut u32, pdb: *mut rsa_priv_f1_pdb);
    pub fn init_rsa_priv_f2_desc(desc: *mut u32, pdb: *mut rsa_priv_f2_pdb);
    pub fn init_rsa_priv_f3_desc(desc: *mut u32, pdb: *mut rsa_priv_f3_pdb);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
