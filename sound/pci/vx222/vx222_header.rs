/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Driver for Digigram VX222 PCI soundcards
 *
 * Copyright (c) 2002 by Takashi Iwai <tiwai@suse.de>
 */

/* C header dependency: <sound/vx_core.h> */

use std::os::raw::{c_int, c_ulong};

#[repr(C)]
pub struct snd_vx222 {
    pub core: vx_core,

    /* h/w config; for PLX and for DSP */
    pub pci: *mut pci_dev,
    pub port: [c_ulong; 2],

    pub regCDSP: u32,   /* current CDSP register */
    pub regCFG: u32,    /* current CFG register */
    pub regSELMIC: u32, /* current SELMIC reg. (for VX222 Mic) */

    pub input_level: [c_int; 2], /* input level for vx222 mic */
    pub mic_level: c_int,        /* mic level for vx222 mic */
}

/* C macro: container_of(x, struct snd_vx222, core) */
#[macro_export]
macro_rules! to_vx222 {
    ($x:expr) => {
        container_of!($x, snd_vx222, core)
    };
}

/* we use a lookup table with 148 values, see vx_mixer.c */
pub const VX2_AKM_LEVEL_MAX: u32 = 0x93;

unsafe extern "C" {
    pub static vx222_ops: snd_vx_ops;
    pub static vx222_old_ops: snd_vx_ops;
}

/* Offset of registers with base equal to portDSP. */
pub const VX_RESET_DMA_REGISTER_OFFSET: u32 = 0x00000008;

/* Constants used to access the INTCSR register. */
pub const VX_INTCSR_VALUE: u32 = 0x00000001;
pub const VX_PCI_INTERRUPT_MASK: u32 = 0x00000040;

/* Constants used to access the CDSP register (0x20). */
pub const VX_CDSP_TEST1_MASK: u32 = 0x00000080;
pub const VX_CDSP_TOR1_MASK: u32 = 0x00000040;
pub const VX_CDSP_TOR2_MASK: u32 = 0x00000020;
pub const VX_CDSP_RESERVED0_0_MASK: u32 = 0x00000010;
pub const VX_CDSP_CODEC_RESET_MASK: u32 = 0x00000008;
pub const VX_CDSP_VALID_IRQ_MASK: u32 = 0x00000004;
pub const VX_CDSP_TEST0_MASK: u32 = 0x00000002;
pub const VX_CDSP_DSP_RESET_MASK: u32 = 0x00000001;

pub const VX_CDSP_GPIO_OUT_MASK: u32 = 0x00000060;
pub const VX_GPIO_OUT_BIT_OFFSET: u32 = 5; /* transform output to bit 0 and 1 */

/* Constants used to access the CFG register (0x24). */
pub const VX_CFG_SYNCDSP_MASK: u32 = 0x00000080;
pub const VX_CFG_RESERVED0_0_MASK: u32 = 0x00000040;
pub const VX_CFG_RESERVED1_0_MASK: u32 = 0x00000020;
pub const VX_CFG_RESERVED2_0_MASK: u32 = 0x00000010;
pub const VX_CFG_DATAIN_SEL_MASK: u32 = 0x00000008; /* 0 (ana), 1 (UER) */
pub const VX_CFG_RESERVED3_0_MASK: u32 = 0x00000004;
pub const VX_CFG_RESERVED4_0_MASK: u32 = 0x00000002;
pub const VX_CFG_CLOCKIN_SEL_MASK: u32 = 0x00000001; /* 0 (internal), 1 (AES/EBU) */

/* Constants used to access the STATUS register (0x30). */
pub const VX_STATUS_DATA_XICOR_MASK: u32 = 0x00000080;
pub const VX_STATUS_VAL_TEST1_MASK: u32 = 0x00000040;
pub const VX_STATUS_VAL_TEST0_MASK: u32 = 0x00000020;
pub const VX_STATUS_RESERVED0_MASK: u32 = 0x00000010;
pub const VX_STATUS_VAL_TOR1_MASK: u32 = 0x00000008;
pub const VX_STATUS_VAL_TOR0_MASK: u32 = 0x00000004;
pub const VX_STATUS_LEVEL_IN_MASK: u32 = 0x00000002; /* 6 dBu (0), 22 dBu (1) */
pub const VX_STATUS_MEMIRQ_MASK: u32 = 0x00000001;

pub const VX_STATUS_GPIO_IN_MASK: u32 = 0x0000000C;
pub const VX_GPIO_IN_BIT_OFFSET: u32 = 0; /* leave input as bit 2 and 3 */

/* Constants used to access the MICRO INPUT SELECT register (0x40). */
pub const MICRO_SELECT_INPUT_NORM: u32 = 0x00;
pub const MICRO_SELECT_INPUT_MUTE: u32 = 0x01;
pub const MICRO_SELECT_INPUT_LIMIT: u32 = 0x02;
pub const MICRO_SELECT_INPUT_MASK: u32 = 0x03;

pub const MICRO_SELECT_PREAMPLI_G_0: u32 = 0x00;
pub const MICRO_SELECT_PREAMPLI_G_1: u32 = 0x04;
pub const MICRO_SELECT_PREAMPLI_G_2: u32 = 0x08;
pub const MICRO_SELECT_PREAMPLI_G_3: u32 = 0x0C;
pub const MICRO_SELECT_PREAMPLI_MASK: u32 = 0x0C;
pub const MICRO_SELECT_PREAMPLI_OFFSET: u32 = 2;

pub const MICRO_SELECT_RAISE_COMPR: u32 = 0x10;

pub const MICRO_SELECT_NOISE_T_52DB: u32 = 0x00;
pub const MICRO_SELECT_NOISE_T_42DB: u32 = 0x20;
pub const MICRO_SELECT_NOISE_T_32DB: u32 = 0x40;
pub const MICRO_SELECT_NOISE_T_MASK: u32 = 0x60;

pub const MICRO_SELECT_PHANTOM_ALIM: u32 = 0x80;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
