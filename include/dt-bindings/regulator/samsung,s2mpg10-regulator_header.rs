/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright 2021 Google LLC
 * Copyright 2025 Linaro Ltd.
 *
 * Device Tree binding constants for the Samsung S2MPG1x PMIC regulators
 */

/*
 * Several regulators may be controlled via external signals instead of via
 * software. These constants describe the possible signals for such regulators
 * and generally correspond to the respecitve on-chip pins.
 *
 * S2MPG10 regulators supporting these are:
 * - buck1m .. buck7m buck10m
 * - ldo3m .. ldo19m
 *
 * ldo20m supports external control, but using a different set of control
 * signals.
 *
 * S2MPG11 regulators supporting these are:
 * - buck1s .. buck3s buck5s buck8s buck9s bucka buckd
 * - ldo1s ldo2s ldo8s ldo13s
 */
pub const S2MPG10_EXTCTRL_PWREN: i32 = 0; /* PWREN pin */
pub const S2MPG10_EXTCTRL_PWREN_MIF: i32 = 1; /* PWREN_MIF pin */
pub const S2MPG10_EXTCTRL_AP_ACTIVE_N: i32 = 2; /* ~AP_ACTIVE_N pin */
pub const S2MPG10_EXTCTRL_CPUCL1_EN: i32 = 3; /* CPUCL1_EN pin */
pub const S2MPG10_EXTCTRL_CPUCL1_EN2: i32 = 4; /* CPUCL1_EN & PWREN pins */
pub const S2MPG10_EXTCTRL_CPUCL2_EN: i32 = 5; /* CPUCL2_EN pin */
pub const S2MPG10_EXTCTRL_CPUCL2_EN2: i32 = 6; /* CPUCL2_E2 & PWREN pins */
pub const S2MPG10_EXTCTRL_TPU_EN: i32 = 7; /* TPU_EN pin */
pub const S2MPG10_EXTCTRL_TPU_EN2: i32 = 8; /* TPU_EN & ~AP_ACTIVE_N pins */
pub const S2MPG10_EXTCTRL_TCXO_ON: i32 = 9; /* TCXO_ON pin */
pub const S2MPG10_EXTCTRL_TCXO_ON2: i32 = 10; /* TCXO_ON & ~AP_ACTIVE_N pins */

pub const S2MPG10_EXTCTRL_LDO20M_EN2: i32 = 11; /* VLDO20M_EN & LDO20M_SFR */
pub const S2MPG10_EXTCTRL_LDO20M_EN: i32 = 12; /* VLDO20M_EN pin */

pub const S2MPG11_EXTCTRL_PWREN: i32 = 0; /* PWREN pin */
pub const S2MPG11_EXTCTRL_PWREN_MIF: i32 = 1; /* PWREN_MIF pin */
pub const S2MPG11_EXTCTRL_AP_ACTIVE_N: i32 = 2; /* ~AP_ACTIVE_N pin */
pub const S2MPG11_EXTCTRL_G3D_EN: i32 = 3; /* G3D_EN pin */
pub const S2MPG11_EXTCTRL_G3D_EN2: i32 = 4; /* G3D_EN & ~AP_ACTIVE_N pins */
pub const S2MPG11_EXTCTRL_AOC_VDD: i32 = 5; /* AOC_VDD pin */
pub const S2MPG11_EXTCTRL_AOC_RET: i32 = 6; /* AOC_RET pin */
pub const S2MPG11_EXTCTRL_UFS_EN: i32 = 7; /* UFS_EN pin */
pub const S2MPG11_EXTCTRL_LDO13S_EN: i32 = 8; /* VLDO13S_EN pin */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
