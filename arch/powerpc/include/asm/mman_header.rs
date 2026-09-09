/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 */

// Dependency intent from <uapi/asm/mman.h> and the other included kernel headers
// is preserved here; those names are supplied by the surrounding translation.

#[cfg(all(feature = "CONFIG_PPC64", not(feature = "BUILD_VDSO")))]
pub unsafe fn arch_calc_vm_prot_bits(prot: ::core::ffi::c_ulong, pkey: ::core::ffi::c_ulong) -> vm_flags_t {
    #[cfg(feature = "CONFIG_PPC_MEM_KEYS")]
    {
        return (((if prot & PROT_SAO != 0 { VM_SAO } else { 0 })
            | pkey_to_vmflag_bits(pkey)) as vm_flags_t);
    }

    #[cfg(not(feature = "CONFIG_PPC_MEM_KEYS"))]
    {
        (if prot & PROT_SAO != 0 { VM_SAO } else { 0 }) as vm_flags_t
    }
}

// C macro: arch_calc_vm_prot_bits(prot, pkey) arch_calc_vm_prot_bits(prot, pkey)

#[cfg(all(feature = "CONFIG_PPC64", not(feature = "BUILD_VDSO")))]
pub unsafe fn arch_validate_prot(
    prot: ::core::ffi::c_ulong,
    _addr: ::core::ffi::c_ulong,
) -> bool {
    if prot & !(PROT_READ | PROT_WRITE | PROT_EXEC | PROT_SEM | PROT_SAO) != 0 {
        return false;
    }
    if prot & PROT_SAO != 0 {
        if !cpu_has_feature(CPU_FTR_SAO) {
            return false;
        }
        if firmware_has_feature(FW_FEATURE_LPAR)
            // IS_ENABLED(CONFIG_PPC_PROT_SAO_LPAR) is a build-time condition.
            && !cfg!(feature = "CONFIG_PPC_PROT_SAO_LPAR")
        {
            return false;
        }
    }
    true
}

// C macro: arch_validate_prot arch_validate_prot

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
