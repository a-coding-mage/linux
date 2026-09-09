/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Security related feature bit definitions.
 *
 * Copyright 2018, Michael Ellerman, IBM Corporation.
 */

unsafe extern "C" {
    pub static mut powerpc_security_features: u64;
    pub static mut rfi_flush: bool;
}

/* These are bit flags */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stf_barrier_type {
    STF_BARRIER_NONE = 0x1,
    STF_BARRIER_FALLBACK = 0x2,
    STF_BARRIER_EIEIO = 0x4,
    STF_BARRIER_SYNC_ORI = 0x8,
}

unsafe extern "C" {
    pub fn setup_stf_barrier();
    pub fn do_stf_barrier_fixups(types: stf_barrier_type);
    pub fn setup_count_cache_flush();
}

#[inline]
pub unsafe fn security_ftr_set(feature: u64) {
    powerpc_security_features |= feature;
}

#[inline]
pub unsafe fn security_ftr_clear(feature: u64) {
    powerpc_security_features &= !feature;
}

#[inline]
pub unsafe fn security_ftr_enabled(feature: u64) -> bool {
    (powerpc_security_features & feature) != 0
}

#[cfg(CONFIG_PPC_BOOK3S_64)]
unsafe extern "C" {
    pub fn stf_barrier_type_get() -> stf_barrier_type;
}

#[cfg(not(CONFIG_PPC_BOOK3S_64))]
#[inline]
pub fn stf_barrier_type_get() -> stf_barrier_type {
    stf_barrier_type::STF_BARRIER_NONE
}

// Features indicating support for Spectre/Meltdown mitigations

// The L1-D cache can be flushed with ori r30,r30,0
pub const SEC_FTR_L1D_FLUSH_ORI30: u64 = 0x0000000000000001;

// The L1-D cache can be flushed with mtspr 882,r0 (aka SPRN_TRIG2)
pub const SEC_FTR_L1D_FLUSH_TRIG2: u64 = 0x0000000000000002;

// ori r31,r31,0 acts as a speculation barrier
pub const SEC_FTR_SPEC_BAR_ORI31: u64 = 0x0000000000000004;

// Speculation past bctr is disabled
pub const SEC_FTR_BCCTRL_SERIALISED: u64 = 0x0000000000000008;

// Entries in L1-D are private to a SMT thread
pub const SEC_FTR_L1D_THREAD_PRIV: u64 = 0x0000000000000010;

// Indirect branch prediction cache disabled
pub const SEC_FTR_COUNT_CACHE_DISABLED: u64 = 0x0000000000000020;

// bcctr 2,0,0 triggers a hardware assisted count cache flush
pub const SEC_FTR_BCCTR_FLUSH_ASSIST: u64 = 0x0000000000000800;

// bcctr 2,0,0 triggers a hardware assisted link stack flush
pub const SEC_FTR_BCCTR_LINK_FLUSH_ASSIST: u64 = 0x0000000000002000;

// Features indicating need for Spectre/Meltdown mitigations

// The L1-D cache should be flushed on MSR[HV] 1->0 transition (hypervisor to guest)
pub const SEC_FTR_L1D_FLUSH_HV: u64 = 0x0000000000000040;

// The L1-D cache should be flushed on MSR[PR] 0->1 transition (kernel to userspace)
pub const SEC_FTR_L1D_FLUSH_PR: u64 = 0x0000000000000080;

// A speculation barrier should be used for bounds checks (Spectre variant 1)
pub const SEC_FTR_BNDS_CHK_SPEC_BAR: u64 = 0x0000000000000100;

// Firmware configuration indicates user favours security over performance
pub const SEC_FTR_FAVOUR_SECURITY: u64 = 0x0000000000000200;

// Software required to flush count cache on context switch
pub const SEC_FTR_FLUSH_COUNT_CACHE: u64 = 0x0000000000000400;

// Software required to flush link stack on context switch
pub const SEC_FTR_FLUSH_LINK_STACK: u64 = 0x0000000000001000;

// The L1-D cache should be flushed when entering the kernel
pub const SEC_FTR_L1D_FLUSH_ENTRY: u64 = 0x0000000000004000;

// The L1-D cache should be flushed after user accesses from the kernel
pub const SEC_FTR_L1D_FLUSH_UACCESS: u64 = 0x0000000000008000;

// The STF flush should be executed on privilege state switch
pub const SEC_FTR_STF_BARRIER: u64 = 0x0000000000010000;

// Features enabled by default
pub const SEC_FTR_DEFAULT: u64 = SEC_FTR_L1D_FLUSH_HV
    | SEC_FTR_L1D_FLUSH_PR
    | SEC_FTR_BNDS_CHK_SPEC_BAR
    | SEC_FTR_L1D_FLUSH_ENTRY
    | SEC_FTR_L1D_FLUSH_UACCESS
    | SEC_FTR_STF_BARRIER
    | SEC_FTR_FAVOUR_SECURITY;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
