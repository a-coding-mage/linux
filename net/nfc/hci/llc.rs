// SPDX-License-Identifier: GPL-2.0-only
/*
 * Link Layer Control manager
 *
 * Copyright (C) 2012  Intel Corporation. All rights reserved.
 */

// Dependencies supplied by the kernel NFC LLC implementation.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct nfc_llc_engine {
    pub entry: list_head,
    pub name: *const c_char,
    pub ops: *const nfc_llc_ops,
}

#[repr(C)]
pub struct nfc_llc {
    pub data: *mut c_void,
    pub ops: *const nfc_llc_ops,
    pub rx_headroom: c_int,
    pub rx_tailroom: c_int,
}

#[repr(C)]
pub struct nfc_llc_ops {
    pub init: Option<unsafe extern "C" fn(
        *mut nfc_hci_dev,
        xmit_to_drv_t,
        rcv_to_hci_t,
        c_int,
        c_int,
        *mut c_int,
        *mut c_int,
        llc_failure_t,
    ) -> *mut c_void>,
    pub deinit: Option<unsafe extern "C" fn(*mut nfc_llc)>,
    pub start: Option<unsafe extern "C" fn(*mut nfc_llc) -> c_int>,
    pub stop: Option<unsafe extern "C" fn(*mut nfc_llc) -> c_int>,
    pub rcv_from_drv: Option<unsafe extern "C" fn(*mut nfc_llc, *mut sk_buff)>,
    pub xmit_from_hci: Option<unsafe extern "C" fn(*mut nfc_llc, *mut sk_buff) -> c_int>,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

pub enum nfc_hci_dev {}
pub enum sk_buff {}
pub type xmit_to_drv_t = Option<unsafe extern "C" fn(*mut c_void)>;
pub type rcv_to_hci_t = Option<unsafe extern "C" fn(*mut c_void)>;
pub type llc_failure_t = Option<unsafe extern "C" fn(*mut c_void)>;

extern "C" {
    fn nfc_llc_nop_register() -> c_int;
    fn nfc_llc_shdlc_register() -> c_int;
    fn kfree_const(ptr: *const c_void);
    fn kfree(ptr: *mut c_void);
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn kstrdup_const(name: *const c_char, flags: c_int) -> *const c_char;
}

// The kernel list and allocation helpers are represented by their C semantics.
static mut llc_engines: list_head = list_head {
    next: core::ptr::null_mut(),
    prev: core::ptr::null_mut(),
};

unsafe fn nfc_llc_del_engine(llc_engine: *mut nfc_llc_engine) {
    list_del(&mut (*llc_engine).entry);
    kfree_const((*llc_engine).name as *const c_void);
    kfree(llc_engine as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn nfc_llc_init() -> c_int {
    let mut r: c_int;

    r = nfc_llc_nop_register();
    if r != 0 {
        nfc_llc_exit();
        return r;
    }

    r = nfc_llc_shdlc_register();
    if r != 0 {
        nfc_llc_exit();
        return r;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn nfc_llc_exit() {
    let mut llc_engine: *mut nfc_llc_engine;
    let mut n: *mut nfc_llc_engine;

    list_for_each_entry_safe(&mut llc_engine, &mut n, &mut llc_engines, nfc_llc_engine::entry);
    while !llc_engine.is_null() {
        nfc_llc_del_engine(llc_engine);
        llc_engine = n;
    }
}

#[no_mangle]
pub unsafe extern "C" fn nfc_llc_register(
    name: *const c_char,
    ops: *const nfc_llc_ops,
) -> c_int {
    let llc_engine = kzalloc_nfc_llc_engine();
    if llc_engine.is_null() {
        return -12;
    }

    (*llc_engine).name = kstrdup_const(name, 0);
    if (*llc_engine).name.is_null() {
        kfree(llc_engine as *mut c_void);
        return -12;
    }
    (*llc_engine).ops = ops;

    init_list_head(&mut (*llc_engine).entry);
    list_add_tail(&mut (*llc_engine).entry, &mut llc_engines);

    0
}

unsafe fn nfc_llc_name_to_engine(name: *const c_char) -> *mut nfc_llc_engine {
    let mut llc_engine: *mut nfc_llc_engine = core::ptr::null_mut();
    list_for_each_entry(&mut llc_engine, &mut llc_engines, nfc_llc_engine::entry);
    while !llc_engine.is_null() {
        if strcmp((*llc_engine).name, name) == 0 {
            return llc_engine;
        }
        llc_engine = (*llc_engine).entry.next as *mut nfc_llc_engine;
    }
    core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn nfc_llc_allocate(
    name: *const c_char,
    hdev: *mut nfc_hci_dev,
    xmit_to_drv: xmit_to_drv_t,
    rcv_to_hci: rcv_to_hci_t,
    tx_headroom: c_int,
    tx_tailroom: c_int,
    llc_failure: llc_failure_t,
) -> *mut nfc_llc {
    let llc_engine = nfc_llc_name_to_engine(name);
    if llc_engine.is_null() {
        return core::ptr::null_mut();
    }

    let llc = kzalloc_nfc_llc();
    if llc.is_null() {
        return core::ptr::null_mut();
    }

    (*llc).data = ((*(*llc_engine).ops).init.unwrap())(
        hdev, xmit_to_drv, rcv_to_hci, tx_headroom, tx_tailroom,
        &mut (*llc).rx_headroom, &mut (*llc).rx_tailroom, llc_failure,
    );
    if (*llc).data.is_null() {
        kfree(llc as *mut c_void);
        return core::ptr::null_mut();
    }
    (*llc).ops = (*llc_engine).ops;
    llc
}

#[no_mangle]
pub unsafe extern "C" fn nfc_llc_free(llc: *mut nfc_llc) {
    ((*(*llc).ops).deinit.unwrap())(llc);
    kfree(llc as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn nfc_llc_start(llc: *mut nfc_llc) -> c_int {
    ((*(*llc).ops).start.unwrap())(llc)
}

#[no_mangle]
pub unsafe extern "C" fn nfc_llc_stop(llc: *mut nfc_llc) -> c_int {
    ((*(*llc).ops).stop.unwrap())(llc)
}

#[no_mangle]
pub unsafe extern "C" fn nfc_llc_rcv_from_drv(llc: *mut nfc_llc, skb: *mut sk_buff) {
    ((*(*llc).ops).rcv_from_drv.unwrap())(llc, skb);
}

#[no_mangle]
pub unsafe extern "C" fn nfc_llc_xmit_from_hci(llc: *mut nfc_llc, skb: *mut sk_buff) -> c_int {
    ((*(*llc).ops).xmit_from_hci.unwrap())(llc, skb)
}

#[no_mangle]
pub unsafe extern "C" fn nfc_llc_get_data(llc: *mut nfc_llc) -> *mut c_void {
    (*llc).data
}

// External kernel list/allocation operations.
extern "C" {
    fn list_del(entry: *mut list_head);
    fn init_list_head(entry: *mut list_head);
    fn list_add_tail(entry: *mut list_head, head: *mut list_head);
    fn list_for_each_entry(entry: *mut *mut nfc_llc_engine, head: *mut list_head, member: usize);
    fn list_for_each_entry_safe(entry: *mut *mut nfc_llc_engine, next: *mut *mut nfc_llc_engine, head: *mut list_head, member: usize);
    fn kzalloc_nfc_llc_engine() -> *mut nfc_llc_engine;
    fn kzalloc_nfc_llc() -> *mut nfc_llc;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
