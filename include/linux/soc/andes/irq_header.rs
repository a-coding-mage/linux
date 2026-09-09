/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2023 Andes Technology Corporation
 */

// Andes PMU irq number
pub const ANDES_RV_IRQ_PMOVI: u32 = 18;
pub const ANDES_RV_IRQ_LAST: u32 = ANDES_RV_IRQ_PMOVI;
pub const ANDES_SLI_CAUSE_BASE: u32 = 256;

// Andes PMU related registers
pub const ANDES_CSR_SLIE: u32 = 0x9c4;
pub const ANDES_CSR_SLIP: u32 = 0x9c5;
pub const ANDES_CSR_SCOUNTEROF: u32 = 0x9d4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
