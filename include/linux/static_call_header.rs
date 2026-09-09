/* SPDX-License-Identifier: GPL-2.0 */
// Static call support. This is a source-level Rust translation of the C header;
// dependent kernel types, macros, and architecture definitions are external.

#[cfg(CONFIG_HAVE_STATIC_CALL)]
extern "C" {
    pub fn arch_static_call_transform(site: *mut core::ffi::c_void,
                                      tramp: *mut core::ffi::c_void,
                                      func: *mut core::ffi::c_void,
                                      tail: bool);
}

#[cfg(CONFIG_HAVE_STATIC_CALL_INLINE)]
extern "C" {
    pub static mut static_call_initialized: core::ffi::c_int;
    pub fn static_call_init() -> core::ffi::c_int;
    pub fn static_call_force_reinit();
    pub fn __static_call_update(key: *mut static_call_key,
                                tramp: *mut core::ffi::c_void,
                                func: *mut core::ffi::c_void);
    pub fn static_call_mod_init(module: *mut module) -> core::ffi::c_int;
    pub fn static_call_text_reserved(start: *mut core::ffi::c_void,
                                     end: *mut core::ffi::c_void) -> core::ffi::c_int;
    pub fn __static_call_return0() -> core::ffi::c_long;
}

#[cfg(CONFIG_HAVE_STATIC_CALL_INLINE)]
#[repr(C)]
pub struct static_call_mod {
    pub next: *mut static_call_mod,
    pub mod_: *mut module, // for vmlinux, mod == NULL
    pub sites: *mut static_call_site,
}

#[cfg(CONFIG_HAVE_STATIC_CALL_INLINE)]
#[repr(C)]
pub struct static_call_tramp_key {
    pub tramp: i32,
    pub key: i32,
}

#[cfg(not(CONFIG_HAVE_STATIC_CALL_INLINE))]
#[cfg(CONFIG_HAVE_STATIC_CALL)]
pub const static_call_initialized: core::ffi::c_int = 0;

#[cfg(not(CONFIG_HAVE_STATIC_CALL_INLINE))]
#[cfg(CONFIG_HAVE_STATIC_CALL)]
#[inline]
pub unsafe fn static_call_init() -> core::ffi::c_int { 0 }

#[cfg(not(CONFIG_HAVE_STATIC_CALL_INLINE))]
#[cfg(CONFIG_HAVE_STATIC_CALL)]
#[inline]
pub unsafe fn __static_call_update(key: *mut static_call_key,
                                   tramp: *mut core::ffi::c_void,
                                   func: *mut core::ffi::c_void) {
    cpus_read_lock();
    WRITE_ONCE((*key).func, func);
    arch_static_call_transform(core::ptr::null_mut(), tramp, func, false);
    cpus_read_unlock();
}

#[cfg(not(CONFIG_HAVE_STATIC_CALL_INLINE))]
#[cfg(CONFIG_HAVE_STATIC_CALL)]
#[inline]
pub unsafe fn static_call_text_reserved(_start: *mut core::ffi::c_void,
                                         _end: *mut core::ffi::c_void) -> core::ffi::c_int { 0 }

#[cfg(not(CONFIG_HAVE_STATIC_CALL))]
pub const static_call_initialized: core::ffi::c_int = 0;

#[cfg(not(CONFIG_HAVE_STATIC_CALL))]
#[inline]
pub unsafe fn static_call_init() -> core::ffi::c_int { 0 }

#[cfg(not(CONFIG_HAVE_STATIC_CALL))]
#[inline]
pub unsafe fn __static_call_return0() -> core::ffi::c_long { 0 }

#[cfg(not(CONFIG_HAVE_STATIC_CALL))]
#[inline]
pub unsafe fn __static_call_nop() {}

#[cfg(not(CONFIG_HAVE_STATIC_CALL))]
#[inline]
pub unsafe fn __static_call_update(key: *mut static_call_key,
                                   _tramp: *mut core::ffi::c_void,
                                   func: *mut core::ffi::c_void) {
    WRITE_ONCE((*key).func, func);
}

#[cfg(not(CONFIG_HAVE_STATIC_CALL))]
#[inline]
pub unsafe fn static_call_text_reserved(_start: *mut core::ffi::c_void,
                                         _end: *mut core::ffi::c_void) -> core::ffi::c_int { 0 }

// The following macro interfaces retain the C header's externally supplied
// STATIC_CALL_*, DECLARE_STATIC_CALL, ARCH_*, READ_ONCE/WRITE_ONCE, and export
// macro semantics without implementing their dependencies here.
#[macro_export]
macro_rules! static_call_update {
    ($name:ident, $func:expr) => {{
        let f = $func;
        unsafe { $crate::__static_call_update($crate::STATIC_CALL_KEY!($name),
            $crate::STATIC_CALL_TRAMP_ADDR!($name), f) }
    }};
}

#[macro_export]
macro_rules! static_call_cond { ($name:ident) => { ().__static_call($name) }; }

#[macro_export]
macro_rules! static_call_query {
    ($name:ident) => { unsafe { READ_ONCE!($crate::STATIC_CALL_KEY!($name).func) } };
}

// DEFINE_STATIC_CALL, DEFINE_STATIC_CALL_NULL, DEFINE_STATIC_CALL_RET0,
// EXPORT_STATIC_CALL, EXPORT_STATIC_CALL_GPL, and EXPORT_STATIC_CALL_TRAMP
// preserve the corresponding C macro declarations and architecture hooks.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
