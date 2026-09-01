// SPDX-License-Identifier: GPL-2.0
/*
 * Ideapad helper functions for Lenovo Ideapad LED control,
 * It should be included from codec driver.
 */

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

#[repr(C)]
pub struct hda_codec_core {
    pub subsystem_id: u32,
}

#[repr(C)]
pub struct hda_codec {
    pub core: hda_codec_core,
}

#[repr(C)]
pub struct hda_fixup {
    _private: [u8; 0],
}

pub const HDA_FIXUP_ACT_PRE_PROBE: c_int = 0;

/* C source condition: IS_ENABLED(CONFIG_IDEAPAD_LAPTOP) */
#[cfg(CONFIG_IDEAPAD_LAPTOP)]
unsafe extern "C" {
    fn acpi_dev_found(hid: *const c_char) -> bool;
    fn snd_hda_gen_add_mute_led_cdev(codec: *mut hda_codec, cdev: *mut c_void);
    fn snd_hda_gen_add_micmute_led_cdev(codec: *mut hda_codec, cdev: *mut c_void);
}

#[cfg(CONFIG_IDEAPAD_LAPTOP)]
unsafe fn is_ideapad(codec: *mut hda_codec) -> bool {
    ((*codec).core.subsystem_id >> 16 == 0x17aa)
        && (acpi_dev_found(c"LHK2019".as_ptr()) || acpi_dev_found(c"VPC2004".as_ptr()))
}

#[cfg(CONFIG_IDEAPAD_LAPTOP)]
unsafe fn hda_fixup_ideapad_acpi(
    codec: *mut hda_codec,
    fix: *const hda_fixup,
    action: c_int,
) {
    let _ = fix;

    if action == HDA_FIXUP_ACT_PRE_PROBE {
        if !is_ideapad(codec) {
            return;
        }
        snd_hda_gen_add_mute_led_cdev(codec, ptr::null_mut());
        snd_hda_gen_add_micmute_led_cdev(codec, ptr::null_mut());
    }
}

/* C source condition: !IS_ENABLED(CONFIG_IDEAPAD_LAPTOP) */
#[cfg(not(CONFIG_IDEAPAD_LAPTOP))]
unsafe fn hda_fixup_ideapad_acpi(
    codec: *mut hda_codec,
    fix: *const hda_fixup,
    action: c_int,
) {
    let _ = codec;
    let _ = fix;
    let _ = action;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
