// SPDX-License-Identifier: GPL-2.0
//
// Direct Rust translation of luo_flb.c. Linux kernel types and helpers are
// supplied by the surrounding kernel translation.

use core::ffi::{c_char, c_int, c_long, c_void};

const LUO_FLB_PGCNT: u64 = 1;
const LUO_FLB_MAX: usize = (((LUO_FLB_PGCNT << PAGE_SHIFT)
    - core::mem::size_of::<luo_flb_header_ser>())
    / core::mem::size_of::<luo_flb_ser>());

#[repr(C)]
pub struct luo_flb_header {
    pub header_ser: *mut luo_flb_header_ser,
    pub ser: *mut luo_flb_ser,
    pub active: bool,
}

#[repr(C)]
pub struct luo_flb_global {
    pub incoming: luo_flb_header,
    pub outgoing: luo_flb_header,
    pub list: list_head,
    pub count: c_long,
}

static mut luo_flb_global: luo_flb_global = luo_flb_global {
    incoming: luo_flb_header { header_ser: core::ptr::null_mut(), ser: core::ptr::null_mut(), active: false },
    outgoing: luo_flb_header { header_ser: core::ptr::null_mut(), ser: core::ptr::null_mut(), active: false },
    list: list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() },
    count: 0,
};

#[repr(C)]
pub struct luo_flb_link {
    pub flb: *mut liveupdate_flb,
    pub list: list_head,
}

unsafe fn luo_flb_get_private(flb: *mut liveupdate_flb) -> *mut luo_flb_private {
    let private = &mut (*flb).private as *mut luo_flb_private;
    static mut LUO_FLB_INIT_LOCK: spinlock_t = spinlock_t::new();
    if smp_load_acquire(&(*private).initialized) { return private; }
    let _guard = spinlock_guard(&mut LUO_FLB_INIT_LOCK);
    if !(*private).initialized {
        mutex_init(&mut (*private).incoming.lock);
        mutex_init(&mut (*private).outgoing.lock);
        INIT_LIST_HEAD(&mut (*private).list);
        (*private).users = 0;
        smp_store_release(&mut (*private).initialized, true);
    }
    private
}

unsafe fn luo_flb_file_preserve_one(flb: *mut liveupdate_flb) -> c_int {
    let private = luo_flb_get_private(flb);
    let _guard = mutex_guard(&mut (*private).outgoing.lock);
    if refcount_read(&(*private).outgoing.count) == 0 {
        let mut args: liveupdate_flb_op_args = core::mem::zeroed();
        if !try_module_get((*(*flb).ops).owner) { return -ENODEV; }
        args.flb = flb;
        let err = ((*(*flb).ops).preserve.unwrap())(&mut args);
        if err != 0 { module_put((*(*flb).ops).owner); return err; }
        (*private).outgoing.data = args.data;
        (*private).outgoing.obj = args.obj;
        refcount_set(&mut (*private).outgoing.count, 1);
    } else { refcount_inc(&mut (*private).outgoing.count); }
    0
}

pub unsafe fn liveupdate_flb_put_outgoing(flb: *mut liveupdate_flb) {
    let private = luo_flb_get_private(flb);
    let _guard = mutex_guard(&mut (*private).outgoing.lock);
    if refcount_dec_and_test(&mut (*private).outgoing.count) {
        let mut args: liveupdate_flb_op_args = core::mem::zeroed();
        args.flb = flb; args.data = (*private).outgoing.data; args.obj = (*private).outgoing.obj;
        if let Some(unpreserve) = (*(*flb).ops).unpreserve { unpreserve(&mut args); }
        (*private).outgoing.data = 0; (*private).outgoing.obj = core::ptr::null_mut();
        module_put((*(*flb).ops).owner);
    }
}

unsafe fn luo_flb_retrieve_one(flb: *mut liveupdate_flb) -> c_int {
    let private = luo_flb_get_private(flb);
    let fh = &mut luo_flb_global.incoming;
    let mut args: liveupdate_flb_op_args = core::mem::zeroed();
    if (*private).incoming.finished { return -ENODATA; }
    if (*private).incoming.retrieve_status < 0 { return (*private).incoming.retrieve_status; }
    if (*private).incoming.retrieve_status > 0 { return 0; }
    if !fh.active { return -ENODATA; }
    let mut found = false;
    for i in 0..(*fh.header_ser).count {
        if strcmp((*fh.ser.add(i as usize)).name.as_ptr(), (*flb).compatible.as_ptr()) == 0 {
            (*private).incoming.data = (*fh.ser.add(i as usize)).data;
            refcount_set(&mut (*private).incoming.count, (*fh.ser.add(i as usize)).count as u32);
            found = true; break;
        }
    }
    if !found { return -ENOENT; }
    if !try_module_get((*(*flb).ops).owner) { return -ENODEV; }
    args.flb = flb; args.data = (*private).incoming.data;
    let err = ((*(*flb).ops).retrieve.unwrap())(&mut args);
    if err != 0 { (*private).incoming.retrieve_status = err; module_put((*(*flb).ops).owner); return err; }
    (*private).incoming.obj = args.obj; (*private).incoming.retrieve_status = 1; 0
}

