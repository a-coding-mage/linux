/* SPDX-License-Identifier: GPL-2.0 */

/* C includes and the __KERNEL__ && !__ASSEMBLER__ build condition are
 * intentionally represented by the external Rust dependencies used below. */

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum cpuid_leafs {
    CPUID_1_EDX = 0,
    CPUID_8000_0001_EDX,
    CPUID_8086_0001_EDX,
    CPUID_LNX_1,
    CPUID_1_ECX,
    CPUID_C000_0001_EDX,
    CPUID_8000_0001_ECX,
    CPUID_LNX_2,
    CPUID_LNX_3,
    CPUID_7_0_EBX,
    CPUID_D_1_EAX,
    CPUID_LNX_4,
    CPUID_7_1_EAX,
    CPUID_8000_0008_EBX,
    CPUID_6_EAX,
    CPUID_8000_000A_EDX,
    CPUID_7_ECX,
    CPUID_LNX_6,
    CPUID_7_EDX,
    CPUID_8000_001F_EAX,
    CPUID_8000_0021_EAX,
    CPUID_LNX_5,
    NR_CPUID_WORDS,
}

extern "C" {
    pub static x86_cap_flags: [*const core::ffi::c_char; NCAPINTS * 32];
    pub static x86_power_flags: [*const core::ffi::c_char; 32];
    pub static x86_bug_flags: [*const core::ffi::c_char; NBUGINTS * 32];

    pub fn setup_clear_cpu_cap(bit: u32);
    pub fn clear_cpu_cap(c: *mut cpuinfo_x86, bit: u32);
    pub fn check_cpufeature_deps(c: *mut cpuinfo_x86);
}

/* In order to save room, index this array by X86_BUG_<name> - NCAPINTS*32. */
#[inline(always)]
pub unsafe fn x86_bug_flag(flag: usize) -> *const core::ffi::c_char {
    x86_bug_flags[flag]
}

#[macro_export]
macro_rules! test_cpu_cap {
    ($c:expr, $bit:expr) => {
        unsafe { arch_test_bit($bit, (*$c).x86_capability.as_ptr() as *mut usize) }
    };
}

#[macro_export]
macro_rules! cpu_has {
    ($c:expr, $bit:expr) => {
        if REQUIRED_MASK_BIT_SET!($bit) { 1 } else { test_cpu_cap!($c, $bit) }
    };
}

#[macro_export]
macro_rules! this_cpu_has {
    ($bit:expr) => {
        if REQUIRED_MASK_BIT_SET!($bit) { 1 } else { x86_this_cpu_test_bit($bit, cpu_info.x86_capability) }
    };
}

/* Default CPU feature testing uses kernel infrastructure and may not directly
 * test the CPU itself. */
#[macro_export]
macro_rules! cpu_feature_enabled {
    ($bit:expr) => {
        if DISABLED_MASK_BIT_SET!($bit) { 0 } else { _static_cpu_has($bit) }
    };
}

#[macro_export]
macro_rules! boot_cpu_has { ($bit:expr) => { cpu_has!(&boot_cpu_data, $bit) }; }

#[macro_export]
macro_rules! set_cpu_cap {
    ($c:expr, $bit:expr) => {
        unsafe { set_bit($bit, (*$c).x86_capability.as_ptr() as *mut usize) }
    };
}

#[macro_export]
macro_rules! setup_force_cpu_cap {
    ($bit:expr) => {{
        if boot_cpu_has!($bit) == 0 { unsafe { WARN_ON(alternatives_patched); } }
        set_cpu_cap!(&boot_cpu_data, $bit);
        unsafe { set_bit($bit, cpu_caps_set as *mut usize); }
    }};
}

#[macro_export]
macro_rules! setup_force_cpu_bug { ($bit:expr) => { setup_force_cpu_cap!($bit) }; }

/* The original uses GCC asm-goto and ALTERNATIVE_TERNARY.  The fallback path
 * below preserves the observable feature-bit test; patching is external. */
#[inline(always)]
pub unsafe fn __static_cpu_has(bit: u16) -> bool {
    let byte = *((boot_cpu_data.x86_capability.as_ptr() as *const u8).add((bit >> 3) as usize));
    (byte & (1u8 << (bit & 7))) != 0
}

#[inline(always)]
pub unsafe fn _static_cpu_has(bit: u16) -> bool {
    if boot_cpu_has!(bit) != 0 { true } else { __static_cpu_has(bit) }
}

#[macro_export]
macro_rules! cpu_has_bug { ($c:expr, $bit:expr) => { cpu_has!($c, $bit) }; }
#[macro_export]
macro_rules! set_cpu_bug { ($c:expr, $bit:expr) => { set_cpu_cap!($c, $bit) }; }
#[macro_export]
macro_rules! static_cpu_has_bug { ($bit:expr) => { _static_cpu_has($bit) }; }
#[macro_export]
macro_rules! boot_cpu_has_bug { ($bit:expr) => { cpu_has_bug!(&boot_cpu_data, $bit) }; }
#[macro_export]
macro_rules! boot_cpu_set_bug { ($bit:expr) => { set_cpu_cap!(&boot_cpu_data, $bit) }; }

pub const MAX_CPU_FEATURES: usize = NCAPINTS * 32;
/* C alias: cpu_have_feature boot_cpu_has */
pub const CPU_FEATURE_TYPEFMT: &str = "x86,ven%04Xfam%04Xmod%04X";

#[macro_export]
macro_rules! cpu_have_feature { ($bit:expr) => { boot_cpu_has!($bit) }; }

#[macro_export]
macro_rules! CPU_FEATURE_TYPEVAL {
    () => { (boot_cpu_data.x86_vendor, boot_cpu_data.x86, boot_cpu_data.x86_model) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
