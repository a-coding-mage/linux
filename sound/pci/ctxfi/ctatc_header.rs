/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2008, Creative Technology Ltd. All Rights Reserved.
 *
 * @File	ctatc.h
 *
 * @Brief
 * This file contains the definition of the device resource management object.
 *
 * @Author	Liu Chun
 * @Date 	Mar 28 2008
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// C header dependencies:
// linux/types.h, linux/mutex.h, linux/pci.h, linux/timer.h, sound/core.h
// ctvmem.h, cthardware.h, ctresource.h

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CTALSADEVS {
    FRONT,
    SURROUND,
    CLFE,
    SIDE,
    IEC958,
    MIXER,
    NUM_CTALSADEVS, /* This should always be the last */
}

#[repr(C)]
pub struct ct_atc_chip_sub_details {
    pub subsys: u16,
    pub nm_model: *const c_char,
}

#[repr(C)]
pub struct ct_atc_chip_details {
    pub vendor: u16,
    pub device: u16,
    pub sub_details: *const ct_atc_chip_sub_details,
    pub nm_card: *const c_char,
}

#[repr(C)]
pub struct ct_atc {
    pub pci: *mut pci_dev,
    pub card: *mut snd_card,
    pub rsr: c_uint,      /* reference sample rate in Hz */
    pub msr: c_uint,      /* master sample rate in rsr */
    pub pll_rate: c_uint, /* current rate of Phase Lock Loop */

    pub chip_type: c_int,
    pub model: c_int,
    pub chip_name: *const c_char,
    pub model_name: *const c_char,

    pub rca_state: u8, /* 0 = dedicated RCA, 1 = 7.1ch Front */

    pub vm: *mut ct_vm, /* device virtual memory manager for this card */
    pub map_audio_buffer:
        Option<unsafe extern "C" fn(atc: *mut ct_atc, apcm: *mut ct_atc_pcm) -> c_int>,
    pub unmap_audio_buffer:
        Option<unsafe extern "C" fn(atc: *mut ct_atc, apcm: *mut ct_atc_pcm)>,
    pub get_ptp_phys: Option<unsafe extern "C" fn(atc: *mut ct_atc, index: c_int) -> c_ulong>,

    pub atc_mutex: mutex,

    pub pcm_playback_prepare:
        Option<unsafe extern "C" fn(atc: *mut ct_atc, apcm: *mut ct_atc_pcm) -> c_int>,
    pub pcm_playback_start:
        Option<unsafe extern "C" fn(atc: *mut ct_atc, apcm: *mut ct_atc_pcm) -> c_int>,
    pub pcm_playback_stop:
        Option<unsafe extern "C" fn(atc: *mut ct_atc, apcm: *mut ct_atc_pcm) -> c_int>,
    pub pcm_playback_position:
        Option<unsafe extern "C" fn(atc: *mut ct_atc, apcm: *mut ct_atc_pcm) -> c_int>,
    pub spdif_passthru_playback_prepare:
        Option<unsafe extern "C" fn(atc: *mut ct_atc, apcm: *mut ct_atc_pcm) -> c_int>,
    pub pcm_capture_prepare:
        Option<unsafe extern "C" fn(atc: *mut ct_atc, apcm: *mut ct_atc_pcm) -> c_int>,
    pub pcm_capture_start:
        Option<unsafe extern "C" fn(atc: *mut ct_atc, apcm: *mut ct_atc_pcm) -> c_int>,
    pub pcm_capture_stop:
        Option<unsafe extern "C" fn(atc: *mut ct_atc, apcm: *mut ct_atc_pcm) -> c_int>,
    pub pcm_capture_position:
        Option<unsafe extern "C" fn(atc: *mut ct_atc, apcm: *mut ct_atc_pcm) -> c_int>,
    pub pcm_release_resources:
        Option<unsafe extern "C" fn(atc: *mut ct_atc, apcm: *mut ct_atc_pcm) -> c_int>,
    pub select_line_in: Option<unsafe extern "C" fn(atc: *mut ct_atc) -> c_int>,
    pub select_mic_in: Option<unsafe extern "C" fn(atc: *mut ct_atc) -> c_int>,
    pub select_digit_io: Option<unsafe extern "C" fn(atc: *mut ct_atc) -> c_int>,
    pub line_front_unmute:
        Option<unsafe extern "C" fn(atc: *mut ct_atc, state: u8) -> c_int>,
    pub line_surround_unmute:
        Option<unsafe extern "C" fn(atc: *mut ct_atc, state: u8) -> c_int>,
    pub line_clfe_unmute: Option<unsafe extern "C" fn(atc: *mut ct_atc, state: u8) -> c_int>,
    pub line_rear_unmute: Option<unsafe extern "C" fn(atc: *mut ct_atc, state: u8) -> c_int>,
    pub line_in_unmute: Option<unsafe extern "C" fn(atc: *mut ct_atc, state: u8) -> c_int>,
    pub mic_unmute: Option<unsafe extern "C" fn(atc: *mut ct_atc, state: u8) -> c_int>,
    pub rca_unmute: Option<unsafe extern "C" fn(atc: *mut ct_atc, state: u8) -> c_int>,
    pub spdif_out_unmute:
        Option<unsafe extern "C" fn(atc: *mut ct_atc, state: u8) -> c_int>,
    pub spdif_in_unmute: Option<unsafe extern "C" fn(atc: *mut ct_atc, state: u8) -> c_int>,
    pub spdif_out_get_status:
        Option<unsafe extern "C" fn(atc: *mut ct_atc, status: *mut c_uint) -> c_int>,
    pub spdif_out_set_status:
        Option<unsafe extern "C" fn(atc: *mut ct_atc, status: c_uint) -> c_int>,
    pub spdif_out_passthru:
        Option<unsafe extern "C" fn(atc: *mut ct_atc, state: u8) -> c_int>,
    pub capabilities: Option<unsafe extern "C" fn(atc: *mut ct_atc) -> capabilities>,
    pub dedicated_rca_select: Option<unsafe extern "C" fn(atc: *mut ct_atc)>,
    pub output_switch_get: Option<unsafe extern "C" fn(atc: *mut ct_atc) -> c_int>,
    pub output_switch_put:
        Option<unsafe extern "C" fn(atc: *mut ct_atc, position: c_int) -> c_int>,
    pub mic_source_switch_get: Option<unsafe extern "C" fn(atc: *mut ct_atc) -> c_int>,
    pub mic_source_switch_put:
        Option<unsafe extern "C" fn(atc: *mut ct_atc, position: c_int) -> c_int>,

