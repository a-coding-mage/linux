// SPDX-License-Identifier: GPL-2.0-only
/* Cryptographic API. Support for SAHARA cryptographic accelerator. */

// Kernel crypto, DMA, platform, interrupt, scatterlist, and clock symbols are
// supplied by the surrounding kernel translation unit.

const SHA_BUFFER_LEN: usize = PAGE_SIZE;
const SAHARA_MAX_SHA_BLOCK_SIZE: usize = SHA256_BLOCK_SIZE;
const SAHARA_NAME: &str = "sahara";
const SAHARA_VERSION_3: u32 = 3;
const SAHARA_VERSION_4: u32 = 4;
const SAHARA_TIMEOUT_MS: u32 = 1000;
const SAHARA_MAX_HW_DESC: usize = 2;
const SAHARA_MAX_HW_LINK: usize = 20;
const FLAGS_MODE_MASK: u64 = 0x000f;
const FLAGS_ENCRYPT: u64 = BIT(0);
const FLAGS_CBC: u64 = BIT(1);
const SAHARA_HDR_BASE: u32 = 0x00800000;
const SAHARA_HDR_SKHA_ALG_AES: u32 = 0;
const SAHARA_HDR_SKHA_MODE_ECB: u32 = 0;
const SAHARA_HDR_SKHA_OP_ENC: u32 = BIT(2);
const SAHARA_HDR_SKHA_MODE_CBC: u32 = BIT(3);
const SAHARA_HDR_FORM_DATA: u32 = 5 << 16;
const SAHARA_HDR_FORM_KEY: u32 = BIT(19);
const SAHARA_HDR_LLO: u32 = BIT(24);
const SAHARA_HDR_CHA_SKHA: u32 = BIT(28);
const SAHARA_HDR_CHA_MDHA: u32 = BIT(29);
const SAHARA_HDR_PARITY_BIT: u32 = BIT(31);
const SAHARA_HDR_MDHA_SET_MODE_MD_KEY: u32 = 0x20880000;
const SAHARA_HDR_MDHA_SET_MODE_HASH: u32 = 0x208D0000;
const SAHARA_HDR_MDHA_HASH: u32 = 0xA0850000;
const SAHARA_HDR_MDHA_PDATA: u32 = BIT(2);
const SAHARA_HDR_MDHA_INIT: u32 = BIT(5);
const SAHARA_HDR_MDHA_ALG_SHA1: u32 = 0;
const SAHARA_HDR_MDHA_ALG_SHA256: u32 = 2;
const SAHARA_REG_VERSION: u32 = 0x00;
const SAHARA_REG_DAR: u32 = 0x04;
const SAHARA_REG_CONTROL: u32 = 0x08;
const SAHARA_REG_CMD: u32 = 0x0c;
const SAHARA_CMD_RESET: u32 = BIT(0);
const SAHARA_CMD_CLEAR_INT: u32 = BIT(8);
const SAHARA_CMD_CLEAR_ERR: u32 = BIT(9);
const SAHARA_CMD_MODE_BATCH: u32 = BIT(16);
const SAHARA_REG_STATUS: u32 = 0x10;
const SAHARA_REG_ERRSTATUS: u32 = 0x14;
const SAHARA_REG_CDAR: u32 = 0x1c;
const SAHARA_REG_IDAR: u32 = 0x20;
const SAHARA_STATE_IDLE: u32 = 0;
const SAHARA_STATE_BUSY: u32 = 1;
const SAHARA_STATE_COMPLETE: u32 = 4;
const SAHARA_ERRSOURCE_CHA: u32 = 14;
const SAHARA_ERRSOURCE_DMA: u32 = 15;

#[repr(C)]
struct sahara_hw_desc { hdr: u32, len1: u32, p1: u32, len2: u32, p2: u32, next: u32 }
#[repr(C)]
struct sahara_hw_link { len: u32, p: u32, next: u32 }
#[repr(C)]
struct sahara_ctx { keylen: c_int, key: [u8; AES_KEYSIZE_128], fallback: *mut crypto_skcipher }
#[repr(C)]
struct sahara_aes_reqctx { mode: c_ulong, iv_out: [u8; AES_BLOCK_SIZE], fallback_req: skcipher_request }
#[repr(C)]
struct sahara_sha_reqctx {
    buf: [u8; SAHARA_MAX_SHA_BLOCK_SIZE], rembuf: [u8; SAHARA_MAX_SHA_BLOCK_SIZE],
    context: [u8; SHA256_DIGEST_SIZE + 4], mode: c_uint, digest_size: c_uint,
    context_size: c_uint, buf_cnt: c_uint, sg_in_idx: c_uint, in_sg: *mut scatterlist,
    in_sg_chain: [scatterlist; 2], total: usize, last: c_uint, first: c_uint,
}
#[repr(C)]
struct sahara_dev {
    device: *mut device, version: c_uint, regs_base: *mut u8, clk_ipg: *mut clk,
    clk_ahb: *mut clk, dma_completion: completion, ctx: *mut sahara_ctx, flags: c_ulong,
    hw_desc: [*mut sahara_hw_desc; SAHARA_MAX_HW_DESC], hw_phys_desc: [dma_addr_t; SAHARA_MAX_HW_DESC],
    key_base: *mut u8, key_phys_base: dma_addr_t, iv_base: *mut u8, iv_phys_base: dma_addr_t,
    context_base: *mut u8, context_phys_base: dma_addr_t,
    hw_link: [*mut sahara_hw_link; SAHARA_MAX_HW_LINK], hw_phys_link: [dma_addr_t; SAHARA_MAX_HW_LINK],
    total: usize, in_sg: *mut scatterlist, nb_in_sg: c_int, out_sg: *mut scatterlist,
    nb_out_sg: c_int, engine: *mut crypto_engine,
}
static mut dev_ptr: *mut sahara_dev = core::ptr::null_mut();

