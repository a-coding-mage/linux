// SPDX-License-Identifier: GPL-2.0-only
// Faithful low-level Rust translation of stm32-cryp.c. Kernel dependencies are
// intentionally left as external symbols supplied by the surrounding tree.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

extern "C" {
    fn readl_relaxed(addr: *const u8) -> u32;
    fn writel_relaxed(value: u32, addr: *mut u8);
    fn memcpy(dst: *mut u8, src: *const u8, len: usize);
    fn memset(dst: *mut u8, value: i32, len: usize);
}

type u8_t = u8;
type u32_t = u32;
type phys_addr_t = usize;
type __be32 = u32;

const DRIVER_NAME: &str = "stm32-cryp";
const FLG_ENCRYPT: u32 = 1 << 0;
const FLG_AES: u32 = 1 << 1;
const FLG_DES: u32 = 1 << 2;
const FLG_TDES: u32 = 1 << 3;
const FLG_ECB: u32 = 1 << 4;
const FLG_CBC: u32 = 1 << 5;
const FLG_CTR: u32 = 1 << 6;
const FLG_GCM: u32 = 1 << 7;
const FLG_CCM: u32 = 1 << 8;
const FLG_MODE_MASK: u32 = 0xffff;
const FLG_IN_OUT_DMA: u32 = 1 << 16;
const FLG_HEADER_DMA: u32 = 1 << 17;

const CRYP_CR: u32 = 0x00; const CRYP_SR: u32 = 0x04; const CRYP_DIN: u32 = 0x08;
const CRYP_DOUT: u32 = 0x0c; const CRYP_DMACR: u32 = 0x10; const CRYP_IMSCR: u32 = 0x14;
const CRYP_MISR: u32 = 0x1c; const CRYP_K0LR: u32 = 0x20; const CRYP_K0RR: u32 = 0x24;
const CRYP_K1LR: u32 = 0x28; const CRYP_K1RR: u32 = 0x2c; const CRYP_K2LR: u32 = 0x30;
const CRYP_K2RR: u32 = 0x34; const CRYP_K3LR: u32 = 0x38; const CRYP_K3RR: u32 = 0x3c;
const CRYP_IV0LR: u32 = 0x40; const CRYP_IV0RR: u32 = 0x44; const CRYP_IV1LR: u32 = 0x48;
const CRYP_IV1RR: u32 = 0x4c; const CRYP_CSGCMCCM0R: u32 = 0x50;
const UX500_CRYP_CR: u32 = 0; const UX500_CRYP_SR: u32 = 4; const UX500_CRYP_DIN: u32 = 8;
const UX500_CRYP_DOUT: u32 = 0x10; const UX500_CRYP_DMACR: u32 = 0x18;
const UX500_CRYP_IMSC: u32 = 0x1c; const UX500_CRYP_MIS: u32 = 0x24;
const UX500_CRYP_K1L: u32 = 0x28; const UX500_CRYP_K1R: u32 = 0x2c;
const UX500_CRYP_K3R: u32 = 0x3c; const UX500_CRYP_IV0L: u32 = 0x48;
const UX500_CRYP_IV0R: u32 = 0x4c; const UX500_CRYP_IV1L: u32 = 0x50; const UX500_CRYP_IV1R: u32 = 0x54;

const CR_DEC_NOT_ENC: u32 = 4; const CR_TDES_ECB: u32 = 0; const CR_TDES_CBC: u32 = 8;
const CR_DES_ECB: u32 = 0x10; const CR_DES_CBC: u32 = 0x18; const CR_AES_ECB: u32 = 0x20;
const CR_AES_CBC: u32 = 0x28; const CR_AES_CTR: u32 = 0x30; const CR_AES_KP: u32 = 0x38;
const CR_AES_GCM: u32 = 0x80000; const CR_AES_CCM: u32 = 0x80008; const CR_AES_UNKNOWN: u32 = 0xffff_ffff;
const CR_ALGO_MASK: u32 = 0x80038; const CR_DATA8: u32 = 0x80; const CR_KEY128: u32 = 0;
const CR_KEY192: u32 = 0x100; const CR_KEY256: u32 = 0x200; const CR_KEYRDEN: u32 = 0x400;
const CR_KSE: u32 = 0x800; const CR_FFLUSH: u32 = 0x4000; const CR_CRYPEN: u32 = 0x8000;
const CR_PH_INIT: u32 = 0; const CR_PH_HEADER: u32 = 0x10000; const CR_PH_PAYLOAD: u32 = 0x20000;
const CR_PH_FINAL: u32 = 0x30000; const CR_PH_MASK: u32 = 0x30000; const CR_NBPBL_SHIFT: u32 = 20;
const SR_IFNF: u32 = 1 << 1; const SR_OFNE: u32 = 1 << 2; const SR_BUSY: u32 = 1 << 8;
const DMACR_DIEN: u32 = 1; const DMACR_DOEN: u32 = 2; const IMSCR_IN: u32 = 1; const IMSCR_OUT: u32 = 2;
const MISR_IN: u32 = 1; const MISR_OUT: u32 = 2; const AES_BLOCK_SIZE: usize = 16;
const AES_BLOCK_32: usize = AES_BLOCK_SIZE / 4; const GCM_CTR_INIT: u32 = 2;