pub unsafe fn liveupdate_flb_put_incoming(flb: *mut liveupdate_flb) {
    let private = luo_flb_get_private(flb);
    let _guard = mutex_guard(&mut (*private).incoming.lock);
    if !refcount_dec_and_test(&mut (*private).incoming.count) { return; }
    if (*private).incoming.retrieve_status <= 0 {
        let err = luo_flb_retrieve_one(flb); if WARN_ON(err != 0) { return; }
    }
    let mut args: liveupdate_flb_op_args = core::mem::zeroed();
    args.flb = flb; args.obj = (*private).incoming.obj;
    ((*(*flb).ops).finish.unwrap())(&mut args);
    (*private).incoming.data = 0; (*private).incoming.obj = core::ptr::null_mut();
    (*private).incoming.finished = true; module_put((*(*flb).ops).owner);
}

pub unsafe fn luo_flb_file_preserve(fh: *mut liveupdate_file_handler) -> c_int {
    let flb_list = &mut (*fh).flb_list;
    let mut iter = list_first_entry::<luo_flb_link>(flb_list);
    down_read(&mut luo_register_rwlock);
    while !iter.is_null() {
        let err = luo_flb_file_preserve_one((*iter).flb);
        if err != 0 { list_for_each_entry_continue_reverse(iter, flb_list); up_read(&mut luo_register_rwlock); return err; }
        iter = list_next_entry(iter, flb_list);
    }
    up_read(&mut luo_register_rwlock); 0
}

pub unsafe fn luo_flb_file_unpreserve(fh: *mut liveupdate_file_handler) {
    let _guard = rwsem_read_guard(&mut luo_register_rwlock);
    let mut iter = list_last_entry::<luo_flb_link>(&mut (*fh).flb_list);
    while !iter.is_null() { liveupdate_flb_put_outgoing((*iter).flb); iter = list_prev_entry(iter, &mut (*fh).flb_list); }
}

pub unsafe fn luo_flb_file_finish(fh: *mut liveupdate_file_handler) {
    let _guard = rwsem_read_guard(&mut luo_register_rwlock);
    let mut iter = list_last_entry::<luo_flb_link>(&mut (*fh).flb_list);
    while !iter.is_null() { liveupdate_flb_put_incoming((*iter).flb); iter = list_prev_entry(iter, &mut (*fh).flb_list); }
}

// The remaining registration and serialization routines retain the kernel
// list/mutex allocation primitives and their source control flow.
pub unsafe fn liveupdate_unregister_flb(fh: *mut liveupdate_file_handler, flb: *mut liveupdate_flb) {
    if !liveupdate_enabled() { return; }
    let _guard = rwsem_write_guard(&mut luo_register_rwlock);
    luo_flb_unregister_one(fh, flb);
}

pub unsafe fn liveupdate_flb_get_incoming(flb: *mut liveupdate_flb, objp: *mut *mut c_void) -> c_int {
    let private = luo_flb_get_private(flb);
    if !liveupdate_enabled() { return -EOPNOTSUPP; }
    let _guard = mutex_guard(&mut (*private).incoming.lock);
    if (*private).incoming.obj.is_null() { let err = luo_flb_retrieve_one(flb); if err != 0 { return err; } }
    refcount_inc(&mut (*private).incoming.count); *objp = (*private).incoming.obj; 0
}

pub unsafe fn liveupdate_flb_get_outgoing(flb: *mut liveupdate_flb, objp: *mut *mut c_void) -> c_int {
    let private = luo_flb_get_private(flb);
    if !liveupdate_enabled() { return -EOPNOTSUPP; }
    let _guard = mutex_guard(&mut (*private).outgoing.lock);
    if (*private).outgoing.obj.is_null() { return -ENOENT; }
    refcount_inc(&mut (*private).outgoing.count); *objp = (*private).outgoing.obj; 0
}

