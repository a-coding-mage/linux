/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from linux/seq_file.h. Kernel dependency types and symbols are external. */

pub const SEQ_SKIP: i32 = 1;

#[repr(C)]
pub struct seq_file {
    pub buf: *mut core::ffi::c_char,
    pub size: usize,
    pub from: usize,
    pub count: usize,
    pub pad_until: usize,
    pub index: loff_t,
    pub read_pos: loff_t,
    pub lock: mutex,
    pub op: *const seq_operations,
    pub poll_event: i32,
    pub file: *const file,
    pub private: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct seq_operations {
    pub start: Option<unsafe extern "C" fn(*mut seq_file, *mut loff_t) -> *mut core::ffi::c_void>,
    pub stop: Option<unsafe extern "C" fn(*mut seq_file, *mut core::ffi::c_void)>,
    pub next: Option<unsafe extern "C" fn(*mut seq_file, *mut core::ffi::c_void, *mut loff_t) -> *mut core::ffi::c_void>,
    pub show: Option<unsafe extern "C" fn(*mut seq_file, *mut core::ffi::c_void) -> i32>,
}

#[inline]
pub unsafe fn seq_has_overflowed(m: *mut seq_file) -> bool {
    (*m).count == (*m).size
}

#[inline]
pub unsafe fn seq_get_buf(m: *mut seq_file, bufp: *mut *mut core::ffi::c_char) -> usize {
    BUG_ON((*m).count > (*m).size);
    if (*m).count < (*m).size {
        *bufp = (*m).buf.add((*m).count);
    } else {
        *bufp = core::ptr::null_mut();
    }
    (*m).size - (*m).count
}

#[inline]
pub unsafe fn seq_commit(m: *mut seq_file, num: i32) {
    if num < 0 {
        (*m).count = (*m).size;
    } else {
        BUG_ON((*m).count + num as usize > (*m).size);
        (*m).count += num as usize;
    }
}

#[inline]
pub unsafe fn seq_setwidth(m: *mut seq_file, size: usize) {
    (*m).pad_until = (*m).count + size;
}

extern "C" {
    pub fn seq_pad(m: *mut seq_file, c: core::ffi::c_char);
    pub fn seq_mangle_path(s: *mut core::ffi::c_char, p: *const core::ffi::c_char, esc: *const core::ffi::c_char) -> *mut core::ffi::c_char;
    pub fn seq_open(file: *mut file, op: *const seq_operations) -> i32;
    pub fn seq_read(file: *mut file, buf: *mut core::ffi::c_char, size: usize, ppos: *mut loff_t) -> isize;
    pub fn seq_read_iter(iocb: *mut kiocb, iter: *mut iov_iter) -> isize;
    pub fn seq_lseek(file: *mut file, offset: loff_t, whence: i32) -> loff_t;
    pub fn seq_release(inode: *mut inode, file: *mut file) -> i32;
    pub fn seq_write(seq: *mut seq_file, data: *const core::ffi::c_void, len: usize) -> i32;
    pub fn seq_vprintf(m: *mut seq_file, fmt: *const core::ffi::c_char, args: va_list);
    pub fn seq_printf(m: *mut seq_file, fmt: *const core::ffi::c_char, ...);
    pub fn seq_putc(m: *mut seq_file, c: core::ffi::c_char);
    pub fn __seq_puts(m: *mut seq_file, s: *const core::ffi::c_char);
    pub fn seq_put_decimal_ull_width(m: *mut seq_file, delimiter: *const core::ffi::c_char, num: u64, width: u32);
    pub fn seq_put_decimal_ull(m: *mut seq_file, delimiter: *const core::ffi::c_char, num: u64);
    pub fn seq_put_decimal_ll(m: *mut seq_file, delimiter: *const core::ffi::c_char, num: i64);
    pub fn seq_put_hex_ll(m: *mut seq_file, delimiter: *const core::ffi::c_char, v: u64, width: u32);
    pub fn seq_escape_mem(m: *mut seq_file, src: *const core::ffi::c_char, len: usize, flags: u32, esc: *const core::ffi::c_char);
    pub fn seq_hex_dump(m: *mut seq_file, prefix_str: *const core::ffi::c_char, prefix_type: i32, rowsize: i32, groupsize: i32, buf: *const core::ffi::c_void, len: usize, ascii: bool) -> i32;
    pub fn seq_path(m: *mut seq_file, path: *const path, esc: *const core::ffi::c_char) -> i32;
    pub fn seq_file_path(m: *mut seq_file, file: *mut file, esc: *const core::ffi::c_char) -> i32;
    pub fn seq_dentry(m: *mut seq_file, dentry: *mut dentry, esc: *const core::ffi::c_char) -> i32;
    pub fn seq_path_root(m: *mut seq_file, path: *const path, root: *const path, esc: *const core::ffi::c_char) -> i32;
    pub fn single_start(m: *mut seq_file, pos: *mut loff_t) -> *mut core::ffi::c_void;
    pub fn single_open(file: *mut file, show: Option<unsafe extern "C" fn(*mut seq_file, *mut core::ffi::c_void) -> i32>, data: *mut core::ffi::c_void) -> i32;
    pub fn single_open_size(file: *mut file, show: Option<unsafe extern "C" fn(*mut seq_file, *mut core::ffi::c_void) -> i32>, data: *mut core::ffi::c_void, size: usize) -> i32;
    pub fn single_release(inode: *mut inode, file: *mut file) -> i32;
    pub fn __seq_open_private(file: *mut file, ops: *const seq_operations, psize: i32) -> *mut core::ffi::c_void;
    pub fn seq_open_private(file: *mut file, ops: *const seq_operations, psize: i32) -> i32;
    pub fn seq_release_private(inode: *mut inode, file: *mut file) -> i32;
}

#[inline]
pub unsafe fn seq_puts(m: *mut seq_file, s: *const core::ffi::c_char) {
    if !__builtin_constant_p(*s) {
        __seq_puts(m, s);
    } else if *s != 0 && *s.add(1) == 0 {
        seq_putc(m, *s);
    } else {
        seq_write(m, s as *const core::ffi::c_void, __builtin_strlen(s));
    }
}

#[inline]
pub unsafe fn seq_escape_str(m: *mut seq_file, src: *const core::ffi::c_char, flags: u32, esc: *const core::ffi::c_char) {
    seq_escape_mem(m, src, strlen(src), flags, esc);
}

#[inline]
pub unsafe fn seq_escape(m: *mut seq_file, s: *const core::ffi::c_char, esc: *const core::ffi::c_char) {
    seq_escape_str(m, s, ESCAPE_OCTAL, esc);
}

#[inline]
pub unsafe fn seq_user_ns(seq: *mut seq_file) -> *mut user_namespace {
    #[cfg(CONFIG_USER_NS)]
    { (*(*seq).file).f_cred.user_ns }
    #[cfg(not(CONFIG_USER_NS))]
    { &raw mut init_user_ns }
}

#[inline]
pub unsafe fn seq_show_option(m: *mut seq_file, name: *const core::ffi::c_char, value: *const core::ffi::c_char) {
    seq_putc(m, b',' as core::ffi::c_char);
    seq_escape(m, name, b",= \t\n\\\0".as_ptr() as *const _);
    if !value.is_null() {
        seq_putc(m, b'=' as core::ffi::c_char);
        seq_escape(m, value, b", \t\n\\\0".as_ptr() as *const _);
    }
}

pub const SEQ_START_TOKEN: *mut core::ffi::c_void = 1 as *mut core::ffi::c_void;

extern "C" {
    pub fn seq_list_start(head: *mut list_head, pos: loff_t) -> *mut list_head;
    pub fn seq_list_start_head(head: *mut list_head, pos: loff_t) -> *mut list_head;
    pub fn seq_list_next(v: *mut core::ffi::c_void, head: *mut list_head, ppos: *mut loff_t) -> *mut list_head;
    pub fn seq_list_start_rcu(head: *mut list_head, pos: loff_t) -> *mut list_head;
    pub fn seq_list_start_head_rcu(head: *mut list_head, pos: loff_t) -> *mut list_head;
    pub fn seq_list_next_rcu(v: *mut core::ffi::c_void, head: *mut list_head, ppos: *mut loff_t) -> *mut list_head;
    pub fn seq_hlist_start(head: *mut hlist_head, pos: loff_t) -> *mut hlist_node;
    pub fn seq_hlist_start_head(head: *mut hlist_head, pos: loff_t) -> *mut hlist_node;
    pub fn seq_hlist_next(v: *mut core::ffi::c_void, head: *mut hlist_head, ppos: *mut loff_t) -> *mut hlist_node;
    pub fn seq_hlist_start_rcu(head: *mut hlist_head, pos: loff_t) -> *mut hlist_node;
    pub fn seq_hlist_start_head_rcu(head: *mut hlist_head, pos: loff_t) -> *mut hlist_node;
    pub fn seq_hlist_next_rcu(v: *mut core::ffi::c_void, head: *mut hlist_head, ppos: *mut loff_t) -> *mut hlist_node;
    pub fn seq_hlist_start_percpu(head: *mut hlist_head, cpu: *mut i32, pos: loff_t) -> *mut hlist_node;
    pub fn seq_hlist_next_percpu(v: *mut core::ffi::c_void, head: *mut hlist_head, cpu: *mut i32, pos: *mut loff_t) -> *mut hlist_node;
    pub fn seq_file_init();
}

/* C macros DEFINE_SEQ_ATTRIBUTE, DEFINE_SHOW_ATTRIBUTE, DEFINE_SHOW_STORE_ATTRIBUTE,
 * DEFINE_PROC_SHOW_ATTRIBUTE, and seq_show_option_n require token pasting,
 * designated initializers, and build-time kernel structures; their source-level
 * intent is retained here for dependent translation units. */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
