/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * User-mode machine state access
 *
 * Copyright (C) 2007 Red Hat, Inc.  All rights reserved.
 *
 * Red Hat Author: Roland McGrath.
 */

/* C dependencies: linux/compiler.h, linux/types.h, linux/bug.h, linux/uaccess.h. */

use core::ffi::c_void;

pub enum task_struct {}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct membuf {
    pub p: *mut c_void,
    pub left: usize,
}

#[inline]
pub unsafe fn membuf_zero(s: *mut membuf, mut size: usize) -> i32 {
    if (*s).left != 0 {
        if size > (*s).left { size = (*s).left; }
        core::ptr::write_bytes((*s).p as *mut u8, 0, size);
        (*s).p = ((*s).p as *mut u8).add(size) as *mut c_void;
        (*s).left -= size;
    }
    (*s).left as i32
}

#[inline]
pub unsafe fn membuf_write(s: *mut membuf, v: *const c_void, mut size: usize) -> i32 {
    if (*s).left != 0 {
        if size > (*s).left { size = (*s).left; }
        core::ptr::copy_nonoverlapping(v as *const u8, (*s).p as *mut u8, size);
        (*s).p = ((*s).p as *mut u8).add(size) as *mut c_void;
        (*s).left -= size;
    }
    (*s).left as i32
}

#[inline]
pub unsafe fn membuf_at(s: *const membuf, mut offs: usize) -> membuf {
    let mut n = *s;
    if offs > n.left { offs = n.left; }
    n.p = (n.p as *mut u8).add(offs) as *mut c_void;
    n.left -= offs;
    n
}

/* current s->p must be aligned for v; v must be a scalar */
#[macro_export]
macro_rules! membuf_store {
    ($s:expr, $v:expr) => {{
        let __s: *mut $crate::membuf = $s;
        if unsafe { (*__s).left != 0 } {
            let __v = $v;
            let __size = core::mem::size_of_val(&__v);
            let __copy_size = if __size > unsafe { (*__s).left } { unsafe { (*__s).left } } else { __size };
            unsafe {
                if __size > (*__s).left {
                    core::ptr::copy_nonoverlapping((&__v as *const _).cast::<u8>(), (*__s).p.cast::<u8>(), __copy_size);
                } else {
                    core::ptr::write((*__s).p.cast::<_>(), __v);
                }
                (*__s).p = (*__s).p.cast::<u8>().add(__copy_size).cast();
                (*__s).left -= __copy_size;
            }
        }
        unsafe { (*__s).left as i32 }
    }};
}

pub type user_regset_active_fn = unsafe extern "C" fn(*mut task_struct, *const user_regset) -> i32;
pub type user_regset_get2_fn = unsafe extern "C" fn(*mut task_struct, *const user_regset, membuf) -> i32;
pub type user_regset_set_fn = unsafe extern "C" fn(*mut task_struct, *const user_regset, u32, u32, *const c_void, *const c_void) -> i32;
pub type user_regset_writeback_fn = unsafe extern "C" fn(*mut task_struct, *const user_regset, i32) -> i32;

#[repr(C)]
pub struct user_regset {
    pub regset_get: Option<user_regset_get2_fn>,
    pub set: Option<user_regset_set_fn>,
    pub active: Option<user_regset_active_fn>,
    pub writeback: Option<user_regset_writeback_fn>,
    pub n: u32,
    pub size: u32,
    pub align: u32,
    pub bias: u32,
    pub core_note_type: u32,
    pub core_note_name: *const i8,
}

#[macro_export]
macro_rules! USER_REGSET_NOTE_TYPE {
    ($type:ident) => { core::compile_error!("USER_REGSET_NOTE_TYPE requires NT_/NN_ constants from the including environment") };
}

#[repr(C)]
pub struct user_regset_view {
    pub name: *const i8,
    pub regsets: *const user_regset,
    pub n: u32,
    pub e_flags: u32,
    pub e_machine: u16,
    pub ei_osabi: u8,
}

extern "C" {
    pub fn task_user_regset_view(tsk: *mut task_struct) -> *const user_regset_view;
    pub fn regset_get(target: *mut task_struct, regset: *const user_regset, size: u32, data: *mut c_void) -> i32;
    pub fn regset_get_alloc(target: *mut task_struct, regset: *const user_regset, size: u32, data: *mut *mut c_void) -> i32;
    pub fn copy_regset_to_user(target: *mut task_struct, view: *const user_regset_view, setno: u32, offset: u32, size: u32, data: *mut c_void) -> i32;
}

#[inline]
pub unsafe fn user_regset_copyin(pos: *mut u32, count: *mut u32, kbuf: *mut *const c_void, ubuf: *mut *const c_void, data: *mut c_void, start_pos: i32, end_pos: i32) -> i32 {
    if *count == 0 { return 0; }
    debug_assert!(*pos >= start_pos as u32);
    if end_pos < 0 || *pos < end_pos as u32 {
        let copy = if end_pos < 0 { *count } else { core::cmp::min(*count, end_pos as u32 - *pos) };
        let dst = (data as *mut u8).add((*pos - start_pos as u32) as usize);
        if !(*kbuf).is_null() {
            core::ptr::copy_nonoverlapping(*kbuf as *const u8, dst, copy as usize);
            *kbuf = (*kbuf as *const u8).add(copy as usize) as *const c_void;
        } else {
            core::ptr::copy_nonoverlapping(*ubuf as *const u8, dst, copy as usize);
            *ubuf = (*ubuf as *const u8).add(copy as usize) as *const c_void;
        }
        *pos += copy; *count -= copy;
    }
    0
}

#[inline]
pub unsafe fn user_regset_copyin_ignore(pos: *mut u32, count: *mut u32, kbuf: *mut *const c_void, ubuf: *mut *const c_void, start_pos: i32, end_pos: i32) {
    if *count == 0 { return; }
    debug_assert!(*pos >= start_pos as u32);
    if end_pos < 0 || *pos < end_pos as u32 {
        let copy = if end_pos < 0 { *count } else { core::cmp::min(*count, end_pos as u32 - *pos) };
        if !(*kbuf).is_null() { *kbuf = (*kbuf as *const u8).add(copy as usize) as *const c_void; }
        else { *ubuf = (*ubuf as *const u8).add(copy as usize) as *const c_void; }
        *pos += copy; *count -= copy;
    }
}

#[inline]
pub unsafe fn copy_regset_from_user(target: *mut task_struct, view: *const user_regset_view, setno: u32, offset: u32, size: u32, data: *const c_void) -> i32 {
    let regset = &*(*view).regsets.add(setno as usize);
    let set = match regset.set { Some(f) => f, None => return -95 };
    set(target, regset, offset, size, core::ptr::null(), data)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
