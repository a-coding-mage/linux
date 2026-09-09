// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2025 MIPS.
 */

// C dependencies supplied by the surrounding kernel translation unit:
// asm/vendor_extensions.h
// asm/vendor_extensions/mips.h
// asm/vendor_extensions/mips_hwprobe.h
// asm/vendor_extensions/vendor_hwprobe.h
// linux/cpumask.h
// linux/types.h
// uapi/asm/hwprobe.h
// uapi/asm/vendor/mips.h

#[repr(C)]
pub struct riscv_hwprobe {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cpumask {
    _private: [u8; 0],
}

extern "C" {
    static riscv_isa_vendor_ext_list_mips: riscv_isa_vendor_ext_list;
}

#[repr(C)]
pub struct riscv_isa_vendor_ext_list {
    pub per_hart_isa_bitmap: *const u64,
}

// The following macros are provided by the corresponding kernel headers.
// macro_rules! VENDOR_EXTENSION_SUPPORTED;
// macro_rules! VENDOR_EXT_KEY;

pub unsafe fn hwprobe_isa_vendor_ext_mips_0(
    pair: *mut riscv_hwprobe,
    cpus: *const cpumask,
) {
    VENDOR_EXTENSION_SUPPORTED!(
        pair,
        cpus,
        riscv_isa_vendor_ext_list_mips.per_hart_isa_bitmap,
        { VENDOR_EXT_KEY!(XMIPSEXECTL); }
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
