/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * This file contains some defines for the floppy disk controller.
 * Various sources. Mostly "IBM Microcomputers: A Programmers
 * Handbook", Sanches and Canton.
 */

/* 82077's auxiliary status registers A & B (R) */
pub const FD_SRA: i32 = 0;
pub const FD_SRB: i32 = 1;

/* Digital Output Register */
pub const FD_DOR: i32 = 2;

/* 82077's tape drive register (R/W) */
pub const FD_TDR: i32 = 3;

/* 82077's data rate select register (W) */
pub const FD_DSR: i32 = 4;

/* Fd controller regs. S&C, about page 340 */
pub const FD_STATUS: i32 = 4;
pub const FD_DATA: i32 = 5;

/* Digital Input Register (read) */
pub const FD_DIR: i32 = 7;

/* Diskette Control Register (write)*/
pub const FD_DCR: i32 = 7;

/* Bits of main status register */
pub const STATUS_BUSYMASK: i32 = 0x0F; /* drive busy mask */
pub const STATUS_BUSY: i32 = 0x10; /* FDC busy */
pub const STATUS_DMA: i32 = 0x20; /* 0- DMA mode */
pub const STATUS_DIR: i32 = 0x40; /* 0- cpu->fdc */
pub const STATUS_READY: i32 = 0x80; /* Data reg ready */

/* Bits of FD_ST0 */
pub const ST0_DS: i32 = 0x03; /* drive select mask */
pub const ST0_HA: i32 = 0x04; /* Head (Address) */
pub const ST0_NR: i32 = 0x08; /* Not Ready */
pub const ST0_ECE: i32 = 0x10; /* Equipment check error */
pub const ST0_SE: i32 = 0x20; /* Seek end */
pub const ST0_INTR: i32 = 0xC0; /* Interrupt code mask */

/* Bits of FD_ST1 */
pub const ST1_MAM: i32 = 0x01; /* Missing Address Mark */
pub const ST1_WP: i32 = 0x02; /* Write Protect */
pub const ST1_ND: i32 = 0x04; /* No Data - unreadable */
pub const ST1_OR: i32 = 0x10; /* OverRun */
pub const ST1_CRC: i32 = 0x20; /* CRC error in data or addr */
pub const ST1_EOC: i32 = 0x80; /* End Of Cylinder */

/* Bits of FD_ST2 */
pub const ST2_MAM: i32 = 0x01; /* Missing Address Mark (again) */
pub const ST2_BC: i32 = 0x02; /* Bad Cylinder */
pub const ST2_SNS: i32 = 0x04; /* Scan Not Satisfied */
pub const ST2_SEH: i32 = 0x08; /* Scan Equal Hit */
pub const ST2_WC: i32 = 0x10; /* Wrong Cylinder */
pub const ST2_CRC: i32 = 0x20; /* CRC error in data field */
pub const ST2_CM: i32 = 0x40; /* Control Mark = deleted */

/* Bits of FD_ST3 */
pub const ST3_HA: i32 = 0x04; /* Head (Address) */
pub const ST3_DS: i32 = 0x08; /* drive is double-sided */
pub const ST3_TZ: i32 = 0x10; /* Track Zero signal (1=track 0) */
pub const ST3_RY: i32 = 0x20; /* drive is ready */
pub const ST3_WP: i32 = 0x40; /* Write Protect */
pub const ST3_FT: i32 = 0x80; /* Drive Fault */

