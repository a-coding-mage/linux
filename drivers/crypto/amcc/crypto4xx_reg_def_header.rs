/* SPDX-License-Identifier: GPL-2.0-or-later */
/* AMCC SoC PPC4xx Crypto Driver register definitions. */

pub const CRYPTO4XX_DESCRIPTOR: u32 = 0x00000000;
pub const CRYPTO4XX_CTRL_STAT: u32 = 0x00000000;
pub const CRYPTO4XX_SOURCE: u32 = 0x00000004;
pub const CRYPTO4XX_DEST: u32 = 0x00000008;
pub const CRYPTO4XX_SA: u32 = 0x0000000C;
pub const CRYPTO4XX_SA_LENGTH: u32 = 0x00000010;
pub const CRYPTO4XX_LENGTH: u32 = 0x00000014;
pub const CRYPTO4XX_PE_DMA_CFG: u32 = 0x40;
pub const CRYPTO4XX_PE_DMA_STAT: u32 = 0x44;
pub const CRYPTO4XX_PDR_BASE: u32 = 0x48;
pub const CRYPTO4XX_RDR_BASE: u32 = 0x4c;
pub const CRYPTO4XX_RING_SIZE: u32 = 0x50;
pub const CRYPTO4XX_RING_CTRL: u32 = 0x54;
pub const CRYPTO4XX_INT_RING_STAT: u32 = 0x58;
pub const CRYPTO4XX_EXT_RING_STAT: u32 = 0x5c;
pub const CRYPTO4XX_IO_THRESHOLD: u32 = 0x60;
pub const CRYPTO4XX_GATH_RING_BASE: u32 = 0x64;
pub const CRYPTO4XX_SCAT_RING_BASE: u32 = 0x68;
pub const CRYPTO4XX_PART_RING_SIZE: u32 = 0x6c;
pub const CRYPTO4XX_PART_RING_CFG: u32 = 0x70;
pub const CRYPTO4XX_PDR_BASE_UADDR: u32 = 0x80;
pub const CRYPTO4XX_RDR_BASE_UADDR: u32 = 0x84;
pub const CRYPTO4XX_PKT_SRC_UADDR: u32 = 0x88;
pub const CRYPTO4XX_PKT_DEST_UADDR: u32 = 0x8c;
pub const CRYPTO4XX_SA_UADDR: u32 = 0x90;
pub const CRYPTO4XX_GATH_RING_BASE_UADDR: u32 = 0xA0;
pub const CRYPTO4XX_SCAT_RING_BASE_UADDR: u32 = 0xA4;
pub const CRYPTO4XX_SEQ_RD: u32 = 0x408;
pub const CRYPTO4XX_SEQ_MASK_RD: u32 = 0x40C;
pub const CRYPTO4XX_SA_CMD_0: u32 = 0x10600;
pub const CRYPTO4XX_SA_CMD_1: u32 = 0x10604;
pub const CRYPTO4XX_STATE_PTR: u32 = 0x106dc;
pub const CRYPTO4XX_STATE_IV: u32 = 0x10700;
pub const CRYPTO4XX_STATE_HASH_BYTE_CNT_0: u32 = 0x10710;
pub const CRYPTO4XX_STATE_HASH_BYTE_CNT_1: u32 = 0x10714;
pub const CRYPTO4XX_STATE_IDIGEST_0: u32 = 0x10718;
pub const CRYPTO4XX_STATE_IDIGEST_1: u32 = 0x1071c;
pub const CRYPTO4XX_DATA_IN: u32 = 0x18000;
pub const CRYPTO4XX_DATA_OUT: u32 = 0x1c000;
pub const CRYPTO4XX_INT_UNMASK_STAT: u32 = 0x500a0;
pub const CRYPTO4XX_INT_MASK_STAT: u32 = 0x500a4;
pub const CRYPTO4XX_INT_CLR: u32 = 0x500a4;
pub const CRYPTO4XX_INT_EN: u32 = 0x500a8;
pub const CRYPTO4XX_INT_PKA: u32 = 0x2;
pub const CRYPTO4XX_INT_PDR_DONE: u32 = 0x8000;
pub const CRYPTO4XX_INT_MA_WR_ERR: u32 = 0x20000;
pub const CRYPTO4XX_INT_MA_RD_ERR: u32 = 0x10000;
pub const CRYPTO4XX_INT_PE_ERR: u32 = 0x200;
pub const CRYPTO4XX_INT_USER_DMA_ERR: u32 = 0x40;
pub const CRYPTO4XX_INT_SLAVE_ERR: u32 = 0x10;
pub const CRYPTO4XX_INT_MASTER_ERR: u32 = 0x8;
pub const CRYPTO4XX_INT_ERROR: u32 = 0x30258;
pub const CRYPTO4XX_INT_CFG: u32 = 0x500ac;
pub const CRYPTO4XX_INT_DESCR_RD: u32 = 0x500b0;
pub const CRYPTO4XX_INT_DESCR_CNT: u32 = 0x500b4;
pub const CRYPTO4XX_INT_TIMEOUT_CNT: u32 = 0x500b8;
pub const CRYPTO4XX_DEVICE_CTRL: u32 = 0x60080;
pub const CRYPTO4XX_DEVICE_ID: u32 = 0x60084;
pub const CRYPTO4XX_DEVICE_INFO: u32 = 0x60088;
pub const CRYPTO4XX_DMA_USER_SRC: u32 = 0x60094;
pub const CRYPTO4XX_DMA_USER_DEST: u32 = 0x60098;
pub const CRYPTO4XX_DMA_USER_CMD: u32 = 0x6009C;
pub const CRYPTO4XX_DMA_CFG: u32 = 0x600d4;
pub const CRYPTO4XX_BYTE_ORDER_CFG: u32 = 0x600d8;
pub const CRYPTO4XX_ENDIAN_CFG: u32 = 0x600d8;
pub const CRYPTO4XX_PRNG_CTRL: u32 = 0x70004;
pub const CRYPTO4XX_PRNG_SEED_L: u32 = 0x70008;
pub const CRYPTO4XX_PRNG_SEED_H: u32 = 0x7000c;

