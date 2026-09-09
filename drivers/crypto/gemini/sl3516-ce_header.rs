/* SPDX-License-Identifier: GPL-2.0 */
/*
 * sl3516-ce.h - hardware cryptographic offloader for cortina/gemini SoC
 *
 * C header translated to Rust. External kernel types and functions are
 * intentionally referenced but not implemented here.
 */

pub const TQ0_TYPE_DATA: u32 = 0;
pub const TQ0_TYPE_CTRL: u32 = 1 << 0;
pub const TQ1_CIPHER: u32 = 1 << 1;
pub const TQ2_AUTH: u32 = 1 << 2;
pub const TQ3_IV: u32 = 1 << 3;
pub const TQ4_KEY0: u32 = 1 << 4;
pub const TQ5_KEY4: u32 = 1 << 5;
pub const TQ6_KEY6: u32 = 1 << 6;
pub const TQ7_AKEY0: u32 = 1 << 7;
pub const TQ8_AKEY2: u32 = 1 << 8;
pub const TQ9_AKEY2: u32 = 1 << 9;

pub const ECB_AES: u32 = 0x2;
pub const DESC_LAST: u32 = 0x01;
pub const DESC_FIRST: u32 = 0x02;
pub const IPSEC_ID: u32 = 0x0000;
pub const IPSEC_STATUS_REG: u32 = 0x00a8;
pub const IPSEC_RAND_NUM_REG: u32 = 0x00ac;
pub const IPSEC_DMA_DEVICE_ID: u32 = 0xff00;
pub const IPSEC_DMA_STATUS: u32 = 0xff04;
pub const IPSEC_TXDMA_CTRL: u32 = 0xff08;
pub const IPSEC_TXDMA_FIRST_DESC: u32 = 0xff0c;
pub const IPSEC_TXDMA_CURR_DESC: u32 = 0xff10;
pub const IPSEC_RXDMA_CTRL: u32 = 0xff14;
pub const IPSEC_RXDMA_FIRST_DESC: u32 = 0xff18;
pub const IPSEC_RXDMA_CURR_DESC: u32 = 0xff1c;
pub const IPSEC_TXDMA_BUF_ADDR: u32 = 0xff28;
pub const IPSEC_RXDMA_BUF_ADDR: u32 = 0xff38;
pub const IPSEC_RXDMA_BUF_SIZE: u32 = 0xff30;
pub const CE_ENCRYPTION: u32 = 0x01;
pub const CE_DECRYPTION: u32 = 0x03;
pub const MAXDESC: usize = 6;
pub const DMA_STATUS_RS_EOFI: u32 = 1 << 22;
pub const DMA_STATUS_RS_PERR: u32 = 1 << 24;
pub const DMA_STATUS_RS_DERR: u32 = 1 << 25;
pub const DMA_STATUS_TS_EOFI: u32 = 1 << 27;
pub const DMA_STATUS_TS_PERR: u32 = 1 << 29;
pub const DMA_STATUS_TS_DERR: u32 = 1 << 30;
pub const TXDMA_CTRL_START: u32 = 1 << 31;
pub const TXDMA_CTRL_CONTINUE: u32 = 1 << 30;
pub const TXDMA_CTRL_CHAIN_MODE: u32 = 1 << 29;
/* the burst value is not documented in the datasheet */
pub const TXDMA_CTRL_BURST_UNK: u32 = 1 << 22;
pub const TXDMA_CTRL_INT_FAIL: u32 = 1 << 17;
pub const TXDMA_CTRL_INT_PERR: u32 = 1 << 16;
pub const RXDMA_CTRL_START: u32 = 1 << 31;
pub const RXDMA_CTRL_CONTINUE: u32 = 1 << 30;
pub const RXDMA_CTRL_CHAIN_MODE: u32 = 1 << 29;
/* the burst value is not documented in the datasheet */
pub const RXDMA_CTRL_BURST_UNK: u32 = 1 << 22;
pub const RXDMA_CTRL_INT_FINISH: u32 = 1 << 18;
pub const RXDMA_CTRL_INT_FAIL: u32 = 1 << 17;
pub const RXDMA_CTRL_INT_PERR: u32 = 1 << 16;
pub const RXDMA_CTRL_INT_EOD: u32 = 1 << 15;
pub const RXDMA_CTRL_INT_EOF: u32 = 1 << 14;
pub const CE_CPU: u32 = 0;
pub const CE_DMA: u32 = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct desc_frame_ctrl { pub raw: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct desc_tx_flag_status { pub raw: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct desc_next { pub raw: u32 }
#[repr(C)]
pub union descriptor_frame_ctrl { pub raw: u32, pub bits: desc_frame_ctrl }
#[repr(C)]
pub union descriptor_flag_status { pub raw: u32, pub tx_flag: desc_tx_flag_status }
#[repr(C)]
pub union descriptor_next_desc { pub next_descriptor: u32, pub bits: desc_next }
#[repr(C)]
pub struct descriptor {
    pub frame_ctrl: descriptor_frame_ctrl,
    pub flag_status: descriptor_flag_status,
    pub buf_adr: u32,
    pub next_desc: descriptor_next_desc,
}

/* C bit-fields are represented by their containing word to preserve layout. */
#[repr(C)]
pub struct pkt_control_header { pub raw: u32 }
#[repr(C)]
pub struct pkt_control_cipher { pub algorithm_len: u16, pub header_len: u16 }
#[repr(C)]
pub struct pkt_control_ecb {
    pub control: pkt_control_header,
    pub cipher: pkt_control_cipher,
    pub key: [u8; AES_MAX_KEY_SIZE],
}

#[repr(C)]
pub struct sl3516_ce_dev {
    pub base: *mut core::ffi::c_void,
    pub clks: *mut clk,
    pub reset: *mut reset_control,
    pub dev: *mut device,
    pub engine: *mut crypto_engine,
    pub complete: completion,
    pub status: i32,
    pub dtx: dma_addr_t,
    pub tx: *mut descriptor,
    pub drx: dma_addr_t,
    pub rx: *mut descriptor,
    pub ctx: i32,
    pub crx: i32,
    pub trng: hwrng,
    pub hwrng_stat_req: c_ulong,
    pub hwrng_stat_bytes: c_ulong,
    pub stat_irq: c_ulong,
    pub stat_irq_tx: c_ulong,
    pub stat_irq_rx: c_ulong,
    pub stat_req: c_ulong,
    pub fallback_sg_count_tx: c_ulong,
    pub fallback_sg_count_rx: c_ulong,
    pub fallback_not_same_len: c_ulong,
    pub fallback_mod16: c_ulong,
    pub fallback_align16: c_ulong,
    pub dbgfs_dir: *mut dentry,
    pub dbgfs_stats: *mut dentry,
    pub pctrl: *mut core::ffi::c_void,
    pub dctrl: dma_addr_t,
}

#[repr(C)]
pub struct sginfo { pub addr: u32, pub len: u32 }
#[repr(C)]
pub struct sl3516_ce_cipher_req_ctx {
    pub t_src: [sginfo; MAXDESC], pub t_dst: [sginfo; MAXDESC],
    pub op_dir: u32, pub pctrllen: u32, pub tqflag: u32,
    pub h: *mut pkt_control_cipher, pub nr_sgs: i32, pub nr_sgd: i32,
    pub fallback_req: skcipher_request, // keep at the end
}
#[repr(C)]
pub struct sl3516_ce_cipher_tfm_ctx {
    pub key: *mut u32, pub keylen: u32, pub ce: *mut sl3516_ce_dev,
    pub fallback_tfm: *mut crypto_skcipher,
}
#[repr(C)]
pub union sl3516_ce_alg_union { pub skcipher: skcipher_engine_alg }
#[repr(C)]
pub struct sl3516_ce_alg_template {
    pub type_: u32, pub mode: u32, pub ce: *mut sl3516_ce_dev,
    pub alg: sl3516_ce_alg_union, pub stat_req: c_ulong,
    pub stat_fb: c_ulong, pub stat_bytes: c_ulong,
}

extern "C" {
    pub fn sl3516_ce_aes_setkey(tfm: *mut crypto_skcipher, key: *const u8, keylen: c_uint) -> i32;
    pub fn sl3516_ce_cipher_init(tfm: *mut crypto_tfm) -> i32;
    pub fn sl3516_ce_cipher_exit(tfm: *mut crypto_tfm);
    pub fn sl3516_ce_skdecrypt(areq: *mut skcipher_request) -> i32;
    pub fn sl3516_ce_skencrypt(areq: *mut skcipher_request) -> i32;
    pub fn sl3516_ce_run_task(ce: *mut sl3516_ce_dev, rctx: *mut sl3516_ce_cipher_req_ctx, name: *const i8) -> i32;
    pub fn sl3516_ce_rng_register(ce: *mut sl3516_ce_dev) -> i32;
    pub fn sl3516_ce_rng_unregister(ce: *mut sl3516_ce_dev);
    pub fn sl3516_ce_handle_cipher_request(engine: *mut crypto_engine, areq: *mut core::ffi::c_void) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
