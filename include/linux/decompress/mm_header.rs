/* SPDX-License-Identifier: GPL-2.0 */
/*
 * linux/compr_mm.h
 *
 * Memory management for pre-boot and ramdisk uncompressors
 *
 * Authors: Alain Knaff <alain@knaff.lu>
 */

/* The C header guard is unnecessary in Rust. */

/* Code active when included from the pre-boot environment (STATIC). */
#[cfg(feature = "STATIC")]
mod static_environment {
    use core::ffi::c_void;

    extern "C" {
        static mut free_mem_ptr: core::ffi::c_ulong;
        static mut free_mem_end_ptr: core::ffi::c_ulong;
    }

    /*
     * Some architectures want to ensure there is no local data in their
     * pre-boot environment, so that data can arbitrarily relocated (via
     * GOT references). This is achieved by defining STATIC_RW_DATA to be
     * null. Rust's linkage/storage visibility is supplied by the build.
     */
    #[no_mangle]
    pub static mut malloc_ptr: core::ffi::c_ulong = 0;
    #[no_mangle]
    pub static mut malloc_count: core::ffi::c_int = 0;

    /* A trivial malloc implementation, adapted from malloc by Hannu
     * Savolainen 1993 and Matthias Urlichs 1994. */
    #[no_mangle]
    pub unsafe extern "C" fn malloc(size: core::ffi::c_int) -> *mut c_void {
        let p: *mut c_void;

        if size < 0 {
            return core::ptr::null_mut();
        }
        if malloc_ptr == 0 {
            malloc_ptr = free_mem_ptr;
        }

        malloc_ptr = (malloc_ptr.wrapping_add(7)) & !7; /* Align */

        p = malloc_ptr as *mut c_void;
        malloc_ptr = malloc_ptr.wrapping_add(size as core::ffi::c_ulong);

        if free_mem_end_ptr != 0 && malloc_ptr >= free_mem_end_ptr {
            return core::ptr::null_mut();
        }

        malloc_count += 1;
        p
    }

    #[no_mangle]
    pub unsafe extern "C" fn free(_where: *mut c_void) {
        malloc_count -= 1;
        if malloc_count == 0 {
            malloc_ptr = free_mem_ptr;
        }
    }

    #[macro_export]
    macro_rules! large_malloc {
        ($a:expr) => { $crate::malloc($a) };
    }

    #[macro_export]
    macro_rules! large_free {
        ($a:expr) => { $crate::free($a) };
    }

    /* #define INIT */
}

/* Code active when compiled standalone for use when loading ramdisk. */
#[cfg(not(feature = "STATIC"))]
extern "C" {
    fn kmalloc(size: usize, flags: core::ffi::c_uint) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn vmalloc(size: usize) -> *mut core::ffi::c_void;
    fn vfree(ptr: *mut core::ffi::c_void);
}

#[cfg(not(feature = "STATIC"))]
pub const GFP_KERNEL: core::ffi::c_uint = 0; /* supplied by linux/kernel.h */

#[cfg(not(feature = "STATIC"))]
#[macro_export]
macro_rules! malloc {
    ($a:expr) => { unsafe { $crate::kmalloc($a, $crate::GFP_KERNEL) } };
}

#[cfg(not(feature = "STATIC"))]
#[macro_export]
macro_rules! free {
    ($a:expr) => { unsafe { $crate::kfree($a) } };
}

#[cfg(not(feature = "STATIC"))]
#[macro_export]
macro_rules! large_malloc {
    ($a:expr) => { unsafe { $crate::vmalloc($a) } };
}

#[cfg(not(feature = "STATIC"))]
#[macro_export]
macro_rules! large_free {
    ($a:expr) => { unsafe { $crate::vfree($a) } };
}

/* #define INIT __init; #define STATIC */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
