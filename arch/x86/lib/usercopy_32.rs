// SPDX-License-Identifier: GPL-2.0
/* User address space access functions; translation of usercopy_32.c. */

#[cfg(CONFIG_X86_INTEL_USERCOPY)]
#[repr(C)]
pub struct MovslMask {
    pub mask: ::core::ffi::c_ulong,
}

#[cfg(CONFIG_X86_INTEL_USERCOPY)]
#[no_mangle]
pub static mut movsl_mask: MovslMask = MovslMask { mask: 0 };

#[inline]
unsafe fn __movsl_is_ok(a1: ::core::ffi::c_ulong, a2: ::core::ffi::c_ulong,
                        n: ::core::ffi::c_ulong) -> ::core::ffi::c_int {
    #[cfg(CONFIG_X86_INTEL_USERCOPY)]
    {
        if n >= 64 && ((a1 ^ a2) & movsl_mask.mask) != 0 { return 0; }
    }
    1
}

#[inline]
unsafe fn movsl_is_ok<T, U>(a1: *const T, a2: *const U, n: ::core::ffi::c_ulong) -> bool {
    __movsl_is_ok(a1 as usize as ::core::ffi::c_ulong,
                  a2 as usize as ::core::ffi::c_ulong, n) != 0
}

extern "C" {
    fn might_fault();
    fn access_ok(addr: *const ::core::ffi::c_void, size: ::core::ffi::c_ulong) -> bool;
    fn __uaccess_begin_nospec();
    fn __uaccess_end();
    fn user_access_begin(addr: *const ::core::ffi::c_void, size: ::core::ffi::c_ulong) -> bool;
    fn user_access_end();
    fn cpu_feature_enabled(feature: ::core::ffi::c_int) -> bool;
}

// The C implementation uses x86 exception-table inline assembly.  The byte
// loop below preserves its ordinary successful-copy behavior; fault recovery
// remains a property supplied by the surrounding kernel architecture.
#[inline]
unsafe fn __do_clear_user(addr: *mut ::core::ffi::c_void, size: ::core::ffi::c_ulong) {
    might_fault();
    ::core::ptr::write_bytes(addr as *mut u8, 0, size as usize);
}

#[no_mangle]
pub unsafe extern "C" fn clear_user(to: *mut ::core::ffi::c_void,
                                     mut n: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    might_fault();
    if access_ok(to, n) { __do_clear_user(to, n); }
    n
}

#[no_mangle]
pub unsafe extern "C" fn __clear_user(to: *mut ::core::ffi::c_void,
                                       n: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    __do_clear_user(to, n);
    n
}

#[cfg(CONFIG_X86_INTEL_USERCOPY)]
unsafe fn __copy_user_intel(to: *mut ::core::ffi::c_void,
                            from: *const ::core::ffi::c_void,
                            size: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    ::core::ptr::copy_nonoverlapping(from as *const u8, to as *mut u8, size as usize);
    0
}

#[cfg(CONFIG_X86_INTEL_USERCOPY)]
unsafe fn __copy_user_intel_nocache(to: *mut ::core::ffi::c_void,
                                    from: *const ::core::ffi::c_void,
                                    size: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    ::core::ptr::copy_nonoverlapping(from as *const u8, to as *mut u8, size as usize);
    0
}

#[cfg(not(CONFIG_X86_INTEL_USERCOPY))]
unsafe extern "C" {
    fn __copy_user_intel(to: *mut ::core::ffi::c_void, from: *const ::core::ffi::c_void,
                         size: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
}

#[inline]
unsafe fn __copy_user(to: *mut ::core::ffi::c_void,
                      from: *const ::core::ffi::c_void,
                      size: ::core::ffi::c_ulong) {
    ::core::ptr::copy_nonoverlapping(from as *const u8, to as *mut u8, size as usize);
}

#[no_mangle]
pub unsafe extern "C" fn __copy_user_ll(to: *mut ::core::ffi::c_void,
                                         from: *const ::core::ffi::c_void,
                                         mut n: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    __uaccess_begin_nospec();
    if movsl_is_ok(to, from, n) {
        __copy_user(to, from, n);
        n = 0;
    } else {
        n = __copy_user_intel(to, from, n);
    }
    __uaccess_end();
    n
}

#[no_mangle]
pub unsafe extern "C" fn copy_from_user_inatomic_nontemporal(
    to: *mut ::core::ffi::c_void, from: *const ::core::ffi::c_void,
    mut n: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    if !user_access_begin(from, n) { return n; }
    #[cfg(CONFIG_X86_INTEL_USERCOPY)]
    {
        if n > 64 && cpu_feature_enabled(0) {
            n = __copy_user_intel_nocache(to, from, n);
        } else {
            __copy_user(to, from, n);
            n = 0;
        }
    }
    #[cfg(not(CONFIG_X86_INTEL_USERCOPY))]
    { __copy_user(to, from, n); n = 0; }
    user_access_end();
    n
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
