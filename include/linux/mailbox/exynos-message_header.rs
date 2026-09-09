/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Exynos mailbox message.
 *
 * Copyright 2024 Linaro Ltd.
 */

pub const EXYNOS_MBOX_CHAN_TYPE_DOORBELL: u32 = 0;
pub const EXYNOS_MBOX_CHAN_TYPE_DATA: u32 = 1;

#[repr(C)]
pub struct exynos_mbox_msg {
    pub chan_id: u32,
    pub chan_type: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
