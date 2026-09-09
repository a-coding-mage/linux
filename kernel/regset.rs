// SPDX-License-Identifier: GPL-2.0-only
// Dependencies supplied by the surrounding kernel translation unit:
// linux/export.h, linux/slab.h, and linux/regset.h.

use core::ffi::c_void;

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct user_regset {
    pub n: usize,
    pub size: usize,
    pub regset_get: Option<unsafe extern "C" fn(*mut task_struct, *const user_regset, membuf) -> i32>,
}

#[repr(C)]
pub struct user_regset_view {
    pub regsets: *const user_regset,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct membuf {
    pub p: *mut c_void,
    pub left: usize,
}

extern "C" {
    fn kvzalloc(size: usize, flags: usize) -> *mut c_void;
    fn kvfree(addr: *mut c_void);
    fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> usize;
}

const GFP_KERNEL: usize = 0;
extern "C" {
    static EOPNOTSUPP: i32;
    static ENOMEM: i32;
    static EFAULT: i32;
}

unsafe fn __regset_get(
    target: *mut task_struct,
    regset: *const user_regset,
    mut size: u32,
    data: *mut *mut c_void,
) -> i32 {
    let mut p = *data;
    let mut to_free: *mut c_void = core::ptr::null_mut();
    let res: i32;

    if (*regset).regset_get.is_none() {
        return -EOPNOTSUPP;
    }
    let maximum = (*regset).n.wrapping_mul((*regset).size);
    if (size as usize) > maximum {
        size = maximum as u32;
    }
    if p.is_null() {
        to_free = kvzalloc(size as usize, GFP_KERNEL);
        p = to_free;
        if p.is_null() {
            return -ENOMEM;
        }
    }
    res = ((*regset).regset_get.unwrap())(
        target,
        regset,
        membuf { p, left: size as usize },
    );
    if res < 0 {
        kvfree(to_free);
        return res;
    }
    *data = p;
    size as i32 - res
}

pub unsafe fn regset_get(
    target: *mut task_struct,
    regset: *const user_regset,
    size: u32,
    data: *mut c_void,
) -> i32 {
    __regset_get(target, regset, size, &mut (data as *mut c_void))
}

// EXPORT_SYMBOL(regset_get)

pub unsafe fn regset_get_alloc(
    target: *mut task_struct,
    regset: *const user_regset,
    size: u32,
    data: *mut *mut c_void,
) -> i32 {
    *data = core::ptr::null_mut();
    __regset_get(target, regset, size, data)
}

// EXPORT_SYMBOL(regset_get_alloc)

/**
 * copy_regset_to_user - fetch a thread's user_regset data into user memory
 * @target: thread to be examined
 * @view: &struct user_regset_view describing user thread machine state
 * @setno: index in @view->regsets
 * @offset: offset into the regset data, in bytes
 * @size: amount of data to copy, in bytes
 * @data: user-mode pointer to copy into
 */
pub unsafe fn copy_regset_to_user(
    target: *mut task_struct,
    view: *const user_regset_view,
    setno: u32,
    _offset: u32,
    size: u32,
    data: *mut c_void,
) -> i32 {
    let regset = (*view).regsets.add(setno as usize);
    let mut buf: *mut c_void = core::ptr::null_mut();
    let mut ret = regset_get_alloc(target, regset, size, &mut buf);
    if ret > 0 {
        ret = if copy_to_user(data, buf, ret as usize) != 0 { -EFAULT } else { 0 };
    }
    kvfree(buf);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
