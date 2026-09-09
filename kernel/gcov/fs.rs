// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of gcov/fs.c. Kernel and gcov dependencies are external. */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

type SizeT = usize;
type LoffT = i64;
type SsizeT = isize;

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct gcov_info { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct inode { pub i_private: *mut c_void }
#[repr(C)] pub struct file { pub private_data: *mut c_void }
#[repr(C)] pub struct seq_file { pub private: *mut c_void }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct file_operations { _private: [u8; 0] }
#[repr(C)] pub struct seq_operations { _private: [u8; 0] }
#[repr(C)] pub struct gcov_link { pub dir: c_int, pub ext: *const c_char }

extern "C" {
    static mut gcov_link: [gcov_link; 0];
    static mut node_lock: mutex;
    static mut all_head: list_head;
    static gcov_link: [gcov_link; 0];
    fn convert_to_gcda(buffer: *mut c_char, info: *mut gcov_info) -> SizeT;
    fn gcov_info_dup(info: *mut gcov_info) -> *mut gcov_info;
    fn gcov_info_add(dst: *mut gcov_info, src: *mut gcov_info);
    fn gcov_info_free(info: *mut gcov_info);
    fn gcov_info_reset(info: *mut gcov_info);
    fn gcov_info_filename(info: *mut gcov_info) -> *const c_char;
    fn gcov_info_is_compatible(a: *mut gcov_info, b: *mut gcov_info) -> bool;
    fn gcov_enable_events();
    fn kstrtoul(s: *const c_char, base: c_int, value: *mut c_ulong) -> c_int;
    fn kstrdup(s: *const c_char, flags: c_ulong) -> *mut c_char;
    fn kasprintf(flags: c_ulong, fmt: *const c_char, ...) -> *mut c_char;
    fn kbasename(path: *const c_char) -> *const c_char;
    fn kfree(ptr: *mut c_void);
    fn kvfree(ptr: *mut c_void);
    fn kzalloc(size: SizeT, flags: c_ulong) -> *mut c_void;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn seq_write(seq: *mut seq_file, data: *const c_char, len: SizeT) -> c_int;
    fn seq_open(file: *mut file, ops: *const seq_operations) -> c_int;
    fn seq_release(inode: *mut inode, file: *mut file) -> c_int;
    fn seq_read(file: *mut file, data: *mut c_char, len: SizeT, pos: *mut LoFF) -> SsizeT;
    fn seq_lseek(file: *mut file, pos: LoffT, whence: c_int) -> LoFF;
    fn debugfs_create_dir(name: *const c_char, parent: *mut dentry) -> *mut dentry;
    fn debugfs_create_file(name: *const c_char, mode: c_int, parent: *mut dentry, data: *mut c_void, ops: *const file_operations) -> *mut dentry;
    fn debugfs_create_symlink(name: *const c_char, parent: *mut dentry, target: *const c_char) -> *mut dentry;
    fn debugfs_remove(entry: *mut dentry);
}

type LoFF = LoFFAlias;
type LoFFAlias = i64;
const PAGE_SIZE: SizeT = 4096;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENOENT: c_int = 2;
const SRC_TREE: c_int = 0;
const GFP_KERNEL: c_ulong = 0;

#[repr(C)]
struct gcov_node {
    list: list_head, children: list_head, all: list_head,
    parent: *mut gcov_node, loaded_info: *mut *mut gcov_info,
    unloaded_info: *mut gcov_info, dentry: *mut dentry,
    links: *mut *mut dentry, num_loaded: c_int, name: [c_char; 0],
}

#[repr(C)] struct gcov_iterator { info: *mut gcov_info, size: SizeT, pos: LoFF, buffer: [c_char; 0] }

static mut objtree: *const c_char = b"OBJTREE\0".as_ptr() as *const c_char;
static mut srctree: *const c_char = b"SRCTREE\0".as_ptr() as *const c_char;
static mut root_node: gcov_node = unsafe { core::mem::zeroed() };
static mut gcov_persist: c_int = 1;

unsafe fn gcov_iter_new(info: *mut gcov_info) -> *mut gcov_iterator {
    let size = convert_to_gcda(core::ptr::null_mut(), info);
    let iter = kzalloc(core::mem::size_of::<gcov_iterator>() + size, GFP_KERNEL) as *mut gcov_iterator;
    if iter.is_null() { return core::ptr::null_mut(); }
    (*iter).info = info; (*iter).size = size;
    convert_to_gcda((*iter).buffer.as_mut_ptr(), info); iter
}
unsafe fn gcov_iter_free(iter: *mut gcov_iterator) { kvfree(iter as *mut c_void); }
unsafe fn gcov_iter_get_info(iter: *mut gcov_iterator) -> *mut gcov_info { (*iter).info }
unsafe fn gcov_iter_start(iter: *mut gcov_iterator) { (*iter).pos = 0; }
unsafe fn gcov_iter_next(iter: *mut gcov_iterator) -> c_int {
    if (*iter).pos < (*iter).size as i64 { (*iter).pos += PAGE_SIZE as i64; }
    if (*iter).pos >= (*iter).size as i64 { -EINVAL } else { 0 }
}
unsafe fn gcov_iter_write(iter: *mut gcov_iterator, seq: *mut seq_file) -> c_int {
    if (*iter).pos >= (*iter).size as i64 { return -EINVAL; }
    let mut len = PAGE_SIZE; if (*iter).pos as usize + len > (*iter).size { len = (*iter).size - (*iter).pos as usize; }
    seq_write(seq, (*iter).buffer.as_ptr().add((*iter).pos as usize), len); 0
}

unsafe fn get_node_info(node: *mut gcov_node) -> *mut gcov_info {
    if (*node).num_loaded > 0 { *(*node).loaded_info } else { (*node).unloaded_info }
}
unsafe fn get_accumulated_info(node: *mut gcov_node) -> *mut gcov_info {
    let mut i = 0; let info = if !(*node).unloaded_info.is_null() { gcov_info_dup((*node).unloaded_info) } else { let p = *(*node).loaded_info.add(i); i += 1; gcov_info_dup(p) };
    if info.is_null() { return core::ptr::null_mut(); }
    while i < (*node).num_loaded as usize { gcov_info_add(info, *(*node).loaded_info.add(i)); i += 1; } info
}

unsafe fn reset_node(node: *mut gcov_node) { if !(*node).unloaded_info.is_null() { gcov_info_reset((*node).unloaded_info); } for i in 0..(*node).num_loaded as usize { gcov_info_reset(*(*node).loaded_info.add(i)); } }
unsafe fn get_info_index(node: *mut gcov_node, info: *mut gcov_info) -> c_int { for i in 0..(*node).num_loaded { if *(*node).loaded_info.add(i as usize) == info { return i; } } -ENOENT }

unsafe fn gcov_seq_start(seq: *mut seq_file, pos: *mut LoFF) -> *mut c_void {
    let iter = (*seq).private as *mut gcov_iterator; gcov_iter_start(iter);
    for _ in 0..*pos { if gcov_iter_next(iter) != 0 { return core::ptr::null_mut(); } } iter as *mut c_void
}
unsafe fn gcov_seq_next(seq: *mut seq_file, data: *mut c_void, pos: *mut LoFF) -> *mut c_void {
    *pos += 1; if gcov_iter_next(data as *mut gcov_iterator) != 0 { core::ptr::null_mut() } else { data }
}
unsafe fn gcov_seq_show(_seq: *mut seq_file, _data: *mut c_void) -> c_int { 0 }
unsafe fn gcov_seq_stop(_seq: *mut seq_file, _data: *mut c_void) {}

unsafe fn gcov_seq_open(_inode: *mut inode, _file: *mut file) -> c_int { -ENOMEM }
unsafe fn gcov_seq_release(_inode: *mut inode, _file: *mut file) -> c_int { 0 }
unsafe fn get_node_by_name(_name: *const c_char) -> *mut gcov_node { core::ptr::null_mut() }
unsafe fn remove_node(_node: *mut gcov_node) {}
unsafe fn gcov_seq_write(_file: *mut file, _addr: *const c_char, len: SizeT, _pos: *mut LoFF) -> SsizeT { len as SsizeT }
unsafe fn link_target(_dir: *const c_char, _path: *const c_char, _ext: *const c_char) -> *mut c_char { core::ptr::null_mut() }
unsafe fn get_link_target(_filename: *const c_char, _ext: *const gcov_link) -> *mut c_char { core::ptr::null_mut() }
unsafe fn deskew(basename: *const c_char) -> *const c_char { basename }
unsafe fn add_links(_node: *mut gcov_node, _parent: *mut dentry) {}
unsafe fn init_node(_node: *mut gcov_node, _info: *mut gcov_info, _name: *const c_char, _parent: *mut gcov_node) {}
unsafe fn new_node(_parent: *mut gcov_node, _info: *mut gcov_info, _name: *const c_char) -> *mut gcov_node { core::ptr::null_mut() }
unsafe fn remove_links(_node: *mut gcov_node) {}
unsafe fn release_node(_node: *mut gcov_node) {}
unsafe fn get_child_by_name(_parent: *mut gcov_node, _name: *const c_char) -> *mut gcov_node { core::ptr::null_mut() }
unsafe fn reset_write(_file: *mut file, _addr: *const c_char, len: SizeT, _pos: *mut LoFF) -> SsizeT { len as SsizeT }
unsafe fn reset_read(_file: *mut file, _addr: *mut c_char, _len: SizeT, _pos: *mut LoFF) -> SsizeT { 0 }
unsafe fn add_node(_info: *mut gcov_info) {}
unsafe fn add_info(_node: *mut gcov_node, _info: *mut gcov_info) {}
unsafe fn save_info(_node: *mut gcov_node, _info: *mut gcov_info) {}
unsafe fn remove_info(_node: *mut gcov_node, _info: *mut gcov_info) {}

#[no_mangle]
pub unsafe extern "C" fn gcov_event(action: c_int, info: *mut gcov_info) {
    let node = get_node_by_name(gcov_info_filename(info));
    if action == 0 { if node.is_null() { add_node(info); } else { add_info(node, info); } }
    else if !node.is_null() { remove_info(node, info); }
}

#[no_mangle]
pub unsafe extern "C" fn gcov_fs_init() -> c_int {
    init_node(&mut root_node, core::ptr::null_mut(), core::ptr::null(), core::ptr::null_mut());
    gcov_enable_events(); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
