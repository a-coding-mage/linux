/* SPDX-License-Identifier: GPL-2.0 */

// include/asm-ppc/redboot.h
//   Copyright (c) 2002, 2003 Gary Thomas (<gary@mlbassoc.com>
//   Copyright (c) 1997 Dan Malek (dmalek@jlc.net)

// Board specific details, as provided by RedBoot.

/* A Board Information structure that is given to a program when
 * RedBoot starts it up.  Note: not all fields make sense for all
 * architectures and it's up to the platform specific code to fill
 * in the details.
 */
#[repr(C)]
pub struct bd_info {
    pub bi_tag: u32,        /* Should be 0x42444944 "BDID" */
    pub bi_size: u32,       /* Size of this structure */
    pub bi_revision: u32,   /* revision of this structure */
    pub bi_bdate: u32,      /* bootstrap date, i.e. 0x19971106 */
    pub bi_memstart: u32,   /* Memory start address */
    pub bi_memsize: u32,    /* Memory (end) size in bytes */
    pub bi_intfreq: u32,    /* Internal Freq, in Hz */
    pub bi_busfreq: u32,    /* Bus Freq, in Hz */
    pub bi_cpmfreq: u32,    /* CPM Freq, in Hz */
    pub bi_brgfreq: u32,    /* BRG Freq, in Hz */
    pub bi_vco: u32,        /* VCO Out from PLL */
    pub bi_pci_freq: u32,   /* PCI Freq, in Hz */
    pub bi_baudrate: u32,   /* Default console baud rate */
    pub bi_immr: u32,       /* IMMR when called from boot rom */
    pub bi_enetaddr: [u8; 6],
    pub bi_flashbase: u32,  /* Physical address of FLASH memory */
    pub bi_flashsize: u32,  /* Length of FLASH memory */
    pub bi_flashwidth: i32, /* Width (8,16,32,64) */
    pub bi_cmdline: *mut u8, /* Pointer to command line */
    pub bi_esa: [[u8; 6]; 3], /* Ethernet station addresses */
    pub bi_ramdisk_begin: u32,
    pub bi_ramdisk_end: u32,
    pub bi_video: bd_video_info, /* Information about [main] video screen */
    pub bi_cputc: Option<unsafe extern "C" fn(char)>, /* Write a character to the RedBoot console */
    pub bi_cgetc: Option<unsafe extern "C" fn() -> i8>, /* Read a character from the RedBoot console */
    pub bi_ctstc: Option<unsafe extern "C" fn() -> i32>, /* Test for input on the RedBoot console */
}

#[repr(C)]
pub struct bd_video_info {
    pub x_res: i16,       /* Horizontal resolution in pixels */
    pub y_res: i16,       /* Vertical resolution in pixels */
    pub bpp: i16,         /* Bits/pixel */
    pub mode: i16,        /* Type of pixels (packed, indexed) */
    pub fb: usize,        /* Pointer to frame buffer (pixel) memory */
}

pub type bd_t = bd_info;

pub const BI_REV: u32 = 0x0102; /* Version 1.02 */

// C field aliases:
// #define bi_pci_busfreq bi_pci_freq
// #define bi_immr_base   bi_immr

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
