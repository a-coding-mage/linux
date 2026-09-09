/* SPDX-License-Identifier: GPL-2.0 */

/* These are irrelevant for Sparc DMA, but we leave it in so that
 * things can compile.
 */
pub const MAX_DMA_CHANNELS: usize = 8;
pub const DMA_MODE_READ: usize = 1;
pub const DMA_MODE_WRITE: usize = 2;
pub const MAX_DMA_ADDRESS: usize = usize::MAX;

/* Useful constants */
pub const SIZE_16MB: usize = 16 * 1024 * 1024;
pub const SIZE_64K: usize = 64 * 1024;

/* SBUS DMA controller reg offsets */
pub const DMA_CSR: usize = 0x00;
pub const DMA_ADDR: usize = 0x04;
pub const DMA_COUNT: usize = 0x08;
pub const DMA_TEST: usize = 0x0c;

/* Fields in the cond_reg register */
/* First, the version identification bits */
pub const DMA_DEVICE_ID: u32 = 0xf0000000;
pub const DMA_VERS0: u32 = 0x00000000;
pub const DMA_ESCV1: u32 = 0x40000000;
pub const DMA_VERS1: u32 = 0x80000000;
pub const DMA_VERS2: u32 = 0xa0000000;
pub const DMA_VERHME: u32 = 0xb0000000;
pub const DMA_VERSPLUS: u32 = 0x90000000;

pub const DMA_HNDL_INTR: u32 = 0x00000001;
pub const DMA_HNDL_ERROR: u32 = 0x00000002;
pub const DMA_FIFO_ISDRAIN: u32 = 0x0000000c;
pub const DMA_INT_ENAB: u32 = 0x00000010;
pub const DMA_FIFO_INV: u32 = 0x00000020;
pub const DMA_ACC_SZ_ERR: u32 = 0x00000040;
pub const DMA_FIFO_STDRAIN: u32 = 0x00000040;
pub const DMA_RST_SCSI: u32 = 0x00000080;
pub const DMA_RST_ENET: u32 = DMA_RST_SCSI;
pub const DMA_ST_WRITE: u32 = 0x00000100;
pub const DMA_ENABLE: u32 = 0x00000200;
pub const DMA_PEND_READ: u32 = 0x00000400;
pub const DMA_ESC_BURST: u32 = 0x00000800;
pub const DMA_READ_AHEAD: u32 = 0x00001800;
pub const DMA_DSBL_RD_DRN: u32 = 0x00001000;
pub const DMA_BCNT_ENAB: u32 = 0x00002000;
pub const DMA_TERM_CNTR: u32 = 0x00004000;
pub const DMA_SCSI_SBUS64: u32 = 0x00008000;
pub const DMA_CSR_DISAB: u32 = 0x00010000;
pub const DMA_SCSI_DISAB: u32 = 0x00020000;
pub const DMA_DSBL_WR_INV: u32 = 0x00020000;
pub const DMA_ADD_ENABLE: u32 = 0x00040000;
pub const DMA_E_BURSTS: u32 = 0x000c0000;
pub const DMA_E_BURST32: u32 = 0x00040000;
pub const DMA_E_BURST16: u32 = 0x00000000;
pub const DMA_BRST_SZ: u32 = 0x000c0000;
pub const DMA_BRST64: u32 = 0x000c0000;
pub const DMA_BRST32: u32 = 0x00040000;
pub const DMA_BRST16: u32 = 0x00000000;
pub const DMA_BRST0: u32 = 0x00080000;
pub const DMA_ADDR_DISAB: u32 = 0x00100000;
pub const DMA_2CLKS: u32 = 0x00200000;
pub const DMA_3CLKS: u32 = 0x00400000;
pub const DMA_EN_ENETAUI: u32 = DMA_3CLKS;
pub const DMA_CNTR_DISAB: u32 = 0x00800000;
pub const DMA_AUTO_NADDR: u32 = 0x01000000;
pub const DMA_SCSI_ON: u32 = 0x02000000;
pub const DMA_PARITY_OFF: u32 = 0x02000000;
pub const DMA_LOADED_ADDR: u32 = 0x04000000;
pub const DMA_LOADED_NADDR: u32 = 0x08000000;
pub const DMA_RESET_FAS366: u32 = 0x08000000;

/* Values describing the burst-size property from the PROM */
pub const DMA_BURST1: u32 = 0x01;
pub const DMA_BURST2: u32 = 0x02;
pub const DMA_BURST4: u32 = 0x04;
pub const DMA_BURST8: u32 = 0x08;
pub const DMA_BURST16: u32 = 0x10;
pub const DMA_BURST32: u32 = 0x20;
pub const DMA_BURST64: u32 = 0x40;
pub const DMA_BURSTBITS: u32 = 0x7f;

/* The following declarations are present only when CONFIG_SPARC32 is enabled. */
#[cfg(CONFIG_SPARC32)]
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[cfg(CONFIG_SPARC32)]
extern "C" {
    pub fn sparc_dma_alloc_resource(dev: *mut device, len: usize) -> usize;
    pub fn sparc_dma_free_resource(cpu_addr: *mut core::ffi::c_void, size: usize) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
