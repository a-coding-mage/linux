// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2013 Uwe Kleine-Koenig for Pengutronix
 */

// External kernel declarations and constants supplied by the corresponding
// Linux headers: reboot_mode, dsb, __raw_writel, V7M_SCB_AIRCR_VECTKEY,
// V7M_SCB_AIRCR_SYSRESETREQ, BASEADDR_V7M_SCB, and V7M_SCB_AIRCR.

pub unsafe fn armv7m_restart(mode: reboot_mode, cmd: *const core::ffi::c_char) {
    let _ = mode;
    let _ = cmd;

    dsb();
    __raw_writel(
        V7M_SCB_AIRCR_VECTKEY | V7M_SCB_AIRCR_SYSRESETREQ,
        BASEADDR_V7M_SCB + V7M_SCB_AIRCR,
    );
    dsb();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
