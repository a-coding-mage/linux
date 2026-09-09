/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2015, Intel Corporation
 * Author: Jiang Liu <jiang.liu@linux.intel.com>
 */

// Dependencies supplied by the surrounding Linux translation are intentionally
// referenced here rather than reimplemented.

/* Represent resource window for bridge devices */
#[repr(C)]
pub struct resource_win {
    pub res: resource,                 /* In master (CPU) address space */
    pub offset: resource_size_t,       /* Translation offset for bridge */
}

/*
 * Common resource list management data structure and interfaces to support
 * ACPI, PNP and PCI host bridge etc.
 */
#[repr(C)]
pub struct resource_entry {
    pub node: list_head,
    pub res: *mut resource,             /* In master (CPU) address space */
    pub offset: resource_size_t,        /* Translation offset for bridge */
    pub __res: resource,                /* Default storage for res */
}

extern "C" {
    pub fn resource_list_create_entry(res: *mut resource, extra_size: usize)
        -> *mut resource_entry;
    pub fn resource_list_free(head: *mut list_head);
}

pub unsafe fn resource_list_add(entry: *mut resource_entry, head: *mut list_head) {
    list_add(&mut (*entry).node, head);
}

pub unsafe fn resource_list_add_tail(entry: *mut resource_entry, head: *mut list_head) {
    list_add_tail(&mut (*entry).node, head);
}

pub unsafe fn resource_list_del(entry: *mut resource_entry) {
    list_del(&mut (*entry).node);
}

pub unsafe fn resource_list_free_entry(entry: *mut resource_entry) {
    kfree(entry.cast());
}

pub unsafe fn resource_list_destroy_entry(entry: *mut resource_entry) {
    resource_list_del(entry);
    resource_list_free_entry(entry);
}

macro_rules! resource_list_for_each_entry {
    ($entry:ident, $list:expr) => {
        list_for_each_entry!($entry, $list, node)
    };
}

macro_rules! resource_list_for_each_entry_safe {
    ($entry:ident, $tmp:ident, $list:expr) => {
        list_for_each_entry_safe!($entry, $tmp, $list, node)
    };
}

pub unsafe fn resource_list_first_type(
    list: *mut list_head,
    type_: c_ulong,
) -> *mut resource_entry {
    let mut entry: *mut resource_entry;

    resource_list_for_each_entry!(entry, list) {
        if resource_type((*entry).res) == type_ {
            return entry;
        }
    }
    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
