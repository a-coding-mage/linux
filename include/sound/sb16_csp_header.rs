/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  Copyright (c) 1999 by Uros Bizjak <uros@kss-loka.si>
 *                        Takashi Iwai <tiwai@suse.de>
 *
 *  SB16ASP/AWE32 CSP control
 */

// Dependencies supplied by the corresponding sound and kernel interfaces.

/// Indices for the known CSP programs.
pub const CSP_PROGRAM_MULAW: i32 = 0;
pub const CSP_PROGRAM_ALAW: i32 = 1;
pub const CSP_PROGRAM_ADPCM_INIT: i32 = 2;
pub const CSP_PROGRAM_ADPCM_PLAYBACK: i32 = 3;
pub const CSP_PROGRAM_ADPCM_CAPTURE: i32 = 4;
pub const CSP_PROGRAM_COUNT: usize = 5;

/*
 * CSP operators
 */
#[repr(C)]
pub struct snd_sb_csp_ops {
    pub csp_use: Option<unsafe extern "C" fn(p: *mut snd_sb_csp) -> ::core::ffi::c_int>,
    pub csp_unuse: Option<unsafe extern "C" fn(p: *mut snd_sb_csp) -> ::core::ffi::c_int>,
    pub csp_autoload: Option<
        unsafe extern "C" fn(
            p: *mut snd_sb_csp,
            pcm_sfmt: snd_pcm_format_t,
            play_rec_mode: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub csp_start: Option<
        unsafe extern "C" fn(
            p: *mut snd_sb_csp,
            sample_width: ::core::ffi::c_int,
            channels: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub csp_stop: Option<unsafe extern "C" fn(p: *mut snd_sb_csp) -> ::core::ffi::c_int>,
    pub csp_qsound_transfer:
        Option<unsafe extern "C" fn(p: *mut snd_sb_csp) -> ::core::ffi::c_int>,
}

/*
 * CSP private data
 */
#[repr(C)]
pub struct snd_sb_csp {
    pub chip: *mut snd_sb, /* SB16 DSP */
    pub used: ::core::ffi::c_int, /* usage flag - exclusive */
    pub codec_name: [::core::ffi::c_char; 16], /* name of codec */
    pub func_nr: u16, /* function number */
    pub acc_format: ::core::ffi::c_uint, /* accepted PCM formats */
    pub acc_channels: ::core::ffi::c_int, /* accepted channels */
    pub acc_width: ::core::ffi::c_int, /* accepted sample width */
    pub acc_rates: ::core::ffi::c_int, /* accepted sample rates */
    pub mode: ::core::ffi::c_int, /* MODE */
    pub run_channels: ::core::ffi::c_int, /* current CSP channels */
    pub run_width: ::core::ffi::c_int, /* current sample width */
    pub version: ::core::ffi::c_int, /* CSP version (0x10 - 0x1f) */
    pub running: ::core::ffi::c_int, /* running state */

    pub ops: snd_sb_csp_ops, /* operators */

    pub q_lock: spinlock_t, /* locking */
    pub q_enabled: ::core::ffi::c_int, /* enabled flag */
    pub qpos_left: ::core::ffi::c_int, /* left position */
    pub qpos_right: ::core::ffi::c_int, /* right position */
    pub qpos_changed: ::core::ffi::c_int, /* position changed flag */

    pub qsound_switch: *mut snd_kcontrol,
    pub qsound_space: *mut snd_kcontrol,

    pub access_mutex: mutex, /* locking */

    pub csp_programs: [*const firmware; CSP_PROGRAM_COUNT],
}

pub unsafe extern "C" fn snd_sb_csp_new(
    chip: *mut snd_sb,
    device: ::core::ffi::c_int,
    rhwdep: *mut *mut snd_hwdep,
) -> ::core::ffi::c_int;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
