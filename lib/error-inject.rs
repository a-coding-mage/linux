// SPDX-License-Identifier: GPL-2.0
// error-inject.c: Function-level error injection table
// Kernel dependencies supplied by the surrounding build are intentionally not
// redefined here.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct error_injection_entry {
    pub addr: *const c_void,
    pub etype: c_int,
}

#[repr(C)]
pub struct module {
    pub num_ei_funcs: usize,
    pub ei_funcs: *mut error_injection_entry,
}

#[repr(C)]
pub struct notifier_block {
    pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, usize, *mut c_void) -> c_int>,
    pub priority: c_int,
}

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct seq_operations {
    pub start: Option<unsafe extern "C" fn(*mut seq_file, *mut i64) -> *mut c_void>,
    pub next: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void, *mut i64) -> *mut c_void>,
    pub stop: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void)>,
    pub show: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void) -> c_int>,
}

extern "C" {
    static mut error_injection_list: list_head;
    static mut ei_mutex: mutex;
    static mut __start_error_injection_whitelist: error_injection_entry;
    static mut __stop_error_injection_whitelist: error_injection_entry;

    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn dereference_symbol_descriptor(addr: *mut c_void) -> *mut c_void;
    fn kernel_text_address(addr: usize) -> bool;
    fn kallsyms_lookup_size_offset(addr: usize, size: *mut usize, offset: *mut usize) -> bool;
    fn pr_err(format: *const c_char, ...);
    fn kmalloc(size: usize, flags: usize) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn debugfs_create_dir(name: *const c_char, parent: *mut dentry) -> *mut dentry;
    fn debugfs_create_file(name: *const c_char, mode: u32, parent: *mut dentry,
                           data: *mut c_void, fops: *const c_void) -> *mut dentry;
    fn debugfs_remove(dentry: *mut dentry);
    fn seq_list_start(list: *mut list_head, pos: i64) -> *mut c_void;
    fn seq_list_next(v: *mut c_void, list: *mut list_head, pos: *mut i64) -> *mut c_void;
    fn seq_printf(m: *mut seq_file, format: *const c_char, ...);
    fn register_module_notifier(nb: *mut notifier_block) -> c_int;
}

const EINVAL: c_int = 22;
const EI_ETYPE_NULL: c_int = 0;
const EI_ETYPE_ERRNO: c_int = 1;
const EI_ETYPE_ERRNO_NULL: c_int = 2;
const EI_ETYPE_TRUE: c_int = 3;
const MODULE_STATE_COMING: usize = 1;
const MODULE_STATE_GOING: usize = 2;
const NOTIFY_DONE: c_int = 0;

#[repr(C)]
struct ei_entry {
    list: list_head,
    start_addr: usize,
    end_addr: usize,
    etype: c_int,
    priv_: *mut c_void,
}

unsafe fn within_error_injection_list(addr: usize) -> bool {
    let mut ent: *mut ei_entry;
    let mut ret = false;
    mutex_lock(&mut ei_mutex);
    let mut node = error_injection_list.next;
    while node != &mut error_injection_list as *mut list_head {
        ent = (node as *mut u8).sub(core::mem::offset_of!(ei_entry, list)) as *mut ei_entry;
        if addr >= (*ent).start_addr && addr < (*ent).end_addr { ret = true; break; }
        node = (*node).next;
    }
    mutex_unlock(&mut ei_mutex);
    ret
}

unsafe fn get_injectable_error_type(addr: usize) -> c_int {
    let mut ei_type = -EINVAL;
    mutex_lock(&mut ei_mutex);
    let mut node = error_injection_list.next;
    while node != &mut error_injection_list as *mut list_head {
        let ent = (node as *mut u8).sub(core::mem::offset_of!(ei_entry, list)) as *mut ei_entry;
        if addr >= (*ent).start_addr && addr < (*ent).end_addr { ei_type = (*ent).etype; break; }
        node = (*node).next;
    }
    mutex_unlock(&mut ei_mutex);
    ei_type
}

unsafe fn populate_error_injection_list(start: *mut error_injection_entry,
                                        end: *mut error_injection_entry,
                                        priv_: *mut c_void) {
    mutex_lock(&mut ei_mutex);
    let mut iter = start;
    let mut offset = 0usize;
    let mut size = 0usize;
    while iter < end {
        let entry = dereference_symbol_descriptor((*iter).addr as *mut c_void) as usize;
        if !kernel_text_address(entry) || !kallsyms_lookup_size_offset(entry, &mut size, &mut offset) {
            pr_err(b"Failed to find error inject entry at %p\0".as_ptr() as *const c_char, entry as *mut c_void);
            iter = iter.add(1);
            continue;
        }
        let ent = kmalloc(core::mem::size_of::<ei_entry>(), 0) as *mut ei_entry;
        if ent.is_null() { break; }
        (*ent).start_addr = entry;
        (*ent).end_addr = entry + size;
        (*ent).etype = (*iter).etype;
        (*ent).priv_ = priv_;
        (*ent).list.next = &mut (*ent).list;
        (*ent).list.prev = &mut (*ent).list;
        let tail = error_injection_list.prev;
        (*ent).list.next = &mut error_injection_list;
        (*ent).list.prev = tail;
        (*tail).next = &mut (*ent).list;
        error_injection_list.prev = &mut (*ent).list;
        iter = iter.add(1);
    }
    mutex_unlock(&mut ei_mutex);
}

