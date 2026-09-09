/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2014-2016, The Linux Foundation. All rights reserved.
 */

// return true if s1 is a prefix of s2
// C dependency intent: this is the direct equivalent of
// !strncmp(s1, s2, strlen(s1)) for NUL-terminated byte strings.
#[inline]
pub unsafe fn str_prfx_equal(s1: *const u8, s2: *const u8) -> bool {
    unsafe extern "C" {
        fn strlen(s: *const u8) -> usize;
        fn strncmp(s1: *const u8, s2: *const u8, n: usize) -> i32;
    }

    strncmp(s1, s2, strlen(s1)) == 0
}

pub const UFS_ANY_VENDOR: u16 = 0xFFFF;
pub const UFS_ANY_MODEL: &[u8] = b"ANY_MODEL\0";

pub const UFS_VENDOR_MICRON: u16 = 0x12C;
pub const UFS_VENDOR_SAMSUNG: u16 = 0x1CE;
pub const UFS_VENDOR_SKHYNIX: u16 = 0x1AD;
pub const UFS_VENDOR_TOSHIBA: u16 = 0x198;
pub const UFS_VENDOR_WDC: u16 = 0x145;

/**
 * ufs_dev_quirk - ufs device quirk info
 * @card: ufs card details
 * @quirk: device quirk
 */
#[repr(C)]
pub struct ufs_dev_quirk {
    pub wmanufacturerid: u16,
    pub model: *const u8,
    pub quirk: u32,
}

/*
 * Some vendor's UFS device sends back to back NACs for the DL data frames
 * causing the host controller to raise the DFES error status. Sometimes
 * such UFS devices send back to back NAC without waiting for new
 * retransmitted DL frame from the host and in such cases it might be possible
 * the Host UniPro goes into bad state without raising the DFES error
 * interrupt. If this happens then all the pending commands would timeout
 * only after respective SW command (which is generally too large).
 *
 * We can workaround such device behaviour like this:
 * - As soon as SW sees the DL NAC error, it should schedule the error handler
 * - Error handler would sleep for 50ms to see if there are any fatal errors
 *   raised by UFS controller.
 *    - If there are fatal errors then SW does normal error recovery.
 *    - If there are no fatal errors then SW sends the NOP command to device
 *      to check if link is alive.
 *        - If NOP command times out, SW does normal error recovery
 *        - If NOP command succeed, skip the error handling.
 *
 * If DL NAC error is seen multiple times with some vendor's UFS devices then
 * enable this quirk to initiate quick error recovery and also silence related
 * error logs to reduce spamming of kernel logs.
 */
pub const UFS_DEVICE_QUIRK_RECOVERY_FROM_DL_NAC_ERRORS: u32 = 1 << 2;

/* See the source header for the rationale for this quirk. */
pub const UFS_DEVICE_QUIRK_PA_TACTIVATE: u32 = 1 << 4;

/* Add a delay before putting affected UFS rails in LPM mode. */
pub const UFS_DEVICE_QUIRK_DELAY_BEFORE_LPM: u32 = 1 << 6;

/* Require host PA_TACTIVATE to be lower than device PA_TACTIVATE. */
pub const UFS_DEVICE_QUIRK_HOST_PA_TACTIVATE: u32 = 1 << 7;

/* Increase host PA_SaveConfigTime beyond the standard maximum. */
pub const UFS_DEVICE_QUIRK_HOST_PA_SAVECONFIGTIME: u32 = 1 << 8;

/* Require host VS_DebugSaveConfigTime to be 0x10. */
pub const UFS_DEVICE_QUIRK_HOST_VS_DEBUGSAVECONFIGTIME: u32 = 1 << 9;

/* Enable supported extended features on pre-3.1 UFS devices. */
pub const UFS_DEVICE_QUIRK_SUPPORT_EXTENDED_FEATURES: u32 = 1 << 10;

/* Give devices additional time in hibern8 before exiting. */
pub const UFS_DEVICE_QUIRK_PA_HIBER8TIME: u32 = 1 << 12;

/* Some UFS 4 devices do not support the qTimestamp attribute */
pub const UFS_DEVICE_QUIRK_NO_TIMESTAMP_SUPPORT: u32 = 1 << 13;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
