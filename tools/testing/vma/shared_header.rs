// SPDX-License-Identifier: GPL-2.0-or-later

// C header dependencies translated as external Rust dependencies:
// generated/bit-length.h, maple-shared.h, vma_internal.h, ../../../mm/vma.h

use core::ffi::c_int;
use std::os::raw::c_ulong;

pub const CONFIG_DEFAULT_MMAP_MIN_ADDR: c_ulong = 65536;

unsafe extern "C" {
    pub static mut fail_prealloc: bool;

    pub static mut mmap_min_addr: c_ulong;
    pub static mut dac_mmap_min_addr: c_ulong;
    pub static mut stack_guard_gap: c_ulong;

    pub static vma_dummy_vm_ops: vm_operations_struct;
    pub static mut dummy_anon_vma: anon_vma;
    pub static mut __current: task_struct;

    /*
     * Helper function which provides a wrapper around a merge existing VMA
     * operation.
     *
     * Declared in main.c as uses static VMA function.
     */
    pub fn merge_existing(vmg: *mut vma_merge_struct) -> *mut vm_area_struct;

    /*
     * Helper function to allocate a VMA and link it to the tree.
     *
     * Declared in main.c as uses static VMA function.
     */
    pub fn attach_vma(mm: *mut mm_struct, vma: *mut vm_area_struct) -> c_int;

    /* Helper function to simply allocate a VMA. */
    pub fn alloc_vma(
        mm: *mut mm_struct,
        start: c_ulong,
        end: c_ulong,
        pgoff: pgoff_t,
        vma_flags: vma_flags_t,
    ) -> *mut vm_area_struct;

    /* Helper function to detach and free a VMA. */
    pub fn detach_free_vma(vma: *mut vm_area_struct);

    /* Helper function to allocate a VMA and link it to the tree. */
    pub fn alloc_and_link_vma(
        mm: *mut mm_struct,
        start: c_ulong,
        end: c_ulong,
        pgoff: pgoff_t,
        vma_flags: vma_flags_t,
    ) -> *mut vm_area_struct;

    /*
     * Helper function to reset the dummy anon_vma to indicate it has not been
     * duplicated.
     */
    pub fn reset_dummy_anon_vma();

    /*
     * Helper function to remove all VMAs and destroy the maple tree associated with
     * a virtual address space. Returns a count of VMAs in the tree.
     */
    pub fn cleanup_mm(mm: *mut mm_struct, vmi: *mut vma_iterator) -> c_int;

    /* Helper function to determine if VMA has had vma_start_write() performed. */
    pub fn vma_write_started(vma: *mut vm_area_struct) -> bool;

    pub fn __vma_set_dummy_anon_vma(
        vma: *mut vm_area_struct,
        avc: *mut anon_vma_chain,
        anon_vma: *mut anon_vma,
    );

    /* Provide a simple dummy VMA/anon_vma dummy setup for testing. */
    pub fn vma_set_dummy_anon_vma(vma: *mut vm_area_struct, avc: *mut anon_vma_chain);
}

/* Helper function providing a dummy vm_ops->close() method.*/
#[inline]
pub unsafe extern "C" fn dummy_close(_: *mut vm_area_struct) {}

/* Simple test runner. Assumes local num_[fail, tests] counters. */
macro_rules! TEST {
    ($name:ident) => {{
        *num_tests += 1;
        if !concat_idents!(test_, $name)() {
            *num_fail += 1;
            eprintln!("Test {} FAILED", stringify!($name));
        }
    }};
}

macro_rules! __ASSERT_TRUE {
    ($expr:expr, $fmt:literal $(, $args:expr)* $(,)?) => {{
        if !$expr {
            eprintln!(
                concat!(
                    "Assert FAILED at {}:{}:{}(): {} is FALSE",
                    $fmt,
                    "."
                ),
                file!(),
                line!(),
                module_path!(),
                stringify!($expr)
                $(, $args)*
            );
            return false;
        }
    }};
}

macro_rules! __TO_SCALAR {
    ($x:expr) => {
        ($x as usize as u64)
    };
}

macro_rules! ASSERT_TRUE {
    ($expr:expr) => {
        __ASSERT_TRUE!($expr, "")
    };
}

macro_rules! ASSERT_FALSE {
    ($expr:expr) => {
        ASSERT_TRUE!(!$expr)
    };
}

macro_rules! ASSERT_EQ {
    ($val1:expr, $val2:expr) => {{
        let __val1 = $val1;
        let __val2 = $val2;
        __ASSERT_TRUE!(
            __val1 == __val2,
            " (0x{:x} != 0x{:x})",
            __TO_SCALAR!(__val1),
            __TO_SCALAR!(__val2)
        );
    }};
}

macro_rules! ASSERT_NE {
    ($val1:expr, $val2:expr) => {{
        let __val1 = $val1;
        let __val2 = $val2;
        __ASSERT_TRUE!(
            __val1 != __val2,
            " (0x{:x} == 0x{:x})",
            __TO_SCALAR!(__val1),
            __TO_SCALAR!(__val2)
        );
    }};
}

macro_rules! ASSERT_FLAGS_SAME_MASK {
    ($flags:expr, $flags_other:expr) => {
        ASSERT_TRUE!(vma_flags_same_mask($flags, $flags_other))
    };
}

macro_rules! ASSERT_FLAGS_NOT_SAME_MASK {
    ($flags:expr, $flags_other:expr) => {
        ASSERT_FALSE!(vma_flags_same_mask($flags, $flags_other))
    };
}

macro_rules! ASSERT_FLAGS_SAME {
    ($flags:expr $(, $args:expr)* $(,)?) => {
        ASSERT_TRUE!(vma_flags_same($flags $(, $args)*))
    };
}

macro_rules! ASSERT_FLAGS_NOT_SAME {
    ($flags:expr $(, $args:expr)* $(,)?) => {
        ASSERT_FALSE!(vma_flags_same($flags $(, $args)*))
    };
}

macro_rules! ASSERT_FLAGS_EMPTY {
    ($flags:expr) => {
        ASSERT_TRUE!(vma_flags_empty($flags))
    };
}

macro_rules! ASSERT_FLAGS_NONEMPTY {
    ($flags:expr) => {
        ASSERT_FALSE!(vma_flags_empty($flags))
    };
}

/* Override vma_iter_prealloc() so we can choose to fail it. */
macro_rules! vma_iter_prealloc {
    ($vmi:expr, $vma:expr) => {{
        if fail_prealloc {
            -ENOMEM
        } else {
            mas_preallocate(&mut (*$vmi).mas, $vma, GFP_KERNEL)
        }
    }};
}

pub(crate) use __ASSERT_TRUE;
pub(crate) use __TO_SCALAR;
pub(crate) use ASSERT_EQ;
pub(crate) use ASSERT_FALSE;
pub(crate) use ASSERT_FLAGS_EMPTY;
pub(crate) use ASSERT_FLAGS_NONEMPTY;
pub(crate) use ASSERT_FLAGS_NOT_SAME;
pub(crate) use ASSERT_FLAGS_NOT_SAME_MASK;
pub(crate) use ASSERT_FLAGS_SAME;
pub(crate) use ASSERT_FLAGS_SAME_MASK;
pub(crate) use ASSERT_NE;
pub(crate) use ASSERT_TRUE;
pub(crate) use TEST;
pub(crate) use vma_iter_prealloc;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
