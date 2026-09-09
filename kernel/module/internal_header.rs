/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Module internals
 *
 * Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 * Copyright (C) 2023 Luis Chamberlain <mcgrof@kernel.org>
 */

// Linux kernel dependencies are supplied by other translated units.

#[cfg(not(feature = "arch_shf_small"))]
pub const ARCH_SHF_SMALL: usize = 0;

pub const SH_ENTSIZE_TYPE_BITS: usize = 4;
pub const SH_ENTSIZE_TYPE_SHIFT: usize = BITS_PER_LONG - SH_ENTSIZE_TYPE_BITS;
pub const SH_ENTSIZE_TYPE_MASK: usize = (1usize << SH_ENTSIZE_TYPE_BITS) - 1;
pub const SH_ENTSIZE_OFFSET_MASK: usize = (1usize << (BITS_PER_LONG - SH_ENTSIZE_TYPE_BITS)) - 1;
pub const MODULE_FLAGS_BUF_SIZE: usize = TAINT_FLAGS_COUNT + 4;

#[repr(C)]
pub struct kernel_symbol {
    #[cfg(feature = "config_have_arch_prel32_relocations")]
    pub value_offset: i32,
    #[cfg(feature = "config_have_arch_prel32_relocations")]
    pub name_offset: i32,
    #[cfg(feature = "config_have_arch_prel32_relocations")]
    pub namespace_offset: i32,
    #[cfg(not(feature = "config_have_arch_prel32_relocations"))]
    pub value: usize,
    #[cfg(not(feature = "config_have_arch_prel32_relocations"))]
    pub name: *const core::ffi::c_char,
    #[cfg(not(feature = "config_have_arch_prel32_relocations"))]
    pub namespace: *const core::ffi::c_char,
}

extern "C" {
    pub static mut module_mutex: mutex;
    pub static mut modules: list_head;
    pub static modinfo_attrs: *const *const module_attribute;
    pub static modinfo_attrs_count: usize;
    pub static __start___ksymtab: kernel_symbol;
    pub static __stop___ksymtab: kernel_symbol;
    pub static __start___kcrctab: u32;
    pub static __start___kflagstab: u8;
    pub static mut modprobe_path: [core::ffi::c_char; KMOD_PATH_LEN];
}

pub const KMOD_PATH_LEN: usize = 256;

#[repr(C)]
pub struct load_info {
    pub name: *const core::ffi::c_char,
    pub modu: *mut module,
    pub hdr: *mut Elf_Ehdr,
    pub len: usize,
    pub sechdrs: *mut Elf_Shdr,
    pub secstrings: *mut core::ffi::c_char,
    pub strtab: *mut core::ffi::c_char,
    pub symoffs: usize,
    pub stroffs: usize,
    pub init_typeoffs: usize,
    pub core_typeoffs: usize,
    pub sig_ok: bool,
    #[cfg(feature = "config_kallsyms")]
    pub mod_kallsyms_init_off: usize,
    #[cfg(feature = "config_module_decompress")]
    #[cfg(feature = "config_module_stats")]
    pub compressed_len: usize,
    #[cfg(feature = "config_module_decompress")]
    pub pages: *mut *mut page,
    #[cfg(feature = "config_module_decompress")]
    pub max_pages: u32,
    #[cfg(feature = "config_module_decompress")]
    pub used_pages: u32,
    pub index: load_info_index,
}

#[repr(C)]
pub struct load_info_index { pub sym: u32, pub str_: u32, pub modu: u32, pub vers: u32, pub info: u32, pub pcpu: u32, pub vers_ext_crc: u32, pub vers_ext_name: u32 }

#[repr(C)]
pub enum mod_license { NOT_GPL_ONLY, GPL_ONLY }

#[repr(C)]
pub struct find_symbol_arg {
    pub name: *const core::ffi::c_char,
    pub gplok: bool,
    pub warn: bool,
    pub owner: *mut module,
    pub crc: *const u32,
    pub sym: *const kernel_symbol,
    pub license: mod_license,
}

