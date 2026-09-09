/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: <asm/msr.h>, <asm/cpuid/api.h>

#[repr(C)]
pub struct cpu_signature {
    pub sig: core::ffi::c_uint,
    pub pf: core::ffi::c_uint,
    pub rev: core::ffi::c_uint,
}

#[repr(C)]
pub struct ucode_cpu_info {
    pub cpu_sig: cpu_signature,
    pub mc: *mut core::ffi::c_void,
}

// CONFIG_MICROCODE selects the external implementations in the C header.
#[cfg(feature = "config_microcode")]
extern "C" {
    pub fn load_ucode_bsp();
    pub fn load_ucode_ap();
    pub fn microcode_bsp_resume();
    pub fn microcode_loader_disabled() -> bool;
}

#[cfg(not(feature = "config_microcode"))]
#[inline]
pub fn load_ucode_bsp() {}

#[cfg(not(feature = "config_microcode"))]
#[inline]
pub fn load_ucode_ap() {}

#[cfg(not(feature = "config_microcode"))]
#[inline]
pub fn microcode_bsp_resume() {}

#[cfg(not(feature = "config_microcode"))]
#[inline]
pub fn microcode_loader_disabled() -> bool { false }

extern "C" {
    pub static mut initrd_start_early: core::ffi::c_ulong;
}

// CONFIG_CPU_SUP_INTEL: Intel-specific microcode definitions, public for IFS.
#[cfg(feature = "config_cpu_sup_intel")]
#[repr(C)]
pub struct microcode_header_intel {
    pub hdrver: core::ffi::c_uint,
    pub rev: core::ffi::c_uint,
    pub date: core::ffi::c_uint,
    pub sig: core::ffi::c_uint,
    pub cksum: core::ffi::c_uint,
    pub ldrver: core::ffi::c_uint,
    pub pf: core::ffi::c_uint,
    pub datasize: core::ffi::c_uint,
    pub totalsize: core::ffi::c_uint,
    pub metasize: core::ffi::c_uint,
    pub min_req_ver: core::ffi::c_uint,
    pub reserved: core::ffi::c_uint,
}

#[cfg(feature = "config_cpu_sup_intel")]
#[repr(C)]
pub struct microcode_intel {
    pub hdr: microcode_header_intel,
    pub bits: [core::ffi::c_uint; 0],
}

#[cfg(feature = "config_cpu_sup_intel")]
pub const DEFAULT_UCODE_DATASIZE: core::ffi::c_uint = 2000;
#[cfg(feature = "config_cpu_sup_intel")]
pub const MC_HEADER_SIZE: usize = core::mem::size_of::<microcode_header_intel>();
#[cfg(feature = "config_cpu_sup_intel")]
pub const MC_HEADER_TYPE_MICROCODE: core::ffi::c_uint = 1;
#[cfg(feature = "config_cpu_sup_intel")]
pub const MC_HEADER_TYPE_IFS: core::ffi::c_uint = 2;

#[cfg(feature = "config_cpu_sup_intel")]
#[inline]
pub unsafe fn intel_microcode_get_datasize(hdr: *mut microcode_header_intel) -> core::ffi::c_int {
    if (*hdr).datasize != 0 {
        (*hdr).datasize as core::ffi::c_int
    } else {
        DEFAULT_UCODE_DATASIZE as core::ffi::c_int
    }
}

#[cfg(feature = "config_cpu_sup_intel")]
extern "C" {
    pub fn native_wrmsrq(msr: core::ffi::c_uint, value: u64);
    pub fn native_cpuid_eax(eax: core::ffi::c_uint) -> u32;
    pub fn native_rdmsr(msr: core::ffi::c_uint, low: *mut u32, high: *mut u32);
}

#[cfg(feature = "config_cpu_sup_intel")]
#[inline]
pub unsafe fn intel_get_microcode_revision() -> u32 {
    let mut rev: u32 = 0;
    let mut dummy: u32 = 0;

    native_wrmsrq(MSR_IA32_UCODE_REV, 0);

    /* As documented in the SDM: Do a CPUID 1 here */
    native_cpuid_eax(1);

    /* get the current revision from MSR 0x8B */
    native_rdmsr(MSR_IA32_UCODE_REV, &mut dummy, &mut rev);

    rev
}

extern "C" {
    pub fn microcode_nmi_handler() -> bool;
    pub fn microcode_offline_nmi_handler();
}

// CONFIG_MICROCODE_LATE_LOADING controls the static-key-backed implementation.
#[cfg(feature = "config_microcode_late_loading")]
#[inline]
pub unsafe fn microcode_nmi_handler_enabled() -> bool {
    static_branch_unlikely(&microcode_nmi_handler_enable)
}

#[cfg(not(feature = "config_microcode_late_loading"))]
#[inline]
pub fn microcode_nmi_handler_enabled() -> bool { false }

// External symbols supplied by the static-key and MSR/CPUID implementations.
#[cfg(feature = "config_microcode_late_loading")]
extern "C" {
    pub static microcode_nmi_handler_enable: StaticKeyFalse;
    pub fn static_branch_unlikely(key: *const StaticKeyFalse) -> bool;
}

#[cfg(feature = "config_microcode_late_loading")]
#[repr(C)]
pub struct StaticKeyFalse {
    _private: [u8; 0],
}

#[cfg(feature = "config_cpu_sup_intel")]
const MSR_IA32_UCODE_REV: core::ffi::c_uint = 0x8b;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
