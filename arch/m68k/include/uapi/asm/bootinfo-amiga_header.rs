/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
** asm/bootinfo-amiga.h -- Amiga-specific boot information definitions
*/

/*
 *  Amiga-specific tags
 */

pub const BI_AMIGA_MODEL: u32 = 0x8000; /* model (__be32) */
pub const BI_AMIGA_AUTOCON: u32 = 0x8001; /* AutoConfig device */
/* (AmigaOS struct ConfigDev) */
pub const BI_AMIGA_CHIP_SIZE: u32 = 0x8002; /* size of Chip RAM (__be32) */
pub const BI_AMIGA_VBLANK: u32 = 0x8003; /* VBLANK frequency (__u8) */
pub const BI_AMIGA_PSFREQ: u32 = 0x8004; /* power supply frequency (__u8) */
pub const BI_AMIGA_ECLOCK: u32 = 0x8005; /* EClock frequency (__be32) */
pub const BI_AMIGA_CHIPSET: u32 = 0x8006; /* native chipset present (__be32) */
pub const BI_AMIGA_SERPER: u32 = 0x8007; /* serial port period (__be16) */

/*
 *  Amiga models (BI_AMIGA_MODEL)
 */

pub const AMI_UNKNOWN: u32 = 0;
pub const AMI_500: u32 = 1;
pub const AMI_500PLUS: u32 = 2;
pub const AMI_600: u32 = 3;
pub const AMI_1000: u32 = 4;
pub const AMI_1200: u32 = 5;
pub const AMI_2000: u32 = 6;
pub const AMI_2500: u32 = 7;
pub const AMI_3000: u32 = 8;
pub const AMI_3000T: u32 = 9;
pub const AMI_3000PLUS: u32 = 10;
pub const AMI_4000: u32 = 11;
pub const AMI_4000T: u32 = 12;
pub const AMI_CDTV: u32 = 13;
pub const AMI_CD32: u32 = 14;
pub const AMI_DRACO: u32 = 15;

/*
 *  Amiga chipsets (BI_AMIGA_CHIPSET)
 */

pub const CS_STONEAGE: u32 = 0;
pub const CS_OCS: u32 = 1;
pub const CS_ECS: u32 = 2;
pub const CS_AGA: u32 = 3;

/*
 *  Latest Amiga bootinfo version
 */

/* MK_BI_VERSION is supplied by the bootinfo definitions dependency. */
pub const AMIGA_BOOTI_VERSION: u32 = MK_BI_VERSION(2, 0);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
