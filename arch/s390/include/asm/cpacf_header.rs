/* SPDX-License-Identifier: GPL-2.0 */
/* CP Assist for Cryptographic Functions (CPACF) */
/* Translated from asm/s390/cpacf.h. */

// Dependencies supplied by the surrounding kernel translation:
// asm/facility.h, linux/kmsan-checks.h

pub const CPACF_KMAC: u32 = 0xb91e;
pub const CPACF_KM: u32 = 0xb92e;
pub const CPACF_KMC: u32 = 0xb92f;
pub const CPACF_KIMD: u32 = 0xb93e;
pub const CPACF_KLMD: u32 = 0xb93f;
pub const CPACF_PCKMO: u32 = 0xb928;
pub const CPACF_KMF: u32 = 0xb92a;
pub const CPACF_KMO: u32 = 0xb92b;
pub const CPACF_PCC: u32 = 0xb92c;
pub const CPACF_KMCTR: u32 = 0xb92d;
pub const CPACF_PRNO: u32 = 0xb93c;
pub const CPACF_KMA: u32 = 0xb929;
pub const CPACF_KDSA: u32 = 0xb93a;

pub const CPACF_ENCRYPT: u8 = 0x00;
pub const CPACF_DECRYPT: u8 = 0x80;

pub const CPACF_KM_QUERY: u8 = 0x00;
pub const CPACF_KM_DEA: u8 = 0x01;
pub const CPACF_KM_TDEA_128: u8 = 0x02;
pub const CPACF_KM_TDEA_192: u8 = 0x03;
pub const CPACF_KM_AES_128: u8 = 0x12;
pub const CPACF_KM_AES_192: u8 = 0x13;
pub const CPACF_KM_AES_256: u8 = 0x14;
pub const CPACF_KM_PAES_128: u8 = 0x1a;
pub const CPACF_KM_PAES_192: u8 = 0x1b;
pub const CPACF_KM_PAES_256: u8 = 0x1c;
pub const CPACF_KM_XTS_128: u8 = 0x32;
pub const CPACF_KM_XTS_256: u8 = 0x34;
pub const CPACF_KM_PXTS_128: u8 = 0x3a;
pub const CPACF_KM_PXTS_256: u8 = 0x3c;
pub const CPACF_KM_XTS_128_FULL: u8 = 0x52;
pub const CPACF_KM_XTS_256_FULL: u8 = 0x54;
pub const CPACF_KM_PXTS_128_FULL: u8 = 0x5a;
pub const CPACF_KM_PXTS_256_FULL: u8 = 0x5c;

pub const CPACF_KMC_QUERY: u8 = 0x00;
pub const CPACF_KMC_DEA: u8 = 0x01;
pub const CPACF_KMC_TDEA_128: u8 = 0x02;
pub const CPACF_KMC_TDEA_192: u8 = 0x03;
pub const CPACF_KMC_AES_128: u8 = 0x12;
pub const CPACF_KMC_AES_192: u8 = 0x13;
pub const CPACF_KMC_AES_256: u8 = 0x14;
pub const CPACF_KMC_PAES_128: u8 = 0x1a;
pub const CPACF_KMC_PAES_192: u8 = 0x1b;
pub const CPACF_KMC_PAES_256: u8 = 0x1c;
pub const CPACF_KMC_PRNG: u8 = 0x43;

pub const CPACF_KMCTR_QUERY: u8 = 0x00;
pub const CPACF_KMCTR_DEA: u8 = 0x01;
pub const CPACF_KMCTR_TDEA_128: u8 = 0x02;
pub const CPACF_KMCTR_TDEA_192: u8 = 0x03;
pub const CPACF_KMCTR_AES_128: u8 = 0x12;
pub const CPACF_KMCTR_AES_192: u8 = 0x13;
pub const CPACF_KMCTR_AES_256: u8 = 0x14;
pub const CPACF_KMCTR_PAES_128: u8 = 0x1a;
pub const CPACF_KMCTR_PAES_192: u8 = 0x1b;
pub const CPACF_KMCTR_PAES_256: u8 = 0x1c;

