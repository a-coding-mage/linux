/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * NVRAM definitions and access functions.
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * as published by the Free Software Foundation; either version
 * 2 of the License, or (at your option) any later version.
 */

/* Signatures for nvram partitions */
pub const NVRAM_SIG_SP: u8 = 0x02; /* support processor */
pub const NVRAM_SIG_OF: u8 = 0x50; /* open firmware config */
pub const NVRAM_SIG_FW: u8 = 0x51; /* general firmware */
pub const NVRAM_SIG_HW: u8 = 0x52; /* hardware (VPD) */
pub const NVRAM_SIG_FLIP: u8 = 0x5a; /* Apple flip/flop header */
pub const NVRAM_SIG_APPL: u8 = 0x5f; /* Apple "system" (???) */
pub const NVRAM_SIG_SYS: u8 = 0x70; /* system env vars */
pub const NVRAM_SIG_CFG: u8 = 0x71; /* config data */
pub const NVRAM_SIG_ELOG: u8 = 0x72; /* error log */
pub const NVRAM_SIG_VEND: u8 = 0x7e; /* vendor defined */
pub const NVRAM_SIG_FREE: u8 = 0x7f; /* Free space */
pub const NVRAM_SIG_OS: u8 = 0xa0; /* OS defined */
pub const NVRAM_SIG_PANIC: u8 = 0xa1; /* Apple OSX "panic" */

/* PowerMac specific nvram stuffs */
#[repr(i32)]
pub enum PmacNvram {
    PmacNvramOf,   /* Open Firmware partition */
    PmacNvramXpram, /* MacOS XPRAM partition */
    PmacNvramNr,   /* MacOS Name Registry partition */
}

/* Some offsets in XPRAM */
pub const PMAC_XPRAM_MACHINE_LOC: u32 = 0xe4;
pub const PMAC_XPRAM_SOUND_VOLUME: u32 = 0x08;

/* Machine location structure in PowerMac XPRAM */
#[repr(C)]
pub struct PmacMachineLocation {
    pub latitude: u32,  /* 2+30 bit Fractional number */
    pub longitude: u32, /* 2+30 bit Fractional number */
    pub delta: u32,     /* mix of GMT delta and DLS */
}

/*
 * /dev/nvram ioctls
 *
 * Note that PMAC_NVRAM_GET_OFFSET is still supported, but is
 * definitely obsolete. Do not use it if you can avoid it
 */

/* _IOWR('p', 0x40, int) */
pub const OBSOLETE_PMAC_NVRAM_GET_OFFSET: u32 = 0xc004_7040;

/* _IOWR('p', 0x42, int) -- Get NVRAM partition offset */
pub const IOC_NVRAM_GET_OFFSET: u32 = 0xc004_7042;
/* _IO('p', 0x43) -- Sync NVRAM image */
pub const IOC_NVRAM_SYNC: u32 = 0x0000_7043;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
