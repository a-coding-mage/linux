/* SPDX-License-Identifier: GPL-2.0 */

// Conditional on CONFIG_X86_INTEL_MEMORY_PROTECTION_KEYS in the source header.
// The referenced VM_PKEY_BIT* symbols are supplied by other dependencies.
#[cfg(CONFIG_X86_INTEL_MEMORY_PROTECTION_KEYS)]
macro_rules! arch_calc_vm_prot_bits {
    ($prot:expr, $key:expr) => {
        (if ($key & 0x1) != 0 { VM_PKEY_BIT0 } else { 0 })
            | (if ($key & 0x2) != 0 { VM_PKEY_BIT1 } else { 0 })
            | (if ($key & 0x4) != 0 { VM_PKEY_BIT2 } else { 0 })
            | (if ($key & 0x8) != 0 { VM_PKEY_BIT3 } else { 0 })
    };
}

// The source includes <uapi/asm/mman.h>; its declarations are supplied by
// another translated dependency.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