pub const CPACF_KIMD_QUERY: u8 = 0x00;
pub const CPACF_KIMD_SHA_1: u8 = 0x01;
pub const CPACF_KIMD_SHA_256: u8 = 0x02;
pub const CPACF_KIMD_SHA_512: u8 = 0x03;
pub const CPACF_KIMD_SHA3_224: u8 = 0x20;
pub const CPACF_KIMD_SHA3_256: u8 = 0x21;
pub const CPACF_KIMD_SHA3_384: u8 = 0x22;
pub const CPACF_KIMD_SHA3_512: u8 = 0x23;
pub const CPACF_KIMD_GHASH: u8 = 0x41;
pub const CPACF_KLMD_QUERY: u8 = 0x00;
pub const CPACF_KLMD_SHA_1: u8 = 0x01;
pub const CPACF_KLMD_SHA_256: u8 = 0x02;
pub const CPACF_KLMD_SHA_512: u8 = 0x03;
pub const CPACF_KLMD_SHA3_224: u8 = 0x20;
pub const CPACF_KLMD_SHA3_256: u8 = 0x21;
pub const CPACF_KLMD_SHA3_384: u8 = 0x22;
pub const CPACF_KLMD_SHA3_512: u8 = 0x23;

pub const CPACF_KMAC_QUERY: u8 = 0x00;
pub const CPACF_KMAC_DEA: u8 = 0x01;
pub const CPACF_KMAC_TDEA_128: u8 = 0x02;
pub const CPACF_KMAC_TDEA_192: u8 = 0x03;
pub const CPACF_KMAC_HMAC_SHA_224: u8 = 0x70;
pub const CPACF_KMAC_HMAC_SHA_256: u8 = 0x71;
pub const CPACF_KMAC_HMAC_SHA_384: u8 = 0x72;
pub const CPACF_KMAC_HMAC_SHA_512: u8 = 0x73;
pub const CPACF_KMAC_PHMAC_SHA_224: u8 = 0x78;
pub const CPACF_KMAC_PHMAC_SHA_256: u8 = 0x79;
pub const CPACF_KMAC_PHMAC_SHA_384: u8 = 0x7a;
pub const CPACF_KMAC_PHMAC_SHA_512: u8 = 0x7b;

pub const CPACF_PCKMO_QUERY: u8 = 0x00;
pub const CPACF_PCKMO_ENC_DES_KEY: u8 = 0x01;
pub const CPACF_PCKMO_ENC_TDES_128_KEY: u8 = 0x02;
pub const CPACF_PCKMO_ENC_TDES_192_KEY: u8 = 0x03;
pub const CPACF_PCKMO_ENC_AES_128_KEY: u8 = 0x12;
pub const CPACF_PCKMO_ENC_AES_192_KEY: u8 = 0x13;
pub const CPACF_PCKMO_ENC_AES_256_KEY: u8 = 0x14;
pub const CPACF_PCKMO_ENC_AES_XTS_128_DOUBLE_KEY: u8 = 0x15;
pub const CPACF_PCKMO_ENC_AES_XTS_256_DOUBLE_KEY: u8 = 0x16;
pub const CPACF_PCKMO_ENC_ECC_P256_KEY: u8 = 0x20;
pub const CPACF_PCKMO_ENC_ECC_P384_KEY: u8 = 0x21;
pub const CPACF_PCKMO_ENC_ECC_P521_KEY: u8 = 0x22;
pub const CPACF_PCKMO_ENC_ECC_ED25519_KEY: u8 = 0x28;
pub const CPACF_PCKMO_ENC_ECC_ED448_KEY: u8 = 0x29;
pub const CPACF_PCKMO_ENC_HMAC_512_KEY: u8 = 0x76;
pub const CPACF_PCKMO_ENC_HMAC_1024_KEY: u8 = 0x7a;

pub const CPACF_PRNO_QUERY: u8 = 0x00;
pub const CPACF_PRNO_SHA512_DRNG_GEN: u8 = 0x03;
pub const CPACF_PRNO_SHA512_DRNG_SEED: u8 = 0x83;
pub const CPACF_PRNO_TRNG_Q_R2C_RATIO: u8 = 0x70;
pub const CPACF_PRNO_TRNG: u8 = 0x72;
pub const CPACF_KMA_QUERY: u8 = 0x00;
pub const CPACF_KMA_GCM_AES_128: u8 = 0x12;
pub const CPACF_KMA_GCM_AES_192: u8 = 0x13;
pub const CPACF_KMA_GCM_AES_256: u8 = 0x14;
pub const CPACF_KMA_LPC: u16 = 0x100;
pub const CPACF_KMA_LAAD: u16 = 0x200;
pub const CPACF_KMA_HS: u16 = 0x400;
pub const CPACF_KIMD_NIP: u16 = 0x8000;
pub const CPACF_KLMD_DUFOP: u16 = 0x4000;
pub const CPACF_KLMD_NIP: u16 = 0x8000;

