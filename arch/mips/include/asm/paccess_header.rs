/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive.
 *
 * Protected memory access. Used for accesses which may raise a DBE error.
 */

// The C header includes <linux/errno.h>.

#[cfg(CONFIG_32BIT)]
pub const __PA_ADDR: &str = ".word";
#[cfg(CONFIG_64BIT)]
pub const __PA_ADDR: &str = ".dword";

extern "C" {
    pub fn handle_ibe();
    pub fn handle_dbe();
}

#[repr(C)]
pub struct __large_pstruct {
    pub buf: [::core::ffi::c_ulong; 100],
}

#[inline]
pub unsafe fn __mp(x: *mut ::core::ffi::c_void) -> *mut __large_pstruct {
    x as *mut __large_pstruct
}

extern "C" {
    pub fn __get_dbe_unknown();
    pub fn __put_dbe_unknown();
    pub fn search_dbe_table(addr: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
}

// The original uses MIPS inline assembly and exception-table fixups. Rust's
// inline-assembly interface cannot express this GCC statement-expression
// machinery portably; these declarations preserve the required low-level
// operation and external interface for a target-specific implementation.
#[inline]
pub unsafe fn __get_dbe_asm<T>(
    _insn: &str,
    _err: *mut ::core::ffi::c_long,
    _val: *mut T,
    _addr: ::core::ffi::c_ulong,
) {
    // Corresponds to lb/lh/lw/ld plus .fixup and __dbe_table entries.
    unimplemented!()
}

#[inline]
pub unsafe fn __put_dbe_asm<T>(
    _insn: &str,
    _err: *mut ::core::ffi::c_long,
    _val: T,
    _addr: ::core::ffi::c_long,
) {
    // Corresponds to sb/sh/sw/sd plus .fixup and __dbe_table entries.
    unimplemented!()
}

// C sizeof(*(ptr)) is supplied explicitly by the Rust caller.
#[inline]
pub unsafe fn __get_dbe<T: Copy>(x: &mut T, ptr: *const T, size: usize) -> ::core::ffi::c_long {
    let mut gu_err: ::core::ffi::c_long = 0;
    let mut gu_val: T = core::mem::zeroed();
    let gu_addr = ptr as ::core::ffi::c_ulong;
    match size {
        1 => __get_dbe_asm("lb", &mut gu_err, &mut gu_val, gu_addr),
        2 => __get_dbe_asm("lh", &mut gu_err, &mut gu_val, gu_addr),
        4 => __get_dbe_asm("lw", &mut gu_err, &mut gu_val, gu_addr),
        8 => __get_dbe_asm("ld", &mut gu_err, &mut gu_val, gu_addr),
        _ => __get_dbe_unknown(),
    }
    *x = gu_val;
    gu_err
}

#[inline]
pub unsafe fn __put_dbe<T: Copy>(x: T, ptr: *mut T, size: usize) -> ::core::ffi::c_long {
    let mut pu_err: ::core::ffi::c_long = 0;
    let pu_addr = ptr as ::core::ffi::c_long;
    match size {
        1 => __put_dbe_asm("sb", &mut pu_err, x, pu_addr),
        2 => __put_dbe_asm("sh", &mut pu_err, x, pu_addr),
        4 => __put_dbe_asm("sw", &mut pu_err, x, pu_addr),
        8 => __put_dbe_asm("sd", &mut pu_err, x, pu_addr),
        _ => __put_dbe_unknown(),
    }
    pu_err
}

#[inline]
pub unsafe fn put_dbe<T: Copy>(x: T, ptr: *mut T) -> ::core::ffi::c_long {
    __put_dbe(x, ptr, core::mem::size_of::<T>())
}

#[inline]
pub unsafe fn get_dbe<T: Copy>(x: &mut T, ptr: *const T) -> ::core::ffi::c_long {
    __get_dbe(x, ptr, core::mem::size_of::<T>())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
