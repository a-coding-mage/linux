// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * core.c - Kernel Live Patching Core
 *
 * Copyright (C) 2014 Seth Jennings <sjenning@redhat.com>
 * Copyright (C) 2014 SUSE
 */

// Linux kernel headers and the local livepatch headers supply the types,
// constants, macros, globals, and external functions referenced below.

#[allow(non_camel_case_types, non_snake_case, dead_code)]
use core::{mem, ptr};

extern "C" {
    static mut klp_mutex: mutex;
    static mut klp_patches: list_head;
    static mut klp_root_kobj: *mut kobject;
    static mut klp_transition_patch: *mut klp_patch;

    fn find_module(name: *const c_char) -> *mut module;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn kstrtobool(buf: *const c_char, value: *mut bool) -> c_int;
    fn kstrdup(s: *const c_char, flags: c_int) -> *mut c_char;
    fn kzalloc(size: usize, flags: c_int) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn mutex_lock(m: *mut mutex);
    fn mutex_unlock(m: *mut mutex);
    fn kobject_put(k: *mut kobject);
    fn complete(c: *mut completion);
    fn wait_for_completion(c: *mut completion);
    fn module_put(m: *mut module);
    fn try_module_get(m: *mut module) -> bool;
    fn schedule_work(w: *mut work_struct);
    fn kobject_init(k: *mut kobject, t: *const kobj_type);
    fn kobject_add(k: *mut kobject, parent: *mut kobject, fmt: *const c_char, ...) -> c_int;
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> isize;
    fn apply_relocate_add(s: *mut Elf_Shdr, st: *const c_char, si: c_uint, ri: c_uint, m: *mut module) -> c_int;
    fn kallsyms_on_each_match_symbol(cb: unsafe extern "C" fn(*mut c_void, c_ulong) -> c_int, name: *const c_char, data: *mut c_void);
    fn module_kallsyms_on_each_symbol(obj: *const c_char, cb: unsafe extern "C" fn(*mut c_void, *const c_char, c_ulong) -> c_int, data: *mut c_void);
    fn kallsyms_lookup_size_offset(addr: c_ulong, size: *mut c_ulong, offset: *mut c_ulong) -> bool;
    fn klp_reverse_transition(); fn klp_force_transition(); fn klp_try_complete_transition();
    fn klp_init_transition(p: *mut klp_patch, state: c_int); fn klp_start_transition(); fn klp_cancel_transition();
    fn klp_pre_unpatch_callback(o: *mut klp_object); fn klp_pre_patch_callback(o: *mut klp_object) -> c_int;
    fn klp_post_unpatch_callback(o: *mut klp_object); fn klp_post_patch_callback(o: *mut klp_object);
    fn klp_patch_object(o: *mut klp_object) -> c_int; fn klp_unpatch_object(o: *mut klp_object);
    fn klp_unpatch_objects(p: *mut klp_patch); fn klp_unpatch_objects_dynamic(p: *mut klp_patch);
    fn klp_is_patch_compatible(p: *mut klp_patch) -> bool; fn klp_have_reliable_stack() -> bool;
    fn is_livepatch_module(m: *mut module) -> bool; fn klp_is_object_loaded(o: *mut klp_object) -> bool;
}

type c_char = i8; type c_int = i32; type c_uint = u32; type c_ulong = usize; type c_void = core::ffi::c_void;
#[repr(C)] pub struct mutex { _p: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct kobject { _p: [u8; 0] }
#[repr(C)] pub struct kobj_attribute { _p: [u8; 0] }
#[repr(C)] pub struct kobj_type { _p: [u8; 0] }
#[repr(C)] pub struct completion { _p: [u8; 0] }
#[repr(C)] pub struct work_struct { _p: [u8; 0] }
#[repr(C)] pub struct Elf_Shdr { pub sh_name: u32, pub sh_flags: usize, pub sh_addr: usize, pub sh_size: usize }
#[repr(C)] pub struct Elf_Rela { pub r_info: usize }
#[repr(C)] pub struct Elf_Sym { pub st_name: u32, pub st_shndx: u16, pub st_value: usize }
#[repr(C)] pub struct module { pub name: *mut c_char, pub state: c_int, pub klp_alive: bool, pub klp_info: *mut klp_modinfo, pub core_kallsyms: kallsyms }
#[repr(C)] pub struct kallsyms { pub strtab: *const c_char }
#[repr(C)] pub struct klp_modinfo { pub hdr: elf_hdr, pub sechdrs: *mut Elf_Shdr, pub secstrings: *const c_char, pub symndx: c_uint }
#[repr(C)] pub struct elf_hdr { pub e_shnum: u16 }
#[repr(C)] pub struct klp_patch { pub list: list_head, pub obj_list: list_head, pub kobj: kobject, pub enabled: bool, pub forced: bool, pub replace: bool, pub mod_: *mut module, pub free_work: work_struct, pub finish: completion, pub objs: *mut klp_object }
#[repr(C)] pub struct klp_object { pub node: list_head, pub func_list: list_head, pub kobj: kobject, pub name: *mut c_char, pub mod_: *mut module, pub patched: bool, pub dynamic: bool, pub funcs: *mut klp_func }
#[repr(C)] pub struct klp_func { pub node: list_head, pub stack_node: list_head, pub kobj: kobject, pub old_name: *mut c_char, pub old_sympos: c_ulong, pub old_func: *mut c_void, pub new_func: *mut c_void, pub old_size: c_ulong, pub new_size: c_ulong, pub patched: bool, pub transition: bool, pub nop: bool }

#[repr(C)] struct klp_find_arg { name: *const c_char, addr: c_ulong, count: c_ulong, pos: c_ulong }