#[repr(C)]
pub struct stm32_cryp_caps {
    pub aeads_support: bool, pub linear_aes_key: bool, pub kp_mode: bool,
    pub iv_protection: bool, pub swap_final: bool, pub padding_wa: bool,
    pub cr: u32, pub sr: u32, pub din: u32, pub dout: u32, pub dmacr: u32,
    pub imsc: u32, pub mis: u32, pub k1l: u32, pub k1r: u32, pub k3r: u32,
    pub iv0l: u32, pub iv0r: u32, pub iv1l: u32, pub iv1r: u32,
}
#[repr(C)] pub struct stm32_cryp_ctx { pub cryp: *mut stm32_cryp, pub keylen: i32, pub key: [__be32; 8], pub flags: usize }
#[repr(C)] pub struct stm32_cryp_reqctx { pub mode: usize }
#[repr(C)] pub struct stm32_cryp {
    pub dev: *mut core::ffi::c_void, pub regs: *mut u8, pub phys_base: phys_addr_t,
    pub flags: u32, pub irq_status: u32, pub caps: *const stm32_cryp_caps,
    pub ctx: *mut stm32_cryp_ctx, pub req: *mut core::ffi::c_void, pub areq: *mut core::ffi::c_void,
    pub authsize: usize, pub hw_blocksize: usize, pub payload_in: usize, pub header_in: usize,
    pub payload_out: usize, pub gcm_ctr: u32, pub last_ctr: [__be32; 4],
}

#[inline] unsafe fn is_aes(c: *const stm32_cryp) -> bool { (*c).flags & FLG_AES != 0 }
#[inline] unsafe fn is_des(c: *const stm32_cryp) -> bool { (*c).flags & FLG_DES != 0 }
#[inline] unsafe fn is_tdes(c: *const stm32_cryp) -> bool { (*c).flags & FLG_TDES != 0 }
#[inline] unsafe fn is_ecb(c: *const stm32_cryp) -> bool { (*c).flags & FLG_ECB != 0 }
#[inline] unsafe fn is_cbc(c: *const stm32_cryp) -> bool { (*c).flags & FLG_CBC != 0 }
#[inline] unsafe fn is_ctr(c: *const stm32_cryp) -> bool { (*c).flags & FLG_CTR != 0 }
#[inline] unsafe fn is_gcm(c: *const stm32_cryp) -> bool { (*c).flags & FLG_GCM != 0 }
#[inline] unsafe fn is_ccm(c: *const stm32_cryp) -> bool { (*c).flags & FLG_CCM != 0 }
#[inline] unsafe fn is_encrypt(c: *const stm32_cryp) -> bool { (*c).flags & FLG_ENCRYPT != 0 }
#[inline] unsafe fn is_decrypt(c: *const stm32_cryp) -> bool { !is_encrypt(c) }
#[inline] unsafe fn stm32_cryp_read(c: *const stm32_cryp, o: u32) -> u32 { readl_relaxed((*c).regs.add(o as usize)) }
#[inline] unsafe fn stm32_cryp_write(c: *mut stm32_cryp, o: u32, v: u32) { writel_relaxed(v, (*c).regs.add(o as usize)); }

#[inline] fn ux500_swap_bits_in_byte(b: u8) -> u8 {
    const R4:u8=0xc0; const R2:u8=0x28; const R1:u8=0x1e; const L4:u8=3; const L2:u8=0x14; const L1:u8=0x78;
    let mut n1=((b&R4)>>4)|(b&!(R4>>4)); n1=((n1&R2)>>2)|(n1&!(R2>>2)); n1=(n1&R1)>>1;
    let mut n2=((b&L4)<<4)|(b&!(L4<<4)); n2=((n2&L2)<<2)|(n2&!(L2<<2)); n2=(n2&L1)<<1; n1|n2
}
#[inline] unsafe fn ux500_swizzle_key(input: *const u8, output: *mut u8, len: u32) {
    let mut j = len as i32 - 4; while j >= 0 { for i in 0..4 { let index=len as i32-j-4+i; *output.add((j+i) as usize)=ux500_swap_bits_in_byte(*input.add(index as usize)); } j-=4; }
}

// External kernel-facing operations and the remaining driver entry points retain
// their C ABI and are supplied/linked by the translated kernel environment.
extern "C" {
    fn stm32_cryp_hw_write_iv(c: *mut stm32_cryp, iv: *mut __be32);
    fn stm32_cryp_hw_write_key(c: *mut stm32_cryp);
    fn stm32_cryp_finish_req(c: *mut stm32_cryp, err: i32);
    fn stm32_cryp_dma_start(c: *mut stm32_cryp) -> i32;
    fn stm32_cryp_it_start(c: *mut stm32_cryp) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
