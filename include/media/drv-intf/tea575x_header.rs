/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 *   ALSA driver for TEA5757/5759 Philips AM/FM tuner chips
 *
 *	Copyright (c) 2004 Jaroslav Kysela <perex@perex.cz>
 */

/* Declarations from linux/videodev2.h and media/v4l2-*.h are external
 * dependencies supplied by the surrounding translation unit. */

pub const TEA575X_FMIF: i32 = 10700;
pub const TEA575X_AMIF: i32 = 450;

pub const TEA575X_DATA: u32 = 1 << 0;
pub const TEA575X_CLK: u32 = 1 << 1;
pub const TEA575X_WREN: u32 = 1 << 2;
pub const TEA575X_MOST: u32 = 1 << 3;

#[repr(C)]
pub struct snd_tea575x;

#[repr(C)]
pub struct snd_tea575x_ops {
    /* Drivers using snd_tea575x must either define read_ and write_val */
    pub write_val: Option<unsafe extern "C" fn(tea: *mut snd_tea575x, val: u32)>,
    pub read_val: Option<unsafe extern "C" fn(tea: *mut snd_tea575x) -> u32>,
    /* Or define the 3 pin functions */
    pub set_pins: Option<unsafe extern "C" fn(tea: *mut snd_tea575x, pins: u8)>,
    pub get_pins: Option<unsafe extern "C" fn(tea: *mut snd_tea575x) -> u8>,
    pub set_direction: Option<unsafe extern "C" fn(tea: *mut snd_tea575x, output: bool)>,
}

#[repr(C)]
pub struct snd_tea575x {
    pub v4l2_dev: *mut v4l2_device,
    pub fops: v4l2_file_operations,
    pub vd: video_device, /* video device */
    pub radio_nr: i32, /* radio_nr */
    pub tea5759: bool, /* 5759 chip is present */
    pub has_am: bool, /* Device can tune to AM freqs */
    pub cannot_read_data: bool, /* Device cannot read the data pin */
    pub cannot_mute: bool, /* Device cannot mute */
    pub mute: bool, /* Device is muted? */
    pub stereo: bool, /* receiving stereo */
    pub tuned: bool, /* tuned to a station */
    pub val: u32, /* hw value */
    pub band: u32, /* 0: FM, 1: FM-Japan, 2: AM */
    pub freq: u32, /* frequency */
    pub mutex: mutex,
    pub ops: *const snd_tea575x_ops,
    pub private_data: *mut core::ffi::c_void,
    pub card: [u8; 32],
    pub bus_info: [u8; 32],
    pub ctrl_handler: v4l2_ctrl_handler,
    pub ext_init: Option<unsafe extern "C" fn(tea: *mut snd_tea575x) -> i32>,
}

unsafe extern "C" {
    pub fn snd_tea575x_enum_freq_bands(
        tea: *mut snd_tea575x,
        band: *mut v4l2_frequency_band,
    ) -> i32;
    pub fn snd_tea575x_g_tuner(tea: *mut snd_tea575x, v: *mut v4l2_tuner) -> i32;
    pub fn snd_tea575x_s_hw_freq_seek(
        file: *mut file,
        tea: *mut snd_tea575x,
        a: *const v4l2_hw_freq_seek,
    ) -> i32;
    pub fn snd_tea575x_hw_init(tea: *mut snd_tea575x) -> i32;
    pub fn snd_tea575x_init(tea: *mut snd_tea575x, owner: *mut module) -> i32;
    pub fn snd_tea575x_exit(tea: *mut snd_tea575x);
    pub fn snd_tea575x_set_freq(tea: *mut snd_tea575x);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
