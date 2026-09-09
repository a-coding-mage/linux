/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * pmc.h
 * Copyright (C) 2004  David Gibson, IBM Corporation
 */

// C header guard: _POWERPC_PMC_H
// The declarations below are kernel-only in the original header (__KERNEL__).

pub type PerfIrqT = unsafe extern "C" fn(regs: *mut PtRegs);

unsafe extern "C" {
    pub static mut perf_irq: PerfIrqT;

    pub fn reserve_pmc_hardware(new_perf_irq: PerfIrqT) -> ::core::ffi::c_int;
    pub fn release_pmc_hardware();
    pub fn ppc_enable_pmcs();
}

// CONFIG_PPC_BOOK3S_64
#[cfg(CONFIG_PPC_BOOK3S_64)]
#[inline]
pub unsafe fn ppc_set_pmu_inuse(inuse: ::core::ffi::c_int) {
    // CONFIG_PPC_PSERIES || CONFIG_KVM_BOOK3S_HV_POSSIBLE
    #[cfg(any(CONFIG_PPC_PSERIES, CONFIG_KVM_BOOK3S_HV_POSSIBLE))]
    {
        if firmware_has_feature(FW_FEATURE_LPAR) {
            // CONFIG_PPC_PSERIES
            #[cfg(CONFIG_PPC_PSERIES)]
            {
                (*get_lppaca()).pmcregs_in_use = inuse;
            }
        }

        // CONFIG_KVM_BOOK3S_HV_POSSIBLE
        #[cfg(CONFIG_KVM_BOOK3S_HV_POSSIBLE)]
        {
            (*get_paca()).pmcregs_in_use = inuse;
        }
    }
}

// CONFIG_KVM_BOOK3S_HV_POSSIBLE
#[cfg(all(CONFIG_PPC_BOOK3S_64, CONFIG_KVM_BOOK3S_HV_POSSIBLE))]
#[inline]
pub unsafe fn ppc_get_pmu_inuse() -> ::core::ffi::c_int {
    (*get_paca()).pmcregs_in_use
}

#[cfg(CONFIG_PPC_BOOK3S_64)]
unsafe extern "C" {
    pub fn power4_enable_pmcs();
}

// Else branch: !CONFIG_PPC_BOOK3S_64 (original comment: CONFIG_PPC64).
#[cfg(not(CONFIG_PPC_BOOK3S_64))]
#[inline]
pub fn ppc_set_pmu_inuse(_inuse: ::core::ffi::c_int) {}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
