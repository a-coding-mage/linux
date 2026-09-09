/* SPDX-License-Identifier: (GPL-2.0-only OR MIT) */
/*
 * Copyright 2025 NXP
 */

/* consumer type - master or peripheral */
pub const IMX8MP_AIPSTZ_MASTER: i32 = 0x0;
pub const IMX8MP_AIPSTZ_PERIPH: i32 = 0x1;

/* master configuration options */
pub const IMX8MP_AIPSTZ_MPL: i32 = 1 << 0;
pub const IMX8MP_AIPSTZ_MTW: i32 = 1 << 1;
pub const IMX8MP_AIPSTZ_MTR: i32 = 1 << 2;
pub const IMX8MP_AIPSTZ_MBW: i32 = 1 << 3;

/* peripheral configuration options */
pub const IMX8MP_AIPSTZ_TP: i32 = 1 << 0;
pub const IMX8MP_AIPSTZ_WP: i32 = 1 << 1;
pub const IMX8MP_AIPSTZ_SP: i32 = 1 << 2;
pub const IMX8MP_AIPSTZ_BW: i32 = 1 << 3;

/* master ID definitions */
pub const IMX8MP_AIPSTZ_EDMA: i32 = 0; /* AUDIOMIX EDMA */
pub const IMX8MP_AIPSTZ_CA53: i32 = 1; /* Cortex-A53 cluster */
pub const IMX8MP_AIPSTZ_SDMA2: i32 = 3; /* AUDIOMIX SDMA2 */
pub const IMX8MP_AIPSTZ_SDMA3: i32 = 3; /* AUDIOMIX SDMA3 */
pub const IMX8MP_AIPSTZ_HIFI4: i32 = 5; /* HIFI4 DSP */
pub const IMX8MP_AIPSTZ_CM7: i32 = 6; /* Cortex-M7 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
