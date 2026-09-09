// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2025 MIPS.
 */

/* All MIPS vendor extensions supported in Linux */
static riscv_isa_vendor_ext_mips: [riscv_isa_ext_data; 1] = [
    __RISCV_ISA_EXT_DATA!(xmipsexectl, RISCV_ISA_VENDOR_EXT_XMIPSEXECTL),
];

static mut riscv_isa_vendor_ext_list_mips: riscv_isa_vendor_ext_data_list =
    riscv_isa_vendor_ext_data_list {
        ext_data_count: riscv_isa_vendor_ext_mips.len(),
        ext_data: riscv_isa_vendor_ext_mips.as_ptr(),
    };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
