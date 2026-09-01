// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2008, Creative Technology Ltd. All Rights Reserved.
 *
 * @File	ctimap.c
 *
 * @Brief
 * This file contains the implementation of generic input mapper operations
 * for input mapper management.
 *
 * @Author	Liu Chun
 * @Date 	May 23 2008
 */

/* Dependencies from ctimap.h and linux/slab.h are expected to be supplied by
 * the surrounding translation unit.
 */

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct imapper {
    pub list: list_head,
    pub slot: i32,
    pub addr: i32,
    pub next: i32,
    pub user: i32,
}

unsafe extern "C" {
    fn list_empty(head: *const list_head) -> i32;
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn __list_add(new: *mut list_head, prev: *mut list_head, next: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn kfree(ptr: *const core::ffi::c_void);
}

const IMAPPER_LIST_OFFSET: usize = core::mem::offset_of!(imapper, list);

unsafe fn list_entry_imapper(ptr: *mut list_head) -> *mut imapper {
    (ptr as *mut u8).sub(IMAPPER_LIST_OFFSET) as *mut imapper
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn input_mapper_add(
    mappers: *mut list_head,
    entry: *mut imapper,
    map_op: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut imapper) -> i32>,
    data: *mut core::ffi::c_void,
) -> i32 {
    let mut pos: *mut list_head;
    let mut pre: *mut list_head;
    let head: *mut list_head;
    let pre_ent: *mut imapper;
    let mut pos_ent: *mut imapper;

    head = mappers;

    if unsafe { list_empty(head) } != 0 {
        unsafe {
            (*entry).next = (*entry).addr;
            map_op.expect("non-null function pointer")(data, entry);
            list_add(&mut (*entry).list, head);
        }
        return 0;
    }

    pos = unsafe { (*head).next };
    while pos != head {
        unsafe {
            pos_ent = list_entry_imapper(pos);
            if (*pos_ent).slot > (*entry).slot {
                /* found a position in list */
                break;
            }
            pos = (*pos).next;
        }
    }

    if pos != head {
        unsafe {
            pre = (*pos).prev;
            if pre == head {
                pre = (*head).prev;
            }

            __list_add(&mut (*entry).list, (*pos).prev, pos);
        }
    } else {
        unsafe {
            pre = (*head).prev;
            pos = (*head).next;
            list_add_tail(&mut (*entry).list, head);
        }
    }

    unsafe {
        pre_ent = list_entry_imapper(pre);
        pos_ent = list_entry_imapper(pos);

        (*entry).next = (*pos_ent).addr;
        map_op.expect("non-null function pointer")(data, entry);
        (*pre_ent).next = (*entry).addr;
        map_op.expect("non-null function pointer")(data, pre_ent);
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn input_mapper_delete(
    mappers: *mut list_head,
    entry: *mut imapper,
    map_op: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut imapper) -> i32>,
    data: *mut core::ffi::c_void,
) -> i32 {
    let next: *mut list_head;
    let pre: *mut list_head;
    let head: *mut list_head;
    let pre_ent: *mut imapper;
    let next_ent: *mut imapper;

    head = mappers;

    if unsafe { list_empty(head) } != 0 {
        return 0;
    }

    unsafe {
        pre = if (*entry).list.prev == head {
            (*head).prev
        } else {
            (*entry).list.prev
        };
        next = if (*entry).list.next == head {
            (*head).next
        } else {
            (*entry).list.next
        };

        if pre == &mut (*entry).list {
            /* entry is the only one node in mappers list */
            (*entry).slot = 0;
            (*entry).user = (*entry).slot;
            (*entry).addr = (*entry).user;
            (*entry).next = (*entry).addr;
            map_op.expect("non-null function pointer")(data, entry);
            list_del(&mut (*entry).list);
            return 0;
        }

        pre_ent = list_entry_imapper(pre);
        next_ent = list_entry_imapper(next);

        (*pre_ent).next = (*next_ent).addr;
        map_op.expect("non-null function pointer")(data, pre_ent);
        list_del(&mut (*entry).list);
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_input_mapper_list(head: *mut list_head) {
    let entry: *mut imapper;
    let pos: *mut list_head;

    while unsafe { list_empty(head) } == 0 {
        unsafe {
            pos = (*head).next;
            list_del(pos);
            entry = list_entry_imapper(pos);
            kfree(entry as *const core::ffi::c_void);
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
