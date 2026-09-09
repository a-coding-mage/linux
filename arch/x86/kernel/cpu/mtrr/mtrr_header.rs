/* SPDX-License-Identifier: GPL-2.0 */
/*
 * local MTRR defines.
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/types.h, linux/stddef.h

pub const MTRR_CHANGE_MASK_FIXED: u32 = 0x01;
pub const MTRR_CHANGE_MASK_VARIABLE: u32 = 0x02;
pub const MTRR_CHANGE_MASK_DEFTYPE: u32 = 0x04;

extern "C" {
    pub static mut mtrr_debug: bool;
}

// Equivalent to the C Dprintk(x...) macro; `pr_info!` is supplied externally.
#[macro_export]
macro_rules! Dprintk {
    ($($arg:tt)*) => {
        unsafe {
            if $crate::mtrr_debug {
                pr_info!($($arg)*);
            }
        }
    };
}

extern "C" {
    pub static mut mtrr_usage_table: [u32; MTRR_MAX_VAR_RANGES as usize];
}

#[repr(C)]
pub struct mtrr_ops {
    pub var_regs: u32,
    pub set: Option<unsafe extern "C" fn(
        reg: u32,
        base: usize,
        size: usize,
        type_: mtrr_type,
    )>,
    pub get: Option<unsafe extern "C" fn(
        reg: u32,
        base: *mut usize,
        size: *mut usize,
        type_: *mut mtrr_type,
    )>,
    pub get_free_region: Option<unsafe extern "C" fn(base: usize, size: usize, replace_reg: i32) -> i32>,
    pub validate_add_page: Option<unsafe extern "C" fn(base: usize, size: usize, type_: u32) -> i32>,
    pub have_wrcomb: Option<unsafe extern "C" fn() -> i32>,
}

extern "C" {
    pub fn generic_get_free_region(base: usize, size: usize, replace_reg: i32) -> i32;
    pub fn generic_validate_add_page(base: usize, size: usize, type_: u32) -> i32;

    pub static generic_mtrr_ops: mtrr_ops;

    pub fn positive_have_wrcomb() -> i32;
}

/* library functions for processor-specific routines */
#[repr(C)]
pub struct set_mtrr_context {
    pub flags: usize,
    pub cr4val: usize,
    pub deftype_lo: u32,
    pub deftype_hi: u32,
    pub ccr3: u32,
}

extern "C" {
    pub fn fill_mtrr_var_range(
        index: u32,
        base_lo: u32,
        base_hi: u32,
        mask_lo: u32,
        mask_hi: u32,
    );
    pub fn get_mtrr_state() -> bool;

    pub static mut mtrr_if: *const mtrr_ops;
    pub static mut mtrr_mutex: mutex;

    pub static mut num_var_ranges: u32;
    pub static mut mtrr_tom2: u64;
    pub static mut mtrr_state: mtrr_state_type;
    pub static mut phys_hi_rsvd: u32;

    pub fn mtrr_state_warn();
    pub fn mtrr_attrib_to_str(x: i32) -> *const core::ffi::c_char;
    pub fn mtrr_wrmsr(a: u32, b: u32, c: u32);
}

#[cfg(target_arch = "x86")]
extern "C" {
    pub fn mtrr_set_if();
    pub fn mtrr_register_syscore();
}

#[cfg(not(target_arch = "x86"))]
#[inline]
pub fn mtrr_set_if() {}

#[cfg(not(target_arch = "x86"))]
#[inline]
pub fn mtrr_register_syscore() {}

extern "C" {
    pub fn mtrr_build_map();
    pub fn mtrr_copy_map();

    /* CPU specific mtrr_ops vectors. */
    pub static amd_mtrr_ops: mtrr_ops;
    pub static cyrix_mtrr_ops: mtrr_ops;
    pub static centaur_mtrr_ops: mtrr_ops;

    pub static mut changed_by_mtrr_cleanup: i32;
    pub fn mtrr_cleanup() -> i32;
}

/*
 * Must be used by code which uses mtrr_if to call platform-specific
 * MTRR manipulation functions.
 */
#[inline]
pub unsafe fn mtrr_enabled() -> bool {
    !mtrr_if.is_null()
}

extern "C" {
    pub fn generic_rebuild_map();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
