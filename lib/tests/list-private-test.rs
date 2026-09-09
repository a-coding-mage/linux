// SPDX-License-Identifier: GPL-2.0
/*
 * KUnit compilation/smoke test for Private list primitives.
 *
 * Copyright (c) 2025, Google LLC.
 * Pasha Tatashin <pasha.tatashin@soleen.com>
 */

// The C test deliberately redefines __private as volatile and ACCESS_PRIVATE
// as a cast through the address of the private member.  The corresponding
// kernel list primitives and KUnit declarations are supplied externally.

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct kunit {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_test_struct {
    pub data: i32,
    pub list: core::cell::UnsafeCell<list_head>,
}

extern "C" {
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn list_private_entry(
        ptr: *mut list_head,
    ) -> *mut list_test_struct;
    fn list_private_first_entry(head: *mut list_head) -> *mut list_test_struct;
    fn list_private_last_entry(head: *mut list_head) -> *mut list_test_struct;
    fn list_private_next_entry(pos: *mut list_test_struct) -> *mut list_test_struct;
    fn list_private_prev_entry(pos: *mut list_test_struct) -> *mut list_test_struct;
    fn list_private_next_entry_circular(
        pos: *mut list_test_struct,
        head: *mut list_head,
    ) -> *mut list_test_struct;
    fn list_private_prev_entry_circular(
        pos: *mut list_test_struct,
        head: *mut list_head,
    ) -> *mut list_test_struct;
    fn list_private_entry_is_head(
        pos: *mut list_test_struct,
        head: *mut list_head,
    ) -> bool;
    fn list_private_safe_reset_next(
        pos: *mut list_test_struct,
        n: *mut list_test_struct,
    );
}

unsafe fn list_private_compile_test(_test: *mut kunit) {
    let mut entry: list_test_struct = core::mem::zeroed();
    let mut pos: *mut list_test_struct;
    let mut n: *mut list_test_struct;
    let mut head: list_head = core::mem::zeroed();

    INIT_LIST_HEAD(entry.list.get());
    list_add(entry.list.get(), &mut head);
    pos = &mut entry;

    pos = list_private_entry(entry.list.get());
    pos = list_private_first_entry(&mut head);
    pos = list_private_last_entry(&mut head);
    pos = list_private_next_entry(pos);
    pos = list_private_prev_entry(pos);
    pos = list_private_next_entry_circular(pos, &mut head);
    pos = list_private_prev_entry_circular(pos, &mut head);

    if list_private_entry_is_head(pos, &mut head) {
        return;
    }

    // list_private_for_each_entry(pos, &head, list) { }
    // list_private_for_each_entry_reverse(pos, &head, list) { }
    // list_private_for_each_entry_continue(pos, &head, list) { }
    // list_private_for_each_entry_continue_reverse(pos, &head, list) { }
    // list_private_for_each_entry_from(pos, &head, list) { }
    // list_private_for_each_entry_from_reverse(pos, &head, list) { }

    // list_private_for_each_entry_safe(pos, n, &head, list)
    //     list_private_safe_reset_next(pos, n, list);
    list_private_safe_reset_next(pos, n);
    // list_private_for_each_entry_safe_continue(pos, n, &head, list) { }
    // list_private_for_each_entry_safe_from(pos, n, &head, list) { }
    // list_private_for_each_entry_safe_reverse(pos, n, &head, list) { }
}

// static struct kunit_case list_private_test_cases[] = {
//     KUNIT_CASE(list_private_compile_test),
//     {},
// };
// static struct kunit_suite list_private_test_module = {
//     .name = "list-private-kunit-test",
//     .test_cases = list_private_test_cases,
// };
// kunit_test_suite(list_private_test_module);
// MODULE_DESCRIPTION("KUnit compilation test for private list primitives");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
