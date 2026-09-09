/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright 2023-2024 Rivos, Inc
 */

// Translated from <uapi/asm/hwprobe.h>.

pub const RISCV_HWPROBE_MAX_KEY: i64 = 16;

#[inline]
pub fn riscv_hwprobe_key_is_valid(key: i64) -> bool {
    key >= 0 && key <= RISCV_HWPROBE_MAX_KEY
}

#[inline]
pub fn hwprobe_key_is_bitmask(key: i64) -> bool {
    match key {
        RISCV_HWPROBE_KEY_BASE_BEHAVIOR
        | RISCV_HWPROBE_KEY_IMA_EXT_0
        | RISCV_HWPROBE_KEY_IMA_EXT_1
        | RISCV_HWPROBE_KEY_CPUPERF_0
        | RISCV_HWPROBE_KEY_VENDOR_EXT_THEAD_0
        | RISCV_HWPROBE_KEY_VENDOR_EXT_MIPS_0
        | RISCV_HWPROBE_KEY_VENDOR_EXT_SIFIVE_0 => true,
        _ => false,
    }
}

#[inline]
pub unsafe fn riscv_hwprobe_pair_cmp(
    pair: *mut riscv_hwprobe,
    other_pair: *mut riscv_hwprobe,
) -> bool {
    if (*pair).key != (*other_pair).key {
        return false;
    }

    if hwprobe_key_is_bitmask((*pair).key) {
        return ((*pair).value & (*other_pair).value) == (*other_pair).value;
    }

    (*pair).value == (*other_pair).value
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
