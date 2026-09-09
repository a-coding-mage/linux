/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of linux/module.h. Included C dependencies are external. */

pub const MODULE_NAME_LEN: usize = __MODULE_NAME_LEN;

#[repr(C)]
pub struct modversion_info { pub crc: ::core::ffi::c_ulong, pub name: [::core::ffi::c_char; MODULE_NAME_LEN] }

#[repr(C)] pub struct module_kobject { pub kobj: kobject, pub mod_: *mut module, pub drivers_dir: *mut kobject, pub mp: *mut module_param_attrs, pub kobj_completion: *mut completion }
#[repr(C)] pub struct module_attribute {
    pub attr: attribute,
    pub show: Option<unsafe extern "C" fn(*const module_attribute, *mut module_kobject, *mut ::core::ffi::c_char) -> ssize_t>,
    pub store: Option<unsafe extern "C" fn(*const module_attribute, *mut module_kobject, *const ::core::ffi::c_char, usize) -> ssize_t>,
    pub setup: Option<unsafe extern "C" fn(*mut module, *const ::core::ffi::c_char)>,
    pub test: Option<unsafe extern "C" fn(*mut module) -> ::core::ffi::c_int>,
    pub free: Option<unsafe extern "C" fn(*mut module)>,
}
#[repr(C)] pub struct module_version_attribute { pub mattr: module_attribute, pub module_name: *const ::core::ffi::c_char, pub version: *const ::core::ffi::c_char }

extern "C" { pub fn __modver_version_show(*const module_attribute, *mut module_kobject, *mut ::core::ffi::c_char) -> ssize_t; pub static module_uevent: module_attribute; pub fn init_module() -> ::core::ffi::c_int; pub fn cleanup_module(); }

#[repr(C)] pub struct mod_tree_node { pub mod_: *mut module, pub node: latch_tree_node }
#[repr(C)] pub struct module_memory { pub base: *mut ::core::ffi::c_void, pub is_rox: bool, pub size: ::core::ffi::c_uint, pub mtn: mod_tree_node }
#[repr(C)] pub struct mod_kallsyms { pub symtab: *mut Elf_Sym, pub num_symtab: ::core::ffi::c_uint, pub strtab: *mut ::core::ffi::c_char, pub typetab: *mut ::core::ffi::c_char }

#[repr(i32)] pub enum module_state { MODULE_STATE_LIVE, MODULE_STATE_COMING, MODULE_STATE_GOING, MODULE_STATE_UNFORMED }
#[repr(i32)] pub enum mod_mem_type { MOD_TEXT = 0, MOD_DATA, MOD_RODATA, MOD_RO_AFTER_INIT, MOD_INIT_TEXT, MOD_INIT_DATA, MOD_INIT_RODATA, MOD_MEM_NUM_TYPES, MOD_INVALID = -1 }

#[repr(C)] pub struct module {
    pub state: module_state, pub list: list_head, pub name: [::core::ffi::c_char; MODULE_NAME_LEN],
    pub mkobj: module_kobject, pub modinfo_attrs: *mut module_attribute, pub version: *const ::core::ffi::c_char,
    pub srcversion: *const ::core::ffi::c_char, pub imported_namespaces: *const ::core::ffi::c_char, pub holders_dir: *mut kobject,
    pub syms: *const kernel_symbol, pub crcs: *const u32, pub flagstab: *const u8, pub num_syms: ::core::ffi::c_uint,
    pub kp: *mut kernel_param, pub num_kp: ::core::ffi::c_uint, pub using_gplonly_symbols: bool, pub async_probe_requested: bool,
    pub num_exentries: ::core::ffi::c_uint, pub extable: *mut exception_table_entry,
    pub init: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub mem: [module_memory; mod_mem_type::MOD_MEM_NUM_TYPES as usize], pub arch: mod_arch_specific, pub taints: ::core::ffi::c_ulong,
    pub noinstr_text_start: *mut ::core::ffi::c_void, pub noinstr_text_size: ::core::ffi::c_uint,
    pub exit: Option<unsafe extern "C" fn()>, pub refcnt: atomic_t,
}