unsafe fn luo_flb_unregister_one(fh: *mut liveupdate_file_handler, flb: *mut liveupdate_flb) {
    let private = luo_flb_get_private(flb);
    let mut iter = list_first_entry::<luo_flb_link>(&mut (*fh).flb_list);
    while !iter.is_null() {
        let next = list_next_entry(iter, &mut (*fh).flb_list);
        if (*iter).flb == flb {
            list_del(&mut (*iter).list); kfree(iter as *mut c_void);
            (*private).users -= 1;
            if (*private).users == 0 { list_del_init(&mut (*private).list); luo_flb_global.count -= 1; }
            return;
        }
        iter = next;
    }
    pr_warn("Failed to unregister FLB: not found in file handler\n");
}

pub unsafe fn luo_flb_unregister_all(fh: *mut liveupdate_file_handler) {
    if !liveupdate_enabled() { return; }
    let mut iter = list_first_entry::<luo_flb_link>(&mut (*fh).flb_list);
    while !iter.is_null() {
        let next = list_next_entry(iter, &mut (*fh).flb_list);
        luo_flb_unregister_one(fh, (*iter).flb); iter = next;
    }
}

pub unsafe fn liveupdate_register_flb(fh: *mut liveupdate_file_handler, flb: *mut liveupdate_flb) -> c_int {
    let private = luo_flb_get_private(flb);
    if !liveupdate_enabled() { return -EOPNOTSUPP; }
    if (*(*flb).ops).preserve.is_none() || (*(*flb).ops).unpreserve.is_none() ||
       (*(*flb).ops).retrieve.is_none() || (*(*flb).ops).finish.is_none() { return -EINVAL; }
    let link = kzalloc(core::mem::size_of::<luo_flb_link>()) as *mut luo_flb_link;
    if link.is_null() { return -ENOMEM; }
    let _guard = rwsem_write_guard(&mut luo_register_rwlock);
    let mut iter = list_first_entry::<luo_flb_link>(&mut (*fh).flb_list);
    while !iter.is_null() { if (*iter).flb == flb { kfree(link as *mut c_void); return -EEXIST; } iter = list_next_entry(iter, &mut (*fh).flb_list); }
    if (*private).users == 0 {
        if luo_flb_global.count == LUO_FLB_MAX as c_long { kfree(link as *mut c_void); return -ENOSPC; }
        list_add_tail(&mut (*private).list, &mut luo_flb_global.list); luo_flb_global.count += 1;
    }
    (*private).users += 1; (*link).flb = flb; list_add_tail(&mut (*link).list, &mut (*fh).flb_list); 0
}

pub unsafe fn luo_flb_setup_outgoing(flbs_pa: *mut u64) -> c_int {
    let header_ser = kho_alloc_preserve(LUO_FLB_PGCNT << PAGE_SHIFT);
    if IS_ERR(header_ser) { return PTR_ERR(header_ser); }
    *flbs_pa = virt_to_phys(header_ser); (*header_ser).pgcnt = LUO_FLB_PGCNT;
    luo_flb_global.outgoing.header_ser = header_ser;
    luo_flb_global.outgoing.ser = header_ser.add(1) as *mut luo_flb_ser; luo_flb_global.outgoing.active = true; 0
}

pub unsafe fn luo_flb_setup_incoming(flbs_pa: u64) {
    if flbs_pa == 0 { return; }
    let header_ser = phys_to_virt(flbs_pa) as *mut luo_flb_header_ser;
    luo_flb_global.incoming.header_ser = header_ser; luo_flb_global.incoming.ser = header_ser.add(1) as *mut luo_flb_ser; luo_flb_global.incoming.active = true;
}

pub unsafe fn luo_flb_serialize() {
    let fh = &mut luo_flb_global.outgoing; let mut i = 0usize;
    let mut gflb = list_first_entry::<liveupdate_flb>(&mut luo_flb_global.list);
    while !gflb.is_null() {
        let private = luo_flb_get_private(gflb); let count = refcount_read(&(*private).outgoing.count);
        if count > 0 { strscpy((*fh.ser.add(i)).name.as_mut_ptr(), (*gflb).compatible.as_ptr(), (*fh.ser.add(i)).name.len()); (*fh.ser.add(i)).data = (*private).outgoing.data; (*fh.ser.add(i)).count = count as c_long; i += 1; }
        gflb = list_next_entry(gflb, &mut luo_flb_global.list);
    }
    (*fh.header_ser).count = i as c_long;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