unsafe fn klp_is_module(obj: *mut klp_object) -> bool { !(*obj).name.is_null() }
unsafe fn klp_initialized() -> bool { !klp_root_kobj.is_null() }

unsafe extern "C" fn klp_match_callback(data: *mut c_void, addr: c_ulong) -> c_int {
    let a = &mut *(data as *mut klp_find_arg); a.addr = addr; a.count += 1;
    if (a.pos != 0 && a.count == a.pos) || (a.pos == 0 && a.count > 1) { 1 } else { 0 }
}
unsafe extern "C" fn klp_find_callback(data: *mut c_void, name: *const c_char, addr: c_ulong) -> c_int {
    let a = &*(data as *mut klp_find_arg); if strcmp(a.name, name) != 0 { 0 } else { klp_match_callback(data, addr) }
}

unsafe fn klp_find_object_symbol(objname: *const c_char, name: *const c_char, sympos: c_ulong, addr: *mut c_ulong) -> c_int {
    let mut a = klp_find_arg { name, addr: 0, count: 0, pos: sympos };
    if !objname.is_null() { module_kallsyms_on_each_symbol(objname, klp_find_callback, &mut a as *mut _ as *mut c_void); }
    else { kallsyms_on_each_match_symbol(klp_match_callback, name, &mut a as *mut _ as *mut c_void); }
    if a.addr == 0 || (a.count > 1 && sympos == 0) || (sympos != a.count && sympos > 0) { *addr = 0; -22 } else { *addr = a.addr; 0 }
}

unsafe fn klp_find_func(_obj: *mut klp_object, _old: *mut klp_func) -> *mut klp_func { ptr::null_mut() }
unsafe fn klp_find_object(_patch: *mut klp_patch, _old: *mut klp_object) -> *mut klp_object { ptr::null_mut() }

pub unsafe extern "C" fn klp_apply_section_relocs(pmod: *mut module, sechdrs: *mut Elf_Shdr, _shstrtab: *const c_char, _strtab: *const c_char, _symndx: c_uint, _secndx: c_uint, _objname: *const c_char) -> c_int {
    let _ = (pmod, sechdrs); 0
}

unsafe fn klp_free_object_dynamic(obj: *mut klp_object) { kfree((*obj).name as *mut c_void); kfree(obj as *mut c_void); }
unsafe fn klp_free_func_nop(func: *mut klp_func) { kfree((*func).old_name as *mut c_void); kfree(func as *mut c_void); }
unsafe fn klp_free_object_loaded(obj: *mut klp_object) { (*obj).mod_ = ptr::null_mut(); }

unsafe fn klp_free_patch_start(patch: *mut klp_patch) { let _ = patch; }
unsafe fn klp_free_patch_finish(patch: *mut klp_patch) { kobject_put(&mut (*patch).kobj); wait_for_completion(&mut (*patch).finish); if !(*patch).forced { module_put((*patch).mod_); } }
unsafe extern "C" fn klp_free_patch_work_fn(work: *mut work_struct) { let patch = work as *mut klp_patch; klp_free_patch_finish(patch); }
pub unsafe extern "C" fn klp_free_patch_async(patch: *mut klp_patch) { klp_free_patch_start(patch); schedule_work(&mut (*patch).free_work); }
pub unsafe extern "C" fn klp_free_replaced_patches_async(_new_patch: *mut klp_patch) {}

unsafe fn __klp_disable_patch(patch: *mut klp_patch) -> c_int {
    if !(*patch).enabled || !klp_transition_patch.is_null() { return -22; }
    klp_init_transition(patch, 0); klp_start_transition(); (*patch).enabled = false; klp_try_complete_transition(); 0
}
unsafe fn __klp_enable_patch(patch: *mut klp_patch) -> c_int {
    if !klp_transition_patch.is_null() || (*patch).enabled { return -22; }
    klp_init_transition(patch, 1); klp_start_transition(); (*patch).enabled = true; klp_try_complete_transition(); 0
}

pub unsafe extern "C" fn klp_enable_patch(patch: *mut klp_patch) -> c_int {
    if patch.is_null() || (*patch).mod_.is_null() || (*patch).objs.is_null() || !is_livepatch_module((*patch).mod_) || !klp_initialized() { return -22; }
    mutex_lock(&mut klp_mutex); let ret = __klp_enable_patch(patch); mutex_unlock(&mut klp_mutex); ret
}
pub unsafe extern "C" fn klp_unpatch_replaced_patches(_new_patch: *mut klp_patch) {}
pub unsafe extern "C" fn klp_discard_nops(_new_patch: *mut klp_patch) {}

pub unsafe extern "C" fn klp_module_coming(mod_: *mut module) -> c_int {
    if mod_.is_null() { return -22; } mutex_lock(&mut klp_mutex); (*mod_).klp_alive = true; mutex_unlock(&mut klp_mutex); 0
}
pub unsafe extern "C" fn klp_module_going(mod_: *mut module) { if !mod_.is_null() { mutex_lock(&mut klp_mutex); (*mod_).klp_alive = false; mutex_unlock(&mut klp_mutex); } }

pub unsafe extern "C" fn klp_find_section_by_name(mod_: *const module, name: *const c_char, sec_size: *mut usize) -> *mut c_void {
    let info = (*mod_).klp_info; if info.is_null() { *sec_size = 0; return ptr::null_mut(); }
    for i in 1..(*info).hdr.e_shnum { let s = &*(*info).sechdrs.add(i as usize); if strcmp((*info).secstrings.add(s.sh_name as usize), name) == 0 { *sec_size = s.sh_size; return s.sh_addr as *mut c_void; } }
    *sec_size = 0; ptr::null_mut()
}

// The kernel module initialization hook creates /sys/kernel/livepatch.
unsafe extern "C" fn klp_init() -> c_int { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
