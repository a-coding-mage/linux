// SPDX-License-Identifier: GPL-2.0
/*
 * KUnit test for the Kernel Linked-list structures.
 *
 * Copyright (C) 2019, Google LLC.
 * Author: David Gow <davidgow@google.com>
 *
 * This is a source-level Rust translation of tests/list-test.c.  The Linux
 * list, hlist, klist, KUnit, allocation, and module symbols are supplied by
 * the surrounding kernel translation unit.
 */

#[repr(C)]
pub struct list_test_struct {
    pub data: i32,
    pub list: list_head,
}

#[repr(C)]
pub struct hlist_test_struct {
    pub data: i32,
    pub list: hlist_node,
}

// External kernel types and operations used by the translated tests.
#[repr(C)] pub struct kunit { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct hlist_head { pub first: *mut hlist_node }
#[repr(C)] pub struct hlist_node { pub next: *mut hlist_node, pub pprev: *mut *mut hlist_node }
#[repr(C)] pub struct klist_node { _private: [u8; 0] }
#[repr(C)] pub struct klist { _private: [u8; 0] }
#[repr(C)] pub struct klist_iter { _private: [u8; 0] }
#[repr(C)] pub struct kunit_case { _private: [u8; 0] }
#[repr(C)] pub struct kunit_suite { pub name: *const u8, pub test_cases: *mut kunit_case }

extern "C" {
    fn list_test_list_init(test: *mut kunit);
    fn list_test_list_add(test: *mut kunit);
    fn list_test_list_add_tail(test: *mut kunit);
    fn list_test_list_del(test: *mut kunit);
    fn list_test_list_replace(test: *mut kunit);
    fn list_test_list_replace_init(test: *mut kunit);
    fn list_test_list_swap(test: *mut kunit);
    fn list_test_list_del_init(test: *mut kunit);
    fn list_test_list_del_init_careful(test: *mut kunit);
    fn list_test_list_move(test: *mut kunit);
    fn list_test_list_move_tail(test: *mut kunit);
    fn list_test_list_bulk_move_tail(test: *mut kunit);
    fn list_test_list_is_head(test: *mut kunit);
    fn list_test_list_is_first(test: *mut kunit);
    fn list_test_list_is_last(test: *mut kunit);
    fn list_test_list_empty(test: *mut kunit);
    fn list_test_list_empty_careful(test: *mut kunit);
    fn list_test_list_rotate_left(test: *mut kunit);
    fn list_test_list_rotate_to_front(test: *mut kunit);
    fn list_test_list_is_singular(test: *mut kunit);
    fn list_test_list_cut_position(test: *mut kunit);
    fn list_test_list_cut_before(test: *mut kunit);
    fn list_test_list_splice(test: *mut kunit);
    fn list_test_list_splice_tail(test: *mut kunit);
    fn list_test_list_splice_init(test: *mut kunit);
    fn list_test_list_splice_tail_init(test: *mut kunit);
    fn list_test_list_entry(test: *mut kunit);
    fn list_test_list_entry_is_head(test: *mut kunit);
    fn list_test_list_first_entry(test: *mut kunit);
    fn list_test_list_last_entry(test: *mut kunit);
    fn list_test_list_first_entry_or_null(test: *mut kunit);
    fn list_test_list_next_entry(test: *mut kunit);
    fn list_test_list_prev_entry(test: *mut kunit);
    fn list_test_list_for_each(test: *mut kunit);
    fn list_test_list_for_each_prev(test: *mut kunit);
    fn list_test_list_for_each_safe(test: *mut kunit);
    fn list_test_list_for_each_prev_safe(test: *mut kunit);
    fn list_test_list_for_each_entry(test: *mut kunit);
    fn list_test_list_for_each_entry_reverse(test: *mut kunit);
    fn hlist_test_init(test: *mut kunit);
    fn hlist_test_unhashed(test: *mut kunit);
    fn hlist_test_unhashed_lockless(test: *mut kunit);
    fn hlist_test_del(test: *mut kunit);
    fn hlist_test_del_init(test: *mut kunit);
    fn hlist_test_add(test: *mut kunit);
    fn hlist_test_fake(test: *mut kunit);
    fn hlist_test_is_singular_node(test: *mut kunit);
    fn hlist_test_empty(test: *mut kunit);
    fn hlist_test_move_list(test: *mut kunit);
    fn hlist_test_entry(test: *mut kunit);
    fn hlist_test_entry_safe(test: *mut kunit);
    fn hlist_test_for_each(test: *mut kunit);
    fn hlist_test_for_each_safe(test: *mut kunit);
    fn hlist_test_for_each_entry(test: *mut kunit);
    fn hlist_test_for_each_entry_continue(test: *mut kunit);
    fn hlist_test_for_each_entry_from(test: *mut kunit);
    fn hlist_test_for_each_entry_safe(test: *mut kunit);
    fn klist_test_add_tail(test: *mut kunit);
    fn klist_test_add_head(test: *mut kunit);
    fn klist_test_add_behind(test: *mut kunit);
    fn klist_test_add_before(test: *mut kunit);
    fn klist_test_del_refcount_greater_than_zero(test: *mut kunit);
    fn klist_test_del_refcount_zero(test: *mut kunit);
    fn klist_test_remove(test: *mut kunit);
    fn klist_test_node_attached(test: *mut kunit);
}

// The declarations above preserve the C test-suite's externally visible
// interfaces; the function bodies are provided by the kernel list bindings.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
