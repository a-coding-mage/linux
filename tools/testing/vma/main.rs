// SPDX-License-Identifier: GPL-2.0-or-later

use std::ffi::c_char;
use std::os::raw::c_int;

// C source included "shared.h".
// Directly import the VMA implementation here. Our vma_internal.h wrapper
// provides userland-equivalent functionality for everything vma.c uses.
// C source included "../../../mm/vma_init.c".
// C source included "../../../mm/vma_exec.c".
// C source included "../../../mm/vma.c".

// Tests are included directly so they can test static functions in mm/vma.c.
// C source included "tests/merge.c".
// C source included "tests/mmap.c".
// C source included "tests/vma.c".

const EXIT_SUCCESS: c_int = 0;
const EXIT_FAILURE: c_int = 1;

// DEFAULT_MAX_MAP_COUNT is provided by the included C dependencies.
// TODO: replace this declaration with the dependency's Rust constant when
// translating the surrounding test harness.
const DEFAULT_MAX_MAP_COUNT: c_int = 65_530;

#[repr(C)]
pub struct vm_area_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vma_merge_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mm_struct {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn vma_merge_existing_range(vmg: *mut vma_merge_struct) -> *mut vm_area_struct;
    fn vma_assert_attached(vma: *mut vm_area_struct);
    fn vma_link(mm: *mut mm_struct, vma: *mut vm_area_struct) -> c_int;

    fn maple_tree_init();
    fn vma_state_init();

    fn run_merge_tests(num_tests: *mut c_int, num_fail: *mut c_int);
    fn run_mmap_tests(num_tests: *mut c_int, num_fail: *mut c_int);
    fn run_vma_tests(num_tests: *mut c_int, num_fail: *mut c_int);

    fn printf(format: *const c_char, ...) -> c_int;
}

// C declaration used __read_mostly.
#[no_mangle]
pub static mut sysctl_max_map_count: c_int = DEFAULT_MAX_MAP_COUNT;

// Helper functions which utilise static kernel functions.

#[no_mangle]
pub unsafe extern "C" fn merge_existing(vmg: *mut vma_merge_struct) -> *mut vm_area_struct {
    let vma: *mut vm_area_struct;

    vma = unsafe { vma_merge_existing_range(vmg) };
    if !vma.is_null() {
        unsafe { vma_assert_attached(vma) };
    }
    vma
}

#[no_mangle]
pub unsafe extern "C" fn attach_vma(
    mm: *mut mm_struct,
    vma: *mut vm_area_struct,
) -> c_int {
    let res: c_int;

    res = unsafe { vma_link(mm, vma) };
    if res == 0 {
        unsafe { vma_assert_attached(vma) };
    }
    res
}

// Main test running which invokes tests/ *.c runners.
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    let mut num_tests: c_int = 0;
    let mut num_fail: c_int = 0;

    unsafe { maple_tree_init() };
    unsafe { vma_state_init() };

    unsafe { run_merge_tests(&mut num_tests, &mut num_fail) };
    unsafe { run_mmap_tests(&mut num_tests, &mut num_fail) };
    unsafe { run_vma_tests(&mut num_tests, &mut num_fail) };

    unsafe {
        printf(
            c"%d tests run, %d passed, %d failed.\n".as_ptr(),
            num_tests,
            num_tests - num_fail,
            num_fail,
        )
    };

    if num_fail == 0 {
        EXIT_SUCCESS
    } else {
        EXIT_FAILURE
    }
}
