/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by asm/vendor_extensions.h.
// Dependency supplied by linux/types.h.

pub const RISCV_ISA_VENDOR_EXT_XANDESPMU: i32 = 0;

/*
 * Extension keys should be strictly less than max.
 * It is safe to increment this when necessary.
 */
pub const RISCV_ISA_VENDOR_EXT_MAX_ANDES: i32 = 32;

extern "C" {
    pub static mut riscv_isa_vendor_ext_list_andes: riscv_isa_vendor_ext_data_list;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
