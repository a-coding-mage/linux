/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Microchip SAMA7 UDDR Controller and DDR3 PHY Controller registers offsets
 * and bit definitions.
 *
 * Copyright (C) [2020] Microchip Technology Inc. and its subsidiaries
 *
 * Author: Claudu Beznea <claudiu.beznea@microchip.com>
 */

/* DDR3PHY */
pub const DDR3PHY_PIR: u32 = 0x04;
pub const DDR3PHY_PIR_DLLBYP: u32 = 1 << 17;
pub const DDR3PHY_PIR_ITMSRST: u32 = 1 << 4;
pub const DDR3PHY_PIR_DLLLOCK: u32 = 1 << 2;
pub const DDR3PHY_PIR_DLLSRST: u32 = 1 << 1;
pub const DDR3PHY_PIR_INIT: u32 = 1 << 0;

pub const DDR3PHY_PGCR: u32 = 0x08;
pub const DDR3PHY_PGCR_CKDV1: u32 = 1 << 13;
pub const DDR3PHY_PGCR_CKDV0: u32 = 1 << 12;

pub const DDR3PHY_PGSR: u32 = 0x0C;
pub const DDR3PHY_PGSR_IDONE: u32 = 1 << 0;

pub const DDR3PHY_ACDLLCR: u32 = 0x14;
pub const DDR3PHY_ACDLLCR_DLLSRST: u32 = 1 << 30;

pub const DDR3PHY_ACIOCR: u32 = 0x24;
pub const DDR3PHY_ACIOCR_CSPDD_CS0: u32 = 1 << 18;
pub const DDR3PHY_ACIOCR_CKPDD_CK0: u32 = 1 << 8;
pub const DDR3PHY_ACIORC_ACPDD: u32 = 1 << 3;

pub const DDR3PHY_DXCCR: u32 = 0x28;
pub const DDR3PHY_DXCCR_DXPDR: u32 = 1 << 3;

pub const DDR3PHY_DSGCR: u32 = 0x2C;
pub const DDR3PHY_DSGCR_ODTPDD_ODT0: u32 = 1 << 20;

pub const DDR3PHY_ZQ0SR0: u32 = 0x188;
pub const DDR3PHY_ZQ0SR0_PDO_OFF: u32 = 0;
pub const DDR3PHY_ZQ0SR0_PUO_OFF: u32 = 5;
pub const DDR3PHY_ZQ0SR0_PDODT_OFF: u32 = 10;
pub const DDR3PHY_ZQ0SRO_PUODT_OFF: u32 = 15;

pub const DDR3PHY_DX0DLLCR: u32 = 0x1CC;
pub const DDR3PHY_DX1DLLCR: u32 = 0x20C;
pub const DDR3PHY_DXDLLCR_DLLDIS: u32 = 1 << 31;

/* UDDRC */
pub const UDDRC_STAT: u32 = 0x04;
pub const UDDRC_STAT_SELFREF_TYPE_DIS: u32 = 0x0 << 4;
pub const UDDRC_STAT_SELFREF_TYPE_PHY: u32 = 0x1 << 4;
pub const UDDRC_STAT_SELFREF_TYPE_SW: u32 = 0x2 << 4;
pub const UDDRC_STAT_SELFREF_TYPE_AUTO: u32 = 0x3 << 4;
pub const UDDRC_STAT_SELFREF_TYPE_MSK: u32 = 0x3 << 4;
pub const UDDRC_STAT_OPMODE_INIT: u32 = 0x0 << 0;
pub const UDDRC_STAT_OPMODE_NORMAL: u32 = 0x1 << 0;
pub const UDDRC_STAT_OPMODE_PWRDOWN: u32 = 0x2 << 0;
pub const UDDRC_STAT_OPMODE_SELF_REFRESH: u32 = 0x3 << 0;
pub const UDDRC_STAT_OPMODE_MSK: u32 = 0x7 << 0;

pub const UDDRC_PWRCTL: u32 = 0x30;
pub const UDDRC_PWRCTL_SELFREF_EN: u32 = 1 << 0;
pub const UDDRC_PWRCTL_SELFREF_SW: u32 = 1 << 5;

pub const UDDRC_DFIMISC: u32 = 0x1B0;
pub const UDDRC_DFIMISC_DFI_INIT_COMPLETE_EN: u32 = 1 << 0;

pub const UDDRC_SWCTRL: u32 = 0x320;
pub const UDDRC_SWCTRL_SW_DONE: u32 = 1 << 0;

pub const UDDRC_SWSTAT: u32 = 0x324;
pub const UDDRC_SWSTAT_SW_DONE_ACK: u32 = 1 << 0;

pub const UDDRC_PSTAT: u32 = 0x3FC;
pub const UDDRC_PSTAT_ALL_PORTS: u32 = 0x1F001F;

pub const UDDRC_PCTRL_0: u32 = 0x490;
pub const UDDRC_PCTRL_1: u32 = 0x540;
pub const UDDRC_PCTRL_2: u32 = 0x5F0;
pub const UDDRC_PCTRL_3: u32 = 0x6A0;
pub const UDDRC_PCTRL_4: u32 = 0x750;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