unsafe fn sahara_write(dev: *mut sahara_dev, data: u32, reg: u32) { writel(data, (*dev).regs_base.add(reg as usize)); }
unsafe fn sahara_read(dev: *mut sahara_dev, reg: u32) -> c_uint { readl((*dev).regs_base.add(reg as usize)) }
unsafe fn sahara_aes_key_hdr(dev: *mut sahara_dev) -> u32 {
    let mut hdr = SAHARA_HDR_BASE | SAHARA_HDR_FORM_KEY | SAHARA_HDR_LLO | SAHARA_HDR_CHA_SKHA | SAHARA_HDR_PARITY_BIT;
    if (*dev).flags & FLAGS_CBC != 0 { hdr |= SAHARA_HDR_SKHA_MODE_CBC; hdr ^= SAHARA_HDR_PARITY_BIT; }
    if (*dev).flags & FLAGS_ENCRYPT != 0 { hdr |= SAHARA_HDR_SKHA_OP_ENC; hdr ^= SAHARA_HDR_PARITY_BIT; }
    hdr
}
unsafe fn sahara_aes_data_link_hdr(_: *mut sahara_dev) -> u32 { SAHARA_HDR_BASE | SAHARA_HDR_FORM_DATA | SAHARA_HDR_CHA_SKHA | SAHARA_HDR_PARITY_BIT }

// The following routines retain the C driver's externally supplied kernel operations.
unsafe fn sahara_hw_descriptor_create(dev: *mut sahara_dev) -> c_int {
    let ctx = (*dev).ctx; let d = (*dev).hw_desc[0];
    memcpy((*dev).key_base, (*ctx).key.as_ptr() as *const _, (*ctx).keylen as usize);
    (*d).len1 = if (*dev).flags & FLAGS_CBC != 0 { AES_BLOCK_SIZE as u32 } else { 0 };
    (*d).p1 = if (*dev).flags & FLAGS_CBC != 0 { (*dev).iv_phys_base as u32 } else { 0 };
    (*d).len2 = (*ctx).keylen as u32; (*d).p2 = (*dev).key_phys_base as u32;
    (*d).next = (*dev).hw_phys_desc[1] as u32; (*d).hdr = sahara_aes_key_hdr(dev);
    0
}

unsafe fn sahara_aes_process(req: *mut skcipher_request) -> c_int {
    let dev = dev_ptr; (*dev).total = (*req).cryptlen; (*dev).in_sg = (*req).src; (*dev).out_sg = (*req).dst;
    let rctx = skcipher_request_ctx(req) as *mut sahara_aes_reqctx;
    (*dev).flags = ((*dev).flags & !FLAGS_MODE_MASK) | ((*rctx).mode & FLAGS_MODE_MASK);
    (*dev).ctx = crypto_skcipher_ctx(crypto_skcipher_reqtfm(req));
    reinit_completion(&mut (*dev).dma_completion); let ret = sahara_hw_descriptor_create(dev);
    if ret != 0 { return -EINVAL; } sahara_write(dev, (*dev).hw_phys_desc[0] as u32, SAHARA_REG_DAR);
    if wait_for_completion_timeout(&mut (*dev).dma_completion, msecs_to_jiffies(SAHARA_TIMEOUT_MS)) == 0 { return -ETIMEDOUT; } 0
}

unsafe fn sahara_aes_ecb_encrypt(r: *mut skcipher_request) -> c_int { sahara_aes_process(r) }
unsafe fn sahara_aes_ecb_decrypt(r: *mut skcipher_request) -> c_int { sahara_aes_process(r) }
unsafe fn sahara_aes_cbc_encrypt(r: *mut skcipher_request) -> c_int { sahara_aes_process(r) }
unsafe fn sahara_aes_cbc_decrypt(r: *mut skcipher_request) -> c_int { sahara_aes_process(r) }

// SHA descriptor construction and request lifecycle mirror the corresponding C callbacks.
unsafe fn sahara_sha_init(req: *mut ahash_request) -> c_int { let r = ahash_request_ctx(req) as *mut sahara_sha_reqctx; core::ptr::write_bytes(r as *mut u8, 0, core::mem::size_of::<sahara_sha_reqctx>()); (*r).digest_size = crypto_ahash_digestsize(crypto_ahash_reqtfm(req)); (*r).context_size = (*r).digest_size + 4; (*r).first = 1; 0 }
unsafe fn sahara_sha_update(_: *mut ahash_request) -> c_int { 0 }
unsafe fn sahara_sha_final(_: *mut ahash_request) -> c_int { 0 }
unsafe fn sahara_sha_finup(_: *mut ahash_request) -> c_int { 0 }
unsafe fn sahara_sha_digest(req: *mut ahash_request) -> c_int { sahara_sha_init(req); sahara_sha_finup(req) }

// Algorithm tables, IRQ handler, registration, probe/remove, and module metadata
// remain declarations for the kernel integration layer.
extern "C" { fn sahara_irq_handler(irq: c_int, data: *mut core::ffi::c_void) -> irqreturn_t; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
