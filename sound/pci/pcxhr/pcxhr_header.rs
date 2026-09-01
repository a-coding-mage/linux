/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Driver for Digigram pcxhr soundcards
 *
 * main header file
 *
 * Copyright (c) 2004 by Digigram <alsa@digigram.com>
 */

/* C header dependencies:
 * #include <linux/interrupt.h>
 * #include <linux/mutex.h>
 * #include <sound/pcm.h>
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong};

pub const PCXHR_DRIVER_VERSION: c_uint = 0x000906; /* 0.9.6 */
pub const PCXHR_DRIVER_VERSION_STRING: &[u8; 6] = b"0.9.6\0"; /* 0.9.6 */

pub const PCXHR_MAX_CARDS: usize = 6;
pub const PCXHR_PLAYBACK_STREAMS: usize = 4;

pub const PCXHR_GRANULARITY: c_int = 96; /* min 96 and multiple of 48 */
/* transfer granularity of pipes and the dsp time (MBOX4) */
pub const PCXHR_GRANULARITY_MIN: c_int = 96;
/* TODO : granularity could be 64 or 128 */
pub const PCXHR_GRANULARITY_HR22: c_int = 192; /* granularity for stereo cards */

pub type pcxhr_clock_type = c_uint;
pub const PCXHR_CLOCK_TYPE_INTERNAL: pcxhr_clock_type = 0;
pub const PCXHR_CLOCK_TYPE_WORD_CLOCK: pcxhr_clock_type = 1;
pub const PCXHR_CLOCK_TYPE_AES_SYNC: pcxhr_clock_type = 2;
pub const PCXHR_CLOCK_TYPE_AES_1: pcxhr_clock_type = 3;
pub const PCXHR_CLOCK_TYPE_AES_2: pcxhr_clock_type = 4;
pub const PCXHR_CLOCK_TYPE_AES_3: pcxhr_clock_type = 5;
pub const PCXHR_CLOCK_TYPE_AES_4: pcxhr_clock_type = 6;
pub const PCXHR_CLOCK_TYPE_MAX: pcxhr_clock_type = PCXHR_CLOCK_TYPE_AES_4;
pub const HR22_CLOCK_TYPE_INTERNAL: pcxhr_clock_type = PCXHR_CLOCK_TYPE_INTERNAL;
pub const HR22_CLOCK_TYPE_AES_SYNC: pcxhr_clock_type = 7;
pub const HR22_CLOCK_TYPE_AES_1: pcxhr_clock_type = 8;
pub const HR22_CLOCK_TYPE_MAX: pcxhr_clock_type = HR22_CLOCK_TYPE_AES_1;

#[repr(C)]
pub struct pcxhr_mgr {
    pub num_cards: c_uint,
    pub chip: [*mut snd_pcxhr; PCXHR_MAX_CARDS],

    pub pci: *mut pci_dev,

    pub irq: c_int,

    pub granularity: c_int,

    /* card access with 1 mem bar and 2 io bar's */
    pub port: [c_ulong; 3],

    /* share the name */
    pub name: [c_char; 40], /* name of this soundcard */

    pub prmh: *mut pcxhr_rmh,

    pub lock: mutex,     /* interrupt lock */
    pub msg_lock: mutex, /* message lock */

    pub setup_mutex: mutex, /* mutex used in hw_params, open and close */
    pub mixer_mutex: mutex, /* mutex for mixer */

    /* hardware interface */
    pub dsp_loaded: c_uint,  /* bit flags of loaded dsp indices */
    pub dsp_version: c_uint, /* read from embedded once firmware is loaded */
    pub playback_chips: c_int,
    pub capture_chips: c_int,
    pub fw_file_set: c_int,
    pub firmware_num: c_int,
    /*
     * C bitfields, all unsigned int :1:
     * is_hr_stereo, board_has_aes1, board_has_analog, board_has_mic,
     * board_aes_in_192k, mono_capture, capture_ltc.
     */
    pub bitfields: c_uint,

    pub hostport: snd_dma_buffer,

    pub use_clock_type: pcxhr_clock_type, /* clock type selected by mixer */
    pub cur_clock_type: pcxhr_clock_type, /* current clock type synced */
    pub sample_rate: c_int,
    pub ref_count_rate: c_int,
    pub timer_toggle: c_int, /* timer interrupt toggles between the two values 0x200 and 0x300 */
    pub dsp_time_last: c_int, /* the last dsp time (read by interrupt) */
    pub dsp_time_err: c_int, /* dsp time errors */
    pub src_it_dsp: c_uint, /* dsp interrupt source */
    pub io_num_reg_cont: c_uint, /* backup of IO_NUM_REG_CONT */
    pub codec_speed: c_uint, /* speed mode of the codecs */
    pub sample_rate_real: c_uint, /* current real sample rate */
    pub last_reg_stat: c_int,
    pub async_err_stream_xrun: c_int,
    pub async_err_pipe_xrun: c_int,
    pub async_err_other_last: c_int,

    pub xlx_cfg: u8,    /* copy of PCXHR_XLX_CFG register */
    pub xlx_selmic: u8, /* copy of PCXHR_XLX_SELMIC register */
    pub dsp_reset: u8,  /* copy of PCXHR_DSP_RESET register */
}

