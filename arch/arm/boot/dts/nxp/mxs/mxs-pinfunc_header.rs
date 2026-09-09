/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Header providing constants for i.MX28 pinctrl bindings.
 *
 * Copyright (C) 2013 Lothar Waßmann <LW@KARO-electronics.de>
 */

/* Translated from the C header guard __DT_BINDINGS_MXS_PINCTRL_H__. */

/* fsl,drive-strength property */
pub const MXS_DRIVE_4mA: u32 = 0;
pub const MXS_DRIVE_8mA: u32 = 1;
pub const MXS_DRIVE_12mA: u32 = 2;
pub const MXS_DRIVE_16mA: u32 = 3;

/* fsl,voltage property */
pub const MXS_VOLTAGE_LOW: u32 = 0;
pub const MXS_VOLTAGE_HIGH: u32 = 1;

/* fsl,pull-up property */
pub const MXS_PULL_DISABLE: u32 = 0;
pub const MXS_PULL_ENABLE: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
