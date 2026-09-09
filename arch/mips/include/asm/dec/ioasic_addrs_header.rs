/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Definitions for the address map in the JUNKIO Asic
 *
 * Created with Information from:
 *
 * "DEC 3000 300/400/500/600/700/800/900 AXP Models System Programmer's Manual"
 *
 * and the Mach Sources
 *
 * Copyright (C) 199x  the Anonymous
 * Copyright (C) 2002, 2003  Maciej W. Rozycki
 */

pub const IOASIC_SLOT_SIZE: u32 = 0x0004_0000;

/* Address ranges decoded by the I/O ASIC for onboard devices. */
pub const IOASIC_SYS_ROM: u32 = 0 * IOASIC_SLOT_SIZE; // system board ROM
pub const IOASIC_IOCTL: u32 = 1 * IOASIC_SLOT_SIZE; // I/O ASIC
pub const IOASIC_ESAR: u32 = 2 * IOASIC_SLOT_SIZE; // LANCE MAC address chip
pub const IOASIC_LANCE: u32 = 3 * IOASIC_SLOT_SIZE; // LANCE Ethernet
pub const IOASIC_SCC0: u32 = 4 * IOASIC_SLOT_SIZE; // SCC #0
pub const IOASIC_VDAC_HI: u32 = 5 * IOASIC_SLOT_SIZE; // VDAC (maxine)
pub const IOASIC_SCC1: u32 = 6 * IOASIC_SLOT_SIZE; // SCC #1 (3min, 3max+)
pub const IOASIC_VDAC_LO: u32 = 7 * IOASIC_SLOT_SIZE; // VDAC (maxine)
pub const IOASIC_TOY: u32 = 8 * IOASIC_SLOT_SIZE; // RTC
pub const IOASIC_ISDN: u32 = 9 * IOASIC_SLOT_SIZE; // ISDN (maxine)
pub const IOASIC_ERRADDR: u32 = 9 * IOASIC_SLOT_SIZE; // bus error address (3max+)
pub const IOASIC_CHKSYN: u32 = 10 * IOASIC_SLOT_SIZE; // ECC syndrome (3max+)
pub const IOASIC_ACC_BUS: u32 = 10 * IOASIC_SLOT_SIZE; // ACCESS.bus (maxine)
pub const IOASIC_MCR: u32 = 11 * IOASIC_SLOT_SIZE; // memory control (3max+)
pub const IOASIC_FLOPPY: u32 = 11 * IOASIC_SLOT_SIZE; // FDC (maxine)
pub const IOASIC_SCSI: u32 = 12 * IOASIC_SLOT_SIZE; // ASC SCSI
pub const IOASIC_FDC_DMA: u32 = 13 * IOASIC_SLOT_SIZE; // FDC DMA (maxine)
pub const IOASIC_SCSI_DMA: u32 = 14 * IOASIC_SLOT_SIZE; // ???
pub const IOASIC_RES_15: u32 = 15 * IOASIC_SLOT_SIZE; // unused?

/* Offsets for I/O ASIC registers (relative to (dec_kn_slot_base + IOASIC_IOCTL)). */
pub const IO_REG_SCSI_DMA_P: u32 = 0x00; // SCSI DMA Pointer
pub const IO_REG_SCSI_DMA_BP: u32 = 0x10; // SCSI DMA Buffer Pointer
pub const IO_REG_LANCE_DMA_P: u32 = 0x20; // LANCE DMA Pointer
pub const IO_REG_SCC0A_T_DMA_P: u32 = 0x30; // SCC0A Transmit DMA Pointer
pub const IO_REG_SCC0A_R_DMA_P: u32 = 0x40; // SCC0A Receive DMA Pointer
pub const IO_REG_SCC1A_T_DMA_P: u32 = 0x50; // SCC1A Transmit DMA Pointer
pub const IO_REG_SCC1A_R_DMA_P: u32 = 0x60; // SCC1A Receive DMA Pointer
pub const IO_REG_AB_T_DMA_P: u32 = 0x50; // ACCESS.bus Transmit DMA Pointer
pub const IO_REG_AB_R_DMA_P: u32 = 0x60; // ACCESS.bus Receive DMA Pointer
pub const IO_REG_FLOPPY_DMA_P: u32 = 0x70; // Floppy DMA Pointer
pub const IO_REG_ISDN_T_DMA_P: u32 = 0x80; // ISDN Transmit DMA Pointer
pub const IO_REG_ISDN_T_DMA_BP: u32 = 0x90; // ISDN Transmit DMA Buffer Pointer
pub const IO_REG_ISDN_R_DMA_P: u32 = 0xa0; // ISDN Receive DMA Pointer
pub const IO_REG_ISDN_R_DMA_BP: u32 = 0xb0; // ISDN Receive DMA Buffer Pointer
pub const IO_REG_DATA_0: u32 = 0xc0; // System Data Buffer 0
pub const IO_REG_DATA_1: u32 = 0xd0; // System Data Buffer 1
pub const IO_REG_DATA_2: u32 = 0xe0; // System Data Buffer 2
pub const IO_REG_DATA_3: u32 = 0xf0; // System Data Buffer 3
pub const IO_REG_SSR: u32 = 0x100; // System Support Register
pub const IO_REG_SIR: u32 = 0x110; // System Interrupt Register
pub const IO_REG_SIMR: u32 = 0x120; // System Interrupt Mask Reg.
pub const IO_REG_SAR: u32 = 0x130; // System Address Register
pub const IO_REG_ISDN_T_DATA: u32 = 0x140; // ISDN Xmit Data Register
pub const IO_REG_ISDN_R_DATA: u32 = 0x150; // ISDN Receive Data Register
pub const IO_REG_LANCE_SLOT: u32 = 0x160; // LANCE I/O Slot Register
pub const IO_REG_SCSI_SLOT: u32 = 0x170; // SCSI Slot Register
pub const IO_REG_SCC0A_SLOT: u32 = 0x180; // SCC0A DMA Slot Register
pub const IO_REG_SCC1A_SLOT: u32 = 0x190; // SCC1A DMA Slot Register
pub const IO_REG_AB_SLOT: u32 = 0x190; // ACCESS.bus DMA Slot Register
pub const IO_REG_FLOPPY_SLOT: u32 = 0x1a0; // Floppy Slot Register
pub const IO_REG_SCSI_SCR: u32 = 0x1b0; // SCSI Partial-Word DMA Control
pub const IO_REG_SCSI_SDR0: u32 = 0x1c0; // SCSI DMA Partial Word 0
pub const IO_REG_SCSI_SDR1: u32 = 0x1d0; // SCSI DMA Partial Word 1
pub const IO_REG_FCTR: u32 = 0x1e0; // Free-Running Counter
pub const IO_REG_RES_31: u32 = 0x1f0; // unused

