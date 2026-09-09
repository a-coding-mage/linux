/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent from <asm/cpufeatures.h> and <asm/processor-flags.h>.

#[repr(C)]
pub struct cpu_features {
    pub level: ::core::ffi::c_int, // Family, or 64 for x86-64
    pub family: ::core::ffi::c_int, // Family, always
    pub model: ::core::ffi::c_int,
    pub flags: [u32; NCAPINTS],
}

extern "C" {
    pub static mut cpu: cpu_features;
    pub static mut cpu_vendor: [u32; 3];
}

// CONFIG_X86_32 conditionally selects the external implementation.
// When CONFIG_X86_32 is enabled, provide the declaration supplied elsewhere.
#[cfg(CONFIG_X86_32)]
extern "C" {
    pub fn has_eflag(mask: ::core::ffi::c_ulong) -> bool;
}

// On non-32-bit x86, the original header defines this inline function to
// always return true.
#[cfg(not(CONFIG_X86_32))]
#[inline]
pub fn has_eflag(_mask: ::core::ffi::c_ulong) -> bool {
    true
}

extern "C" {
    pub fn get_cpuflags();
    pub fn cpuid_count(
        id: u32,
        count: u32,
        a: *mut u32,
        b: *mut u32,
        c: *mut u32,
        d: *mut u32,
    );
    pub fn has_cpuflag(flag: ::core::ffi::c_int) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