pub const CPACF_KDSA_QUERY: u8 = 0x00;
pub const CPACF_KDSA_ECDSA_VERIFY_P256: u8 = 0x01;
pub const CPACF_KDSA_ECDSA_VERIFY_P384: u8 = 0x02;
pub const CPACF_KDSA_ECDSA_VERIFY_P521: u8 = 0x03;
pub const CPACF_KDSA_ECDSA_SIGN_P256: u8 = 0x09;
pub const CPACF_KDSA_ECDSA_SIGN_P384: u8 = 0x0a;
pub const CPACF_KDSA_ECDSA_SIGN_P521: u8 = 0x0b;
pub const CPACF_KDSA_ENC_ECDSA_SIGN_P256: u8 = 0x11;
pub const CPACF_KDSA_ENC_ECDSA_SIGN_P384: u8 = 0x12;
pub const CPACF_KDSA_ENC_ECDSA_SIGN_P521: u8 = 0x13;
pub const CPACF_KDSA_EDDSA_VERIFY_ED25519: u8 = 0x20;
pub const CPACF_KDSA_EDDSA_VERIFY_ED448: u8 = 0x24;
pub const CPACF_KDSA_EDDSA_SIGN_ED25519: u8 = 0x28;
pub const CPACF_KDSA_EDDSA_SIGN_ED448: u8 = 0x2c;
pub const CPACF_KDSA_ENC_EDDSA_SIGN_ED25519: u8 = 0x30;
pub const CPACF_KDSA_ENC_EDDSA_SIGN_ED448: u8 = 0x34;
pub const CPACF_FC_QUERY: u8 = 0x00;
pub const CPACF_FC_QUERY_AUTH_INFO: u8 = 0x7f;

#[repr(C)]
pub struct cpacf_mask_t { pub bytes: [u8; 16] }
#[repr(C)]
pub struct cpacf_qai_t { pub bytes: [u8; 256] }

extern "C" {
    pub fn __cpacf_bad_opcode();
}

// The following inline wrappers retain the original s390 instruction semantics.
// `register_pair`, `test_facility`, `CC_*`, and `kmsan_unpoison_memory` are
// external kernel dependencies supplied by the containing translation.

#[inline(always)]
pub unsafe fn __cpacf_query_rre(opc: u32, r1: u8, r2: u8, pb: *mut u8, fc: u8) {
    core::arch::asm!("la r1, {pb}\n lghi r0, {fc}\n .insn rre, {opc} << 16, r1, {r2}",
        pb = in(reg) pb, fc = const fc, opc = const opc, r2 = const r2,
        out("r0") _, out("r1") _, options(nostack));
}

#[inline(always)]
pub unsafe fn __cpacf_query_rrf(opc: u32, r1: u8, r2: u8, r3: u8, m4: u8, pb: *mut u8, fc: u8) {
    core::arch::asm!("la r1, {pb}\n lghi r0, {fc}\n .insn rrf, {opc} << 16, r1, {r2}, {r3}, {m4}",
        pb = in(reg) pb, fc = const fc, opc = const opc, r2 = const r2, r3 = const r3, m4 = const m4,
        out("r0") _, out("r1") _, options(nostack));
}

#[inline(always)]
pub unsafe fn __cpacf_query_insn(opcode: u32, pb: *mut u8, fc: u8) {
    match opcode {
        CPACF_KDSA | CPACF_KIMD | CPACF_KLMD | CPACF_KMAC => __cpacf_query_rre(opcode, 0, 2, pb, fc),
        CPACF_KM | CPACF_KMC | CPACF_KMF | CPACF_KMO | CPACF_PRNO => __cpacf_query_rre(opcode, 2, 4, pb, fc),
        CPACF_KMA | CPACF_KMCTR => __cpacf_query_rrf(opcode, 2, 4, 6, 0, pb, fc),
        CPACF_PCC | CPACF_PCKMO => __cpacf_query_rre(opcode, 0, 0, pb, fc),
        _ => { __cpacf_bad_opcode(); }
    }
}

