// SPDX-License-Identifier: GPL-2.0
//
// Faithful low-level Rust translation of the Samsung S5P/Exynos SSS driver.
// Kernel-provided types, functions, and register accessors are intentionally
// left as external dependencies, matching the original translation boundary.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

const fn bit(n: u32) -> u32 { 1u32 << n }
const fn sbf(s: u32, v: u32) -> u32 { v << s }

// Feed-control and AES register constants.
const SSS_REG_FCINTSTAT: usize = 0x0000;
const SSS_FCINTSTAT_HPARTINT: u32 = bit(7);
const SSS_FCINTSTAT_HDONEINT: u32 = bit(5);
const SSS_FCINTSTAT_BRDMAINT: u32 = bit(3);
const SSS_FCINTSTAT_BTDMAINT: u32 = bit(2);
const SSS_FCINTSTAT_HRDMAINT: u32 = bit(1);
const SSS_FCINTSTAT_PKDMAINT: u32 = bit(0);
const SSS_REG_FCINTENSET: usize = 0x0004;
const SSS_REG_FCINTENCLR: usize = 0x0008;
const SSS_REG_FCINTPEND: usize = 0x000c;
const SSS_REG_FCFIFOSTAT: usize = 0x0010;
const SSS_REG_FCFIFOCTRL: usize = 0x0014;
const SSS_FCFIFOCTRL_DESSEL: u32 = bit(2);
const SSS_HASHIN_INDEPENDENT: u32 = sbf(0, 0x00);
const SSS_HASHIN_CIPHER_INPUT: u32 = sbf(0, 0x01);
const SSS_HASHIN_CIPHER_OUTPUT: u32 = sbf(0, 0x02);
const SSS_HASHIN_MASK: u32 = sbf(0, 0x03);
const SSS_REG_FCBRDMAS: usize = 0x20;
const SSS_REG_FCBRDMAL: usize = 0x24;
const SSS_REG_FCBTDMAS: usize = 0x30;
const SSS_REG_FCBTDMAL: usize = 0x34;
const SSS_REG_FCHRDMAS: usize = 0x40;
const SSS_REG_FCHRDMAL: usize = 0x44;
const SSS_REG_FCHRDMAC: usize = 0x48;
const SSS_FCHRDMAC_FLUSH: u32 = bit(0);
const SSS_REG_FCPKDMAS: usize = 0x50;
const SSS_REG_FCPKDMAL: usize = 0x54;
const SSS_REG_AES_CONTROL: usize = 0;
const SSS_AES_BYTESWAP_DI: u32 = bit(11);
const SSS_AES_BYTESWAP_DO: u32 = bit(10);
const SSS_AES_BYTESWAP_IV: u32 = bit(9);
const SSS_AES_BYTESWAP_CNT: u32 = bit(8);
const SSS_AES_BYTESWAP_KEY: u32 = bit(7);
const SSS_AES_KEY_CHANGE_MODE: u32 = bit(6);
const SSS_AES_KEY_SIZE_128: u32 = sbf(4, 0);
const SSS_AES_KEY_SIZE_192: u32 = sbf(4, 1);
const SSS_AES_KEY_SIZE_256: u32 = sbf(4, 2);
const SSS_AES_FIFO_MODE: u32 = bit(3);
const SSS_AES_CHAIN_MODE_ECB: u32 = sbf(1, 0);
const SSS_AES_CHAIN_MODE_CBC: u32 = sbf(1, 1);
const SSS_AES_CHAIN_MODE_CTR: u32 = sbf(1, 2);
const SSS_AES_MODE_DECRYPT: u32 = bit(0);
const FLAGS_AES_DECRYPT: u32 = bit(0);
const FLAGS_AES_MODE_MASK: u32 = sbf(1, 3);
const FLAGS_AES_CBC: u32 = sbf(1, 1);
const FLAGS_AES_CTR: u32 = sbf(1, 2);
const AES_KEY_LEN: usize = 16;
const CRYPTO_QUEUE_LEN: usize = 1;
const HASH_BLOCK_SIZE: usize = 64;
const BUFLEN: usize = HASH_BLOCK_SIZE;
const SSS_HASH_QUEUE_LENGTH: usize = 10;

#[repr(C)]
pub struct samsung_aes_variant {
    pub aes_offset: u32,
    pub hash_offset: u32,
    pub clk_names: [*const core::ffi::c_char; 2],
}

#[repr(C)]
pub struct s5p_aes_reqctx { pub mode: usize }

#[repr(C)]
pub struct s5p_aes_ctx {
    pub dev: *mut s5p_aes_dev,
    pub aes_key: [u8; 32],
    pub nonce: [u8; 4],
    pub keylen: i32,
}

#[repr(C)]
pub struct s5p_hash_reqctx {
    pub dd: *mut s5p_aes_dev,
    pub op_update: bool,
    pub digcnt: u64,
    pub digest: [u8; 32],
    pub nregs: u32,
    pub engine: u32,
    pub sg: *mut core::ffi::c_void,
    pub sg_len: u32,
    pub sgl: [core::ffi::c_void; 2],
    pub skip: usize,
    pub total: usize,
    pub finup: bool,
    pub error: bool,
    pub bufcnt: u32,
    pub buffer: [u8; BUFLEN],
}

#[repr(C)]
pub struct s5p_aes_dev {
    pub dev: *mut core::ffi::c_void,
    pub clk: *mut core::ffi::c_void,
    pub pclk: *mut core::ffi::c_void,
    pub ioaddr: *mut u8,
    pub aes_ioaddr: *mut u8,
    pub irq_fc: i32,
    pub req: *mut core::ffi::c_void,
    pub ctx: *mut s5p_aes_ctx,
    pub sg_src: *mut core::ffi::c_void,
    pub sg_dst: *mut core::ffi::c_void,
    pub sg_src_cpy: *mut core::ffi::c_void,
    pub sg_dst_cpy: *mut core::ffi::c_void,
    pub busy: bool,
    pub res: *mut core::ffi::c_void,
    pub io_hash_base: *mut u8,
    pub hash_flags: usize,
    pub xmit_buf: [u8; BUFLEN],
    pub hash_req: *mut core::ffi::c_void,
    pub hash_sg_iter: *mut core::ffi::c_void,
    pub hash_sg_cnt: u32,
    pub use_hash: bool,
}

// The remaining driver routines retain their C ABI and are supplied by the
// kernel integration layer; their declarations are intentionally external.
extern "C" {
    fn s5p_aes_interrupt(irq: i32, dev_id: *mut core::ffi::c_void) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