#[repr(C)]
pub struct module_use { pub source_list: list_head, pub target_list: list_head, pub source: *mut module, pub target: *mut module }

extern "C" {
    pub fn mod_verify_sig(modu: *const core::ffi::c_void, info: *mut load_info) -> i32;
    pub fn try_to_force_load(modu: *mut module, reason: *const core::ffi::c_char) -> i32;
    pub fn find_symbol(fsa: *mut find_symbol_arg) -> bool;
    pub fn find_module_all(name: *const core::ffi::c_char, len: usize, even_unformed: bool) -> *mut module;
    pub fn cmp_name(name: *const core::ffi::c_void, sym: *const core::ffi::c_void) -> i32;
    pub fn module_get_offset_and_type(modu: *mut module, ty: mod_mem_type, sechdr: *mut Elf_Shdr, section: u32) -> i64;
    pub fn module_flags(modu: *mut module, buf: *mut core::ffi::c_char, show_state: bool) -> *mut core::ffi::c_char;
    pub fn module_flags_taint(taints: usize, buf: *mut core::ffi::c_char) -> usize;
    pub fn module_next_tag_pair(string: *mut core::ffi::c_char, secsize: *mut usize) -> *mut core::ffi::c_char;
}

pub unsafe fn kernel_symbol_value(sym: *const kernel_symbol) -> usize {
    #[cfg(feature = "config_have_arch_prel32_relocations")]
    { offset_to_ptr(&(*sym).value_offset) as usize }
    #[cfg(not(feature = "config_have_arch_prel32_relocations"))]
    { (*sym).value }
}

#[cfg(feature = "config_livepatch")]
extern "C" { pub fn copy_module_elf(modu: *mut module, info: *mut load_info) -> i32; pub fn free_module_elf(modu: *mut module); }
#[cfg(not(feature = "config_livepatch"))]
pub unsafe fn copy_module_elf(_: *mut module, _: *mut load_info) -> i32 { 0 }
#[cfg(not(feature = "config_livepatch"))]
pub unsafe fn free_module_elf(_: *mut module) {}

pub unsafe fn set_livepatch_module(modu: *mut module) -> bool {
    #[cfg(feature = "config_livepatch")] { (*modu).klp = true; true }
    #[cfg(not(feature = "config_livepatch"))] { let _ = modu; false }
}

#[repr(C)]
pub enum fail_dup_mod_reason { FAIL_DUP_MOD_BECOMING = 0, FAIL_DUP_MOD_LOAD }

#[cfg(feature = "config_module_debugfs")]
extern "C" { pub static mut mod_debugfs_root: *mut dentry; }

#[cfg(feature = "config_module_stats")]
extern "C" {
    pub static mut total_mod_size: atomic_long_t;
    pub static mut total_text_size: atomic_long_t;
    pub static mut invalid_kread_bytes: atomic_long_t;
    pub static mut invalid_decompress_bytes: atomic_long_t;
    pub static mut modcount: atomic_t;
    pub static mut failed_kreads: atomic_t;
    pub static mut failed_decompress: atomic_t;
}

#[cfg(feature = "config_module_stats")]
#[repr(C)]
pub struct mod_fail_load { pub list: list_head, pub name: [core::ffi::c_char; MODULE_NAME_LEN], pub count: atomic_long_t, pub dup_fail_mask: usize }

#[cfg(not(feature = "config_module_stats"))]
pub unsafe fn try_add_failed_module(_: *const core::ffi::c_char, _: fail_dup_mod_reason) -> i32 { 0 }
#[cfg(feature = "config_module_stats")]
extern "C" { pub fn try_add_failed_module(name: *const core::ffi::c_char, reason: fail_dup_mod_reason) -> i32; pub fn mod_stat_bump_invalid(info: *mut load_info, flags: i32); pub fn mod_stat_bump_becoming(info: *mut load_info, flags: i32); }
#[cfg(not(feature = "config_module_stats"))]
pub unsafe fn mod_stat_bump_invalid(_: *mut load_info, _: i32) {}
#[cfg(not(feature = "config_module_stats"))]
pub unsafe fn mod_stat_bump_becoming(_: *mut load_info, _: i32) {}

