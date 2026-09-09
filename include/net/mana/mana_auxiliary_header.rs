/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (c) 2022, Microsoft Corporation. */

// Translated from mana.h and linux/auxiliary_bus.h dependencies.

#[repr(C)]
pub struct mana_adev {
    pub adev: auxiliary_device,
    pub mdev: *mut gdma_dev,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