unsafe fn populate_kernel_ei_list() {
    populate_error_injection_list(&mut __start_error_injection_whitelist,
        &mut __stop_error_injection_whitelist, core::ptr::null_mut());
}

#[cfg(feature = "CONFIG_MODULES")]
unsafe fn module_load_ei_list(mod_: *mut module) {
    if (*mod_).num_ei_funcs == 0 { return; }
    populate_error_injection_list((*mod_).ei_funcs,
        (*mod_).ei_funcs.add((*mod_).num_ei_funcs), mod_ as *mut c_void);
}

#[cfg(feature = "CONFIG_MODULES")]
unsafe fn module_unload_ei_list(mod_: *mut module) {
    if (*mod_).num_ei_funcs == 0 { return; }
    mutex_lock(&mut ei_mutex);
    let mut node = error_injection_list.next;
    while node != &mut error_injection_list as *mut list_head {
        let next = (*node).next;
        let ent = (node as *mut u8).sub(core::mem::offset_of!(ei_entry, list)) as *mut ei_entry;
        if (*ent).priv_ == mod_ as *mut c_void { (*node).next = node; (*node).prev = node; kfree(ent as *mut c_void); }
        node = next;
    }
    mutex_unlock(&mut ei_mutex);
}

#[cfg(feature = "CONFIG_MODULES")]
unsafe extern "C" fn ei_module_callback(_: *mut notifier_block, val: usize, data: *mut c_void) -> c_int {
    let mod_ = data as *mut module;
    if val == MODULE_STATE_COMING { module_load_ei_list(mod_); }
    else if val == MODULE_STATE_GOING { module_unload_ei_list(mod_); }
    NOTIFY_DONE
}

#[cfg(feature = "CONFIG_MODULES")]
static mut ei_module_nb: notifier_block = notifier_block { notifier_call: Some(ei_module_callback), priority: 0 };

#[cfg(feature = "CONFIG_MODULES")]
unsafe fn module_ei_init() -> c_int { register_module_notifier(&mut ei_module_nb) }
#[cfg(not(feature = "CONFIG_MODULES"))]
unsafe fn module_ei_init() -> c_int { 0 }

unsafe fn ei_seq_start(_: *mut seq_file, pos: *mut i64) -> *mut c_void { mutex_lock(&mut ei_mutex); seq_list_start(&mut error_injection_list, *pos) }
unsafe fn ei_seq_stop(_: *mut seq_file, _: *mut c_void) { mutex_unlock(&mut ei_mutex); }
unsafe fn ei_seq_next(_: *mut seq_file, v: *mut c_void, pos: *mut i64) -> *mut c_void { seq_list_next(v, &mut error_injection_list, pos) }

unsafe fn error_type_string(etype: c_int) -> &'static [u8] {
    match etype { EI_ETYPE_NULL => b"NULL\0", EI_ETYPE_ERRNO => b"ERRNO\0", EI_ETYPE_ERRNO_NULL => b"ERRNO_NULL\0", EI_ETYPE_TRUE => b"TRUE\0", _ => b"(unknown)\0" }
}

unsafe fn ei_seq_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    let ent = (v as *mut u8).sub(core::mem::offset_of!(ei_entry, list)) as *mut ei_entry;
    seq_printf(m, b"%ps\t%s\n\0".as_ptr() as *const c_char, (*ent).start_addr as *mut c_void, error_type_string((*ent).etype).as_ptr());
    0
}

static ei_sops: seq_operations = seq_operations { start: Some(ei_seq_start), next: Some(ei_seq_next), stop: Some(ei_seq_stop), show: Some(ei_seq_show) };

unsafe fn ei_debugfs_init() -> c_int {
    let dir = debugfs_create_dir(b"error_injection\0".as_ptr() as *const c_char, core::ptr::null_mut());
    let file = debugfs_create_file(b"list\0".as_ptr() as *const c_char, 0o444, dir, core::ptr::null_mut(), &ei_sops as *const _ as *const c_void);
    if file.is_null() { debugfs_remove(dir); return -1; }
    0
}

unsafe fn init_error_injection() -> c_int {
    populate_kernel_ei_list();
    if module_ei_init() == 0 { ei_debugfs_init(); }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
