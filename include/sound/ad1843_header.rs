/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright 2003 Vivien Chappelier <vivien.chappelier@linux-mips.org>
 * Copyright 2008 Thomas Bogendoerfer <tsbogend@franken.de>
 */

#[repr(C)]
pub struct snd_ad1843 {
    pub chip: *mut core::ffi::c_void,
    pub read: Option<unsafe extern "C" fn(chip: *mut core::ffi::c_void, reg: core::ffi::c_int) -> core::ffi::c_int>,
    pub write: Option<unsafe extern "C" fn(
        chip: *mut core::ffi::c_void,
        reg: core::ffi::c_int,
        val: core::ffi::c_int,
    ) -> core::ffi::c_int>,
}

pub const AD1843_GAIN_RECLEV: core::ffi::c_int = 0;
pub const AD1843_GAIN_LINE: core::ffi::c_int = 1;
pub const AD1843_GAIN_LINE_2: core::ffi::c_int = 2;
pub const AD1843_GAIN_MIC: core::ffi::c_int = 3;
pub const AD1843_GAIN_PCM_0: core::ffi::c_int = 4;
pub const AD1843_GAIN_PCM_1: core::ffi::c_int = 5;
pub const AD1843_GAIN_SIZE: core::ffi::c_int = AD1843_GAIN_PCM_1 + 1;

extern "C" {
    pub fn ad1843_get_gain_max(ad1843: *mut snd_ad1843, id: core::ffi::c_int) -> core::ffi::c_int;
    pub fn ad1843_get_gain(ad1843: *mut snd_ad1843, id: core::ffi::c_int) -> core::ffi::c_int;
    pub fn ad1843_set_gain(
        ad1843: *mut snd_ad1843,
        id: core::ffi::c_int,
        newval: core::ffi::c_int,
    ) -> core::ffi::c_int;
    pub fn ad1843_get_recsrc(ad1843: *mut snd_ad1843) -> core::ffi::c_int;
    pub fn ad1843_set_recsrc(ad1843: *mut snd_ad1843, newsrc: core::ffi::c_int) -> core::ffi::c_int;
    pub fn ad1843_setup_dac(
        ad1843: *mut snd_ad1843,
        id: core::ffi::c_uint,
        framerate: core::ffi::c_uint,
        fmt: snd_pcm_format_t,
        channels: core::ffi::c_uint,
    );
    pub fn ad1843_shutdown_dac(ad1843: *mut snd_ad1843, id: core::ffi::c_uint);
    pub fn ad1843_setup_adc(
        ad1843: *mut snd_ad1843,
        framerate: core::ffi::c_uint,
        fmt: snd_pcm_format_t,
        channels: core::ffi::c_uint,
    );
    pub fn ad1843_shutdown_adc(ad1843: *mut snd_ad1843);
    pub fn ad1843_init(ad1843: *mut snd_ad1843) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
