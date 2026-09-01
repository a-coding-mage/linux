// SPDX-License-Identifier: GPL-2.0
/* Helper functions for Thinkpad LED control;
 * to be included from codec driver
 */

/* C dependency intent:
 * #if IS_ENABLED(CONFIG_THINKPAD_ACPI)
 * #include <linux/acpi.h>
 * #include <linux/leds.h>
 */

#[cfg(CONFIG_THINKPAD_ACPI)]
unsafe fn is_thinkpad(codec: *mut hda_codec) -> bool {
    unsafe {
        ((*codec).core.subsystem_id >> 16 == 0x17aa)
            && (acpi_dev_found(c"LEN0068".as_ptr())
                || acpi_dev_found(c"LEN0268".as_ptr())
                || acpi_dev_found(c"IBM0068".as_ptr()))
    }
}

#[cfg(CONFIG_THINKPAD_ACPI)]
unsafe fn hda_fixup_thinkpad_acpi(
    codec: *mut hda_codec,
    fix: *const hda_fixup,
    action: core::ffi::c_int,
) {
    if action == HDA_FIXUP_ACT_PRE_PROBE {
        unsafe {
            if !is_thinkpad(codec) {
                return;
            }
            snd_hda_gen_add_mute_led_cdev(codec, core::ptr::null_mut());
            snd_hda_gen_add_micmute_led_cdev(codec, core::ptr::null_mut());
        }
    }
}

/* #else CONFIG_THINKPAD_ACPI */

#[cfg(not(CONFIG_THINKPAD_ACPI))]
unsafe fn hda_fixup_thinkpad_acpi(
    codec: *mut hda_codec,
    fix: *const hda_fixup,
    action: core::ffi::c_int,
) {
}

/* #endif CONFIG_THINKPAD_ACPI */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
