/* SPDX-License-Identifier: GPL-2.0-only */
/*****************************************************************************
 *
 * Copyright (C) 2008 Cedric Bregardis <cedric.bregardis@free.fr> and
 * Jean-Christian Hassler <jhassler@free.fr>
 *
 * This file is part of the Audiowerk2 ALSA driver
 *
 *****************************************************************************/

use core::ffi::{c_int, c_uint, c_ulong, c_void};

pub const NB_STREAM_PLAYBACK: c_int = 2;
pub const NB_STREAM_CAPTURE: c_int = 1;

pub const NUM_STREAM_PLAYBACK_ANA: c_int = 0;
pub const NUM_STREAM_PLAYBACK_DIG: c_int = 1;

pub const NUM_STREAM_CAPTURE_ANA: c_int = 0;

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

pub type snd_aw2_saa7146_it_cb = Option<unsafe extern "C" fn(*mut snd_pcm_substream)>;

#[repr(C)]
pub struct snd_aw2_saa7146_cb_param {
    pub p_it_callback: snd_aw2_saa7146_it_cb,
    pub p_callback_param: *mut snd_pcm_substream,
}

/* definition of the chip-specific record */

#[repr(C)]
pub struct snd_aw2_saa7146 {
    /* C type: void __iomem * */
    pub base_addr: *mut c_void,
}

unsafe extern "C" {
    pub fn snd_aw2_saa7146_setup(chip: *mut snd_aw2_saa7146, pci_base_addr: *mut c_void);
    pub fn snd_aw2_saa7146_free(chip: *mut snd_aw2_saa7146) -> c_int;

    pub fn snd_aw2_saa7146_pcm_init_playback(
        chip: *mut snd_aw2_saa7146,
        stream_number: c_int,
        dma_addr: c_ulong,
        period_size: c_ulong,
        buffer_size: c_ulong,
    );
    pub fn snd_aw2_saa7146_pcm_init_capture(
        chip: *mut snd_aw2_saa7146,
        stream_number: c_int,
        dma_addr: c_ulong,
        period_size: c_ulong,
        buffer_size: c_ulong,
    );
    pub fn snd_aw2_saa7146_define_it_playback_callback(
        stream_number: c_uint,
        p_it_callback: snd_aw2_saa7146_it_cb,
        p_callback_param: *mut c_void,
    );
    pub fn snd_aw2_saa7146_define_it_capture_callback(
        stream_number: c_uint,
        p_it_callback: snd_aw2_saa7146_it_cb,
        p_callback_param: *mut c_void,
    );
    pub fn snd_aw2_saa7146_pcm_trigger_start_capture(
        chip: *mut snd_aw2_saa7146,
        stream_number: c_int,
    );
    pub fn snd_aw2_saa7146_pcm_trigger_stop_capture(
        chip: *mut snd_aw2_saa7146,
        stream_number: c_int,
    );

    pub fn snd_aw2_saa7146_pcm_trigger_start_playback(
        chip: *mut snd_aw2_saa7146,
        stream_number: c_int,
    );
    pub fn snd_aw2_saa7146_pcm_trigger_stop_playback(
        chip: *mut snd_aw2_saa7146,
        stream_number: c_int,
    );

    pub fn snd_aw2_saa7146_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
    pub fn snd_aw2_saa7146_get_hw_ptr_playback(
        chip: *mut snd_aw2_saa7146,
        stream_number: c_int,
        start_addr: *mut u8,
        buffer_size: c_uint,
    ) -> c_uint;
    pub fn snd_aw2_saa7146_get_hw_ptr_capture(
        chip: *mut snd_aw2_saa7146,
        stream_number: c_int,
        start_addr: *mut u8,
        buffer_size: c_uint,
    ) -> c_uint;

    pub fn snd_aw2_saa7146_use_digital_input(chip: *mut snd_aw2_saa7146, use_digital: c_int);

    pub fn snd_aw2_saa7146_is_using_digital_input(chip: *mut snd_aw2_saa7146) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