pub const PCXHR_MGR_IS_HR_STEREO_SHIFT: c_uint = 0;
pub const PCXHR_MGR_BOARD_HAS_AES1_SHIFT: c_uint = 1;
pub const PCXHR_MGR_BOARD_HAS_ANALOG_SHIFT: c_uint = 2;
pub const PCXHR_MGR_BOARD_HAS_MIC_SHIFT: c_uint = 3;
pub const PCXHR_MGR_BOARD_AES_IN_192K_SHIFT: c_uint = 4;
pub const PCXHR_MGR_MONO_CAPTURE_SHIFT: c_uint = 5;
pub const PCXHR_MGR_CAPTURE_LTC_SHIFT: c_uint = 6;

pub type pcxhr_stream_status = c_uint;
pub const PCXHR_STREAM_STATUS_FREE: pcxhr_stream_status = 0;
pub const PCXHR_STREAM_STATUS_OPEN: pcxhr_stream_status = 1;
pub const PCXHR_STREAM_STATUS_SCHEDULE_RUN: pcxhr_stream_status = 2;
pub const PCXHR_STREAM_STATUS_STARTED: pcxhr_stream_status = 3;
pub const PCXHR_STREAM_STATUS_RUNNING: pcxhr_stream_status = 4;
pub const PCXHR_STREAM_STATUS_SCHEDULE_STOP: pcxhr_stream_status = 5;
pub const PCXHR_STREAM_STATUS_STOPPED: pcxhr_stream_status = 6;
pub const PCXHR_STREAM_STATUS_PAUSED: pcxhr_stream_status = 7;

#[repr(C)]
pub struct pcxhr_stream {
    pub substream: *mut snd_pcm_substream,
    pub format: snd_pcm_format_t,
    pub pipe: *mut pcxhr_pipe,

    pub status: pcxhr_stream_status, /* free, open, running, draining, pause */

    pub timer_abs_periods: u64, /* timer: samples elapsed since TRIGGER_START (multiple of period_size) */
    pub timer_period_frag: u32, /* timer: samples elapsed since last call to snd_pcm_period_elapsed (0..period_size) */
    pub timer_buf_periods: u32, /* nb of periods in the buffer that have already elapsed */
    pub timer_is_synced: c_int, /* if(0) : timer needs to be resynced with real hardware pointer */

    pub channels: c_int,
}

pub type pcxhr_pipe_status = c_uint;
pub const PCXHR_PIPE_UNDEFINED: pcxhr_pipe_status = 0;
pub const PCXHR_PIPE_DEFINED: pcxhr_pipe_status = 1;

#[repr(C)]
pub struct pcxhr_pipe {
    pub status: pcxhr_pipe_status,
    pub is_capture: c_int, /* this is a capture pipe */
    pub first_audio: c_int, /* first audio num */
}

#[repr(C)]
pub struct snd_pcxhr {
    pub card: *mut snd_card,
    pub mgr: *mut pcxhr_mgr,
    pub chip_idx: c_int, /* zero based */

    pub pcm: *mut snd_pcm, /* PCM */

    pub playback_pipe: pcxhr_pipe,      /* 1 stereo pipe only */
    pub capture_pipe: [pcxhr_pipe; 2],  /* 1 stereo or 2 mono pipes */

    pub playback_stream: [pcxhr_stream; PCXHR_PLAYBACK_STREAMS],
    pub capture_stream: [pcxhr_stream; 2], /* 1 stereo or 2 mono streams */
    pub nb_streams_play: c_int,
    pub nb_streams_capt: c_int,

    pub analog_playback_active: [c_int; 2], /* Mixer : Master Playback !mute */
    pub analog_playback_volume: [c_int; 2], /* Mixer : Master Playback Volume */
    pub analog_capture_volume: [c_int; 2],  /* Mixer : Master Capture Volume */
    pub digital_playback_active: [[c_int; 2]; PCXHR_PLAYBACK_STREAMS],
    pub digital_playback_volume: [[c_int; 2]; PCXHR_PLAYBACK_STREAMS],
    pub digital_capture_volume: [c_int; 2], /* Mixer : Digital Capture Volume */
    pub monitoring_active: [c_int; 2],      /* Mixer : Monitoring Active */
    pub monitoring_volume: [c_int; 2],      /* Mixer : Monitoring Volume */
    pub audio_capture_source: c_int,        /* Mixer : Audio Capture Source */
    pub mic_volume: c_int,                  /* used by cards with MIC only */
    pub mic_boost: c_int,                   /* used by cards with MIC only */
    pub mic_active: c_int,                  /* used by cards with MIC only */
    pub analog_capture_active: c_int,       /* used by cards with MIC only */
    pub phantom_power: c_int,               /* used by cards with MIC only */

    pub aes_bits: [u8; 5], /* Mixer : IEC958_AES bits */
}

#[repr(C)]
pub struct pcxhr_hostport {
    pub purgebuffer: [c_char; 6],
    pub reserved: [c_char; 2],
}

/* exported */
unsafe extern "C" {
    pub fn pcxhr_pll_freq_register(
        freq: c_uint,
        max_freq: c_uint,
        pllreg: *mut c_uint,
        realfreq: *mut c_uint,
    ) -> c_int;
    pub fn pcxhr_create_pcm(chip: *mut snd_pcxhr) -> c_int;
    pub fn pcxhr_set_clock(mgr: *mut pcxhr_mgr, rate: c_uint) -> c_int;
    pub fn pcxhr_get_external_clock(
        mgr: *mut pcxhr_mgr,
        clock_type: pcxhr_clock_type,
        sample_rate: *mut c_int,
    ) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
