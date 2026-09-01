// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   ALSA driver for ICEnsemble VT1724 (Envy24HT)
 *
 *   Lowlevel functions for Advanced Micro Peripherals Ltd AUDIO2000
 *
 *      Copyright (c) 2000 Jaroslav Kysela <perex@perex.cz>
 */

use core::ffi::{c_char, c_int, c_uint, c_ushort, c_void};

/*
 * C dependencies:
 *   <linux/delay.h>, <linux/interrupt.h>, <linux/init.h>, <sound/core.h>
 *   "ice1712.h", "envy24ht.h", "amp.h"
 *
 * The constants below are supplied by those headers in the original source.
 */
extern "C" {
    static WM_DEV: c_int;
    static WM_ATTEN_L: c_ushort;
    static WM_ATTEN_R: c_ushort;
    static WM_DAC_CTRL: c_ushort;
    static WM_INT_CTRL: c_ushort;
    static VT1724_SUBDEVICE_AV710: c_uint;
    static VT1724_SUBDEVICE_AUDIO2000: c_uint;

    fn snd_vt1724_write_i2c(
        ice: *mut snd_ice1712,
        dev: c_int,
        addr: c_ushort,
        data: c_ushort,
    );
    fn snd_ac97_write_cache(ac97: *mut c_void, reg: c_ushort, value: c_ushort);
    fn snd_ac97_read(ac97: *mut c_void, reg: c_ushort) -> c_ushort;
}

#[repr(C)]
pub struct snd_ice1712_eeprom {
    pub subvendor: c_uint,
}

#[repr(C)]
pub struct snd_ice1712 {
    pub num_total_dacs: c_uint,
    pub num_total_adcs: c_uint,
    pub eeprom: snd_ice1712_eeprom,
    pub ac97: *mut c_void,
}

#[repr(C)]
pub struct snd_ice1712_card_info {
    pub subvendor: c_uint,
    pub name: *const c_char,
    pub model: *const c_char,
    pub chip_init: Option<unsafe extern "C" fn(*mut snd_ice1712) -> c_int>,
    pub build_controls: Option<unsafe extern "C" fn(*mut snd_ice1712) -> c_int>,
}

unsafe extern "C" fn wm_put(ice: *mut snd_ice1712, reg: c_int, val: c_ushort) {
    let cval: c_ushort = ((reg << 9) as c_ushort) | val;
    snd_vt1724_write_i2c(
        ice,
        WM_DEV,
        (cval >> 8) as c_ushort,
        (cval & 0xff) as c_ushort,
    );
}

unsafe extern "C" fn snd_vt1724_amp_init(ice: *mut snd_ice1712) -> c_int {
    static WM_INITS: [c_ushort; 8] = [
        WM_ATTEN_L,
        0x0000, /* 0 db */
        WM_ATTEN_R,
        0x0000, /* 0 db */
        WM_DAC_CTRL,
        0x0008, /* 24bit I2S */
        WM_INT_CTRL,
        0x0001, /* 24bit I2S */
    ];

    let mut i: c_uint;

    /* only use basic functionality for now */

    /* VT1616 6ch codec connected to PSDOUT0 using packed mode */
    (*ice).num_total_dacs = 6;
    (*ice).num_total_adcs = 2;

    /*
     * Chaintech AV-710 has another WM8728 codec connected to PSDOUT4
     * (shared with the SPDIF output). Mixer control for this codec
     * is not yet supported.
     */
    if (*ice).eeprom.subvendor == VT1724_SUBDEVICE_AV710 {
        i = 0;
        while (i as usize) < WM_INITS.len() {
            wm_put(ice, WM_INITS[i as usize] as c_int, WM_INITS[(i + 1) as usize]);
            i += 2;
        }
    }

    0
}

unsafe extern "C" fn snd_vt1724_amp_add_controls(ice: *mut snd_ice1712) -> c_int {
    if !(*ice).ac97.is_null() {
        /*
         * we use pins 39 and 41 of the VT1616 for left and right
         * read outputs
         */
        snd_ac97_write_cache(
            (*ice).ac97,
            0x5a,
            snd_ac97_read((*ice).ac97, 0x5a) & !0x8000,
        );
    }
    0
}

/* entry point */
#[no_mangle]
pub static mut snd_vt1724_amp_cards: [snd_ice1712_card_info; 3] = [
    snd_ice1712_card_info {
        subvendor: VT1724_SUBDEVICE_AV710,
        name: b"Chaintech AV-710\0".as_ptr() as *const c_char,
        model: b"av710\0".as_ptr() as *const c_char,
        chip_init: Some(snd_vt1724_amp_init),
        build_controls: Some(snd_vt1724_amp_add_controls),
    },
    snd_ice1712_card_info {
        subvendor: VT1724_SUBDEVICE_AUDIO2000,
        name: b"AMP Ltd AUDIO2000\0".as_ptr() as *const c_char,
        model: b"amp2000\0".as_ptr() as *const c_char,
        chip_init: Some(snd_vt1724_amp_init),
        build_controls: Some(snd_vt1724_amp_add_controls),
    },
    snd_ice1712_card_info {
        subvendor: 0,
        name: core::ptr::null(),
        model: core::ptr::null(),
        chip_init: None,
        build_controls: None,
    }, /* terminator */
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