/* The upper 16 bits of the System Support Register are common to all I/O ASIC machines;
 * Maxine uses the FLOPPY and ISDN bits (otherwise unused) and has different SCC wiring. */
pub const IO_SSR_SCC0A_TX_DMA_EN: u32 = 1 << 31; // SCC0A transmit DMA enable
pub const IO_SSR_SCC0A_RX_DMA_EN: u32 = 1 << 30; // SCC0A receive DMA enable
pub const IO_SSR_RES_27: u32 = 1 << 27; // unused
pub const IO_SSR_RES_26: u32 = 1 << 26; // unused
pub const IO_SSR_RES_25: u32 = 1 << 25; // unused
pub const IO_SSR_RES_24: u32 = 1 << 24; // unused
pub const IO_SSR_RES_23: u32 = 1 << 23; // unused
pub const IO_SSR_SCSI_DMA_DIR: u32 = 1 << 18; // SCSI DMA direction
pub const IO_SSR_SCSI_DMA_EN: u32 = 1 << 17; // SCSI DMA enable
pub const IO_SSR_LANCE_DMA_EN: u32 = 1 << 16; // LANCE DMA enable
pub const IO_SSR_SCC1A_TX_DMA_EN: u32 = 1 << 29; // SCC1A transmit DMA enable
pub const IO_SSR_SCC1A_RX_DMA_EN: u32 = 1 << 28; // SCC1A receive DMA enable
pub const IO_SSR_RES_22: u32 = 1 << 22; // unused
pub const IO_SSR_RES_21: u32 = 1 << 21; // unused
pub const IO_SSR_RES_20: u32 = 1 << 20; // unused
pub const IO_SSR_RES_19: u32 = 1 << 19; // unused
pub const IO_SSR_AB_TX_DMA_EN: u32 = 1 << 29; // ACCESS.bus xmit DMA enable
pub const IO_SSR_AB_RX_DMA_EN: u32 = 1 << 28; // ACCESS.bus recv DMA enable
pub const IO_SSR_FLOPPY_DMA_DIR: u32 = 1 << 22; // Floppy DMA direction
pub const IO_SSR_FLOPPY_DMA_EN: u32 = 1 << 21; // Floppy DMA enable
pub const IO_SSR_ISDN_TX_DMA_EN: u32 = 1 << 20; // ISDN transmit DMA enable
pub const IO_SSR_ISDN_RX_DMA_EN: u32 = 1 << 19; // ISDN receive DMA enable

/* The lower 16 bits are system-specific. Bits 15,11:8 are common. */
pub const KN0X_IO_SSR_DIAGDN: u32 = 1 << 15; // diagnostic jumper
pub const KN0X_IO_SSR_SCC_RST: u32 = 1 << 11; // ~SCC0,1 (Z85C30) reset
pub const KN0X_IO_SSR_RTC_RST: u32 = 1 << 10; // ~RTC (DS1287) reset
pub const KN0X_IO_SSR_ASC_RST: u32 = 1 << 9; // ~ASC (NCR53C94) reset
pub const KN0X_IO_SSR_LANCE_RST: u32 = 1 << 8; // ~LANCE (Am7990) reset

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
