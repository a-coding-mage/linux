/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * atmel_ssc_dai.h - ALSA SSC interface for the Atmel  SoC
 *
 * Copyright (C) 2005 SAN People
 * Copyright (C) 2008 Atmel
 *
 * Author: Sedji Gaouaou <sedji.gaouaou@atmel.com>
 *         ATMEL CORP.
 *
 * Based on at91-ssc.c by
 * Frank Mandarino <fmandarino@endrelia.com>
 * Based on pxa2xx Platform drivers by
 * Liam Girdwood <lrg@slimlogic.co.uk>
 */

/* C header guard _ATMEL_SSC_DAI_H omitted in Rust. */

/* Dependencies from C includes:
 * <linux/types.h>
 * <linux/atmel-ssc.h>
 * "atmel-pcm.h"
 */

use core::ffi::{c_char, c_int, c_ulong};

/* SSC system clock ids */
pub const ATMEL_SYSCLK_MCK: u32 = 0; /* SSC uses AT91 MCK as system clock */

/* SSC divider ids */
pub const ATMEL_SSC_CMR_DIV: u32 = 0; /* MCK divider for BCLK */
pub const ATMEL_SSC_TCMR_PERIOD: u32 = 1; /* BCLK divider for transmit FS */
pub const ATMEL_SSC_RCMR_PERIOD: u32 = 2; /* BCLK divider for receive FS */
/*
 * SSC direction masks
 */
pub const SSC_DIR_MASK_UNUSED: u32 = 0;
pub const SSC_DIR_MASK_PLAYBACK: u32 = 1;
pub const SSC_DIR_MASK_CAPTURE: u32 = 2;

/*
 * SSC register values that Atmel left out of <linux/atmel-ssc.h>.  These
 * are expected to be used with SSC_BF
 */
/* START bit field values */
pub const SSC_START_CONTINUOUS: u32 = 0;
pub const SSC_START_TX_RX: u32 = 1;
pub const SSC_START_LOW_RF: u32 = 2;
pub const SSC_START_HIGH_RF: u32 = 3;
pub const SSC_START_FALLING_RF: u32 = 4;
pub const SSC_START_RISING_RF: u32 = 5;
pub const SSC_START_LEVEL_RF: u32 = 6;
pub const SSC_START_EDGE_RF: u32 = 7;
pub const SSS_START_COMPARE_0: u32 = 8;

/* CKI bit field values */
pub const SSC_CKI_FALLING: u32 = 0;
pub const SSC_CKI_RISING: u32 = 1;

/* CKO bit field values */
pub const SSC_CKO_NONE: u32 = 0;
pub const SSC_CKO_CONTINUOUS: u32 = 1;
pub const SSC_CKO_TRANSFER: u32 = 2;

/* CKS bit field values */
pub const SSC_CKS_DIV: u32 = 0;
pub const SSC_CKS_CLOCK: u32 = 1;
pub const SSC_CKS_PIN: u32 = 2;

/* FSEDGE bit field values */
pub const SSC_FSEDGE_POSITIVE: u32 = 0;
pub const SSC_FSEDGE_NEGATIVE: u32 = 1;

/* FSOS bit field values */
pub const SSC_FSOS_NONE: u32 = 0;
pub const SSC_FSOS_NEGATIVE: u32 = 1;
pub const SSC_FSOS_POSITIVE: u32 = 2;
pub const SSC_FSOS_LOW: u32 = 3;
pub const SSC_FSOS_HIGH: u32 = 4;
pub const SSC_FSOS_TOGGLE: u32 = 5;

pub const START_DELAY: u32 = 1;

#[repr(C)]
pub struct atmel_ssc_state {
    pub ssc_cmr: u32,
    pub ssc_rcmr: u32,
    pub ssc_rfmr: u32,
    pub ssc_tcmr: u32,
    pub ssc_tfmr: u32,
    pub ssc_sr: u32,
    pub ssc_imr: u32,
}

#[repr(C)]
pub struct atmel_ssc_info {
    pub name: *mut c_char,
    pub ssc: *mut ssc_device,
    pub dir_mask: u16, /* 0=unused, 1=playback, 2=capture */
    pub initialized: u16, /* true if SSC has been initialized */
    pub daifmt: u16,
    pub cmr_div: u16,
    pub tcmr_period: u16,
    pub rcmr_period: u16,
    pub forced_divider: u32,
    pub dma_params: [*mut atmel_pcm_dma_params; 2],
    pub ssc_state: atmel_ssc_state,
    pub mck_rate: c_ulong,
}

unsafe extern "C" {
    pub fn atmel_ssc_set_audio(ssc_id: c_int) -> c_int;
    pub fn atmel_ssc_put_audio(ssc_id: c_int);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
