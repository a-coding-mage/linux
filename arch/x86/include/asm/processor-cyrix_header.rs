/* SPDX-License-Identifier: GPL-2.0 */
/*
 * NSC/Cyrix CPU indexed register access. Must be inlined instead of
 * macros to ensure correct access ordering
 * Access order is always 0x22 (=offset), 0x23 (=value)
 */

// Dependency supplied by asm/pc-conf-reg.h in the original source.

#[inline]
unsafe fn getCx86(reg: u8) -> u8 {
    pc_conf_get(reg)
}

#[inline]
unsafe fn setCx86(reg: u8, data: u8) {
    pc_conf_set(reg, data);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
