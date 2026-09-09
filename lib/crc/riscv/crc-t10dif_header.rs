// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * RISC-V optimized CRC-T10DIF function
 *
 * Copyright 2025 Google LLC
 */

// Dependencies supplied by the included architecture and CRC headers:
// asm/hwcap.h, asm/alternative-macros.h, and crc-clmul.h.

#[inline]
fn crc_t10dif_arch(crc: u16, p: *const u8, len: usize) -> u16 {
    unsafe {
        if riscv_has_extension_likely(RISCV_ISA_EXT_ZBC) {
            crc16_msb_clmul(crc, p, len, &crc16_msb_0x8bb7_consts)
        } else {
            crc_t10dif_generic(crc, p, len)
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
