// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the surrounding kernel Rust bindings:
// asm/vendor_extensions/thead.h
// asm/vendor_extensions/thead_hwprobe.h
// asm/vendor_extensions/vendor_hwprobe.h
// linux/cpumask.h
// linux/types.h
// uapi/asm/hwprobe.h
// uapi/asm/vendor/thead.h

/// Rust spelling of `struct riscv_hwprobe` supplied by the kernel bindings.
use crate::riscv_hwprobe;
/// Rust spelling of `struct cpumask` supplied by the kernel bindings.
use crate::cpumask;

/// Probe the T-Head ISA vendor extensions for the supplied CPUs.
pub unsafe fn hwprobe_isa_vendor_ext_thead_0(
    pair: *mut riscv_hwprobe,
    cpus: *const cpumask,
) {
    // `VENDOR_EXTENSION_SUPPORTED` and `VENDOR_EXT_KEY` are external kernel
    // macros. Their expansion is retained here as a Rust macro invocation so
    // the surrounding bindings can provide the exact kernel implementation.
    crate::VENDOR_EXTENSION_SUPPORTED!(
        pair,
        cpus,
        crate::riscv_isa_vendor_ext_list_thead.per_hart_isa_bitmap,
        {
            crate::VENDOR_EXT_KEY!(crate::XTHEADVECTOR);
        }
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
