/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Driver for Digigram VXpocket soundcards
 *
 * Copyright (c) 2002 by Takashi Iwai <tiwai@suse.de>
 */

// C header dependencies:
// #include <sound/vx_core.h>
// #include <pcmcia/cistpl.h>
// #include <pcmcia/ds.h>

#[repr(C)]
pub struct snd_vxpocket {
    pub core: vx_core,
    pub port: libc::c_ulong,
    pub mic_level: libc::c_int, /* analog mic level (or boost) */
    pub regCDSP: libc::c_uint,  /* current CDSP register */
    pub regDIALOG: libc::c_uint, /* current DIALOG register */
    pub index: libc::c_int,     /* card index */

    /* pcmcia stuff */
    pub p_dev: *mut pcmcia_device,
}

#[inline]
pub unsafe fn to_vxpocket(x: *mut vx_core) -> *mut snd_vxpocket {
    (x as *mut u8).sub(core::mem::offset_of!(snd_vxpocket, core)) as *mut snd_vxpocket
}

unsafe extern "C" {
    pub static snd_vxpocket_ops: snd_vx_ops;

    pub fn vx_set_mic_boost(chip: *mut vx_core, boost: libc::c_int);
    pub fn vx_set_mic_level(chip: *mut vx_core, level: libc::c_int);

    pub fn vxp_add_mic_controls(chip: *mut vx_core) -> libc::c_int;
}

/* Constants used to access the CDSP register (0x08). */
pub const CDSP_MAGIC: libc::c_int = 0xA7; /* magic value (for read) */
/* for write */
pub const VXP_CDSP_CLOCKIN_SEL_MASK: libc::c_int = 0x80; /* 0 (internal), 1 (AES/EBU) */
pub const VXP_CDSP_DATAIN_SEL_MASK: libc::c_int = 0x40; /* 0 (analog), 1 (UER) */
pub const VXP_CDSP_SMPTE_SEL_MASK: libc::c_int = 0x20;
pub const VXP_CDSP_RESERVED_MASK: libc::c_int = 0x10;
pub const VXP_CDSP_MIC_SEL_MASK: libc::c_int = 0x08;
pub const VXP_CDSP_VALID_IRQ_MASK: libc::c_int = 0x04;
pub const VXP_CDSP_CODEC_RESET_MASK: libc::c_int = 0x02;
pub const VXP_CDSP_DSP_RESET_MASK: libc::c_int = 0x01;
/* VXPOCKET 240/440 */
pub const P24_CDSP_MICS_SEL_MASK: libc::c_int = 0x18;
pub const P24_CDSP_MIC20_SEL_MASK: libc::c_int = 0x10;
pub const P24_CDSP_MIC38_SEL_MASK: libc::c_int = 0x08;

/* Constants used to access the MEMIRQ register (0x0C). */
pub const P44_MEMIRQ_MASTER_SLAVE_SEL_MASK: libc::c_int = 0x08;
pub const P44_MEMIRQ_SYNCED_ALONE_SEL_MASK: libc::c_int = 0x04;
pub const P44_MEMIRQ_WCLK_OUT_IN_SEL_MASK: libc::c_int = 0x02; /* Not used */
pub const P44_MEMIRQ_WCLK_UER_SEL_MASK: libc::c_int = 0x01; /* Not used */

/* Micro levels (0x0C) */

/* Constants used to access the DIALOG register (0x0D). */
pub const VXP_DLG_XILINX_REPROG_MASK: libc::c_int = 0x80; /* W */
pub const VXP_DLG_DATA_XICOR_MASK: libc::c_int = 0x80; /* R */
pub const VXP_DLG_RESERVED4_0_MASK: libc::c_int = 0x40;
pub const VXP_DLG_RESERVED2_0_MASK: libc::c_int = 0x20;
pub const VXP_DLG_RESERVED1_0_MASK: libc::c_int = 0x10;
pub const VXP_DLG_DMAWRITE_SEL_MASK: libc::c_int = 0x08; /* W */
pub const VXP_DLG_DMAREAD_SEL_MASK: libc::c_int = 0x04; /* W */
pub const VXP_DLG_MEMIRQ_MASK: libc::c_int = 0x02; /* R */
pub const VXP_DLG_DMA16_SEL_MASK: libc::c_int = 0x02; /* W */
pub const VXP_DLG_ACK_MEMIRQ_MASK: libc::c_int = 0x01; /* R/W */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
