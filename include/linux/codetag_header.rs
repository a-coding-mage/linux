/* SPDX-License-Identifier: GPL-2.0 */
/*
 * code tagging framework
 */

use core::ffi::{c_char, c_void};

/* Dependency supplied by the surrounding kernel translation. */

pub const CODETAG_SECTION_START_PREFIX: &str = "__start_";
pub const CODETAG_SECTION_STOP_PREFIX: &str = "__stop_";

/* codetag flags */
pub const CODETAG_FLAG_INACCURATE: u32 = 1 << 0;

pub enum codetag_type {}
pub enum codetag_module {}
pub enum seq_buf {}
pub enum module {}

/*
 * An instance of this structure is created in a special ELF section at every
 * code location being tagged.  At runtime, the special section is treated as
 * an array of these.
 */
#[repr(C, align(8))]
pub struct codetag {
    pub flags: u32,
    pub lineno: u32,
    pub modname: *const c_char,
    pub function: *const c_char,
    pub filename: *const c_char,
}

#[repr(C)]
pub union codetag_ref {
    pub ct: *mut codetag,
}

#[repr(C)]
pub struct codetag_type_desc {
    pub section: *const c_char,
    pub tag_size: usize,
    pub module_load: Option<unsafe extern "C" fn(
        mod_: *mut module,
        start: *mut codetag,
        end: *mut codetag,
    ) -> i32>,
    pub module_unload: Option<unsafe extern "C" fn(
        mod_: *mut module,
        start: *mut codetag,
        end: *mut codetag,
    )>,
    /* CONFIG_MODULES fields are present when the corresponding build option is enabled. */
    #[cfg(feature = "CONFIG_MODULES")]
    pub module_replaced: Option<unsafe extern "C" fn(*mut module, *mut module)>,
    #[cfg(feature = "CONFIG_MODULES")]
    pub needs_section_mem: Option<unsafe extern "C" fn(*mut module, usize) -> bool>,
    #[cfg(feature = "CONFIG_MODULES")]
    pub alloc_section_mem: Option<unsafe extern "C" fn(*mut module, usize, u32, usize) -> *mut c_void>,
    #[cfg(feature = "CONFIG_MODULES")]
    pub free_section_mem: Option<unsafe extern "C" fn(*mut module, bool)>,
}

#[repr(C)]
pub struct codetag_iterator {
    pub cttype: *mut codetag_type,
    pub cmod: *mut codetag_module,
    pub mod_id: c_ulong,
    pub ct: *mut codetag,
    pub mod_seq: c_ulong,
}

pub type c_ulong = usize;

/* MODULE/KBUILD_MODNAME configuration is supplied by the build environment. */

#[macro_export]
macro_rules! CODE_TAG_INIT {
    () => {
        $crate::codetag {
            modname: core::ptr::null(),
            function: concat!(module_path!(), "\0").as_ptr() as *const core::ffi::c_char,
            filename: concat!(file!(), "\0").as_ptr() as *const core::ffi::c_char,
            lineno: line!(),
            flags: 0,
        }
    };
}

unsafe extern "C" {
    pub fn codetag_lock_module_list(cttype: *mut codetag_type);
    pub fn codetag_trylock_module_list(cttype: *mut codetag_type) -> bool;
    pub fn codetag_unlock_module_list(cttype: *mut codetag_type);
    pub fn codetag_get_content_id(cttype: *mut codetag_type) -> c_ulong;
    pub fn codetag_get_count(cttype: *mut codetag_type) -> u32;
    pub fn codetag_get_ct_iter(cttype: *mut codetag_type) -> codetag_iterator;
    pub fn codetag_next_ct(iter: *mut codetag_iterator) -> *mut codetag;
    pub fn codetag_to_text(out: *mut seq_buf, ct: *mut codetag);
    pub fn codetag_register_type(desc: *const codetag_type_desc) -> *mut codetag_type;
}

/* Retained conditional intent: enabled only with CONFIG_CODE_TAGGING and CONFIG_MODULES. */
#[cfg(all(feature = "CONFIG_CODE_TAGGING", feature = "CONFIG_MODULES"))]
unsafe extern "C" {
    pub fn codetag_needs_module_section(mod_: *mut module, name: *const c_char, size: c_ulong) -> bool;
    pub fn codetag_alloc_module_section(mod_: *mut module, name: *const c_char, size: c_ulong, prepend: u32, align: c_ulong) -> *mut c_void;
    pub fn codetag_free_module_sections(mod_: *mut module);
    pub fn codetag_module_replaced(mod_: *mut module, new_mod: *mut module);
    pub fn codetag_load_module(mod_: *mut module) -> i32;
    pub fn codetag_unload_module(mod_: *mut module);
}

#[cfg(not(all(feature = "CONFIG_CODE_TAGGING", feature = "CONFIG_MODULES")))]
pub unsafe fn codetag_needs_module_section(_: *mut module, _: *const c_char, _: c_ulong) -> bool { false }
#[cfg(not(all(feature = "CONFIG_CODE_TAGGING", feature = "CONFIG_MODULES")))]
pub unsafe fn codetag_alloc_module_section(_: *mut module, _: *const c_char, _: c_ulong, _: u32, _: c_ulong) -> *mut c_void { core::ptr::null_mut() }
#[cfg(not(all(feature = "CONFIG_CODE_TAGGING", feature = "CONFIG_MODULES")))]
pub unsafe fn codetag_free_module_sections(_: *mut module) {}
#[cfg(not(all(feature = "CONFIG_CODE_TAGGING", feature = "CONFIG_MODULES")))]
pub unsafe fn codetag_module_replaced(_: *mut module, _: *mut module) {}
#[cfg(not(all(feature = "CONFIG_CODE_TAGGING", feature = "CONFIG_MODULES")))]
pub unsafe fn codetag_load_module(_: *mut module) -> i32 { 0 }
#[cfg(not(all(feature = "CONFIG_CODE_TAGGING", feature = "CONFIG_MODULES")))]
pub unsafe fn codetag_unload_module(_: *mut module) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
