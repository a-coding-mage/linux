/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: _LINUX_FAULT_INJECT_H
// C dependencies: linux/err.h, linux/types.h

pub enum dentry {}
pub enum kmem_cache {}

#[repr(C)]
pub enum fault_flags {
    FAULT_NOWARN = 1 << 0,
}

// CONFIG_FAULT_INJECTION
// C dependencies: linux/atomic.h, linux/configfs.h, linux/ratelimit.h

#[repr(C)]
pub struct fault_attr {
    pub probability: ::core::ffi::c_ulong,
    pub interval: ::core::ffi::c_ulong,
    pub times: atomic_t,
    pub space: atomic_t,
    pub verbose: ::core::ffi::c_ulong,
    pub task_filter: bool,
    pub stacktrace_depth: ::core::ffi::c_ulong,
    pub require_start: ::core::ffi::c_ulong,
    pub require_end: ::core::ffi::c_ulong,
    pub reject_start: ::core::ffi::c_ulong,
    pub reject_end: ::core::ffi::c_ulong,
    pub count: ::core::ffi::c_ulong,
    pub ratelimit_state: ratelimit_state,
    pub dname: *mut dentry,
}

// FAULT_ATTR_INITIALIZER: interval = 1, times = ATOMIC_INIT(1),
// require_end = ULONG_MAX, stacktrace_depth = 32,
// ratelimit_state = RATELIMIT_STATE_INIT_DISABLED, verbose = 2,
// dname = NULL.

// DECLARE_FAULT_ATTR(name)
#[macro_export]
macro_rules! DECLARE_FAULT_ATTR {
    ($name:ident) => {
        let mut $name: fault_attr = fault_attr {
            probability: 0,
            interval: 1,
            times: atomic_t::new(1),
            space: atomic_t::new(0),
            verbose: 2,
            task_filter: false,
            stacktrace_depth: 32,
            require_start: 0,
            require_end: ::core::primitive::usize::MAX as ::core::ffi::c_ulong,
            reject_start: 0,
            reject_end: 0,
            count: 0,
            ratelimit_state: RATELIMIT_STATE_INIT_DISABLED,
            dname: ::core::ptr::null_mut(),
        };
    };
}

unsafe extern "C" {
    pub fn setup_fault_attr(attr: *mut fault_attr, str_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn should_fail_ex(attr: *mut fault_attr, size: ssize_t, flags: ::core::ffi::c_int) -> bool;
    pub fn should_fail(attr: *mut fault_attr, size: ssize_t) -> bool;
}

// !CONFIG_FAULT_INJECTION
#[cfg(not(feature = "CONFIG_FAULT_INJECTION"))]
pub struct fault_attr_disabled {}

#[cfg(not(feature = "CONFIG_FAULT_INJECTION"))]
pub fn setup_fault_attr(_attr: *mut fault_attr_disabled, _str_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    0 // Note: 0 means error for __setup() handlers!
}

#[cfg(not(feature = "CONFIG_FAULT_INJECTION"))]
pub fn should_fail_ex(_attr: *mut fault_attr_disabled, _size: ssize_t, _flags: ::core::ffi::c_int) -> bool { false }

#[cfg(not(feature = "CONFIG_FAULT_INJECTION"))]
pub fn should_fail(_attr: *mut fault_attr_disabled, _size: ssize_t) -> bool { false }

// CONFIG_FAULT_INJECTION_DEBUG_FS
unsafe extern "C" {
    pub fn fault_create_debugfs_attr(
        name: *const ::core::ffi::c_char,
        parent: *mut dentry,
        attr: *mut fault_attr,
    ) -> *mut dentry;
}

// !CONFIG_FAULT_INJECTION_DEBUG_FS
#[cfg(not(feature = "CONFIG_FAULT_INJECTION_DEBUG_FS"))]
pub unsafe fn fault_create_debugfs_attr_disabled(
    _name: *const ::core::ffi::c_char,
    _parent: *mut dentry,
    _attr: *mut fault_attr,
) -> *mut dentry {
    // ERR_PTR(-ENODEV)
    ::core::ptr::without_provenance_mut::<dentry>(-19isize as usize)
}

// CONFIG_FAULT_INJECTION_CONFIGFS
#[repr(C)]
pub struct fault_config {
    pub attr: fault_attr,
    pub group: config_group,
}

unsafe extern "C" {
    pub fn fault_config_init(config: *mut fault_config, name: *const ::core::ffi::c_char);
}

// !CONFIG_FAULT_INJECTION_CONFIGFS
#[cfg(not(feature = "CONFIG_FAULT_INJECTION_CONFIGFS"))]
pub struct fault_config_disabled {}

#[cfg(not(feature = "CONFIG_FAULT_INJECTION_CONFIGFS"))]
pub fn fault_config_init_disabled(_config: *mut fault_config_disabled, _name: *const ::core::ffi::c_char) {}

// CONFIG_FAIL_PAGE_ALLOC
unsafe extern "C" {
    pub fn should_fail_alloc_page(gfp_mask: gfp_t, order: ::core::ffi::c_uint) -> bool;
}

// !CONFIG_FAIL_PAGE_ALLOC
#[cfg(not(feature = "CONFIG_FAIL_PAGE_ALLOC"))]
pub fn should_fail_alloc_page_disabled(_gfp_mask: gfp_t, _order: ::core::ffi::c_uint) -> bool { false }

// CONFIG_FAILSLAB
unsafe extern "C" {
    pub fn should_failslab(s: *mut kmem_cache, gfpflags: gfp_t) -> ::core::ffi::c_int;
}

// !CONFIG_FAILSLAB
#[cfg(not(feature = "CONFIG_FAILSLAB"))]
pub fn should_failslab_disabled(_s: *mut kmem_cache, _gfpflags: gfp_t) -> ::core::ffi::c_int { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