    /* Don't touch! Used for internal object. */
    pub rsc_mgrs: [*mut c_void; NUM_RSCTYP], /* chip resource managers */
    pub mixer: *mut c_void,                  /* internal mixer object */
    pub hw: *mut hw,                         /* chip specific hardware access object */
    pub daios: *mut *mut c_void,             /* digital audio io resources */
    pub pcm: *mut *mut c_void,               /* SUMs for collecting all pcm stream */
    pub srcs: *mut *mut c_void,              /* Sample Rate Converters for input signal */
    pub srcimps: *mut *mut c_void,           /* input mappers for SRCs */

    pub timer: *mut ct_timer,

    // Present in C only when CONFIG_PM_SLEEP is enabled.
    #[cfg(CONFIG_PM_SLEEP)]
    pub suspend: Option<unsafe extern "C" fn(atc: *mut ct_atc) -> c_int>,
    #[cfg(CONFIG_PM_SLEEP)]
    pub resume: Option<unsafe extern "C" fn(atc: *mut ct_atc) -> c_int>,
    #[cfg(CONFIG_PM_SLEEP)]
    pub pcms: [*mut snd_pcm; NUM_PCMS],
}

#[repr(C)]
pub struct ct_atc_pcm {
    pub substream: *mut snd_pcm_substream,
    pub interrupt: Option<unsafe extern "C" fn(apcm: *mut ct_atc_pcm)>,
    pub timer: *mut ct_timer_instance,
    pub started: c_uint, /* unsigned int started:1; */

    /* Only mono and interleaved modes are supported now. */
    pub vm_block: *mut ct_vm_block,
    pub src: *mut c_void,       /* SRC for interacting with host memory */
    pub srccs: *mut *mut c_void, /* SRCs for sample rate conversion */
    pub srcimps: *mut *mut c_void, /* SRC Input Mappers */
    pub amixers: *mut *mut c_void, /* AMIXERs for routing converted data */
    pub mono: *mut c_void,      /* A SUM resource for mixing chs to one */
    pub n_srcc: u8,             /* Number of converting SRCs */
    pub n_srcimp: u8,           /* Number of SRC Input Mappers */
    pub n_amixer: u8,           /* Number of AMIXERs */
}

#[cfg(CONFIG_PM_SLEEP)]
pub const NUM_PCMS: usize = CTALSADEVS::NUM_CTALSADEVS as usize - 1;

unsafe extern "C" {
    pub fn ct_atc_create(
        card: *mut snd_card,
        pci: *mut pci_dev,
        rsr: c_uint,
        msr: c_uint,
        chip_type: c_int,
        subsysid: c_uint,
        ratc: *mut *mut ct_atc,
    ) -> c_int;

    pub fn ct_atc_create_alsa_devs(atc: *mut ct_atc) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
