/* SPDX-License-Identifier: GPL-2.0 */
/*
 * The C header adjusts TRACE_SYSTEM while trace points are being generated.
 * Rust has no preprocessor equivalent; this translation keeps that intent in
 * the conditional sections below.
 */

#[cfg(feature = "create_trace_points")]
// TRACE_SYSTEM is set to `module` while module trace points are processed.
const TRACE_SYSTEM: &str = "module";

#[cfg(feature = "config_modules")]
#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[cfg(feature = "config_modules")]
#[repr(C)]
pub struct module_load_entry {
    pub taints: ::core::ffi::c_uint,
    // __string(name, mod->name) is a dynamically allocated trace string.
    pub name: *const ::core::ffi::c_char,
}

#[cfg(feature = "config_modules")]
#[repr(C)]
pub struct module_free_entry {
    // __string(name, mod->name) is a dynamically allocated trace string.
    pub name: *const ::core::ffi::c_char,
}

#[cfg(all(feature = "config_modules", feature = "config_module_unload"))]
#[repr(C)]
pub struct module_refcnt_entry {
    pub ip: ::core::ffi::c_ulong,
    pub refcnt: ::core::ffi::c_int,
    // __string(name, mod->name) is a dynamically allocated trace string.
    pub name: *const ::core::ffi::c_char,
}

#[cfg(feature = "config_modules")]
#[repr(C)]
pub struct module_request_entry {
    pub ip: ::core::ffi::c_ulong,
    pub wait: bool,
    // __string(name, name) is a dynamically allocated trace string.
    pub name: *const ::core::ffi::c_char,
}

#[cfg(feature = "config_modules")]
pub const MODULE_TRACE_EVENT_MODULE_LOAD: &str = "module_load";

#[cfg(feature = "config_modules")]
pub const MODULE_TRACE_EVENT_MODULE_FREE: &str = "module_free";

#[cfg(all(feature = "config_modules", feature = "config_module_unload"))]
pub const MODULE_TRACE_EVENT_MODULE_GET: &str = "module_get";

#[cfg(all(feature = "config_modules", feature = "config_module_unload"))]
pub const MODULE_TRACE_EVENT_MODULE_PUT: &str = "module_put";

#[cfg(feature = "config_modules")]
pub const MODULE_TRACE_EVENT_MODULE_REQUEST: &str = "module_request";

/*
 * The following tracepoint registrations preserve the C header's external
 * interface and formatting.  TRACE_EVENT/DECLARE_EVENT_CLASS/DEFINE_EVENT,
 * __field, __string, TP_fast_assign, and TP_printk are supplied by the Linux
 * tracepoint implementation and therefore remain dependency references.
 *
 * module_load(mod): taints = mod->taints; name = mod->name;
 * module_free(mod): name = mod->name;
 * module_refcnt(mod, ip): ip = ip; refcnt = atomic_read(&mod->refcnt);
 * module_request(name, wait, ip): ip = ip; wait = wait; name = name;
 * module flags are rendered as P/O/F/C/E for the corresponding taint bits.
 */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
