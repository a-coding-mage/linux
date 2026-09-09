/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Kernel Live Patching Core */

// External kernel types and functions supplied by the corresponding headers
// are intentionally referenced but not implemented here.

use core::ffi::{c_char, c_int, c_ulong, c_uint, c_void};

pub const KLP_TRANSITION_IDLE: c_int = -1;
pub const KLP_TRANSITION_UNPATCHED: c_int = 0;
pub const KLP_TRANSITION_PATCHED: c_int = 1;

#[repr(C)]
pub struct klp_func {
    pub old_name: *const c_char,
    pub new_func: *mut c_void,
    pub old_sympos: c_ulong,
    pub old_func: *mut c_void,
    pub kobj: kobject,
    pub node: list_head,
    pub stack_node: list_head,
    pub old_size: c_ulong,
    pub new_size: c_ulong,
    pub nop: bool,
    pub patched: bool,
    pub transition: bool,
}

#[repr(C)]
pub struct klp_object {
    pub name: *const c_char,
    pub funcs: *mut klp_func,
    pub callbacks: klp_callbacks,
    pub kobj: kobject,
    pub func_list: list_head,
    pub node: list_head,
    pub mod_: *mut module,
    pub dynamic: bool,
    pub patched: bool,
}

#[repr(C)]
pub struct klp_state {
    pub id: c_ulong,
    pub version: c_uint,
    pub data: *mut c_void,
}

#[repr(C)]
pub struct klp_patch {
    pub mod_: *mut module,
    pub objs: *mut klp_object,
    pub states: *mut klp_state,
    pub replace: bool,
    pub list: list_head,
    pub kobj: kobject,
    pub obj_list: list_head,
    pub enabled: bool,
    pub forced: bool,
    pub free_work: work_struct,
    pub finish: completion,
}

#[repr(C)]
pub struct kobject { _private: [u8; 0] }
#[repr(C)]
pub struct list_head { _private: [u8; 0] }
#[repr(C)]
pub struct klp_callbacks { _private: [u8; 0] }
#[repr(C)]
pub struct module { _private: [u8; 0] }
#[repr(C)]
pub struct work_struct { _private: [u8; 0] }
#[repr(C)]
pub struct completion { _private: [u8; 0] }
#[repr(C)]
pub struct task_struct { _private: [u8; 0] }
#[repr(C)]
pub struct Elf_Shdr { _private: [u8; 0] }

// The following iteration macros correspond to the kernel list iteration
// helpers and require the definitions supplied by linux/list.h.
#[macro_export]
macro_rules! klp_for_each_object_static {
    ($patch:expr, $obj:ident) => {
        for $obj in unsafe { $patch.objs } {
            if unsafe { (*$obj).funcs.is_null() && (*$obj).name.is_null() } { break; }
        }
    };
}

#[macro_export]
macro_rules! klp_for_each_object_safe { ($patch:expr, $obj:ident, $tmp_obj:ident) => { /* list_for_each_entry_safe */ }; }
#[macro_export]
macro_rules! klp_for_each_object { ($patch:expr, $obj:ident) => { /* list_for_each_entry */ }; }
#[macro_export]
macro_rules! klp_for_each_func_static { ($obj:expr, $func:ident) => { /* static function array iteration */ }; }
#[macro_export]
macro_rules! klp_for_each_func_safe { ($obj:expr, $func:ident, $tmp_func:ident) => { /* list_for_each_entry_safe */ }; }
#[macro_export]
macro_rules! klp_for_each_func { ($obj:expr, $func:ident) => { /* list_for_each_entry */ }; }

extern "C" {
    pub fn klp_enable_patch(patch: *mut klp_patch) -> c_int;
    pub fn klp_module_coming(mod_: *mut module) -> c_int;
    pub fn klp_module_going(mod_: *mut module);
    pub fn klp_find_section_by_name(mod_: *const module, name: *const c_char, sec_size: *mut usize) -> *mut c_void;
    pub fn klp_copy_process(child: *mut task_struct);
    pub fn klp_update_patch_state(task: *mut task_struct);
    pub fn klp_shadow_get(obj: *mut c_void, id: c_ulong) -> *mut c_void;
    pub fn klp_shadow_alloc(obj: *mut c_void, id: c_ulong, size: usize, gfp_flags: c_uint, ctor: klp_shadow_ctor_t, ctor_data: *mut c_void) -> *mut c_void;
    pub fn klp_shadow_get_or_alloc(obj: *mut c_void, id: c_ulong, size: usize, gfp_flags: c_uint, ctor: klp_shadow_ctor_t, ctor_data: *mut c_void) -> *mut c_void;
    pub fn klp_shadow_free(obj: *mut c_void, id: c_ulong, dtor: klp_shadow_dtor_t);
    pub fn klp_shadow_free_all(id: c_ulong, dtor: klp_shadow_dtor_t);
    pub fn klp_get_state(patch: *mut klp_patch, id: c_ulong) -> *mut klp_state;
    pub fn klp_get_prev_state(id: c_ulong) -> *mut klp_state;
    pub fn klp_apply_section_relocs(pmod: *mut module, sechdrs: *mut Elf_Shdr, shstrtab: *const c_char, strtab: *const c_char, symindex: c_uint, secindex: c_uint, objname: *const c_char) -> c_int;
}

pub type klp_shadow_ctor_t = Option<unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void) -> c_int>;
pub type klp_shadow_dtor_t = Option<unsafe extern "C" fn(*mut c_void, *mut c_void)>;

// Inline helpers and CONFIG_LIVEPATCH fallback behavior depend on external
// kernel flag/configuration definitions and are represented by declarations.
extern "C" {
    pub fn klp_patch_pending(task: *mut task_struct) -> bool;
    pub fn klp_have_reliable_stack() -> bool;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
