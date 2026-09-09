/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * This header provides constants for AT91 pmc status.
 *
 * The constants defined in this header are being used in dts.
 */

pub const PMC_TYPE_CORE: i32 = 0;
pub const PMC_TYPE_SYSTEM: i32 = 1;
pub const PMC_TYPE_PERIPHERAL: i32 = 2;
pub const PMC_TYPE_GCK: i32 = 3;
pub const PMC_TYPE_PROGRAMMABLE: i32 = 4;

pub const PMC_SLOW: i32 = 0;
pub const PMC_MCK: i32 = 1;
pub const PMC_UTMI: i32 = 2;
pub const PMC_MAIN: i32 = 3;
pub const PMC_MCK2: i32 = 4;
pub const PMC_I2S0_MUX: i32 = 5;
pub const PMC_I2S1_MUX: i32 = 6;
pub const PMC_PLLACK: i32 = 7;
pub const PMC_PLLBCK: i32 = 8;
pub const PMC_AUDIOPLLCK: i32 = 9;
pub const PMC_AUDIOPINCK: i32 = 10;

/* SAMA7G5 */
pub const PMC_CPUPLL: i32 = PMC_MAIN + 1;
pub const PMC_SYSPLL: i32 = PMC_MAIN + 2;
pub const PMC_DDRPLL: i32 = PMC_MAIN + 3;
pub const PMC_IMGPLL: i32 = PMC_MAIN + 4;
pub const PMC_BAUDPLL: i32 = PMC_MAIN + 5;
pub const PMC_AUDIOPMCPLL: i32 = PMC_MAIN + 6;
pub const PMC_AUDIOIOPLL: i32 = PMC_MAIN + 7;
pub const PMC_ETHPLL: i32 = PMC_MAIN + 8;
pub const PMC_CPU: i32 = PMC_MAIN + 9;
pub const PMC_MCK1: i32 = PMC_MAIN + 10;

/* SAM9X7 */
pub const PMC_PLLADIV2: i32 = PMC_MAIN + 11;
pub const PMC_LVDSPLL: i32 = PMC_MAIN + 12;

/* SAMA7D65 */
pub const PMC_MCK3: i32 = PMC_MAIN + 13;
pub const PMC_MCK5: i32 = PMC_MAIN + 14;

/* Original C definitions were guarded by #ifndef AT91_PMC_MOSCS. */
pub const AT91_PMC_MOSCS: i32 = 0; /* MOSCS Flag */
pub const AT91_PMC_LOCKA: i32 = 1; /* PLLA Lock */
pub const AT91_PMC_LOCKB: i32 = 2; /* PLLB Lock */
pub const AT91_PMC_MCKRDY: i32 = 3; /* Master Clock */
pub const AT91_PMC_LOCKU: i32 = 6; /* UPLL Lock */
pub const fn AT91_PMC_PCKRDY(id: i32) -> i32 { 8 + id } /* Programmable Clock */
pub const AT91_PMC_MOSCSELS: i32 = 16; /* Main Oscillator Selection */
pub const AT91_PMC_MOSCRCS: i32 = 17; /* Main On-Chip RC */
pub const AT91_PMC_CFDEV: i32 = 18; /* Clock Failure Detector Event */
pub const AT91_PMC_GCKRDY: i32 = 24; /* Generated Clocks */

/* Slow clock. */
pub const SCKC_MD_SLCK: i32 = 0;
pub const SCKC_TD_SLCK: i32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
