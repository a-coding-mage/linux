/* SPDX-License-Identifier: GPL-2.0 */
/* fhc.h: FHC and Clock board register definitions.
 *
 * Copyright (C) 1997, 1999 David S. Miller (davem@redhat.com)
 */

/* Clock board register offsets. */
pub const CLOCK_CTRL: u64 = 0x00;
pub const CLOCK_STAT1: u64 = 0x10;
pub const CLOCK_STAT2: u64 = 0x20;
pub const CLOCK_PWRSTAT: u64 = 0x30;
pub const CLOCK_PWRPRES: u64 = 0x40;
pub const CLOCK_TEMP: u64 = 0x50;
pub const CLOCK_IRQDIAG: u64 = 0x60;
pub const CLOCK_PWRSTAT2: u64 = 0x70;

pub const CLOCK_CTRL_LLED: u32 = 0x04;
pub const CLOCK_CTRL_MLED: u32 = 0x02;
pub const CLOCK_CTRL_RLED: u32 = 0x01;

/* Firehose controller register offsets */
pub const FHC_PREGS_ID: u64 = 0x00;
pub const FHC_ID_VERS: u32 = 0xf0000000;
pub const FHC_ID_PARTID: u32 = 0x0ffff000;
pub const FHC_ID_MANUF: u32 = 0x0000007e;
pub const FHC_ID_RESV: u32 = 0x00000001;
pub const FHC_PREGS_RCS: u64 = 0x10;
pub const FHC_RCS_POR: u32 = 0x80000000;
pub const FHC_RCS_SPOR: u32 = 0x40000000;
pub const FHC_RCS_SXIR: u32 = 0x20000000;
pub const FHC_RCS_BPOR: u32 = 0x10000000;
pub const FHC_RCS_BXIR: u32 = 0x08000000;
pub const FHC_RCS_WEVENT: u32 = 0x04000000;
pub const FHC_RCS_CFATAL: u32 = 0x02000000;
pub const FHC_RCS_FENAB: u32 = 0x01000000;
pub const FHC_PREGS_CTRL: u64 = 0x20;
pub const FHC_CONTROL_ICS: u32 = 0x00100000;
pub const FHC_CONTROL_FRST: u32 = 0x00080000;
pub const FHC_CONTROL_LFAT: u32 = 0x00040000;
pub const FHC_CONTROL_SLINE: u32 = 0x00010000;
pub const FHC_CONTROL_DCD: u32 = 0x00008000;
pub const FHC_CONTROL_POFF: u32 = 0x00004000;
pub const FHC_CONTROL_FOFF: u32 = 0x00002000;
pub const FHC_CONTROL_AOFF: u32 = 0x00001000;
pub const FHC_CONTROL_BOFF: u32 = 0x00000800;
pub const FHC_CONTROL_PSOFF: u32 = 0x00000400;
pub const FHC_CONTROL_IXIST: u32 = 0x00000200;
pub const FHC_CONTROL_XMSTR: u32 = 0x00000100;
pub const FHC_CONTROL_LLED: u32 = 0x00000040;
pub const FHC_CONTROL_MLED: u32 = 0x00000020;
pub const FHC_CONTROL_RLED: u32 = 0x00000010;
pub const FHC_CONTROL_BPINS: u32 = 0x00000003;
pub const FHC_PREGS_BSR: u64 = 0x30;
pub const FHC_BSR_DA64: u32 = 0x00040000;
pub const FHC_BSR_DB64: u32 = 0x00020000;
pub const FHC_BSR_BID: u32 = 0x0001e000;
pub const FHC_BSR_SA: u32 = 0x00001c00;
pub const FHC_BSR_SB: u32 = 0x00000380;
pub const FHC_BSR_NDIAG: u32 = 0x00000040;
pub const FHC_BSR_NTBED: u32 = 0x00000020;
pub const FHC_BSR_NIA: u32 = 0x0000001c;
pub const FHC_BSR_SI: u32 = 0x00000001;
pub const FHC_PREGS_ECC: u64 = 0x40;
pub const FHC_PREGS_JCTRL: u64 = 0xf0;
pub const FHC_JTAG_CTRL_MENAB: u32 = 0x80000000;
pub const FHC_JTAG_CTRL_MNONE: u32 = 0x40000000;
pub const FHC_PREGS_JCMD: u64 = 0x100;
pub const FHC_IREG_IGN: u64 = 0x00;
pub const FHC_FFREGS_IMAP: u64 = 0x00;
pub const FHC_FFREGS_ICLR: u64 = 0x10;
pub const FHC_SREGS_IMAP: u64 = 0x00;
pub const FHC_SREGS_ICLR: u64 = 0x10;
pub const FHC_UREGS_IMAP: u64 = 0x00;
pub const FHC_UREGS_ICLR: u64 = 0x10;
pub const FHC_TREGS_IMAP: u64 = 0x00;
pub const FHC_TREGS_ICLR: u64 = 0x10;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
