/* SPDX-License-Identifier: GPL-2.0 */

pub const CS5BCR: u32 = 0xff802050;
pub const CS5WCR: u32 = 0xff802058;
pub const CS5PCR: u32 = 0xff802070;

pub const BUS_SZ8: u32 = 1;
pub const BUS_SZ16: u32 = 2;
pub const BUS_SZ32: u32 = 3;

pub const PCMCIA_IODYN: u32 = 1;
pub const PCMCIA_ATA: u32 = 0;
pub const PCMCIA_IO8: u32 = 2;
pub const PCMCIA_IO16: u32 = 3;
pub const PCMCIA_COMM8: u32 = 4;
pub const PCMCIA_COMM16: u32 = 5;
pub const PCMCIA_ATTR8: u32 = 6;
pub const PCMCIA_ATTR16: u32 = 7;

pub const TYPE_SRAM: u32 = 0;
pub const TYPE_PCMCIA: u32 = 4;

/* write-read/write-write delay (0-7:0,1,2,3,4,5,6,7) */
pub const IWW5: u32 = 0;
pub const IWW6: u32 = 3;
/* different area, read-write delay (0-7:0,1,2,3,4,5,6,7) */
pub const IWRWD5: u32 = 2;
pub const IWRWD6: u32 = 2;
/* same area, read-write delay (0-7:0,1,2,3,4,5,6,7) */
pub const IWRWS5: u32 = 2;
pub const IWRWS6: u32 = 2;
/* different area, read-read delay (0-7:0,1,2,3,4,5,6,7) */
pub const IWRRD5: u32 = 2;
pub const IWRRD6: u32 = 2;
/* same area, read-read delay (0-7:0,1,2,3,4,5,6,7) */
pub const IWRRS5: u32 = 0;
pub const IWRRS6: u32 = 2;
/* burst count (0-3:4,8,16,32) */
pub const BST5: u32 = 0;
pub const BST6: u32 = 0;
/* bus size */
pub const SZ5: u32 = BUS_SZ16;
pub const SZ6: u32 = BUS_SZ16;
/* RD hold for SRAM (0-1:0,1) */
pub const RDSPL5: u32 = 0;
pub const RDSPL6: u32 = 0;
/* Burst pitch (0-7:0,1,2,3,4,5,6,7) */
pub const BW5: u32 = 0;
pub const BW6: u32 = 0;
/* Multiplex (0-1:0,1) */
pub const MPX5: u32 = 0;
pub const MPX6: u32 = 0;
/* device type */
pub const TYPE5: u32 = TYPE_PCMCIA;
pub const TYPE6: u32 = TYPE_PCMCIA;
/* address setup before assert CSn for SRAM (0-7:0,1,2,3,4,5,6,7) */
pub const ADS5: u32 = 0;
pub const ADS6: u32 = 0;
/* address hold after negate CSn for SRAM (0-7:0,1,2,3,4,5,6,7) */
pub const ADH5: u32 = 0;
pub const ADH6: u32 = 0;
/* CSn assert to RD assert delay for SRAM (0-7:0,1,2,3,4,5,6,7) */
pub const RDS5: u32 = 0;
pub const RDS6: u32 = 0;
/* RD negate to CSn negate delay for SRAM (0-7:0,1,2,3,4,5,6,7) */
pub const RDH5: u32 = 0;
pub const RDH6: u32 = 0;
/* CSn assert to WE assert delay for SRAM (0-7:0,1,2,3,4,5,6,7) */
pub const WTS5: u32 = 0;
pub const WTS6: u32 = 0;
/* WE negate to CSn negate delay for SRAM (0-7:0,1,2,3,4,5,6,7) */
pub const WTH5: u32 = 0;
pub const WTH6: u32 = 0;
/* BS hold (0-1:1,2) */
pub const BSH5: u32 = 0;
pub const BSH6: u32 = 0;
/* wait cycle (0-15:0,1,2,3,4,5,6,7,8,9,11,13,15,17,21,25) */
pub const IW5: u32 = 6; /* 60ns PIO mode 4 */
pub const IW6: u32 = 15; /* 250ns */

pub const SAA5: u32 = PCMCIA_IODYN; /* IDE area b4000000-b5ffffff */
pub const SAB5: u32 = PCMCIA_IODYN; /* CF  area b6000000-b7ffffff */
pub const PCWA5: u32 = 0; /* additional wait A (0-3:0,15,30,50) */
pub const PCWB5: u32 = 0; /* additional wait B (0-3:0,15,30,50) */
/* wait B (0-15:0,1,2,3,4,5,6,7,8,9,11,13,15,17,21,25) */
pub const PCIW5: u32 = 12;
/* Address->OE/WE assert delay A (0-7:0,1,2,3,6,9,12,15) */
pub const TEDA5: u32 = 2;
/* Address->OE/WE assert delay B (0-7:0,1,2,3,6,9,12,15) */
pub const TEDB5: u32 = 4;
/* OE/WE negate->Address delay A (0-7:0,1,2,3,6,9,12,15) */
pub const TEHA5: u32 = 2;
/* OE/WE negate->Address delay B (0-7:0,1,2,3,6,9,12,15) */
pub const TEHB5: u32 = 3;

pub const CS5BCR_D: u32 = (IWW5 << 28) | (IWRWD5 << 24) | (IWRWS5 << 20)
    | (IWRRD5 << 16) | (IWRRS5 << 12) | (BST5 << 10)
    | (SZ5 << 8) | (RDSPL5 << 7) | (BW5 << 4) | (MPX5 << 3) | TYPE5;
pub const CS5WCR_D: u32 = (ADS5 << 28) | (ADH5 << 24) | (RDS5 << 20)
    | (RDH5 << 16) | (WTS5 << 12) | (WTH5 << 8) | (BSH5 << 4) | IW5;
pub const CS5PCR_D: u32 = (SAA5 << 28) | (SAB5 << 24) | (PCWA5 << 22)
    | (PCWB5 << 20) | (PCIW5 << 16) | (TEDA5 << 12)
    | (TEDB5 << 8) | (TEHA5 << 4) | TEHB5;

pub const SMC0_BASE: u32 = 0xb0800000; /* eth0 */
pub const SMC1_BASE: u32 = 0xb0900000; /* eth1 */
pub const CF_BASE: u32 = 0xb6100000; /* Compact Flash (I/O area) */
pub const IDE_BASE: u32 = 0xb4000000; /* IDE */
pub const PC104_IO_BASE: u32 = 0xb8000000;
pub const PC104_MEM_BASE: u32 = 0xba000000;
pub const SMC_IO_SIZE: u32 = 0x100;

pub const CF_OFFSET: u32 = 0x1f0;
pub const IDE_OFFSET: u32 = 0x170;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
