/* SPDX-License-Identifier: GPL-2.0 */
/*
 * mpc8610-pcm.h - ALSA PCM interface for the Freescale MPC8610 SoC
 */

// C header dependencies: u8, u32, u64, __be32, and __be64 are expected to be
// provided by surrounding kernel/bindings context.

#[repr(C)]
pub struct ccsr_dma {
    pub res0: [u8; 0x100],
    pub channel: [ccsr_dma_channel; 4],
    pub dgsr: __be32,
}

#[repr(C)]
pub struct ccsr_dma_channel {
    pub mr: __be32,      /* Mode register */
    pub sr: __be32,      /* Status register */
    pub eclndar: __be32, /* Current link descriptor extended addr reg */
    pub clndar: __be32,  /* Current link descriptor address register */
    pub satr: __be32,    /* Source attributes register */
    pub sar: __be32,     /* Source address register */
    pub datr: __be32,    /* Destination attributes register */
    pub dar: __be32,     /* Destination address register */
    pub bcr: __be32,     /* Byte count register */
    pub enlndar: __be32, /* Next link descriptor extended address reg */
    pub nlndar: __be32,  /* Next link descriptor address register */
    pub res1: [u8; 4],
    pub eclsdar: __be32, /* Current list descriptor extended addr reg */
    pub clsdar: __be32,  /* Current list descriptor address register */
    pub enlsdar: __be32, /* Next list descriptor extended address reg */
    pub nlsdar: __be32,  /* Next list descriptor address register */
    pub ssr: __be32,     /* Source stride register */
    pub dsr: __be32,     /* Destination stride register */
    pub res2: [u8; 0x38],
}

pub const CCSR_DMA_MR_BWC_DISABLED: u32 = 0x0F000000;
pub const CCSR_DMA_MR_BWC_SHIFT: u32 = 24;
pub const CCSR_DMA_MR_BWC_MASK: u32 = 0x0F000000;

// C macro: ((ilog2(x) << CCSR_DMA_MR_BWC_SHIFT) & CCSR_DMA_MR_BWC_MASK)
pub unsafe fn CCSR_DMA_MR_BWC(x: u32) -> u32 {
    ((ilog2(x) as u32) << CCSR_DMA_MR_BWC_SHIFT) & CCSR_DMA_MR_BWC_MASK
}

pub const CCSR_DMA_MR_EMP_EN: u32 = 0x00200000;
pub const CCSR_DMA_MR_EMS_EN: u32 = 0x00040000;
pub const CCSR_DMA_MR_DAHTS_MASK: u32 = 0x00030000;
pub const CCSR_DMA_MR_DAHTS_1: u32 = 0x00000000;
pub const CCSR_DMA_MR_DAHTS_2: u32 = 0x00010000;
pub const CCSR_DMA_MR_DAHTS_4: u32 = 0x00020000;
pub const CCSR_DMA_MR_DAHTS_8: u32 = 0x00030000;
pub const CCSR_DMA_MR_SAHTS_MASK: u32 = 0x0000C000;
pub const CCSR_DMA_MR_SAHTS_1: u32 = 0x00000000;
pub const CCSR_DMA_MR_SAHTS_2: u32 = 0x00004000;
pub const CCSR_DMA_MR_SAHTS_4: u32 = 0x00008000;
pub const CCSR_DMA_MR_SAHTS_8: u32 = 0x0000C000;
pub const CCSR_DMA_MR_DAHE: u32 = 0x00002000;
pub const CCSR_DMA_MR_SAHE: u32 = 0x00001000;
pub const CCSR_DMA_MR_SRW: u32 = 0x00000400;
pub const CCSR_DMA_MR_EOSIE: u32 = 0x00000200;
pub const CCSR_DMA_MR_EOLNIE: u32 = 0x00000100;
pub const CCSR_DMA_MR_EOLSIE: u32 = 0x00000080;
pub const CCSR_DMA_MR_EIE: u32 = 0x00000040;
pub const CCSR_DMA_MR_XFE: u32 = 0x00000020;
pub const CCSR_DMA_MR_CDSM_SWSM: u32 = 0x00000010;
pub const CCSR_DMA_MR_CA: u32 = 0x00000008;
pub const CCSR_DMA_MR_CTM: u32 = 0x00000004;
pub const CCSR_DMA_MR_CC: u32 = 0x00000002;
pub const CCSR_DMA_MR_CS: u32 = 0x00000001;

