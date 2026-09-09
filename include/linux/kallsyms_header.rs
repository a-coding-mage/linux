/* SPDX-License-Identifier: GPL-2.0 */
/* Rewritten and vastly simplified by Rusty Russell for in-kernel
 * module loader:
 *   Copyright 2002 Rusty Russell <rusty@rustcorp.com.au> IBM Corporation
 */

/* C header dependencies are supplied by the surrounding kernel translation. */

pub const KSYM_NAME_LEN: usize = 512;
pub const KSYM_SYMBOL_LEN: usize = core::mem::size_of::<&'static [u8; 25]>()
    + (KSYM_NAME_LEN - 1)
    + 2 * (BITS_PER_LONG * 3 / 10)
    + (MODULE_NAME_LEN - 1)
    + (BUILD_ID_SIZE_MAX * 2)
    + 1;

pub struct cred;
pub struct module;

#[inline]
pub unsafe fn is_kernel_text(addr: c_ulong) -> c_int {
    if __is_kernel_text(addr) != 0 {
        1
    } else {
        in_gate_area_no_mm(addr)
    }
}

#[inline]
pub unsafe fn is_kernel(addr: c_ulong) -> c_int {
    if __is_kernel(addr) != 0 {
        1
    } else {
        in_gate_area_no_mm(addr)
    }
}

#[inline]
pub unsafe fn is_ksym_addr(addr: c_ulong) -> c_int {
    /* CONFIG_KALLSYMS_ALL is a build-time condition from the C header. */
    if IS_ENABLED_CONFIG_KALLSYMS_ALL != 0 {
        is_kernel(addr)
    } else if is_kernel_text(addr) != 0 || is_kernel_inittext(addr) != 0 {
        1
    } else {
        0
    }
}

#[inline]
pub unsafe fn dereference_symbol_descriptor(mut ptr: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    /* CONFIG_HAVE_FUNCTION_DESCRIPTORS is a build-time condition from the C header. */
    #[cfg(CONFIG_HAVE_FUNCTION_DESCRIPTORS)]
    {
        ptr = dereference_kernel_function_descriptor(ptr);
        if is_ksym_addr(ptr as c_ulong) != 0 {
            return ptr;
        }

        /* guard(rcu)(); */
        let mod_: *mut module = __module_address(ptr as c_ulong);
        if !mod_.is_null() {
            ptr = dereference_module_function_descriptor(mod_, ptr);
        }
    }
    ptr
}

/* How and when do we show kallsyms values? */
extern "C" {
    pub fn kallsyms_show_value(cred: *const cred) -> bool;
}

/* CONFIG_KALLSYMS declarations, or the !CONFIG_KALLSYMS inline fallbacks. */
#[cfg(CONFIG_KALLSYMS)]
extern "C" {
    pub fn kallsyms_sym_address(idx: c_int) -> c_ulong;
    pub fn kallsyms_on_each_symbol(fn_: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *const c_char, c_ulong) -> c_int>, data: *mut core::ffi::c_void) -> c_int;
    pub fn kallsyms_on_each_match_symbol(fn_: Option<unsafe extern "C" fn(*mut core::ffi::c_void, c_ulong) -> c_int>, name: *const c_char, data: *mut core::ffi::c_void) -> c_int;
    pub fn kallsyms_lookup_name(name: *const c_char) -> c_ulong;
    pub fn kallsyms_lookup_size_offset(addr: c_ulong, symbolsize: *mut c_ulong, offset: *mut c_ulong) -> c_int;
    pub fn kallsyms_lookup(addr: c_ulong, symbolsize: *mut c_ulong, offset: *mut c_ulong, modname: *mut *mut c_char, namebuf: *mut c_char) -> *const c_char;
    pub fn sprint_symbol(buffer: *mut c_char, address: c_ulong) -> c_int;
    pub fn sprint_symbol_build_id(buffer: *mut c_char, address: c_ulong) -> c_int;
    pub fn sprint_symbol_no_offset(buffer: *mut c_char, address: c_ulong) -> c_int;
    pub fn sprint_backtrace(buffer: *mut c_char, address: c_ulong) -> c_int;
    pub fn sprint_backtrace_build_id(buffer: *mut c_char, address: c_ulong) -> c_int;
    pub fn lookup_symbol_name(addr: c_ulong, symname: *mut c_char) -> c_int;
}

#[cfg(not(CONFIG_KALLSYMS))]
#[inline]
pub unsafe fn kallsyms_lookup_name(_name: *const c_char) -> c_ulong { 0 }
#[cfg(not(CONFIG_KALLSYMS))]
#[inline]
pub unsafe fn kallsyms_lookup_size_offset(_addr: c_ulong, _symbolsize: *mut c_ulong, _offset: *mut c_ulong) -> c_int { 0 }
#[cfg(not(CONFIG_KALLSYMS))]
#[inline]
pub unsafe fn kallsyms_lookup(_addr: c_ulong, _symbolsize: *mut c_ulong, _offset: *mut c_ulong, _modname: *mut *mut c_char, _namebuf: *mut c_char) -> *const c_char { core::ptr::null() }

#[cfg(not(CONFIG_KALLSYMS))]
#[inline]
pub unsafe fn sprint_symbol(buffer: *mut c_char, _addr: c_ulong) -> c_int { *buffer = 0; 0 }
#[cfg(not(CONFIG_KALLSYMS))]
#[inline]
pub unsafe fn sprint_symbol_build_id(buffer: *mut c_char, _address: c_ulong) -> c_int { *buffer = 0; 0 }
#[cfg(not(CONFIG_KALLSYMS))]
#[inline]
pub unsafe fn sprint_symbol_no_offset(buffer: *mut c_char, _addr: c_ulong) -> c_int { *buffer = 0; 0 }
#[cfg(not(CONFIG_KALLSYMS))]
#[inline]
pub unsafe fn sprint_backtrace(buffer: *mut c_char, _addr: c_ulong) -> c_int { *buffer = 0; 0 }
#[cfg(not(CONFIG_KALLSYMS))]
#[inline]
pub unsafe fn sprint_backtrace_build_id(buffer: *mut c_char, _addr: c_ulong) -> c_int { *buffer = 0; 0 }
#[cfg(not(CONFIG_KALLSYMS))]
#[inline]
pub unsafe fn lookup_symbol_name(_addr: c_ulong, _symname: *mut c_char) -> c_int { -ERANGE }
#[cfg(not(CONFIG_KALLSYMS))]
#[inline]
pub unsafe fn kallsyms_on_each_symbol(_fn_: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *const c_char, c_ulong) -> c_int>, _data: *mut core::ffi::c_void) -> c_int { -EOPNOTSUPP }
#[cfg(not(CONFIG_KALLSYMS))]
#[inline]
pub unsafe fn kallsyms_on_each_match_symbol(_fn_: Option<unsafe extern "C" fn(*mut core::ffi::c_void, c_ulong) -> c_int>, _name: *const c_char, _data: *mut core::ffi::c_void) -> c_int { -EOPNOTSUPP }

#[inline]
pub unsafe fn print_ip_sym(loglvl: *const c_char, ip: c_ulong) {
    printk(b"%s[<%px>] %pS\0".as_ptr() as *const c_char, loglvl, ip as *mut core::ffi::c_void, ip as *mut core::ffi::c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
