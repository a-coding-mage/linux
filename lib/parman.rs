/*
 * lib/parman.c - Manager for linear priority array areas
 * Copyright (c) 2017 Mellanox Technologies. All rights reserved.
 * Copyright (c) 2017 Jiri Pirko <jiri@mellanox.com>
 *
 * Rust translation of the original C implementation.
 */

use core::ptr;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct parman_ops {
    pub resize_step: usize,
    pub base_count: usize,
    pub algo: usize,
    pub resize: Option<unsafe extern "C" fn(*mut core::ffi::c_void, usize) -> i32>,
    pub move_: Option<unsafe extern "C" fn(*mut core::ffi::c_void, usize, usize, usize)>,
}

#[repr(C)]
pub struct parman_prio {
    pub list: list_head,
    pub item_list: list_head,
    pub priority: usize,
}

#[repr(C)]
pub struct parman_item {
    pub list: list_head,
    pub index: usize,
}

#[repr(C)]
struct parman_algo {
    item_add: Option<unsafe extern "C" fn(*mut parman, *mut parman_prio, *mut parman_item) -> i32>,
    item_remove: Option<unsafe extern "C" fn(*mut parman, *mut parman_prio, *mut parman_item)>,
}

#[repr(C)]
pub struct parman {
    ops: *const parman_ops,
    priv_: *mut core::ffi::c_void,
    algo: *const parman_algo,
    count: usize,
    limit_count: usize,
    prio_list: list_head,
}

unsafe fn list_init(head: *mut list_head) {
    (*head).next = head;
    (*head).prev = head;
}

unsafe fn list_empty(head: *const list_head) -> bool {
    (*head).next == head as *mut list_head
}

unsafe fn list_insert(new: *mut list_head, prev: *mut list_head, next: *mut list_head) {
    (*next).prev = new;
    (*new).next = next;
    (*new).prev = prev;
    (*prev).next = new;
}

unsafe fn list_del(entry: *mut list_head) {
    (*(*entry).next).prev = (*entry).prev;
    (*(*entry).prev).next = (*entry).next;
    list_init(entry);
}

unsafe fn list_move(entry: *mut list_head, head: *mut list_head) {
    list_del(entry);
    list_insert(entry, head, (*head).next);
}

unsafe fn list_move_tail(entry: *mut list_head, head: *mut list_head) {
    list_del(entry);
    list_insert(entry, (*head).prev, head);
}

unsafe fn list_replace(old: *mut list_head, new: *mut list_head) {
    (*new).next = (*old).next;
    (*new).next.as_mut().unwrap().prev = new;
    (*new).prev = (*old).prev;
    (*new).prev.as_mut().unwrap().next = new;
    list_init(old);
}

unsafe fn parman_enlarge(parman: *mut parman) -> i32 {
    let new_count = (*parman).limit_count + (*(*parman).ops).resize_step;
    let err = ((*(*parman).ops).resize.unwrap())((*parman).priv_, new_count);
    if err != 0 { return err; }
    (*parman).limit_count = new_count;
    0
}

unsafe fn parman_shrink(parman: *mut parman) -> i32 {
    let new_count = (*parman).limit_count - (*(*parman).ops).resize_step;
    if new_count < (*(*parman).ops).base_count { return 0; }
    let err = ((*(*parman).ops).resize.unwrap())((*parman).priv_, new_count);
    if err != 0 { return err; }
    (*parman).limit_count = new_count;
    0
}

unsafe fn parman_prio_used(prio: *mut parman_prio) -> bool { !list_empty(&(*prio).item_list) }
unsafe fn first_item(prio: *mut parman_prio) -> *mut parman_item {
    (*(*prio).item_list.next as *mut parman_item).list.next as *mut parman_item
}
unsafe fn last_item(prio: *mut parman_prio) -> *mut parman_item {
    (*(*prio).item_list.prev as *mut parman_item).list.prev as *mut parman_item
}
unsafe fn first_index(prio: *mut parman_prio) -> usize { (*first_item(prio)).index }
unsafe fn last_index(prio: *mut parman_prio) -> usize { (*last_item(prio)).index }

unsafe fn move_item(parman: *mut parman, item: *mut parman_item, to: usize, count: usize) {
    ((*(*parman).ops).move_.unwrap())((*parman).priv_, (*item).index, to, count);
}