extern "C" {
    pub fn lookup_or_create_module_kobject(name: *const ::core::ffi::c_char) -> *mut module_kobject;
    pub fn __symbol_get(symbol: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_void;
    pub fn __symbol_get_gpl(symbol: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_void;
    pub fn __module_address(addr: ::core::ffi::c_ulong) -> *mut module;
    pub fn __module_text_address(addr: ::core::ffi::c_ulong) -> *mut module;
    pub fn is_module_address(addr: ::core::ffi::c_ulong) -> bool;
    pub fn __is_module_percpu_address(addr: ::core::ffi::c_ulong, can_addr: *mut ::core::ffi::c_ulong) -> bool;
    pub fn is_module_percpu_address(addr: ::core::ffi::c_ulong) -> bool;
    pub fn is_module_text_address(addr: ::core::ffi::c_ulong) -> bool;
    pub fn find_module(name: *const ::core::ffi::c_char) -> *mut module;
    pub fn register_module_notifier(nb: *mut notifier_block) -> ::core::ffi::c_int;
    pub fn unregister_module_notifier(nb: *mut notifier_block) -> ::core::ffi::c_int;
    pub fn print_modules();
}

#[inline] pub unsafe fn module_is_live(mod_: *const module) -> bool { (*mod_).state != module_state::MODULE_STATE_GOING }
#[inline] pub unsafe fn module_is_coming(mod_: *const module) -> bool { (*mod_).state == module_state::MODULE_STATE_COMING }
#[inline] pub unsafe fn within_module_mem_type(addr: usize, mod_: *const module, ty: mod_mem_type) -> bool { let m = &(*mod_).mem[ty as usize]; addr.wrapping_sub(m.base as usize) < m.size as usize }
#[inline] pub unsafe fn within_module(addr: usize, mod_: *const module) -> bool { within_module_mem_type(addr, mod_, mod_mem_type::MOD_TEXT) || within_module_mem_type(addr, mod_, mod_mem_type::MOD_DATA) }

#[cfg(not(CONFIG_MODULE_UNLOAD))] #[inline] pub unsafe fn try_module_get(mod_: *mut module) -> bool { mod_.is_null() || module_is_live(mod_) }
#[cfg(not(CONFIG_MODULE_UNLOAD))] #[inline] pub unsafe fn module_put(_: *mut module) {}
#[cfg(not(CONFIG_MODULE_UNLOAD))] #[inline] pub unsafe fn __module_get(_: *mut module) {}
#[cfg(not(CONFIG_MODULE_UNLOAD))] #[inline] pub unsafe fn module_requested_async_probing(_: *const module) -> bool { false }

pub type ssize_t = isize;
extern "C" { pub fn dereference_module_function_descriptor(mod_: *mut module, ptr: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void; }

extern "C" {
    pub fn module_refcount(mod_: *mut module) -> ::core::ffi::c_int;
    pub fn __symbol_put(symbol: *const ::core::ffi::c_char);
    pub fn symbol_put_addr(addr: *mut ::core::ffi::c_void);
    pub fn __module_get(mod_: *mut module);
    pub fn try_module_get(mod_: *mut module) -> bool;
    pub fn module_put(mod_: *mut module);
    pub fn module_for_each_mod(func: Option<unsafe extern "C" fn(*mut module, *mut ::core::ffi::c_void) -> ::core::ffi::c_int>, data: *mut ::core::ffi::c_void);
    pub fn is_module_sig_enforced() -> bool;
    pub fn set_module_sig_enforced();
    pub fn module_kallsyms_on_each_symbol(modname: *const ::core::ffi::c_char, fn_: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, usize) -> ::core::ffi::c_int>, data: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    pub fn module_address_lookup(addr: usize, symbolsize: *mut usize, offset: *mut usize, modname: *mut *mut ::core::ffi::c_char, modbuildid: *mut *const u8, namebuf: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn lookup_module_symbol_name(addr: usize, symname: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn lookup_module_symbol_attrs(addr: usize, size: *mut usize, offset: *mut usize, modname: *mut ::core::ffi::c_char, name: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn module_get_kallsym(symnum: u32, value: *mut usize, ty: *mut ::core::ffi::c_char, name: *mut ::core::ffi::c_char, module_name: *mut ::core::ffi::c_char, exported: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn module_kallsyms_lookup_name(name: *const ::core::ffi::c_char) -> usize;
    pub fn find_kallsyms_symbol_value(mod_: *mut module, name: *const ::core::ffi::c_char) -> usize;
}

#[inline] pub unsafe fn module_requested_async_probing(mod_: *const module) -> bool { !mod_.is_null() && (*mod_).async_probe_requested }
#[inline] pub unsafe fn is_livepatch_module(_: *const module) -> bool { false }
#[inline] pub unsafe fn module_sig_ok(_: *const module) -> bool { true }
#[inline] pub unsafe fn retpoline_module_ok(_: bool) -> bool { true }
#[inline] pub unsafe fn module_buildid(_: *const module) -> *const u8 { core::ptr::null() }

// Preprocessor-only metadata, initcall, configuration, and symbol-alias macros retain their intent here.
// MODULE_ALIAS, MODULE_SOFTDEP, MODULE_WEAKDEP, MODULE_LICENSE, MODULE_AUTHOR,
// MODULE_DESCRIPTION, MODULE_VERSION, MODULE_FIRMWARE, MODULE_IMPORT_NS,
// MODULE_DEVICE_TABLE, symbol_get, symbol_put, symbol_request, and MODULE_INFO
// are provided by the corresponding Rust build environment.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
