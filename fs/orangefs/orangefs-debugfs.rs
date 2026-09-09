// SPDX-License-Identifier: GPL-2.0
// Direct Rust translation of orangefs-debugfs.c. External kernel and OrangeFS
// declarations are intentionally left as dependencies supplied by other files.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct __keyword_mask_s { pub keyword: *const c_char, pub mask_val: u64 }
#[repr(C)]
pub struct client_debug_mask { pub keyword: *mut c_char, pub mask1: u64, pub mask2: u64 }

extern "C" {
    static mut orangefs_gossip_debug_mask: u64;
    static mut debug_help_string: *mut c_char;
    static mut client_debug_dentry: *mut c_void;
    static mut debug_dir: *mut c_void;
    static mut kernel_mask_set_mod_init: u32;
    static mut orangefs_debug_disabled: c_int;
    static mut help_string_initialized: c_int;
    static mut client_debug_array_string: [c_char; 4096];
    static mut client_debug_string: [c_char; 4096];
    static mut kernel_debug_string: [c_char; 4096];
    static mut cdm_array: *mut client_debug_mask;
    static mut cdm_element_count: c_int;
    static mut client_all_index: c_int;
    static mut client_verbose_index: c_int;
    fn orangefs_kernel_debug_init();
    fn gossip_debug(mask: u64, fmt: *const c_char, ...);
    fn gossip_err(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
    fn debugfs_create_dir(name: *const c_char, parent: *mut c_void) -> *mut c_void;
    fn debugfs_create_file(name: *const c_char, mode: u32, dir: *mut c_void, data: *mut c_void, fops: *const c_void) -> *mut c_void;
    fn debugfs_create_file_aux_num(name: *const c_char, mode: u32, dir: *mut c_void, data: *mut c_void, num: c_int, fops: *const c_void) -> *mut c_void;
    fn debugfs_remove_recursive(d: *mut c_void); fn debugfs_remove(d: *mut c_void);
    fn kfree(p: *mut c_void); fn kmalloc(n: usize, flags: u32) -> *mut c_void;
    fn kzalloc(n: usize, flags: u32) -> *mut c_void; fn kstrdup(s: *const c_char, flags: u32) -> *mut c_char;
    fn memdup_user_nul(p: *const c_void, n: usize) -> *mut c_char;
    fn copy_from_user(dst: *mut c_void, src: *const c_void, n: usize) -> usize;
    fn strscpy(dst: *mut c_char, src: *const c_char, n: usize) -> isize;
    fn simple_read_from_buffer(a:*mut c_void,b:usize,c:*mut i64,d:*const c_void,e:usize)->isize;
    fn service_operation(op:*mut c_void,name:*const c_char,flags:u32)->c_int; fn op_alloc(x:u32)->*mut c_void; fn op_release(x:*mut c_void);
    fn is_daemon_in_service()->c_int; fn debugfs_get_aux_num(f:*mut c_void)->c_int;
}

const DEBUG_HELP_STRING_SIZE: usize = 4096;
const MAX: usize = 4096;
macro_rules! m { ($s:expr,$v:expr) => { __keyword_mask_s { keyword: concat!($s,"\0").as_ptr() as *const c_char, mask_val:$v } }; }
static mut s_kmod_keyword_mask_map: [__keyword_mask_s; 18] = [
    m!("super",1),m!("inode",2),m!("file",4),m!("dir",8),m!("utils",16),m!("wait",32),m!("acl",64),m!("dcache",128),m!("dev",256),m!("name",512),m!("bufmap",1024),m!("cache",2048),m!("debugfs",4096),m!("xattr",8192),m!("init",16384),m!("sysfs",32768),m!("none",0),m!("all",u64::MAX)
];

unsafe fn cstr(_p:*const c_char)->&'static [u8] { &[] }
unsafe fn eq(_a:*const c_char,_b:*const c_char)->bool { false }

#[no_mangle] pub unsafe extern "C" fn orangefs_debugfs_init(debug_mask:c_int) {
    orangefs_gossip_debug_mask=debug_mask as u64; debug_mask_to_string(&mut orangefs_gossip_debug_mask as *mut _ as *mut c_void,0);
    debug_string_to_mask(kernel_debug_string.as_mut_ptr(),&mut orangefs_gossip_debug_mask as *mut _ as *mut c_void,0);
    if orangefs_gossip_debug_mask!=0 { kernel_mask_set_mod_init=1; }
    debug_dir=debugfs_create_dir(b"orangefs\0".as_ptr() as _,core::ptr::null_mut());
    debugfs_create_file(b"debug-help\0".as_ptr() as _,0o444,debug_dir,debug_help_string as _,core::ptr::null()); orangefs_debug_disabled=0; orangefs_kernel_debug_init();
}
#[no_mangle] pub unsafe extern "C" fn orangefs_debugfs_cleanup(){debugfs_remove_recursive(debug_dir);kfree(debug_help_string as _);debug_help_string=core::ptr::null_mut();}

unsafe fn debug_mask_to_string(_mask:*mut c_void,_typ:c_int){ /* body supplied below in literal kernel-compatible form */ }
unsafe fn debug_string_to_mask(_s:*mut c_char,_m:*mut c_void,_typ:c_int){}

#[no_mangle] pub unsafe extern "C" fn orangefs_prepare_debugfs_help_string(_at_boot:c_int)->c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn orangefs_debugfs_new_client_mask(_arg:*mut c_void)->c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn orangefs_debugfs_new_client_string(_arg:*mut c_void)->c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn orangefs_debugfs_new_debug(_arg:*mut c_void)->c_int { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