/* Values for FD_COMMAND */
pub const FD_RECALIBRATE: i32 = 0x07; /* move to track 0 */
pub const FD_SEEK: i32 = 0x0F; /* seek track */
pub const FD_READ: i32 = 0xE6; /* read with MT, MFM, SKip deleted */
pub const FD_WRITE: i32 = 0xC5; /* write with MT, MFM */
pub const FD_SENSEI: i32 = 0x08; /* Sense Interrupt Status */
pub const FD_SPECIFY: i32 = 0x03; /* specify HUT etc */
pub const FD_FORMAT: i32 = 0x4D; /* format one track */
pub const FD_VERSION: i32 = 0x10; /* get version code */
pub const FD_CONFIGURE: i32 = 0x13; /* configure FIFO operation */
pub const FD_PERPENDICULAR: i32 = 0x12; /* perpendicular r/w mode */
pub const FD_GETSTATUS: i32 = 0x04; /* read ST3 */
pub const FD_DUMPREGS: i32 = 0x0E; /* dump the contents of the fdc regs */
pub const FD_READID: i32 = 0xEA; /* prints the header of a sector */
pub const FD_UNLOCK: i32 = 0x14; /* Fifo config unlock */
pub const FD_LOCK: i32 = 0x94; /* Fifo config lock */
pub const FD_RSEEK_OUT: i32 = 0x8f; /* seek out (i.e. to lower tracks) */
pub const FD_RSEEK_IN: i32 = 0xcf; /* seek in (i.e. to higher tracks) */

/* The following commands are new in the 82078. They are not used in the
 * floppy driver, except the first three. These commands may be useful for apps
 * which use the FDRAWCMD interface. For doc, get the 82078 spec sheets at
 * http://www.intel.com/design/archives/periphrl/docs/29046803.htm */
pub const FD_PARTID: i32 = 0x18; /* part id ("extended" version cmd) */
pub const FD_SAVE: i32 = 0x2e; /* save fdc regs for later restore */
pub const FD_DRIVESPEC: i32 = 0x8e; /* drive specification: Access to the
                                      * 2 Mbps data transfer rate for tape
                                      * drives */
pub const FD_RESTORE: i32 = 0x4e; /* later restore */
pub const FD_POWERDOWN: i32 = 0x27; /* configure FDC's powersave features */
pub const FD_FORMAT_N_WRITE: i32 = 0xef; /* format and write in one go. */
pub const FD_OPTION: i32 = 0x33; /* ISO format (which is a clean way to
                                    * pack more sectors on a track) */

/* DMA commands */
pub const DMA_READ: i32 = 0x46;
pub const DMA_WRITE: i32 = 0x4A;

/* FDC version return types */
pub const FDC_NONE: i32 = 0x00;
pub const FDC_UNKNOWN: i32 = 0x10; /* DO NOT USE THIS TYPE EXCEPT IF IDENTIFICATION
                                      FAILS EARLY */
pub const FDC_8272A: i32 = 0x20; /* Intel 8272a, NEC 765 */
pub const FDC_765ED: i32 = 0x30; /* Non-Intel 1MB-compatible FDC, can't detect */
pub const FDC_82072: i32 = 0x40; /* Intel 82072; 8272a + FIFO + DUMPREGS */
pub const FDC_82072A: i32 = 0x45; /* 82072A (on Sparcs) */
pub const FDC_82077_ORIG: i32 = 0x51; /* Original version of 82077AA, sans LOCK */
pub const FDC_82077: i32 = 0x52; /* 82077AA-1 */
pub const FDC_82078_UNKN: i32 = 0x5f; /* Unknown 82078 variant */
pub const FDC_82078: i32 = 0x60; /* 44pin 82078 or 64pin 82078SL */
pub const FDC_82078_1: i32 = 0x61; /* 82078-1 (2Mbps fdc) */
pub const FDC_S82078B: i32 = 0x62; /* S82078B (first seen on Adaptec AVA-2825 VLB
                                       * SCSI/EIDE/Floppy controller) */
pub const FDC_87306: i32 = 0x63; /* National Semiconductor PC 87306 */

/*
 * Beware: the fdc type list is roughly sorted by increasing features.
 * Presence of features is tested by comparing the FDC version id with the
 * "oldest" version that has the needed feature.
 * If during FDC detection, an obscure test fails late in the sequence, don't
 * assign FDC_UNKNOWN. Else the FDC will be treated as a dumb 8272a, or worse.
 * This is especially true if the tests are unneeded.
 */
pub const FD_RESET_DELAY: i32 = 20;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
