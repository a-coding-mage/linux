// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   ALSA driver for VT1720/VT1724 (Envy24PT/Envy24HT)
 *
 *   Lowlevel functions for VT1720-based motherboards
 *
 *	Copyright (c) 2004 Takashi Iwai <tiwai@suse.de>
 */

// C dependencies:
// #include <linux/delay.h>
// #include <linux/interrupt.h>
// #include <linux/init.h>
// #include <sound/core.h>
// #include "ice1712.h"
// #include "envy24ht.h"
// #include "vt1720_mobo.h"

use core::ffi::{c_char, c_int, c_uchar};

unsafe extern "C" fn k8x800_init(ice: *mut snd_ice1712) -> c_int {
    unsafe {
        (*ice).vt1720 = 1;

        /* VT1616 codec */
        (*ice).num_total_dacs = 6;
        (*ice).num_total_adcs = 2;
    }

    /* WM8728 codec */
    /* FIXME: TODO */

    0
}

unsafe extern "C" fn k8x800_add_controls(_ice: *mut snd_ice1712) -> c_int {
    /* FIXME: needs some quirks for VT1616? */
    0
}

/* EEPROM image */

const fn k8x800_eeprom_image() -> [c_uchar; ICE_EEP2_GPIO_STATE2 + 1] {
    let mut image = [0; ICE_EEP2_GPIO_STATE2 + 1];

    image[ICE_EEP2_SYSCONF] = 0x01; /* clock 256, 1ADC, 2DACs */
    image[ICE_EEP2_ACLINK] = 0x02; /* ACLINK, packed */
    image[ICE_EEP2_I2S] = 0x00; /* - */
    image[ICE_EEP2_SPDIF] = 0x00; /* - */
    image[ICE_EEP2_GPIO_DIR] = 0xff;
    image[ICE_EEP2_GPIO_DIR1] = 0xff;
    image[ICE_EEP2_GPIO_DIR2] = 0x00; /* - */
    image[ICE_EEP2_GPIO_MASK] = 0xff;
    image[ICE_EEP2_GPIO_MASK1] = 0xff;
    image[ICE_EEP2_GPIO_MASK2] = 0x00; /* - */
    image[ICE_EEP2_GPIO_STATE] = 0x00;
    image[ICE_EEP2_GPIO_STATE1] = 0x00;
    image[ICE_EEP2_GPIO_STATE2] = 0x00; /* - */

    image
}

const fn sn25p_eeprom_image() -> [c_uchar; ICE_EEP2_GPIO_STATE2 + 1] {
    let mut image = [0; ICE_EEP2_GPIO_STATE2 + 1];

    image[ICE_EEP2_SYSCONF] = 0x01; /* clock 256, 1ADC, 2DACs */
    image[ICE_EEP2_ACLINK] = 0x02; /* ACLINK, packed */
    image[ICE_EEP2_I2S] = 0x00; /* - */
    image[ICE_EEP2_SPDIF] = 0x41; /* - */
    image[ICE_EEP2_GPIO_DIR] = 0xff;
    image[ICE_EEP2_GPIO_DIR1] = 0xff;
    image[ICE_EEP2_GPIO_DIR2] = 0x00; /* - */
    image[ICE_EEP2_GPIO_MASK] = 0xff;
    image[ICE_EEP2_GPIO_MASK1] = 0xff;
    image[ICE_EEP2_GPIO_MASK2] = 0x00; /* - */
    image[ICE_EEP2_GPIO_STATE] = 0x00;
    image[ICE_EEP2_GPIO_STATE1] = 0x00;
    image[ICE_EEP2_GPIO_STATE2] = 0x00; /* - */

    image
}

static K8X800_EEPROM: [c_uchar; ICE_EEP2_GPIO_STATE2 + 1] = k8x800_eeprom_image();
static SN25P_EEPROM: [c_uchar; ICE_EEP2_GPIO_STATE2 + 1] = sn25p_eeprom_image();

/* entry point */
#[no_mangle]
pub static mut snd_vt1720_mobo_cards: [snd_ice1712_card_info; 6] = [
    snd_ice1712_card_info {
        subvendor: VT1720_SUBDEVICE_K8X800,
        name: b"Albatron K8X800 Pro II\0".as_ptr() as *const c_char,
        model: b"k8x800\0".as_ptr() as *const c_char,
        chip_init: Some(k8x800_init),
        build_controls: Some(k8x800_add_controls),
        eeprom_size: core::mem::size_of::<[c_uchar; ICE_EEP2_GPIO_STATE2 + 1]>(),
        eeprom_data: K8X800_EEPROM.as_ptr(),
    },
    snd_ice1712_card_info {
        subvendor: VT1720_SUBDEVICE_ZNF3_150,
        name: b"Chaintech ZNF3-150\0".as_ptr() as *const c_char,
        /* identical with k8x800 */
        model: core::ptr::null(),
        chip_init: Some(k8x800_init),
        build_controls: Some(k8x800_add_controls),
        eeprom_size: core::mem::size_of::<[c_uchar; ICE_EEP2_GPIO_STATE2 + 1]>(),
        eeprom_data: K8X800_EEPROM.as_ptr(),
    },
    snd_ice1712_card_info {
        subvendor: VT1720_SUBDEVICE_ZNF3_250,
        name: b"Chaintech ZNF3-250\0".as_ptr() as *const c_char,
        /* identical with k8x800 */
        model: core::ptr::null(),
        chip_init: Some(k8x800_init),
        build_controls: Some(k8x800_add_controls),
        eeprom_size: core::mem::size_of::<[c_uchar; ICE_EEP2_GPIO_STATE2 + 1]>(),
        eeprom_data: K8X800_EEPROM.as_ptr(),
    },
    snd_ice1712_card_info {
        subvendor: VT1720_SUBDEVICE_9CJS,
        name: b"Chaintech 9CJS\0".as_ptr() as *const c_char,
        /* identical with k8x800 */
        model: core::ptr::null(),
        chip_init: Some(k8x800_init),
        build_controls: Some(k8x800_add_controls),
        eeprom_size: core::mem::size_of::<[c_uchar; ICE_EEP2_GPIO_STATE2 + 1]>(),
        eeprom_data: K8X800_EEPROM.as_ptr(),
    },
    snd_ice1712_card_info {
        subvendor: VT1720_SUBDEVICE_SN25P,
        name: b"Shuttle SN25P\0".as_ptr() as *const c_char,
        model: b"sn25p\0".as_ptr() as *const c_char,
        chip_init: Some(k8x800_init),
        build_controls: Some(k8x800_add_controls),
        eeprom_size: core::mem::size_of::<[c_uchar; ICE_EEP2_GPIO_STATE2 + 1]>(),
        eeprom_data: SN25P_EEPROM.as_ptr(),
    },
    snd_ice1712_card_info {
        subvendor: 0,
        name: core::ptr::null(),
        model: core::ptr::null(),
        chip_init: None,
        build_controls: None,
        eeprom_size: 0,
        eeprom_data: core::ptr::null(),
    }, /* terminator */
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
