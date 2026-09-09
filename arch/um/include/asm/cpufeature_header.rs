/* SPDX-License-Identifier: GPL-2.0 */

// C dependency: <asm/processor.h>
// The following items are conditional on __KERNEL__ && !__ASSEMBLER__ in C.

// C dependency: <asm/asm.h>, <linux/bitops.h>

extern "C" {
    pub static x86_cap_flags: *const *const core::ffi::c_char;
    pub static x86_power_flags: *const *const core::ffi::c_char;
    pub static x86_bug_flags: *const *const core::ffi::c_char;
    pub fn setup_clear_cpu_cap(bit: core::ffi::c_uint);
}

pub const X86_CAP_FMT: &str = "%s";

#[macro_export]
macro_rules! x86_cap_flag {
    ($flag:expr) => {{ unsafe { *$crate::x86_cap_flags.add($flag as usize) } }};
}

#[macro_export]
macro_rules! test_cpu_cap {
    ($c:expr, $bit:expr) => {{
        // C equivalent: test_bit(bit, (unsigned long *)((c)->x86_capability))
        unsafe { $crate::test_bit($bit, (*$c).x86_capability.as_ptr() as *mut _) }
    }};
}

#[macro_export]
macro_rules! CHECK_BIT_IN_MASK_WORD {
    ($maskname:ident, $word:expr, $bit:expr) => {
        ((($bit) >> 5) == ($word)
            && ((1usize << (($bit) & 31)) & $maskname##$word) != 0)
    };
}

#[macro_export]
macro_rules! cpu_has {
    ($c:expr, $bit:expr) => { test_cpu_cap!($c, $bit) };
}

#[macro_export]
macro_rules! this_cpu_has {
    ($bit:expr) => {{
        if REQUIRED_MASK_BIT_SET!($bit) {
            1
        } else {
            unsafe { $crate::x86_this_cpu_test_bit($bit, $crate::cpu_info.x86_capability) }
        }
    }};
}

#[macro_export]
macro_rules! cpu_feature_enabled {
    ($bit:expr) => {{
        if DISABLED_MASK_BIT_SET!($bit) { 0 } else { _static_cpu_has!($bit) }
    }};
}

#[macro_export]
macro_rules! boot_cpu_has {
    ($bit:expr) => { cpu_has!(&raw const $crate::boot_cpu_data, $bit) };
}

#[macro_export]
macro_rules! set_cpu_cap {
    ($c:expr, $bit:expr) => {{
        unsafe { $crate::set_bit($bit, (*$c).x86_capability.as_ptr() as *mut _) }
    }};
}

#[macro_export]
macro_rules! setup_force_cpu_cap {
    ($bit:expr) => {{
        set_cpu_cap!(&raw const $crate::boot_cpu_data, $bit);
        unsafe { $crate::set_bit($bit, $crate::cpu_caps_set as *mut _) };
    }};
}

#[macro_export]
macro_rules! setup_force_cpu_bug {
    ($bit:expr) => { setup_force_cpu_cap!($bit) };
}

// The C implementation uses architecture-specific alternative-instruction
// inline assembly and labels.  No direct file-local Rust equivalent exists.
#[inline(always)]
pub unsafe fn __static_cpu_has(bit: u16) -> bool {
    // TODO: provide the target architecture's alternative-instruction implementation.
    let _ = bit;
    false
}

#[macro_export]
macro_rules! _static_cpu_has {
    ($bit:expr) => {{
        if boot_cpu_has!($bit) { true } else { unsafe { $crate::__static_cpu_has($bit as u16) } }
    }};
}

#[macro_export]
macro_rules! cpu_has_bug {
    ($c:expr, $bit:expr) => { cpu_has!($c, $bit) };
}
#[macro_export]
macro_rules! set_cpu_bug {
    ($c:expr, $bit:expr) => { set_cpu_cap!($c, $bit) };
}
#[macro_export]
macro_rules! static_cpu_has_bug {
    ($bit:expr) => { _static_cpu_has!($bit) };
}
#[macro_export]
macro_rules! boot_cpu_has_bug {
    ($bit:expr) => { cpu_has_bug!(&raw const $crate::boot_cpu_data, $bit) };
}
#[macro_export]
macro_rules! boot_cpu_set_bug {
    ($bit:expr) => { set_cpu_cap!(&raw const $crate::boot_cpu_data, $bit) };
}

pub const MAX_CPU_FEATURES: usize = NCAPINTS * 32;

#[macro_export]
macro_rules! cpu_have_feature {
    ($bit:expr) => { boot_cpu_has!($bit) };
}

pub const CPU_FEATURE_TYPEFMT: &str = "x86,ven%04Xfam%04Xmod%04X";

// C equivalent: boot_cpu_data.x86_vendor, boot_cpu_data.x86,
// boot_cpu_data.x86_model
#[macro_export]
macro_rules! CPU_FEATURE_TYPEVAL {
    () => {{
        ($crate::boot_cpu_data.x86_vendor, $crate::boot_cpu_data.x86,
         $crate::boot_cpu_data.x86_model)
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
