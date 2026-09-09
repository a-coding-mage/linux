/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of linux/jump_label.h. */

use core::ffi::c_void;

extern "C" {
    pub static mut static_key_initialized: bool;
}

/* STATIC_KEY_CHECK_USE(key): WARN if a static key is used before initialization. */

#[repr(C)]
pub struct atomic_t {
    pub counter: i32,
}

#[repr(C)]
pub struct static_key {
    pub enabled: atomic_t,
    #[cfg(feature = "CONFIG_JUMP_LABEL")]
    pub data: static_key_data,
}

#[cfg(feature = "CONFIG_JUMP_LABEL")]
#[repr(C)]
pub union static_key_data {
    pub type_: usize,
    pub entries: *mut jump_entry,
    pub next: *mut static_key_mod,
}

#[repr(C)]
pub struct static_key_mod {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_JUMP_LABEL")]
#[cfg(feature = "CONFIG_HAVE_ARCH_JUMP_LABEL_RELATIVE")]
#[repr(C)]
pub struct jump_entry {
    pub code: i32,
    pub target: i32,
    pub key: isize, // key may be far away from the core kernel under KASLR
}

#[cfg(feature = "CONFIG_JUMP_LABEL")]
#[cfg(not(feature = "CONFIG_HAVE_ARCH_JUMP_LABEL_RELATIVE"))]
#[repr(C)]
pub struct jump_entry {
    pub code: usize,
    pub target: usize,
    pub key: usize,
}

#[cfg(all(feature = "CONFIG_JUMP_LABEL", feature = "CONFIG_HAVE_ARCH_JUMP_LABEL_RELATIVE"))]
#[inline]
pub unsafe fn jump_entry_code(entry: *const jump_entry) -> usize {
    (core::ptr::addr_of!((*entry).code) as usize).wrapping_add((*entry).code as isize as usize)
}

#[cfg(all(feature = "CONFIG_JUMP_LABEL", feature = "CONFIG_HAVE_ARCH_JUMP_LABEL_RELATIVE"))]
#[inline]
pub unsafe fn jump_entry_target(entry: *const jump_entry) -> usize {
    (core::ptr::addr_of!((*entry).target) as usize).wrapping_add((*entry).target as isize as usize)
}

#[cfg(all(feature = "CONFIG_JUMP_LABEL", feature = "CONFIG_HAVE_ARCH_JUMP_LABEL_RELATIVE"))]
#[inline]
pub unsafe fn jump_entry_key(entry: *const jump_entry) -> *mut static_key {
    let offset = ((*entry).key & !3isize) as isize;
    (core::ptr::addr_of!((*entry).key) as usize).wrapping_add(offset as usize) as *mut static_key
}

#[cfg(all(feature = "CONFIG_JUMP_LABEL", not(feature = "CONFIG_HAVE_ARCH_JUMP_LABEL_RELATIVE")))]
#[inline]
pub unsafe fn jump_entry_code(entry: *const jump_entry) -> usize { (*entry).code }

#[cfg(all(feature = "CONFIG_JUMP_LABEL", not(feature = "CONFIG_HAVE_ARCH_JUMP_LABEL_RELATIVE")))]
#[inline]
pub unsafe fn jump_entry_target(entry: *const jump_entry) -> usize { (*entry).target }

#[cfg(all(feature = "CONFIG_JUMP_LABEL", not(feature = "CONFIG_HAVE_ARCH_JUMP_LABEL_RELATIVE")))]
#[inline]
pub unsafe fn jump_entry_key(entry: *const jump_entry) -> *mut static_key {
    ((*entry).key & !3usize) as *mut static_key
}

#[cfg(feature = "CONFIG_JUMP_LABEL")]
#[inline]
pub unsafe fn jump_entry_is_branch(entry: *const jump_entry) -> bool { ((*entry).key as usize & 1) != 0 }

#[cfg(feature = "CONFIG_JUMP_LABEL")]
#[inline]
pub unsafe fn jump_entry_is_init(entry: *const jump_entry) -> bool { ((*entry).key as usize & 2) != 0 }

#[cfg(feature = "CONFIG_JUMP_LABEL")]
#[inline]
pub unsafe fn jump_entry_set_init(entry: *mut jump_entry, set: bool) {
    if set { (*entry).key |= 2; } else { (*entry).key &= !2; }
}

#[cfg(feature = "CONFIG_JUMP_LABEL")]
#[inline]
pub unsafe fn jump_entry_size(entry: *mut jump_entry) -> i32 {
    /* JUMP_LABEL_NOP_SIZE, when configured, is supplied by the architecture. */
    arch_jump_entry_size(entry)
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum jump_label_type { JUMP_LABEL_NOP = 0, JUMP_LABEL_JMP = 1 }

pub enum module {}

pub const JUMP_TYPE_FALSE: usize = 0;
pub const JUMP_TYPE_TRUE: usize = 1;
pub const JUMP_TYPE_LINKED: usize = 2;
pub const JUMP_TYPE_MASK: usize = 3;

extern "C" {
    pub fn arch_static_branch(key: *mut static_key, branch: bool) -> bool;
    pub fn arch_static_branch_jump(key: *mut static_key, branch: bool) -> bool;
    pub fn arch_jump_entry_size(entry: *mut jump_entry) -> i32;
    pub fn jump_label_init();
    pub fn jump_label_init_ro();
    pub fn jump_label_lock();
    pub fn jump_label_unlock();
    pub fn arch_jump_label_transform(entry: *mut jump_entry, ty: jump_label_type);
    pub fn arch_jump_label_transform_queue(entry: *mut jump_entry, ty: jump_label_type) -> bool;
    pub fn arch_jump_label_transform_apply();
    pub fn jump_label_text_reserved(start: *mut c_void, end: *mut c_void) -> i32;
    pub fn static_key_slow_inc(key: *mut static_key) -> bool;
    pub fn static_key_fast_inc_not_disabled(key: *mut static_key) -> bool;
    pub fn static_key_slow_dec(key: *mut static_key);
    pub fn static_key_slow_inc_cpuslocked(key: *mut static_key) -> bool;
    pub fn static_key_slow_dec_cpuslocked(key: *mut static_key);
    pub fn static_key_count(key: *mut static_key) -> i32;
    pub fn static_key_enable(key: *mut static_key);
    pub fn static_key_disable(key: *mut static_key);
    pub fn static_key_enable_cpuslocked(key: *mut static_key);
    pub fn static_key_disable_cpuslocked(key: *mut static_key);
    pub fn jump_label_init_type(entry: *mut jump_entry) -> jump_label_type;
    pub fn ____wrong_branch_error() -> bool;
}

#[inline(always)]
pub unsafe fn static_key_false(key: *mut static_key) -> bool { arch_static_branch(key, false) }

#[inline(always)]
pub unsafe fn static_key_true(key: *mut static_key) -> bool { !arch_static_branch(key, true) }

#[repr(C)]
pub struct static_key_true { pub key: static_key }

#[repr(C)]
pub struct static_key_false { pub key: static_key }

/* STATIC_KEY_INIT_TRUE/FALSE and DEFINE/DECLARE_STATIC_KEY_* are C aggregate
 * and declaration macros; their Rust equivalents are the structs above and
 * ordinary static declarations initialized by the consuming translation unit. */

#[inline]
pub unsafe fn static_key_enabled(x: *mut static_key) -> bool { static_key_count(x) > 0 }

/* The CONFIG_JUMP_LABEL branch macros preserve architecture-specific branch
 * selection; the fallback uses static_key_enabled on the wrapped key. */
#[inline]
pub unsafe fn static_branch_likely(x: *mut static_key) -> bool { static_key_enabled(x) }

#[inline]
pub unsafe fn static_branch_unlikely(x: *mut static_key) -> bool { static_key_enabled(x) }

#[inline]
pub unsafe fn static_branch_inc(x: *mut static_key) -> bool { static_key_slow_inc(x) }
#[inline]
pub unsafe fn static_branch_dec(x: *mut static_key) { static_key_slow_dec(x) }
#[inline]
pub unsafe fn static_branch_inc_cpuslocked(x: *mut static_key) -> bool { static_key_slow_inc_cpuslocked(x) }
#[inline]
pub unsafe fn static_branch_dec_cpuslocked(x: *mut static_key) { static_key_slow_dec_cpuslocked(x) }
#[inline]
pub unsafe fn static_branch_enable(x: *mut static_key) { static_key_enable(x) }
#[inline]
pub unsafe fn static_branch_disable(x: *mut static_key) { static_key_disable(x) }
#[inline]
pub unsafe fn static_branch_enable_cpuslocked(x: *mut static_key) { static_key_enable_cpuslocked(x) }
#[inline]
pub unsafe fn static_branch_disable_cpuslocked(x: *mut static_key) { static_key_disable_cpuslocked(x) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