unsafe fn shift_down(parman: *mut parman, prio: *mut parman_prio) {
    if !parman_prio_used(prio) { return; }
    let item = first_item(prio); let to = last_index(prio) + 1;
    move_item(parman, item, to, 1); list_move_tail(&mut (*item).list, &mut (*prio).item_list); (*item).index = to;
}
unsafe fn shift_up(parman: *mut parman, prio: *mut parman_prio) {
    if !parman_prio_used(prio) { return; }
    let item = last_item(prio); let to = first_index(prio) - 1;
    move_item(parman, item, to, 1); list_move(&mut (*item).list, &mut (*prio).item_list); (*item).index = to;
}

unsafe fn prio_item_remove(parman: *mut parman, prio: *mut parman_prio, item: *mut parman_item) {
    let last = last_item(prio);
    if last == item { list_del(&mut (*item).list); return; }
    let to = (*item).index; move_item(parman, last, to, 1);
    list_del(&mut (*last).list); list_replace(&mut (*item).list, &mut (*last).list); (*last).index = to;
}

unsafe extern "C" fn lsort_item_add(parman: *mut parman, prio: *mut parman_prio, item: *mut parman_item) -> i32 {
    if (*parman).count + 1 > (*parman).limit_count { let err = parman_enlarge(parman); if err != 0 { return err; } }
    let mut pos = (*parman).prio_list.prev;
    while pos != &mut (*parman).prio_list as *mut list_head {
        let p = pos as *mut parman_prio;
        if p == prio { break; }
        shift_down(parman, p); pos = (*pos).prev;
    }
    (*item).index = 0;
    list_insert(&mut (*item).list, (*prio).item_list.prev, &mut (*prio).item_list);
    (*parman).count += 1; 0
}

unsafe extern "C" fn lsort_item_remove(parman: *mut parman, prio: *mut parman_prio, item: *mut parman_item) {
    prio_item_remove(parman, prio, item); (*parman).count -= 1;
    if (*parman).limit_count - (*parman).count >= (*(*parman).ops).resize_step { let _ = parman_shrink(parman); }
}

static PARMAN_LSORT: parman_algo = parman_algo { item_add: Some(lsort_item_add), item_remove: Some(lsort_item_remove) };
static PARMAN_ALGOS: [*const parman_algo; 1] = [&PARMAN_LSORT];

#[no_mangle]
pub unsafe extern "C" fn parman_create(ops: *const parman_ops, priv_: *mut core::ffi::c_void) -> *mut parman {
    let p = Box::into_raw(Box::new(parman { ops, priv_, algo: PARMAN_ALGOS[(*ops).algo], count: 0, limit_count: (*ops).base_count, prio_list: list_head { next: ptr::null_mut(), prev: ptr::null_mut() } }));
    list_init(&mut (*p).prio_list); p
}

#[no_mangle]
pub unsafe extern "C" fn parman_destroy(parman: *mut parman) { let _ = list_empty(&(*parman).prio_list); drop(Box::from_raw(parman)); }

#[no_mangle]
pub unsafe extern "C" fn parman_prio_init(parman: *mut parman, prio: *mut parman_prio, priority: usize) {
    list_init(&mut (*prio).item_list); (*prio).priority = priority;
    let mut pos = &mut (*parman).prio_list as *mut list_head;
    while (*pos).next != &mut (*parman).prio_list as *mut list_head {
        let p = (*pos).next as *mut parman_prio;
        if (*p).priority > (*prio).priority { break; } pos = (*pos).next;
    }
    list_insert(&mut (*prio).list, pos, (*pos).next);
}

#[no_mangle]
pub unsafe extern "C" fn parman_prio_fini(prio: *mut parman_prio) { list_del(&mut (*prio).list); }
#[no_mangle]
pub unsafe extern "C" fn parman_item_add(parman: *mut parman, prio: *mut parman_prio, item: *mut parman_item) -> i32 { ((*(*parman).algo).item_add.unwrap())(parman, prio, item) }
#[no_mangle]
pub unsafe extern "C" fn parman_item_remove(parman: *mut parman, prio: *mut parman_prio, item: *mut parman_item) { ((*(*parman).algo).item_remove.unwrap())(parman, prio, item); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
