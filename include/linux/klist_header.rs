/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * klist.h - Some generic list helpers, extending struct list_head a bit.
 *
 * Implementations are found in lib/klist.c
 *
 * Copyright (C) 2005 Patrick Mochel
 */

// Dependencies supplied by the surrounding Linux translation.

#[repr(C)]
pub struct klist {
    pub k_lock: spinlock_t,
    pub k_list: list_head,
    pub get: Option<unsafe extern "C" fn(*mut klist_node)>,
    pub put: Option<unsafe extern "C" fn(*mut klist_node)>,
}

// C: __attribute__((aligned(sizeof(void *))))

#[macro_export]
macro_rules! KLIST_INIT {
    ($name:ident, $get:expr, $put:expr) => {
        klist {
            k_lock: __SPIN_LOCK_UNLOCKED!($name.k_lock),
            k_list: LIST_HEAD_INIT!($name.k_list),
            get: $get,
            put: $put,
        }
    };
}

#[macro_export]
macro_rules! DEFINE_KLIST {
    ($name:ident, $get:expr, $put:expr) => {
        static mut $name: klist = KLIST_INIT!($name, $get, $put);
    };
}

unsafe extern "C" {
    pub fn klist_init(
        k: *mut klist,
        get: Option<unsafe extern "C" fn(*mut klist_node)>,
        put: Option<unsafe extern "C" fn(*mut klist_node)>,
    );
}

#[repr(C)]
pub struct klist_node {
    pub n_klist: *mut core::ffi::c_void, // never access directly
    pub n_node: list_head,
    pub n_ref: kref,
}

unsafe extern "C" {
    pub fn klist_add_tail(n: *mut klist_node, k: *mut klist);
    pub fn klist_add_head(n: *mut klist_node, k: *mut klist);
    pub fn klist_add_behind(n: *mut klist_node, pos: *mut klist_node);
    pub fn klist_add_before(n: *mut klist_node, pos: *mut klist_node);

    pub fn klist_del(n: *mut klist_node);
    pub fn klist_remove(n: *mut klist_node);

    pub fn klist_node_attached(n: *mut klist_node) -> core::ffi::c_int;
}

#[repr(C)]
pub struct klist_iter {
    pub i_klist: *mut klist,
    pub i_cur: *mut klist_node,
}

unsafe extern "C" {
    pub fn klist_iter_init(k: *mut klist, i: *mut klist_iter);
    pub fn klist_iter_init_node(k: *mut klist, i: *mut klist_iter, n: *mut klist_node);
    pub fn klist_iter_exit(i: *mut klist_iter);
    pub fn klist_prev(i: *mut klist_iter) -> *mut klist_node;
    pub fn klist_next(i: *mut klist_iter) -> *mut klist_node;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