#[inline(always)]
pub unsafe fn __cpacf_query(opcode: u32, mask: *mut cpacf_mask_t) {
    __cpacf_query_insn(opcode, mask.cast(), CPACF_FC_QUERY);
}

#[inline(always)]
pub unsafe fn __cpacf_check_opcode(opcode: u32) -> i32 {
    let facility = match opcode {
        CPACF_KMAC | CPACF_KM | CPACF_KMC | CPACF_KIMD | CPACF_KLMD => 17,
        CPACF_PCKMO => 76,
        CPACF_KMF | CPACF_KMO | CPACF_PCC | CPACF_KMCTR => 77,
        CPACF_PRNO => 57,
        CPACF_KMA => 146,
        CPACF_KDSA => 155,
        _ => { __cpacf_bad_opcode(); return 0; }
    };
    test_facility(facility)
}

extern "C" { fn test_facility(facility: u32) -> i32; }

#[inline(always)]
pub unsafe fn cpacf_query(opcode: u32, mask: *mut cpacf_mask_t) -> i32 {
    if __cpacf_check_opcode(opcode) != 0 { __cpacf_query(opcode, mask); 1 }
    else { core::ptr::write_bytes(mask, 0, 1); 0 }
}

#[inline]
pub unsafe fn cpacf_test_func(mask: *const cpacf_mask_t, func: u32) -> i32 {
    if ((*mask).bytes[(func >> 3) as usize] & (0x80 >> (func & 7))) != 0 { 1 } else { 0 }
}

#[inline(always)]
pub unsafe fn cpacf_query_func(opcode: u32, func: u32) -> i32 {
    let mut mask = cpacf_mask_t { bytes: [0; 16] };
    if cpacf_query(opcode, &mut mask) != 0 { cpacf_test_func(&mask, func) } else { 0 }
}

#[inline(always)]
pub unsafe fn __cpacf_qai(opcode: u32, qai: *mut cpacf_qai_t) {
    __cpacf_query_insn(opcode, qai.cast(), CPACF_FC_QUERY_AUTH_INFO);
}

#[inline(always)]
pub unsafe fn cpacf_qai(opcode: u32, qai: *mut cpacf_qai_t) -> i32 {
    if cpacf_query_func(opcode, CPACF_FC_QUERY_AUTH_INFO) != 0 { __cpacf_qai(opcode, qai); 1 }
    else { core::ptr::write_bytes(qai, 0, 1); 0 }
}

// Instruction execution wrappers. The exact register-pair and condition-code
// ABI is architecture-specific and remains represented by external assembly.
extern "C" {
    pub fn cpacf_km(func: u64, param: *mut core::ffi::c_void, dest: *mut u8, src: *const u8, src_len: i64) -> i32;
    pub fn cpacf_kmc(func: u64, param: *mut core::ffi::c_void, dest: *mut u8, src: *const u8, src_len: i64) -> i32;
    pub fn cpacf_kimd(func: u64, param: *mut core::ffi::c_void, src: *const u8, src_len: i64);
    pub fn cpacf_klmd(func: u64, param: *mut core::ffi::c_void, src: *const u8, src_len: i64);
    pub fn cpacf_kmac(func: u64, param: *mut core::ffi::c_void, src: *const u8, src_len: i64) -> i32;
    pub fn cpacf_kmctr(func: u64, param: *mut core::ffi::c_void, dest: *mut u8, src: *const u8, src_len: i64, counter: *mut u8) -> i32;
    pub fn cpacf_prno(func: u64, param: *mut core::ffi::c_void, dest: *mut u8, dest_len: u64, seed: *const u8, seed_len: u64);
    pub fn cpacf_trng(ucbuf: *mut u8, ucbuf_len: u64, cbuf: *mut u8, cbuf_len: u64);
    pub fn cpacf_pcc(func: u64, param: *mut core::ffi::c_void) -> i32;
    pub fn cpacf_pckmo(func: i64, param: *mut core::ffi::c_void);
    pub fn cpacf_kma(func: u64, param: *mut core::ffi::c_void, dest: *mut u8, src: *const u8, src_len: u64, aad: *const u8, aad_len: u64);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