pub const PPC4XX_PDR_POLL: u32 = 0x3ff;
pub const PPC4XX_OUTPUT_THRESHOLD: u32 = 2;
pub const PPC4XX_INPUT_THRESHOLD: u32 = 2;
pub const PPC4XX_PD_SIZE: u32 = 6;
pub const PPC4XX_CTX_DONE_INT: u32 = 0x2000;
pub const PPC4XX_PD_DONE_INT: u32 = 0x8000;
pub const PPC4XX_TMO_ERR_INT: u32 = 0x40000;
pub const PPC4XX_BYTE_ORDER: u32 = 0x22222;
pub const PPC4XX_INTERRUPT_CLR: u32 = 0x3ffff;
pub const PPC4XX_PRNG_CTRL_AUTO_EN: u32 = 0x3;
pub const PPC4XX_DC_3DES_EN: u32 = 1;
pub const PPC4XX_TRNG_EN: u32 = 0x00020000;
pub const PPC4XX_INT_DESCR_CNT: u32 = 7;
pub const PPC4XX_INT_TIMEOUT_CNT: u32 = 0;
pub const PPC4XX_INT_TIMEOUT_CNT_REVB: u32 = 0x3FF;
pub const PPC4XX_INT_CFG: u32 = 1;
pub const PPC4XX_RING_RETRY: u32 = 100;
pub const PPC4XX_RING_POLL: u32 = 100;
pub const PPC4XX_SDR_SIZE: u32 = PPC4XX_NUM_SD;
pub const PPC4XX_GDR_SIZE: u32 = PPC4XX_NUM_GD;

pub const CRYPTO4XX_DMA_CFG_OFFSET: u32 = 0x40;
#[repr(C, packed)]
pub union ce_pe_dma_cfg { pub bf: u32, pub w: u32 }
pub const CRYPTO4XX_PDR_BASE_OFFSET: u32 = 0x48;
pub const CRYPTO4XX_RDR_BASE_OFFSET: u32 = 0x4c;
pub const CRYPTO4XX_RING_SIZE_OFFSET: u32 = 0x50;
#[repr(C, packed)] pub union ce_ring_size { pub bf: u32, pub w: u32 }
pub const CRYPTO4XX_RING_CONTROL_OFFSET: u32 = 0x54;
#[repr(C, packed)] pub union ce_ring_control { pub bf: u32, pub w: u32 }
pub const CRYPTO4XX_IO_THRESHOLD_OFFSET: u32 = 0x60;
#[repr(C, packed)] pub union ce_io_threshold { pub bf: u32, pub w: u32 }
pub const CRYPTO4XX_GATHER_RING_BASE_OFFSET: u32 = 0x64;
pub const CRYPTO4XX_SCATTER_RING_BASE_OFFSET: u32 = 0x68;
#[repr(C, packed)] pub union ce_part_ring_size { pub bf: u32, pub w: u32 }
pub const MAX_BURST_SIZE_32: u32 = 0;
pub const MAX_BURST_SIZE_64: u32 = 1;
pub const MAX_BURST_SIZE_128: u32 = 2;
pub const MAX_BURST_SIZE_256: u32 = 3;

#[repr(C, packed)]
pub struct gd_ctl_len { pub raw: u32 }
#[repr(C, packed)]
pub struct ce_gd { pub ptr: u32, pub ctl_len: gd_ctl_len }
#[repr(C, packed)]
pub struct sd_ctl { pub raw: u32 }
#[repr(C, packed)]
pub struct ce_sd { pub ptr: u32, pub ctl: sd_ctl }

pub const PD_PAD_CTL_32: u32 = 0x10;
pub const PD_PAD_CTL_64: u32 = 0x20;
pub const PD_PAD_CTL_128: u32 = 0x40;
pub const PD_PAD_CTL_256: u32 = 0x80;
#[repr(C, packed)] pub union ce_pd_ctl { pub bf: u32, pub w: u32 }
pub const PD_CTL_HASH_FINAL: u32 = 1 << 4;
pub const PD_CTL_PE_DONE: u32 = 1 << 1;
pub const PD_CTL_HOST_READY: u32 = 1 << 0;
#[repr(C, packed)] pub union ce_pd_ctl_len { pub bf: u32, pub w: u32 }
#[repr(C, packed)]
pub struct ce_pd {
    pub pd_ctl: ce_pd_ctl,
    pub src: u32,
    pub dest: u32,
    pub sa: u32,
    pub sa_len: u32,
    pub pd_ctl_len: ce_pd_ctl_len,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