pub const CCSR_DMA_SR_TE: u32 = 0x00000080;
pub const CCSR_DMA_SR_CH: u32 = 0x00000020;
pub const CCSR_DMA_SR_PE: u32 = 0x00000010;
pub const CCSR_DMA_SR_EOLNI: u32 = 0x00000008;
pub const CCSR_DMA_SR_CB: u32 = 0x00000004;
pub const CCSR_DMA_SR_EOSI: u32 = 0x00000002;
pub const CCSR_DMA_SR_EOLSI: u32 = 0x00000001;

/* ECLNDAR takes bits 32-36 of the CLNDAR register */
pub unsafe fn CCSR_DMA_ECLNDAR_ADDR(x: u64) -> u32 {
    ((x >> 32) & 0xf) as u32
}

pub const fn CCSR_DMA_CLNDAR_ADDR(x: u32) -> u32 {
    x & 0xFFFFFFFE
}

pub const CCSR_DMA_CLNDAR_EOSIE: u32 = 0x00000008;

/* SATR and DATR, combined */
pub const CCSR_DMA_ATR_PBATMU: u32 = 0x20000000;
pub const CCSR_DMA_ATR_TFLOWLVL_0: u32 = 0x00000000;
pub const CCSR_DMA_ATR_TFLOWLVL_1: u32 = 0x06000000;
pub const CCSR_DMA_ATR_TFLOWLVL_2: u32 = 0x08000000;
pub const CCSR_DMA_ATR_TFLOWLVL_3: u32 = 0x0C000000;
pub const CCSR_DMA_ATR_PCIORDER: u32 = 0x02000000;
pub const CCSR_DMA_ATR_SME: u32 = 0x01000000;
pub const CCSR_DMA_ATR_NOSNOOP: u32 = 0x00040000;
pub const CCSR_DMA_ATR_SNOOP: u32 = 0x00050000;
pub const CCSR_DMA_ATR_ESAD_MASK: u32 = 0x0000000F;

/*
 *  List Descriptor for extended chaining mode DMA operations.
 *
 *  The CLSDAR register points to the first (in a linked-list) List
 *  Descriptor.  Each object must be aligned on a 32-byte boundary. Each
 *  list descriptor points to a linked-list of link Descriptors.
 */
// C attribute intent: __attribute__ ((aligned(32), packed)).
#[repr(C, packed)]
pub struct fsl_dma_list_descriptor {
    pub next: __be64,       /* Address of next list descriptor */
    pub first_link: __be64, /* Address of first link descriptor */
    pub source: __be32,     /* Source stride */
    pub dest: __be32,       /* Destination stride */
    pub res: [u8; 8],       /* Reserved */
}

/*
 *  Link Descriptor for basic and extended chaining mode DMA operations.
 *
 *  A Link Descriptor points to a single DMA buffer.  Each link descriptor
 *  must be aligned on a 32-byte boundary.
 */
// C attribute intent: __attribute__ ((aligned(32), packed)).
#[repr(C, packed)]
pub struct fsl_dma_link_descriptor {
    pub source_attr: __be32, /* Programmed into SATR register */
    pub source_addr: __be32, /* Programmed into SAR register */
    pub dest_attr: __be32,   /* Programmed into DATR register */
    pub dest_addr: __be32,   /* Programmed into DAR register */
    pub next: __be64,        /* Address of next link descriptor */
    pub count: __be32,       /* Byte count */
    pub res: [u8; 4],        /* Reserved */
}

unsafe extern "C" {
    pub fn ilog2(n: u32) -> i32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