#[cfg(feature = "config_module_debug_autoload_dups")]
extern "C" { pub fn kmod_dup_request_exists_wait(module_name: *mut core::ffi::c_char, wait: bool, dup_ret: *mut i32) -> bool; pub fn kmod_dup_request_announce(module_name: *mut core::ffi::c_char, ret: i32); }
#[cfg(not(feature = "config_module_debug_autoload_dups"))]
pub unsafe fn kmod_dup_request_exists_wait(_: *mut core::ffi::c_char, _: bool, _: *mut i32) -> bool { false }
#[cfg(not(feature = "config_module_debug_autoload_dups"))]
pub unsafe fn kmod_dup_request_announce(_: *mut core::ffi::c_char, _: i32) {}

#[cfg(feature = "config_module_unload_taint_tracking")]
#[repr(C)]
pub struct mod_unload_taint { pub list: list_head, pub name: [core::ffi::c_char; MODULE_NAME_LEN], pub taints: usize, pub count: u64 }

#[cfg(not(feature = "config_module_unload_taint_tracking"))]
pub unsafe fn try_add_tainted_module(_: *mut module) -> i32 { 0 }
#[cfg(not(feature = "config_module_unload_taint_tracking"))]
pub unsafe fn print_unloaded_tainted_modules() {}
#[cfg(feature = "config_module_unload_taint_tracking")]
extern "C" { pub fn try_add_tainted_module(modu: *mut module) -> i32; pub fn print_unloaded_tainted_modules(); }

#[cfg(feature = "config_module_decompress")]
extern "C" { pub fn module_decompress(info: *mut load_info, buf: *const core::ffi::c_void, size: usize) -> i32; pub fn module_decompress_cleanup(info: *mut load_info); }
#[cfg(not(feature = "config_module_decompress"))]
pub unsafe fn module_decompress(_: *mut load_info, _: *const core::ffi::c_void, _: usize) -> i32 { -EOPNOTSUPP }
#[cfg(not(feature = "config_module_decompress"))]
pub unsafe fn module_decompress_cleanup(_: *mut load_info) {}

#[repr(C)]
pub struct mod_tree_root {
    #[cfg(feature = "config_modules_tree_lookup")] pub root: latch_tree_root,
    pub addr_min: usize,
    pub addr_max: usize,
    #[cfg(feature = "config_arch_wants_modules_data_in_vmalloc")] pub data_addr_min: usize,
    #[cfg(feature = "config_arch_wants_modules_data_in_vmalloc")] pub data_addr_max: usize,
}
extern "C" { pub static mut mod_tree: mod_tree_root; }

#[cfg(feature = "config_modules_tree_lookup")]
extern "C" { pub fn mod_tree_insert(modu: *mut module); pub fn mod_tree_remove_init(modu: *mut module); pub fn mod_tree_remove(modu: *mut module); pub fn mod_find(addr: usize, tree: *mut mod_tree_root) -> *mut module; }
#[cfg(not(feature = "config_modules_tree_lookup"))]
pub unsafe fn mod_tree_insert(_: *mut module) {}
#[cfg(not(feature = "config_modules_tree_lookup"))]
pub unsafe fn mod_tree_remove_init(_: *mut module) {}
#[cfg(not(feature = "config_modules_tree_lookup"))]
pub unsafe fn mod_tree_remove(_: *mut module) {}
#[cfg(not(feature = "config_modules_tree_lookup"))]
pub unsafe fn mod_find(_: usize, _: *mut mod_tree_root) -> *mut module { core::ptr::null_mut() }

extern "C" {
    pub fn module_enable_rodata_ro(modu: *const module) -> i32;
    pub fn module_enable_rodata_ro_after_init(modu: *const module) -> i32;
    pub fn module_enable_data_nx(modu: *const module) -> i32;
    pub fn module_enable_text_rox(modu: *const module) -> i32;
    pub fn module_enforce_rwx_sections(hdr: *const Elf_Ehdr, sechdrs: *const Elf_Shdr, secstrings: *const core::ffi::c_char, modu: *const module) -> i32;
    pub fn module_mark_ro_after_init(hdr: *const Elf_Ehdr, sechdrs: *mut Elf_Shdr, secstrings: *const core::ffi::c_char);
}

