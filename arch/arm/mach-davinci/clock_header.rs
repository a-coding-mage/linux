/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * TI DaVinci clock definitions
 *
 * Copyright (C) 2006-2007 Texas Instruments.
 * Copyright (C) 2008-2009 Deep Root Systems, LLC
 */

/* PLL/Reset register offsets */
pub const PLLCTL: u32 = 0x100;
pub const PLLCTL_PLLEN: u32 = 1u32 << 0;
pub const PLLCTL_PLLPWRDN: u32 = 1u32 << 1;
pub const PLLCTL_PLLRST: u32 = 1u32 << 3;
pub const PLLCTL_PLLDIS: u32 = 1u32 << 4;
pub const PLLCTL_PLLENSRC: u32 = 1u32 << 5;
pub const PLLCTL_CLKMODE: u32 = 1u32 << 8;

pub const PLLM: u32 = 0x110;
pub const PLLM_PLLM_MASK: u32 = 0xff;

pub const PREDIV: u32 = 0x114;
pub const PLLDIV1: u32 = 0x118;
pub const PLLDIV2: u32 = 0x11c;
pub const PLLDIV3: u32 = 0x120;
pub const POSTDIV: u32 = 0x128;
pub const BPDIV: u32 = 0x12c;
pub const PLLCMD: u32 = 0x138;
pub const PLLSTAT: u32 = 0x13c;
pub const PLLALNCTL: u32 = 0x140;
pub const PLLDCHANGE: u32 = 0x144;
pub const PLLCKEN: u32 = 0x148;
pub const PLLCKSTAT: u32 = 0x14c;
pub const PLLSYSTAT: u32 = 0x150;
pub const PLLDIV4: u32 = 0x160;
pub const PLLDIV5: u32 = 0x164;
pub const PLLDIV6: u32 = 0x168;
pub const PLLDIV7: u32 = 0x16c;
pub const PLLDIV8: u32 = 0x170;
pub const PLLDIV9: u32 = 0x174;
pub const PLLDIV_EN: u32 = 1u32 << 15;
pub const PLLDIV_RATIO_MASK: u32 = 0x1f;

/*
 * OMAP-L138 system reference guide recommends a wait for 4 OSCIN/CLKIN
 * cycles to ensure that the PLLC has switched to bypass mode. Delay of 1us
 * ensures we are good for all > 4MHz OSCIN/CLKIN inputs. Typically the input
 * is ~25MHz. Units are micro seconds.
 */
pub const PLL_BYPASS_TIME: u32 = 1;
/* From OMAP-L138 datasheet table 6-4. Units are micro seconds */
pub const PLL_RESET_TIME: u32 = 1;
/*
 * From OMAP-L138 datasheet table 6-4; assuming prediv = 1, sqrt(pllm) = 4
 * Units are micro seconds.
 */
pub const PLL_LOCK_TIME: u32 = 20;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
