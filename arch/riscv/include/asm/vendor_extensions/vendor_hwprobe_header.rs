/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright 2024 Rivos, Inc
 */

// Dependency intent: symbols from <asm/cpufeature.h> are supplied externally.

/// Set the vendor extension result when the extension is available, otherwise
/// record it as missing.
#[macro_export]
macro_rules! VENDOR_EXT_KEY {
    ($ext:ident) => {
        if __riscv_isa_extension_available(
            isainfo.isa,
            concat_idents!(RISCV_ISA_VENDOR_EXT_, $ext),
        ) {
            pair.value |= concat_idents!(RISCV_HWPROBE_VENDOR_EXT_, $ext);
        } else {
            missing |= concat_idents!(RISCV_HWPROBE_VENDOR_EXT_, $ext);
        }
    };
}

/*
 * Loop through and record extensions that 1) anyone has, and 2) anyone
 * doesn't have.
 *
 * _extension_checks is an arbitrary Rust block to set the values of pair.value
 * and missing. It should be filled with VENDOR_EXT_KEY! expressions.
 */
#[macro_export]
macro_rules! VENDOR_EXTENSION_SUPPORTED {
    ($pair:expr, $cpus:expr, $per_hart_vendor_bitmap:expr, $extension_checks:block) => {{
        let mut missing: u64 = 0;
        for_each_cpu!(cpu, ($cpus), {
            let isainfo: &mut riscv_isavendorinfo =
                &mut ($per_hart_vendor_bitmap)[cpu];
            let pair = &mut *$pair;
            $extension_checks
        });
        (*$pair).value &= !missing;
    }};
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
