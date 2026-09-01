/* SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause) */

// C dependencies removed from executable Rust:
// #include <stdlib.h>
// #include <linux/zalloc.h>

/*
 * Enable reference count checking implicitly with leak checking, which is
 * integrated into address sanitizer.
 *
 * C condition:
 * defined(__SANITIZE_ADDRESS__) || defined(LEAK_SANITIZER) ||
 * defined(ADDRESS_SANITIZER) || __has_feature(address_sanitizer) ||
 * __has_feature(leak_sanitizer)
 *
 * In Rust this file preserves the same two code paths behind the cfg flag
 * `refcnt_checking`, which should be set by the build when the corresponding
 * sanitizer condition applies.
 */

/*
 * Shared reference count checking macros.
 *
 * Reference count checking is an approach to sanitizing the use of reference
 * counted structs. It leverages address and leak sanitizers to make sure gets
 * are paired with a put. Reference count checking adds a malloc-ed layer of
 * indirection on a get, and frees it on a put. A missed put will be reported as
 * a memory leak. A double put will be reported as a double free. Accessing
 * after a put will cause a use-after-free and/or a segfault.
 */

use core::ffi::c_void;

unsafe extern "C" {
    pub fn malloc(size: usize) -> *mut c_void;
    pub fn free(ptr: *mut c_void);
    pub fn zfree(ptr: *mut c_void);
}

#[cfg(not(refcnt_checking))]
#[macro_export]
macro_rules! DECLARE_RC_STRUCT {
    ($struct_name:item) => {
        $struct_name
    };
}

#[cfg(not(refcnt_checking))]
#[macro_export]
macro_rules! RC_STRUCT {
    ($struct_name:ty) => {
        $struct_name
    };
}

#[cfg(not(refcnt_checking))]
#[macro_export]
macro_rules! ADD_RC_CHK {
    ($result:expr, $object:expr) => {{
        $result = $object;
        $object
    }};
}

#[cfg(not(refcnt_checking))]
#[macro_export]
macro_rules! RC_CHK_ACCESS {
    ($object:expr) => {
        $object
    };
}

#[cfg(not(refcnt_checking))]
#[macro_export]
macro_rules! RC_CHK_FREE {
    ($object:expr) => {{
        unsafe {
            free($object as *mut c_void);
        }
    }};
}

#[cfg(not(refcnt_checking))]
#[macro_export]
macro_rules! RC_CHK_GET {
    ($result:expr, $object:expr) => {
        ADD_RC_CHK!($result, $object)
    };
}

#[cfg(not(refcnt_checking))]
#[macro_export]
macro_rules! RC_CHK_PUT {
    ($object:expr) => {{}};
}

#[cfg(not(refcnt_checking))]
#[macro_export]
macro_rules! RC_CHK_EQUAL {
    ($object1:expr, $object2:expr) => {
        $object1 == $object2
    };
}

#[cfg(refcnt_checking)]
#[repr(C)]
pub struct rc_check<T> {
    pub orig: *mut T,
}

#[cfg(refcnt_checking)]
#[macro_export]
macro_rules! DECLARE_RC_STRUCT {
    ($struct_name:item) => {
        $struct_name
    };
}

#[cfg(refcnt_checking)]
#[macro_export]
macro_rules! RC_STRUCT {
    ($struct_name:ty) => {
        $struct_name
    };
}

#[cfg(refcnt_checking)]
#[macro_export]
macro_rules! ADD_RC_CHK {
    ($result:expr, $object:expr) => {{
        if !$object.is_null() {
            $result = unsafe { malloc(core::mem::size_of_val(&*$result)) as _ };
            if !$result.is_null() {
                unsafe {
                    (*$result).orig = $object;
                }
                $result
            } else {
                $result = core::ptr::null_mut();
                core::ptr::null_mut()
            }
        } else {
            $result = core::ptr::null_mut();
            core::ptr::null_mut()
        }
    }};
}

#[cfg(refcnt_checking)]
#[macro_export]
macro_rules! RC_CHK_ACCESS {
    ($object:expr) => {
        unsafe { (*$object).orig }
    };
}

#[cfg(refcnt_checking)]
#[macro_export]
macro_rules! RC_CHK_FREE {
    ($object:expr) => {{
        unsafe {
            zfree(core::ptr::addr_of_mut!((*$object).orig) as *mut c_void);
            free($object as *mut c_void);
        }
    }};
}

#[cfg(refcnt_checking)]
#[macro_export]
macro_rules! RC_CHK_GET {
    ($result:expr, $object:expr) => {
        ADD_RC_CHK!(
            $result,
            if !$object.is_null() {
                unsafe { (*$object).orig }
            } else {
                core::ptr::null_mut()
            }
        )
    };
}

#[cfg(refcnt_checking)]
#[macro_export]
macro_rules! RC_CHK_PUT {
    ($object:expr) => {{
        if !$object.is_null() {
            unsafe {
                (*$object).orig = core::ptr::null_mut();
                free($object as *mut c_void);
            }
        }
    }};
}

#[cfg(refcnt_checking)]
#[macro_export]
macro_rules! RC_CHK_EQUAL {
    ($object1:expr, $object2:expr) => {
        $object1 == $object2
            || (!$object1.is_null()
                && !$object2.is_null()
                && unsafe { (*$object1).orig == (*$object2).orig })
    };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
