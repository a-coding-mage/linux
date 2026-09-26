/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Prototypes for functions that are shared between setup_(32|64|common).c
 *
 * Copyright 2016 Michael Ellerman, IBM Corporation.
 */

use core::ffi::c_ulong;

unsafe extern "C" {
    pub fn initialize_cache_info();
    pub fn irqstack_early_init();
}

#[cfg(CONFIG_PPC32)]
unsafe extern "C" {
    pub fn setup_power_save();
}

#[cfg(not(CONFIG_PPC32))]
#[inline]
pub fn setup_power_save() {}

#[cfg(all(CONFIG_PPC64, CONFIG_SMP))]
unsafe extern "C" {
    pub fn check_smt_enabled();
}

#[cfg(not(all(CONFIG_PPC64, CONFIG_SMP)))]
#[inline]
pub fn check_smt_enabled() {}

#[cfg(all(CONFIG_PPC_BOOK3E_64, CONFIG_SMP))]
unsafe extern "C" {
    pub fn setup_tlb_core_data();
}

#[cfg(not(all(CONFIG_PPC_BOOK3E_64, CONFIG_SMP)))]
#[inline]
pub fn setup_tlb_core_data() {}

#[cfg(CONFIG_BOOKE)]
unsafe extern "C" {
    pub fn exc_lvl_early_init();
}

#[cfg(not(CONFIG_BOOKE))]
#[inline]
pub fn exc_lvl_early_init() {}

#[cfg(any(CONFIG_PPC64, CONFIG_VMAP_STACK))]
unsafe extern "C" {
    pub fn emergency_stack_init();
}

#[cfg(not(any(CONFIG_PPC64, CONFIG_VMAP_STACK)))]
#[inline]
pub fn emergency_stack_init() {}

#[cfg(CONFIG_PPC64)]
unsafe extern "C" {
    pub fn ppc64_bolted_size() -> u64;

    /* Default SPR values from firmware/kexec */
    pub static mut spr_default_dscr: c_ulong;
}

/*
 * Having this in kvm_ppc.h makes include dependencies too
 * tricky to solve for setup-common.c so have it here.
 */
#[cfg(CONFIG_KVM_BOOK3S_HV_POSSIBLE)]
unsafe extern "C" {
    pub fn kvm_cma_reserve();
}

#[cfg(not(CONFIG_KVM_BOOK3S_HV_POSSIBLE))]
#[inline]
pub fn kvm_cma_reserve() {}

#[cfg(CONFIG_TAU)]
unsafe extern "C" {
    pub fn cpu_temp(cpu: c_ulong) -> u32;
    pub fn cpu_temp_both(cpu: c_ulong) -> u32;
    pub fn tau_interrupts(cpu: c_ulong) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
