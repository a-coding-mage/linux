// SPDX-License-Identifier: GPL-2.0
/*
 * Test cases for memcat_p() in lib/memcat_p.c
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
struct test_struct {
    num: c_int,
    magic: c_uint,
}

const MAGIC: c_uint = 0xf00ff00f;
/* Size of each of the NULL-terminated input arrays */
const INPUT_MAX: usize = 128;
/* Expected number of non-NULL elements in the output array */
const EXPECT: usize = INPUT_MAX * 2 - 2;

extern "C" {
    fn kzalloc_objs(size: usize, count: usize) -> *mut *mut test_struct;
    fn kmalloc_obj(size: usize) -> *mut test_struct;
    fn kfree(ptr: *mut c_void);
    fn memcat_p(
        in0: *mut *mut test_struct,
        in1: *mut *mut test_struct,
    ) -> *mut *mut test_struct;
    fn pr_err(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
}

#[allow(non_snake_case)]
unsafe fn test_memcat_p_init() -> c_int {
    let mut in0: *mut *mut test_struct;
    let mut in1: *mut *mut test_struct;
    let mut out: *mut *mut test_struct;
    let mut p: *mut *mut test_struct;
    let mut err: c_int = -12; // -ENOMEM
    let mut i: c_int;
    let mut r: c_int;
    let mut total: c_int = 0;

    in0 = kzalloc_objs(core::mem::size_of::<*mut test_struct>(), INPUT_MAX);
    if in0.is_null() {
        return err;
    }

    in1 = kzalloc_objs(core::mem::size_of::<*mut test_struct>(), INPUT_MAX);
    if in1.is_null() {
        goto_err_free_in0(in0, err)
    }

    r = 1;
    i = 0;
    while i < (INPUT_MAX - 1) as c_int {
        *in0.add(i as usize) = kmalloc_obj(core::mem::size_of::<test_struct>());
        if (*in0.add(i as usize)).is_null() {
            goto_err_free_elements(in0, in1, i);
        }

        *in1.add(i as usize) = kmalloc_obj(core::mem::size_of::<test_struct>());
        if (*in1.add(i as usize)).is_null() {
            kfree(*in0.add(i as usize) as *mut c_void);
            goto_err_free_elements(in0, in1, i);
        }

        /* lifted from test_sort.c */
        r = (r * 725861) % 6599;
        (**in0.add(i as usize)).num = r;
        (**in1.add(i as usize)).num = -r;
        (**in0.add(i as usize)).magic = MAGIC;
        (**in1.add(i as usize)).magic = MAGIC;
        i += 1;
    }

    *in0.add(i as usize) = core::ptr::null_mut();
    *in1.add(i as usize) = core::ptr::null_mut();

    out = memcat_p(in0, in1);
    if out.is_null() {
        goto_err_free_all_elements(in0, in1, i);
    }

    err = -22; // -EINVAL
    p = out;
    i = 0;
    while !(*p).is_null() && i < (INPUT_MAX * 2 - 1) as c_int {
        total += (**p).num;

        if (**p).magic != MAGIC {
            pr_err(b"test failed: wrong magic at %d: %u\n\0".as_ptr() as *const c_char, i, (**p).magic);
            goto_err_free_out(out, in0, in1, i);
        }
        p = p.add(1);
        i += 1;
    }

    if total != 0 {
        pr_err(b"test failed: expected zero total, got %d\n\0".as_ptr() as *const c_char, total);
        goto_err_free_out(out, in0, in1, i);
    }

    if i != EXPECT as c_int {
        pr_err(b"test failed: expected output size %d, got %d\n\0".as_ptr() as *const c_char, EXPECT, i);
        goto_err_free_out(out, in0, in1, i);
    }

    i = 0;
    while i < (INPUT_MAX - 1) as c_int {
        if *out.add(i as usize) != *in0.add(i as usize)
            || *out.add((i as usize) + INPUT_MAX - 1) != *in1.add(i as usize)
        {
            pr_err(b"test failed: wrong element order at %d\n\0".as_ptr() as *const c_char, i);
            goto_err_free_out(out, in0, in1, i);
        }
        i += 1;
    }

    err = 0;
    pr_info(b"test passed\n\0".as_ptr() as *const c_char);
    kfree(out as *mut c_void);
    free_elements(in0, in1, i);
    err
}

unsafe fn free_elements(in0: *mut *mut test_struct, in1: *mut *mut test_struct, mut i: c_int) {
    i -= 1;
    while i >= 0 {
        kfree(*in1.add(i as usize) as *mut c_void);
        kfree(*in0.add(i as usize) as *mut c_void);
        i -= 1;
    }
    kfree(in1 as *mut c_void);
    kfree(in0 as *mut c_void);
}

unsafe fn goto_err_free_elements(in0: *mut *mut test_struct, in1: *mut *mut test_struct, i: c_int) -> ! {
    free_elements(in0, in1, i);
    core::hint::unreachable_unchecked()
}

unsafe fn goto_err_free_all_elements(in0: *mut *mut test_struct, in1: *mut *mut test_struct, i: c_int) -> ! {
    free_elements(in0, in1, i);
    core::hint::unreachable_unchecked()
}

unsafe fn goto_err_free_out(out: *mut *mut test_struct, in0: *mut *mut test_struct, in1: *mut *mut test_struct, i: c_int) -> ! {
    kfree(out as *mut c_void);
    free_elements(in0, in1, i);
    core::hint::unreachable_unchecked()
}

unsafe fn goto_err_free_in0(in0: *mut *mut test_struct, err: c_int) -> c_int {
    kfree(in0 as *mut c_void);
    err
}

unsafe fn test_memcat_p_exit() {}

// module_init(test_memcat_p_init);
// module_exit(test_memcat_p_exit);
// MODULE_DESCRIPTION("Test cases for memcat_p() in lib/memcat_p.c");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