#[cfg(feature = "config_module_sig")]
extern "C" { pub fn module_sig_check(info: *mut load_info, flags: i32) -> i32; }
#[cfg(not(feature = "config_module_sig"))]
pub unsafe fn module_sig_check(_: *mut load_info, _: i32) -> i32 { 0 }

#[cfg(feature = "config_debug_kmemleak")]
extern "C" { pub fn kmemleak_load_module(modu: *const module, info: *const load_info); }
#[cfg(not(feature = "config_debug_kmemleak"))]
pub unsafe fn kmemleak_load_module(_: *const module, _: *const load_info) {}

#[cfg(feature = "config_kallsyms")]
extern "C" { pub fn init_build_id(modu: *mut module, info: *const load_info); pub fn layout_symtab(modu: *mut module, info: *mut load_info); pub fn add_kallsyms(modu: *mut module, info: *const load_info); }
#[cfg(feature = "config_kallsyms")]
pub unsafe fn sect_empty(sect: *const Elf_Shdr) -> bool { ((*sect).sh_flags & SHF_ALLOC) == 0 || (*sect).sh_size == 0 }
#[cfg(not(feature = "config_kallsyms"))]
pub unsafe fn init_build_id(_: *mut module, _: *const load_info) {}
#[cfg(not(feature = "config_kallsyms"))]
pub unsafe fn layout_symtab(_: *mut module, _: *mut load_info) {}
#[cfg(not(feature = "config_kallsyms"))]
pub unsafe fn add_kallsyms(_: *mut module, _: *const load_info) {}

#[cfg(feature = "config_sysfs")]
extern "C" { pub fn mod_sysfs_setup(modu: *mut module, info: *const load_info, kparam: *mut kernel_param, num_params: u32) -> i32; pub fn mod_sysfs_teardown(modu: *mut module); pub fn init_param_lock(modu: *mut module); }
#[cfg(not(feature = "config_sysfs"))]
pub unsafe fn mod_sysfs_setup(_: *mut module, _: *const load_info, _: *mut kernel_param, _: u32) -> i32 { 0 }
#[cfg(not(feature = "config_sysfs"))]
pub unsafe fn mod_sysfs_teardown(_: *mut module) {}
#[cfg(not(feature = "config_sysfs"))]
pub unsafe fn init_param_lock(_: *mut module) {}

#[cfg(feature = "config_modversions")]
extern "C" { pub fn check_version(info: *const load_info, symname: *const core::ffi::c_char, modu: *mut module, crc: *const u32) -> i32; pub fn module_layout(modu: *mut module, ver: *mut modversion_info, kp: *mut kernel_param, ks: *mut kernel_symbol, tp: *const *const tracepoint); pub fn check_modstruct_version(info: *const load_info, modu: *mut module) -> i32; pub fn same_magic(amagic: *const core::ffi::c_char, bmagic: *const core::ffi::c_char, has_crcs: bool) -> i32; pub fn modversion_ext_start(info: *const load_info, ver: *mut modversion_info_ext); pub fn modversion_ext_advance(ver: *mut modversion_info_ext); }
#[repr(C)]
pub struct modversion_info_ext { pub remaining: usize, pub crc: *const u32, pub name: *const core::ffi::c_char }
#[cfg(not(feature = "config_modversions"))]
pub unsafe fn check_version(_: *const load_info, _: *const core::ffi::c_char, _: *mut module, _: *const u32) -> i32 { 1 }
#[cfg(not(feature = "config_modversions"))]
pub unsafe fn check_modstruct_version(_: *const load_info, _: *mut module) -> i32 { 1 }
#[cfg(not(feature = "config_modversions"))]
pub unsafe fn same_magic(a: *const core::ffi::c_char, b: *const core::ffi::c_char, _: bool) -> i32 { strcmp(a, b) == 0 as i32 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
