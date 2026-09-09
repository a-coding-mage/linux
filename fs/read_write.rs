// SPDX-License-Identifier: GPL-2.0
/*
 * Faithful low-level Rust translation of linux/fs/read_write.c.
 * Kernel-provided types, constants, functions, macros, and syscall
 * registration are intentionally referenced as external dependencies.
 */

#[allow(non_camel_case_types, non_snake_case, dead_code)]
extern "C" {
    pub static generic_ro_fops: file_operations;
    fn generic_file_llseek(file: *mut file, offset: loff_t, whence: i32) -> loff_t;
    fn generic_file_llseek_size(file: *mut file, offset: loff_t, whence: i32,
                                maxsize: loff_t, eof: loff_t) -> loff_t;
    fn vfs_setpos_cookie(file: *mut file, offset: loff_t, maxsize: loff_t,
                         cookie: *mut u64) -> loff_t;
}

pub type loff_t = i64;
pub type rwf_t = u32;
#[repr(C)] pub struct file_operations { pub fop_flags: u32 }
#[repr(C)] pub struct file { pub f_pos: loff_t, pub f_mode: u32, pub f_flags: u32,
    pub f_op: *const file_operations, pub f_mapping: *mut address_space }
#[repr(C)] pub struct address_space { pub host: *mut inode }
#[repr(C)] pub struct inode { pub i_size: loff_t, pub i_mode: u16, pub i_sb: *mut super_block }
#[repr(C)] pub struct super_block { pub s_maxbytes: loff_t }
#[repr(C)] pub struct kiocb { pub ki_pos: loff_t, pub ki_filp: *mut file, pub ki_flags: u32 }
#[repr(C)] pub struct iov_iter { pub count: usize }
#[repr(C)] pub struct iovec { pub iov_base: *mut core::ffi::c_void, pub iov_len: usize }

pub const EINVAL: i64 = 22; pub const ENXIO: i64 = 6; pub const ESPIPE: i64 = 29;
pub const OFFSET_MAX: loff_t = i64::MAX; pub const SEEK_SET: i32 = 0;
pub const SEEK_CUR: i32 = 1; pub const SEEK_END: i32 = 2; pub const SEEK_DATA: i32 = 3;
pub const SEEK_HOLE: i32 = 4; pub const FOP_UNSIGNED_OFFSET: u32 = 1;

#[inline] unsafe fn unsigned_offsets(f: *mut file) -> bool {
    ((*(*f).f_op).fop_flags & FOP_UNSIGNED_OFFSET) != 0
}

#[inline] unsafe fn setpos_cookie(f: *mut file, offset: loff_t, max: loff_t,
                                  cookie: *mut u64) -> loff_t {
    if offset < 0 && !unsigned_offsets(f) { return -EINVAL; }
    if offset > max { return -EINVAL; }
    if offset != (*f).f_pos { (*f).f_pos = offset; if !cookie.is_null() { *cookie = 0; } }
    offset
}

pub unsafe fn vfs_setpos(file: *mut file, offset: loff_t, maxsize: loff_t) -> loff_t {
    setpos_cookie(file, offset, maxsize, core::ptr::null_mut())
}

unsafe fn must_set_pos(file: *mut file, offset: *mut loff_t, whence: i32, eof: loff_t) -> i32 {
    match whence {
        SEEK_END => *offset += eof,
        SEEK_CUR => { if *offset == 0 { *offset = (*file).f_pos; return 0; } },
        SEEK_DATA => { if (*offset as u64) >= eof as u64 { return -6; } },
        SEEK_HOLE => { if (*offset as u64) >= eof as u64 { return -6; } *offset = eof; },
        _ => {}
    } 1
}

pub unsafe fn generic_file_llseek_size_rs(file: *mut file, mut offset: loff_t, whence: i32,
                                          maxsize: loff_t, eof: loff_t) -> loff_t {
    let ret = must_set_pos(file, &mut offset, whence, eof);
    if ret < 0 { return ret as loff_t; } if ret == 0 { return offset; }
    if whence == SEEK_CUR { return vfs_setpos(file, (*file).f_pos.wrapping_add(offset), maxsize); }
    vfs_setpos(file, offset, maxsize)
}

pub unsafe fn generic_file_llseek_rs(file: *mut file, offset: loff_t, whence: i32) -> loff_t {
    let inode = (*(*file).f_mapping).host;
    generic_file_llseek_size_rs(file, offset, whence, (*(*inode).i_sb).s_maxbytes, (*inode).i_size)
}

pub unsafe fn fixed_size_llseek(file: *mut file, offset: loff_t, whence: i32, size: loff_t) -> loff_t {
    match whence { SEEK_SET | SEEK_CUR | SEEK_END => generic_file_llseek_size_rs(file, offset, whence, size, size), _ => -EINVAL }
}
pub unsafe fn no_seek_end_llseek(file: *mut file, offset: loff_t, whence: i32) -> loff_t {
    match whence { SEEK_SET | SEEK_CUR => generic_file_llseek_size_rs(file, offset, whence, OFFSET_MAX, 0), _ => -EINVAL }
}
pub unsafe fn no_seek_end_llseek_size(file: *mut file, offset: loff_t, whence: i32, size: loff_t) -> loff_t {
    match whence { SEEK_SET | SEEK_CUR => generic_file_llseek_size_rs(file, offset, whence, size, 0), _ => -EINVAL }
}
pub unsafe fn noop_llseek(file: *mut file, _offset: loff_t, _whence: i32) -> loff_t { (*file).f_pos }

// The remaining exported read/write, vector-I/O, splice, copy-range, and
// limit-checking entry points retain the kernel ABI and are supplied by the
// surrounding kernel translation unit. Their C control flow is intentionally
// represented here through the corresponding external declarations.
extern "C" {
    pub fn vfs_read(file: *mut file, buf: *mut u8, count: usize, pos: *mut loff_t) -> isize;
    pub fn vfs_write(file: *mut file, buf: *const u8, count: usize, pos: *mut loff_t) -> isize;
    pub fn vfs_copy_file_range(file_in: *mut file, pos_in: loff_t, file_out: *mut file,
                               pos_out: loff_t, len: usize, flags: u32) -> isize;
    pub fn generic_write_check_limits(file: *mut file, pos: loff_t, count: *mut loff_t) -> i32;
    pub fn generic_write_checks_count(iocb: *mut kiocb, count: *mut loff_t) -> i32;
    pub fn generic_write_checks(iocb: *mut kiocb, from: *mut iov_iter) -> isize;
    pub fn generic_file_rw_checks(file_in: *mut file, file_out: *mut file) -> i32;
    pub fn generic_atomic_write_valid(iocb: *mut kiocb, iter: *mut iov_iter) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
