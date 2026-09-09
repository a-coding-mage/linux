// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * item.c - library routines for handling generic config items
 *
 * Based on kobject:
 *	kobject is Copyright (c) 2002-2003 Patrick Mochel
 *
 * configfs Copyright (C) 2005 Oracle.  All rights reserved.
 *
 * Please see the file Documentation/filesystems/configfs.rst for
 * critical information about using the config_item interface.
 */

// Dependencies supplied by the kernel/configfs implementation.
use core::ffi::{c_char, c_int, c_void};

#[inline]
unsafe fn to_item(entry: *mut list_head) -> *mut config_item {
    container_of!(entry, config_item, ci_entry)
}

/* Evil kernel */
unsafe extern "C" fn config_item_release(kref: *mut kref);

/**
 *	config_item_init - initialize item.
 *	@item:	item in question.
 */
unsafe fn config_item_init(item: *mut config_item) {
    kref_init(&mut (*item).ci_kref);
    INIT_LIST_HEAD(&mut (*item).ci_entry);
}

/**
 *	config_item_set_name - Set the name of an item
 *	@item:	item.
 *	@fmt:  The vsnprintf()'s format string.
 *
 *	If strlen(name) >= CONFIGFS_ITEM_NAME_LEN, then use a
 *	dynamically allocated string that @item->ci_name points to.
 *	Otherwise, use the static @item->ci_namebuf array.
 */
pub unsafe extern "C" fn config_item_set_name(
    item: *mut config_item,
    fmt: *const c_char,
    mut args: ...,
) -> c_int {
    let limit: c_int = CONFIGFS_ITEM_NAME_LEN;
    let need: c_int;
    let mut name: *mut c_char;

    /* First, try the static array */
    va_start!(args, fmt);
    need = vsnprintf((*item).ci_namebuf.as_mut_ptr(), limit, fmt, args);
    va_end!(args);
    if need < limit {
        name = (*item).ci_namebuf.as_mut_ptr();
    } else {
        va_start!(args, fmt);
        name = kvasprintf(GFP_KERNEL, fmt, args);
        va_end!(args);
        if name.is_null() {
            return -ENOMEM;
        }
    }

    /* Free the old name, if necessary. */
    if !(*item).ci_name.is_null() && (*item).ci_name != (*item).ci_namebuf.as_mut_ptr() {
        kfree((*item).ci_name as *mut c_void);
    }

    /* Now, set the new name */
    (*item).ci_name = name;
    0
}

pub unsafe extern "C" fn config_item_init_type_name(
    item: *mut config_item,
    name: *const c_char,
    type_: *const config_item_type,
) {
    config_item_set_name(item, c"%s".as_ptr(), name);
    (*item).ci_type = type_;
    config_item_init(item);
}

pub unsafe extern "C" fn config_group_init_type_name(
    group: *mut config_group,
    name: *const c_char,
    type_: *const config_item_type,
) {
    config_item_set_name(&mut (*group).cg_item, c"%s".as_ptr(), name);
    (*group).cg_item.ci_type = type_;
    config_group_init(group);
}

pub unsafe extern "C" fn config_item_get(item: *mut config_item) -> *mut config_item {
    if !item.is_null() {
        kref_get(&mut (*item).ci_kref);
    }
    item
}

pub unsafe extern "C" fn config_item_get_unless_zero(item: *mut config_item) -> *mut config_item {
    if !item.is_null() && kref_get_unless_zero(&mut (*item).ci_kref) {
        return item;
    }
    core::ptr::null_mut()
}

unsafe fn config_item_cleanup(item: *mut config_item) {
    let t = (*item).ci_type;
    let s = (*item).ci_group;
    let parent = (*item).ci_parent;

    pr_debug!("config_item %s: cleaning up\n", config_item_name(item));
    if (*item).ci_name != (*item).ci_namebuf.as_mut_ptr() {
        kfree((*item).ci_name as *mut c_void);
    }
    (*item).ci_name = core::ptr::null_mut();
    if !t.is_null() && !(*t).ct_item_ops.is_null() && !(*(*t).ct_item_ops).release.is_none() {
        ((*(*t).ct_item_ops).release)(item);
    }
    if !s.is_null() {
        config_group_put(s);
    }
    if !parent.is_null() {
        config_item_put(parent);
    }
}

unsafe extern "C" fn config_item_release(kref: *mut kref) {
    config_item_cleanup(container_of!(kref, config_item, ci_kref));
}

/**
 *	config_item_put - decrement refcount for item.
 *	@item:	item.
 *
 *	Decrement the refcount, and if 0, call config_item_cleanup().
 */
pub unsafe extern "C" fn config_item_put(item: *mut config_item) {
    if !item.is_null() {
        kref_put(&mut (*item).ci_kref, config_item_release);
    }
}

/**
 *	config_group_init - initialize a group for use
 *	@group:	config_group
 */
pub unsafe extern "C" fn config_group_init(group: *mut config_group) {
    config_item_init(&mut (*group).cg_item);
    INIT_LIST_HEAD(&mut (*group).cg_children);
    INIT_LIST_HEAD(&mut (*group).default_groups);
}

/**
 *	config_group_find_item - search for item in group.
 *	@group:	group we're looking in.
 *	@name:	item's name.
 *
 *	Iterate over @group->cg_list, looking for a matching config_item.
 *	If matching item is found take a reference and return the item.
 *	Caller must have locked group via @group->cg_subsys->su_mtx.
 */
pub unsafe extern "C" fn config_group_find_item(
    group: *mut config_group,
    name: *const c_char,
) -> *mut config_item {
    let mut entry = (*group).cg_children.next;
    let mut ret: *mut config_item = core::ptr::null_mut();

    while entry != &mut (*group).cg_children as *mut list_head {
        let item = to_item(entry);
        if !config_item_name(item).is_null()
            && strcmp(config_item_name(item), name) == 0
        {
            ret = config_item_get(item);
            break;
        }
        entry = (*entry).next;
    }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
