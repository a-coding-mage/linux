/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the corresponding UAPI header:
// use crate::uapi::asm::mman::*;
// The following items are also supplied by kernel dependencies:
// vm_flags_t, system_supports_bti, system_supports_mte, system_supports_poe,
// system_supports_gcs, shmem_file, is_file_hugepages, VM_WARN_ON, and the
// PROT_*, VM_*, MAP_*, and BIT symbols.

// This section is excluded when BUILD_VDSO is defined in the C build.

#[inline]
pub unsafe fn arch_calc_vm_prot_bits(prot: ::core::ffi::c_ulong, pkey: ::core::ffi::c_ulong) -> vm_flags_t {
    let mut ret: vm_flags_t = 0;

    if system_supports_bti() && (prot & PROT_BTI) != 0 {
        ret |= VM_ARM64_BTI;
    }

    if system_supports_mte() && (prot & PROT_MTE) != 0 {
        ret |= VM_MTE;
    }

    // CONFIG_ARCH_HAS_PKEYS
    if system_supports_poe() {
        ret |= if (pkey & BIT(0)) != 0 { VM_PKEY_BIT0 } else { 0 };
        ret |= if (pkey & BIT(1)) != 0 { VM_PKEY_BIT1 } else { 0 };
        ret |= if (pkey & BIT(2)) != 0 { VM_PKEY_BIT2 } else { 0 };
    }

    ret
}

#[inline]
pub unsafe fn arch_calc_vm_flag_bits(file: *mut struct_file, flags: ::core::ffi::c_ulong) -> vm_flags_t {
    /*
     * Only allow MTE on anonymous mappings as these are guaranteed to be
     * backed by tags-capable memory. The vm_flags may be overridden by a
     * filesystem supporting MTE (RAM-based).
     */
    if system_supports_mte() {
        if (flags & (MAP_ANONYMOUS | MAP_HUGETLB)) != 0 {
            return VM_MTE_ALLOWED;
        }
        if shmem_file(file) || is_file_hugepages(file) {
            return VM_MTE_ALLOWED;
        }
    }

    0
}

#[inline]
pub unsafe fn arch_validate_prot(
    prot: ::core::ffi::c_ulong,
    _addr: ::core::ffi::c_ulong,
) -> bool {
    let mut supported = PROT_READ | PROT_WRITE | PROT_EXEC | PROT_SEM;

    if system_supports_bti() {
        supported |= PROT_BTI;
    }

    if system_supports_mte() {
        supported |= PROT_MTE;
    }

    (prot & !supported) == 0
}

#[inline]
pub unsafe fn arch_validate_flags(vm_flags: vm_flags_t) -> bool {
    if system_supports_mte() {
        /*
         * only allow VM_MTE if VM_MTE_ALLOWED has been set
         * previously
         */
        if (vm_flags & VM_MTE) != 0 && (vm_flags & VM_MTE_ALLOWED) == 0 {
            return false;
        }
    }

    if system_supports_gcs() && (vm_flags & VM_SHADOW_STACK) != 0 {
        /* An executable GCS isn't a good idea. */
        if (vm_flags & VM_EXEC) != 0 {
            return false;
        }

        /* The memory management core should prevent this */
        VM_WARN_ON((vm_flags & VM_SHARED) != 0);
    }

    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
