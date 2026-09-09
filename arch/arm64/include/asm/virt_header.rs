/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 ARM Ltd.
 * Author: Marc Zyngier <marc.zyngier@arm.com>
 */

/* The arm64 hcall implementation uses x0 to specify the hcall number. */

pub const HVC_SET_VECTORS: u32 = 0;
pub const HVC_SOFT_RESTART: u32 = 1;
pub const HVC_RESET_VECTORS: u32 = 2;
pub const HVC_FINALISE_EL2: u32 = 3;
pub const HVC_GET_ICH_VTR_EL2: u32 = 4;
pub const HVC_STUB_HCALL_NR: u32 = 5;
pub const HVC_STUB_ERR: u32 = 0x0badca11;

pub const BOOT_CPU_MODE_EL1: u32 = 0xe11;
pub const BOOT_CPU_MODE_EL2: u32 = 0xe12;
pub const BOOT_CPU_FLAG_E2H: u64 = 1u64 << 32;

/* External types and symbols are supplied by the corresponding kernel dependencies. */
extern "C" {
    pub static mut __boot_cpu_mode: [u32; 2];
    pub fn __hyp_set_vectors(phys_vector_base: phys_addr_t);
    pub fn __hyp_reset_vectors();
    pub fn is_kvm_arm_initialised() -> bool;
}

pub const ARM64_VECTOR_TABLE_LEN: usize = 2048;

extern "C" {
    pub static kvm_protected_mode_initialized: StaticKey;
}

#[inline]
pub fn is_pkvm_initialized() -> bool {
    unsafe { IS_ENABLED(CONFIG_KVM) && static_branch_likely(&kvm_protected_mode_initialized) }
}

/* CONFIG_KVM selects the external implementation; otherwise this is the stub. */
#[cfg(CONFIG_KVM)]
extern "C" {
    pub fn pkvm_force_reclaim_guest_page(phys: phys_addr_t) -> bool;
}

#[cfg(not(CONFIG_KVM))]
#[inline]
pub fn pkvm_force_reclaim_guest_page(_phys: phys_addr_t) -> bool {
    false
}

/* Reports the availability of HYP mode. */
#[inline]
pub fn is_hyp_mode_available() -> bool {
    /* If protected mode is initialized, CPUs now come up in EL1. */
    if is_pkvm_initialized() {
        return true;
    }

    unsafe {
        __boot_cpu_mode[0] == BOOT_CPU_MODE_EL2 &&
            __boot_cpu_mode[1] == BOOT_CPU_MODE_EL2
    }
}

/* Check if the bootloader has booted CPUs in different modes. */
#[inline]
pub fn is_hyp_mode_mismatched() -> bool {
    if is_pkvm_initialized() {
        return false;
    }

    unsafe { __boot_cpu_mode[0] != __boot_cpu_mode[1] }
}

#[inline(always)]
pub fn is_kernel_in_hyp_mode() -> bool {
    BUILD_BUG_ON(__is_defined(__KVM_NVHE_HYPERVISOR__) ||
                 __is_defined(__KVM_VHE_HYPERVISOR__));
    read_sysreg(CurrentEL) == CurrentEL_EL2
}

#[inline(always)]
pub fn has_vhe() -> bool {
    if is_vhe_hyp_code() {
        true
    } else if is_nvhe_hyp_code() {
        false
    } else {
        cpus_have_final_cap(ARM64_HAS_VIRT_HOST_EXTN)
    }
}

#[inline(always)]
pub fn is_protected_kvm_enabled() -> bool {
    if is_vhe_hyp_code() {
        false
    } else {
        cpus_have_final_cap(ARM64_KVM_PROTECTED_MODE)
    }
}

#[inline(always)]
pub fn has_hvhe() -> bool {
    if is_vhe_hyp_code() {
        return false;
    }

    cpus_have_final_cap(ARM64_KVM_HVHE)
}

#[inline]
pub fn is_hyp_nvhe() -> bool {
    is_hyp_mode_available() && !is_kernel_in_hyp_mode()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
