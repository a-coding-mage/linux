/* SPDX-License-Identifier: GPL-2.0 */

/*
 * On HIL keyboards of PARISC machines there is no NumLock key and
 * everyone expects the keypad to be used for numbers. That's why
 * we can safely turn on the NUMLOCK bit.
 */

#[inline]
fn kbd_defleds() -> i32 {
    // Equivalent to: #if defined(CONFIG_KEYBOARD_HIL) || defined(CONFIG_KEYBOARD_HIL_OLD)
    #[cfg(any(feature = "CONFIG_KEYBOARD_HIL", feature = "CONFIG_KEYBOARD_HIL_OLD"))]
    {
        1i32 << VC_NUMLOCK
    }
    #[cfg(not(any(feature = "CONFIG_KEYBOARD_HIL", feature = "CONFIG_KEYBOARD_HIL_OLD")))]
    {
        0
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
