/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * include/asm-mips/dec/kn05.h
 *
 * DECstation/DECsystem 5000/260 (4max+ or KN05), 5000/150 (4min
 * or KN04-BA), Personal DECstation/DECsystem 5000/50 (4maxine or
 * KN04-CA) and DECsystem 5900/260 (KN05) R4k CPU card MB ASIC
 * definitions.
 *
 * Copyright (C) 2002, 2003, 2005, 2008 Maciej W. Rozycki
 *
 * WARNING! All this information is pure guesswork based on the
 * ROM. It is provided here in hope it will give someone some
 * food for thought. No documentation for the KN05 nor the KN04
 * module has been located so far.
 */

// Dependency supplied by asm/dec/ioasic_addrs.h:
// IOASIC_SLOT_SIZE

/*
 * The oncard MB (Memory Buffer) ASIC provides an additional address
 * decoder. Certain address ranges within the "high" 16 slots are
 * passed to the I/O ASIC's decoder like with the KN03 or KN02-BA/CA.
 * Others are handled locally. "Low" slots are always passed.
 */
pub const KN4K_SLOT_BASE: u32 = 0x1fc00000;

pub const KN4K_MB_ROM: u32 = 0 * IOASIC_SLOT_SIZE;
pub const KN4K_IOCTL: u32 = 1 * IOASIC_SLOT_SIZE;
pub const KN4K_ESAR: u32 = 2 * IOASIC_SLOT_SIZE;
pub const KN4K_LANCE: u32 = 3 * IOASIC_SLOT_SIZE;
pub const KN4K_MB_INT: u32 = 4 * IOASIC_SLOT_SIZE;
pub const KN4K_MB_EA: u32 = 5 * IOASIC_SLOT_SIZE;
pub const KN4K_MB_EC: u32 = 6 * IOASIC_SLOT_SIZE;
pub const KN4K_MB_CSR: u32 = 7 * IOASIC_SLOT_SIZE;
pub const KN4K_RES_08: u32 = 8 * IOASIC_SLOT_SIZE;
pub const KN4K_RES_09: u32 = 9 * IOASIC_SLOT_SIZE;
pub const KN4K_RES_10: u32 = 10 * IOASIC_SLOT_SIZE;
pub const KN4K_RES_11: u32 = 11 * IOASIC_SLOT_SIZE;
pub const KN4K_SCSI: u32 = 12 * IOASIC_SLOT_SIZE;
pub const KN4K_RES_13: u32 = 13 * IOASIC_SLOT_SIZE;
pub const KN4K_RES_14: u32 = 14 * IOASIC_SLOT_SIZE;
pub const KN4K_RES_15: u32 = 15 * IOASIC_SLOT_SIZE;

/* MB ASIC interrupt bits. */
pub const KN4K_MB_INR_MB: u32 = 4;
pub const KN4K_MB_INR_MT: u32 = 3;
pub const KN4K_MB_INR_RES_2: u32 = 2;
pub const KN4K_MB_INR_RTC: u32 = 1;
pub const KN4K_MB_INR_TC: u32 = 0;

/* Bits for the MB interrupt register. The register appears read-only. */
pub const KN4K_MB_INT_IRQ: u32 = 0x1f << 0;
#[inline]
pub const fn KN4K_MB_INT_IRQ_N(n: u32) -> u32 {
    1 << n
}

/*
 * Bits for the MB control & status register.
 * Set to 0x00bf8001 for KN05 and to 0x003f8000 for KN04 by the firmware.
 */
pub const KN4K_MB_CSR_PF: u32 = 1 << 0;
pub const KN4K_MB_CSR_F: u32 = 1 << 1;
pub const KN4K_MB_CSR_ECC: u32 = 0xff << 2;
pub const KN4K_MB_CSR_OD: u32 = 1 << 10;
pub const KN4K_MB_CSR_CP: u32 = 1 << 11;
pub const KN4K_MB_CSR_UNC: u32 = 1 << 12;
pub const KN4K_MB_CSR_IM: u32 = 1 << 13;
pub const KN4K_MB_CSR_NC: u32 = 1 << 14;
pub const KN4K_MB_CSR_EE: u32 = 1 << 15;
pub const KN4K_MB_CSR_MSK: u32 = 0x1f << 16;
#[inline]
pub const fn KN4K_MB_CSR_MSK_N(n: u32) -> u32 {
    1 << (n + 16)
}
pub const KN4K_MB_CSR_FW: u32 = 1 << 21;
pub const KN4K_MB_CSR_W: u32 = 1 << 31;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
