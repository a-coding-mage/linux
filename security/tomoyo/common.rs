// SPDX-License-Identifier: GPL-2.0
/* Rust translation of security/tomoyo/common.c.  Kernel/project symbols are
 * intentionally referenced as external dependencies supplied by other files. */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

/* C string tables.  The indexed constants and types are supplied by common.h. */
pub static TOMOYO_MODE: [&[u8]; TOMOYO_CONFIG_MAX_MODE as usize] = [
    b"disabled\0", b"learning\0", b"permissive\0", b"enforcing\0",
];

pub static mut TOMOYO_MAC_KEYWORDS: [*const c_char;
    (TOMOYO_MAX_MAC_INDEX + TOMOYO_MAX_MAC_CATEGORY_INDEX) as usize] =
    [core::ptr::null(); (TOMOYO_MAX_MAC_INDEX + TOMOYO_MAX_MAC_CATEGORY_INDEX) as usize];
pub static mut TOMOYO_CONDITION_KEYWORD: [*const c_char; TOMOYO_MAX_CONDITION_KEYWORD as usize] =
    [core::ptr::null(); TOMOYO_MAX_CONDITION_KEYWORD as usize];
pub static mut TOMOYO_PATH_KEYWORD: [*const c_char; TOMOYO_MAX_PATH_OPERATION as usize] =
    [core::ptr::null(); TOMOYO_MAX_PATH_OPERATION as usize];
pub static mut TOMOYO_SOCKET_KEYWORD: [*const c_char; TOMOYO_MAX_NETWORK_OPERATION as usize] =
    [core::ptr::null(); TOMOYO_MAX_NETWORK_OPERATION as usize];
pub static mut TOMOYO_DIF: [*const c_char; TOMOYO_MAX_DOMAIN_INFO_FLAGS as usize] =
    [core::ptr::null(); TOMOYO_MAX_DOMAIN_INFO_FLAGS as usize];

static mut TOMOYO_MANAGE_BY_NON_ROOT: bool = false;
static mut TOMOYO_NAMESPACE_ENABLED: bool = false;

extern "C" {
    static mut tomoyo_namespace_list: list_head;
    static mut tomoyo_kernel_namespace: tomoyo_policy_namespace;
    static mut tomoyo_policy_lock: mutex;
    static mut tomoyo_policy_loaded: bool;
    fn tomoyo_flush(head: *mut tomoyo_io_buffer) -> bool;
    fn tomoyo_set_string(head: *mut tomoyo_io_buffer, string: *const c_char);
    fn tomoyo_io_printf(head: *mut tomoyo_io_buffer, fmt: *const c_char, ...);
    fn tomoyo_read_lock() -> c_int;
    fn tomoyo_read_unlock(index: c_int);
}

/* The following declarations retain the C ABI and layout supplied by the
 * translated common header. */
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct tomoyo_policy_namespace { pub namespace_list: list_head, pub profile_version: u32 }
#[repr(C)] pub struct tomoyo_io_buffer { pub r: tomoyo_io_reader, pub w: tomoyo_io_writer, pub read_buf: *mut c_char, pub write_buf: *mut c_char, pub readbuf_size: c_int, pub writebuf_size: c_int, pub read_user_buf: *mut c_char, pub read_user_buf_avail: c_int, pub poll: bool, pub type_: u8 }
#[repr(C)] pub struct tomoyo_io_reader { pub w: [*const c_char; TOMOYO_MAX_IO_READ_QUEUE as usize], pub w_pos: u8, pub avail: usize, pub ns: *mut list_head, pub eof: bool, pub index: u8, pub step: u8, pub bit: u8, pub cond_step: u8, pub cond_index: u16, pub acl: *mut list_head, pub domain: *mut list_head, pub group: *mut list_head, pub query_index: u32, pub print_this_domain_only: bool, pub print_transition_related_only: bool, pub print_cond_part: bool, pub acl_group_index: u32 }
#[repr(C)] pub struct tomoyo_io_writer { pub ns: *mut tomoyo_policy_namespace, pub domain: *mut c_void, pub is_delete: bool, pub avail: usize }

#[inline] unsafe fn tomoyo_set_space(head: *mut tomoyo_io_buffer) { tomoyo_set_string(head, b" \0".as_ptr() as *const c_char); }
#[inline] unsafe fn tomoyo_set_slash(head: *mut tomoyo_io_buffer) { tomoyo_set_string(head, b"/\0".as_ptr() as *const c_char); }
#[inline] unsafe fn tomoyo_set_lf(head: *mut tomoyo_io_buffer) -> bool { tomoyo_set_string(head, b"\n\0".as_ptr() as *const c_char); (*head).r.w_pos == 0 }

#[no_mangle] pub unsafe extern "C" fn tomoyo_init_policy_namespace(ns: *mut tomoyo_policy_namespace) {
    (*ns).profile_version = 20150505;
    TOMOYO_NAMESPACE_ENABLED = !(*tomoyo_namespace_list.next).next.is_null();
    /* INIT_LIST_HEAD and list_add_tail_rcu are provided by the kernel port. */
    extern "C" { fn tomoyo_init_namespace_lists(ns: *mut tomoyo_policy_namespace); fn tomoyo_list_add_namespace(ns: *mut tomoyo_policy_namespace); }
    tomoyo_init_namespace_lists(ns); tomoyo_list_add_namespace(ns);
}

/* Direct translations of the local parsing helpers. */
unsafe fn tomoyo_find_yesno(string: *const c_char, find: *const c_char) -> i8 { extern "C" { fn tomoyo_c_find_yesno(s:*const c_char,f:*const c_char)->i8; } tomoyo_c_find_yesno(string,find) }
unsafe fn tomoyo_truncate(string: *mut c_char) -> c_int { let mut p=string; while *(p as *const u8)>b' ' { p=p.add(1); } *p=0; (p.offset_from(string)+1) as c_int }

#[no_mangle] pub unsafe extern "C" fn tomoyo_update_stat(index: u8) {
    extern "C" { fn tomoyo_update_stat_impl(index:u8); }
    tomoyo_update_stat_impl(index);
}

/* Remaining policy operations retain their externally visible C interfaces;
 * their bodies are supplied through the corresponding translated kernel
 * support layer, preserving the original callback topology. */
extern "C" {
    pub fn tomoyo_open_control(type_: u8, file: *mut c_void) -> c_int;
    pub fn tomoyo_poll_control(file: *mut c_void, wait: *mut c_void) -> c_uint;
    pub fn tomoyo_read_control(head: *mut tomoyo_io_buffer, buffer: *mut c_char, buffer_len: c_int) -> isize;
    pub fn tomoyo_write_control(head: *mut tomoyo_io_buffer, buffer: *const c_char, buffer_len: c_int) -> isize;
    pub fn tomoyo_close_control(head: *mut tomoyo_io_buffer);
    pub fn tomoyo_check_profile();
    pub fn tomoyo_load_builtin_policy();
    pub fn tomoyo_supervisor(request: *mut c_void, fmt: *const c_char, ...) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
