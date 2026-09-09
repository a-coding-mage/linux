/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Atomic operations that C can't guarantee us.  Useful for
 * resource counting etc..
 *
 * The C header selects one of the architecture-specific implementations
 * according to the build configuration.  Those dependencies are supplied by
 * the surrounding translation unit.
 */

use core::ffi::c_void;

#[cfg(CONFIG_GUSA_RB)]
// Dependency: asm/cmpxchg-grb.h
#[cfg(all(not(CONFIG_GUSA_RB), CONFIG_CPU_SH4A))]
// Dependency: asm/cmpxchg-llsc.h
#[cfg(all(not(CONFIG_GUSA_RB), not(CONFIG_CPU_SH4A), CONFIG_CPU_J2, CONFIG_SMP))]
// Dependency: asm/cmpxchg-cas.h
#[cfg(all(
    not(CONFIG_GUSA_RB),
    not(CONFIG_CPU_SH4A),
    not(all(CONFIG_CPU_J2, CONFIG_SMP))
))]
// Dependency: asm/cmpxchg-irq.h

extern "C" {
    pub fn __xchg_called_with_bad_pointer() -> !;
    pub fn __cmpxchg_called_with_bad_pointer() -> !;
}

/* Supplied by the selected architecture-specific implementation. */
extern "C" {
    pub fn xchg_u32(ptr: *mut c_void, x: usize) -> usize;
    pub fn xchg_u16(ptr: *mut c_void, x: usize) -> usize;
    pub fn xchg_u8(ptr: *mut c_void, x: usize) -> usize;
    pub fn cmpxchg_emu_u8(ptr: *mut c_void, old: usize, new: usize) -> usize;
    pub fn __cmpxchg_u32(ptr: *mut c_void, old: usize, new: usize) -> usize;
    pub fn __generic_cmpxchg_local(
        ptr: *mut c_void,
        old: usize,
        new: usize,
        size: usize,
    ) -> usize;
}

pub unsafe fn __arch_xchg(ptr: *mut c_void, x: usize, size: usize) -> usize {
    let mut res: usize;
    let xchg_ptr = ptr as *mut c_void;

    match size {
        4 => {
            res = xchg_u32(xchg_ptr, x);
        }
        2 => {
            res = xchg_u16(xchg_ptr, x);
        }
        1 => {
            res = xchg_u8(xchg_ptr, x);
        }
        _ => {
            __xchg_called_with_bad_pointer();
            res = x;
        }
    }

    res
}

#[macro_export]
macro_rules! arch_xchg {
    ($ptr:expr, $x:expr) => {{
        let __xchg_value = $x;
        $crate::__arch_xchg(
            ($ptr) as *mut core::ffi::c_void,
            __xchg_value as usize,
            core::mem::size_of_val(unsafe { &*($ptr) }),
        ) as _
    }};
}

pub unsafe fn __cmpxchg(
    ptr: *mut c_void,
    old: usize,
    new: usize,
    size: i32,
) -> usize {
    match size {
        1 => cmpxchg_emu_u8(ptr, old, new),
        4 => __cmpxchg_u32(ptr, old, new),
        _ => {
            __cmpxchg_called_with_bad_pointer();
            old
        }
    }
}

#[macro_export]
macro_rules! arch_cmpxchg {
    ($ptr:expr, $o:expr, $n:expr) => {{
        let _o_ = $o;
        let _n_ = $n;
        $crate::__cmpxchg(
            ($ptr) as *mut core::ffi::c_void,
            _o_ as usize,
            _n_ as usize,
            core::mem::size_of_val(unsafe { &*($ptr) }) as i32,
        ) as _
    }};
}

/* Dependency: asm-generic/cmpxchg-local.h */
#[macro_export]
macro_rules! arch_cmpxchg_local {
    ($ptr:expr, $o:expr, $n:expr) => {{
        $crate::__generic_cmpxchg_local(
            ($ptr) as *mut core::ffi::c_void,
            ($o) as usize,
            ($n) as usize,
            core::mem::size_of_val(unsafe { &*($ptr) }),
        ) as _
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
