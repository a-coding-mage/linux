/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Some laptops take the 789uiojklm,. keys as number pad when NumLock is on.
 * This seems a good reason to start with NumLock off. That's why on X86 we
 * ask the bios for the correct state.
 */

// Dependency provided by asm/setup.h in the original header.

#[inline]
fn kbd_defleds() -> i32 {
    if (boot_params.kbd_status & 0x20) != 0 {
        1 << VC_NUMLOCK
    } else {
        0
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
