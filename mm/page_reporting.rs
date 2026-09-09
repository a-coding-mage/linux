// SPDX-License-Identifier: GPL-2.0

/* External kernel dependencies are supplied by the surrounding translation unit. */
use core::ffi::{c_char, c_int, c_void};

extern "C" {
    static mut pageblock_order: u32;
    static mut system_freezable_wq: *mut c_void;
}

pub const PAGE_REPORTING_ORDER_UNSPECIFIED: u32 = u32::MAX;
pub const PAGE_REPORTING_IDLE: i32 = 0;
pub const PAGE_REPORTING_REQUESTED: i32 = 1;
pub const PAGE_REPORTING_ACTIVE: i32 = 2;

pub static mut page_reporting_order: u32 = PAGE_REPORTING_ORDER_UNSPECIFIED;
static mut page_reporting_delay_ms: u32 = 2 * MSEC_PER_SEC;
static mut pr_dev_info: *mut page_reporting_dev_info = core::ptr::null_mut();

#[repr(C)]
pub struct page_reporting_dev_info {
    pub state: atomic_t,
    pub work: delayed_work,
    pub capacity: u32,
    pub order: u32,
    pub report: Option<unsafe extern "C" fn(*mut page_reporting_dev_info, *mut scatterlist, u32) -> c_int>,
}

#[repr(C)] pub struct atomic_t { pub counter: c_int }
#[repr(C)] pub struct delayed_work { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct scatterlist { pub length: u32, _private: [u8; 0] }
#[repr(C)] pub struct page { pub lru: list_head, _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct free_area { pub free_list: *mut list_head, pub nr_free: u32 }
#[repr(C)] pub struct zone { pub free_area: *mut free_area, pub lock: c_void }
#[repr(C)] pub struct kernel_param { _private: [u8; 0] }
#[repr(C)] pub struct kernel_param_ops { pub set: Option<unsafe extern "C" fn(*const c_char, *const kernel_param) -> c_int>, pub get: Option<unsafe extern "C" fn(*const c_char, *const kernel_param) -> c_int> }

extern "C" {
    fn param_set_uint_minmax(val: *const c_char, kp: *const kernel_param, min: u32, max: u32) -> c_int;
    fn param_get_int(buffer: *mut c_char, kp: *const kernel_param) -> c_int;
    fn queue_delayed_work(wq: *mut c_void, work: *mut delayed_work, delay: u64) -> bool;
    fn msecs_to_jiffies(ms: u32) -> u64;
    fn atomic_read(v: *const atomic_t) -> c_int;
    fn atomic_xchg(v: *mut atomic_t, value: c_int) -> c_int;
    fn atomic_set(v: *mut atomic_t, value: c_int);
    fn atomic_cmpxchg(v: *mut atomic_t, old: c_int, new: c_int) -> c_int;
    fn rcu_read_lock(); fn rcu_read_unlock();
    fn rcu_dereference(p: *mut page_reporting_dev_info) -> *mut page_reporting_dev_info;
    fn rcu_dereference_protected(p: *mut page_reporting_dev_info, condition: bool) -> *mut page_reporting_dev_info;
    fn rcu_assign_pointer(p: *mut *mut page_reporting_dev_info, v: *mut page_reporting_dev_info);
    fn synchronize_rcu(); fn rcu_init_pointer(p: *mut *mut page_reporting_dev_info, v: *mut page_reporting_dev_info);
    fn sg_page(sg: *mut scatterlist) -> *mut page; fn sg_next(sg: *mut scatterlist) -> *mut scatterlist;
    fn sg_set_page(sg: *mut scatterlist, page: *mut page, length: u32, offset: u32);
    fn sg_init_table(sg: *mut scatterlist, nents: u32);
    fn get_pageblock_migratetype(page: *mut page) -> c_int; fn get_order(length: u32) -> u32;
    fn __putback_isolated_page(page: *mut page, order: u32, mt: c_int);
    fn PageBuddy(page: *mut page) -> bool; fn buddy_order(page: *mut page) -> u32; fn __SetPageReported(page: *mut page);
    fn list_empty(list: *mut list_head) -> bool; fn spin_lock_irq(lock: *mut c_void); fn spin_unlock_irq(lock: *mut c_void);
    fn __isolate_free_page(page: *mut page, order: u32) -> bool;
    fn list_is_first(node: *mut list_head, head: *mut list_head) -> bool; fn list_rotate_to_front(node: *mut list_head, head: *mut list_head);
    fn list_entry_is_head(node: *mut page, head: *mut list_head, member: *mut list_head) -> bool;
    fn low_wmark_pages(zone: *mut zone) -> u64; fn zone_watermark_ok(zone: *mut zone, order: u32, watermark: u64, classzone_idx: c_int, alloc_flags: u32) -> bool;
    fn is_migrate_isolate(mt: u32) -> bool; fn kmalloc_objs(size: usize, count: u32) -> *mut scatterlist; fn kfree(p: *mut scatterlist);
    fn to_delayed_work(work: *mut work_struct) -> *mut delayed_work;
    fn container_of(work: *mut delayed_work) -> *mut page_reporting_dev_info;
    fn mutex_lock(mutex: *mut c_void); fn mutex_unlock(mutex: *mut c_void); fn lockdep_is_held(mutex: *mut c_void) -> bool;
    fn init_delayed_work(work: *mut delayed_work, func: unsafe extern "C" fn(*mut work_struct));
    fn static_key_enabled(key: *mut c_void) -> bool; fn static_branch_enable(key: *mut c_void); fn pr_info(fmt: *const c_char, ...);
    fn cancel_delayed_work_sync(work: *mut delayed_work);
}

const MSEC_PER_SEC: u32 = 1000;
const MAX_PAGE_ORDER: u32 = 64;
const PAGE_SIZE: u32 = 4096;
const NR_PAGE_ORDERS: u32 = 11;
const MIGRATE_TYPES: u32 = 6;
const PAGE_REPORTING_CAPACITY: u32 = 16;
const ALLOC_CMA: u32 = 0x40;

static mut page_reporting_mutex: c_void = c_void { };
static mut page_reporting_enabled: c_void = c_void { };

unsafe extern "C" fn page_order_update_notify(val: *const c_char, kp: *const kernel_param) -> c_int {
    param_set_uint_minmax(val, kp, 0, MAX_PAGE_ORDER)
}

static page_reporting_param_ops: kernel_param_ops = kernel_param_ops { set: Some(page_order_update_notify), get: Some(param_get_int) };

unsafe fn page_reporting_schedule_work(prdev: *mut page_reporting_dev_info) {
    queue_delayed_work(system_freezable_wq, &mut (*prdev).work, msecs_to_jiffies(page_reporting_delay_ms));
}

unsafe fn __page_reporting_request(prdev: *mut page_reporting_dev_info) {
    let mut state = atomic_read(&(*prdev).state);
    if state == PAGE_REPORTING_REQUESTED { return; }
    state = atomic_xchg(&mut (*prdev).state, PAGE_REPORTING_REQUESTED);
    if state != PAGE_REPORTING_IDLE { return; }
    page_reporting_schedule_work(prdev);
}

pub unsafe extern "C" fn __page_reporting_notify() {
    rcu_read_lock();
    let prdev = rcu_dereference(pr_dev_info);
    if !prdev.is_null() { __page_reporting_request(prdev); }
    rcu_read_unlock();
}

unsafe fn page_reporting_drain(_prdev: *mut page_reporting_dev_info, sgl: *mut scatterlist, nents: u32, reported: bool) {
    let mut sg = sgl;
    loop {
        let page = sg_page(sg);
        let mt = get_pageblock_migratetype(page);
        let order = get_order((*sg).length);
        __putback_isolated_page(page, order, mt);
        if reported && PageBuddy(page) && buddy_order(page) == order { __SetPageReported(page); }
        sg = sg_next(sg);
        if sg.is_null() { break; }
    }
    sg_init_table(sgl, nents);
}

pub unsafe extern "C" fn page_reporting_register(prdev: *mut page_reporting_dev_info) -> c_int {
    let mut err = 0;
    mutex_lock(&mut page_reporting_mutex);
    if !rcu_dereference_protected(pr_dev_info, lockdep_is_held(&mut page_reporting_mutex)).is_null() { err = -16; }
    else {
        if page_reporting_order == PAGE_REPORTING_ORDER_UNSPECIFIED {
            if (*prdev).order != PAGE_REPORTING_ORDER_UNSPECIFIED && (*prdev).order <= MAX_PAGE_ORDER { page_reporting_order = (*prdev).order; }
            else { page_reporting_order = pageblock_order; }
        }
        if (*prdev).capacity == 0 || (*prdev).capacity > PAGE_REPORTING_CAPACITY { (*prdev).capacity = PAGE_REPORTING_CAPACITY; }
        atomic_set(&mut (*prdev).state, PAGE_REPORTING_IDLE);
        init_delayed_work(&mut (*prdev).work, page_reporting_process);
        __page_reporting_request(prdev);
        rcu_assign_pointer(&mut pr_dev_info, prdev);
        if !static_key_enabled(&mut page_reporting_enabled) { static_branch_enable(&mut page_reporting_enabled); }
    }
    mutex_unlock(&mut page_reporting_mutex);
    err
}

pub unsafe extern "C" fn page_reporting_unregister(prdev: *mut page_reporting_dev_info) {
    mutex_lock(&mut page_reporting_mutex);
    if prdev == rcu_dereference_protected(pr_dev_info, lockdep_is_held(&mut page_reporting_mutex)) {
        rcu_init_pointer(&mut pr_dev_info, core::ptr::null_mut()); synchronize_rcu(); cancel_delayed_work_sync(&mut (*prdev).work);
    }
    mutex_unlock(&mut page_reporting_mutex);
}

unsafe fn page_reporting_cycle(prdev: *mut page_reporting_dev_info, zone: *mut zone, order: u32, mt: u32, sgl: *mut scatterlist, offset: &mut u32) -> c_int {
    let area = (*zone).free_area.add(order as usize);
    let list = (*area).free_list.add(mt as usize);
    let page_len = PAGE_SIZE << order;
    if list_empty(list) { return 0; }
    spin_lock_irq(&mut (*zone).lock);
    let mut budget = (((*area).nr_free as u64) + ((*prdev).capacity as u64 * 16) - 1) / ((*prdev).capacity as u64 * 16);
    let mut page: *mut page = core::ptr::null_mut();
    let mut next: *mut page = core::ptr::null_mut();
    while !page.is_null() {
        if budget == 0 { atomic_set(&mut (*prdev).state, PAGE_REPORTING_REQUESTED); next = page; break; }
        if *offset != 0 {
            if !__isolate_free_page(page, order) { next = page; break; }
            *offset -= 1; sg_set_page(sgl.add(*offset as usize), page, page_len, 0); continue;
        }
        if !list_is_first(&mut (*page).lru, list) { list_rotate_to_front(&mut (*page).lru, list); }
        spin_unlock_irq(&mut (*zone).lock);
        let err = match (*prdev).report { Some(f) => f(prdev, sgl, (*prdev).capacity), None => 0 };
        *offset = (*prdev).capacity; budget = budget.wrapping_sub(1);
        spin_lock_irq(&mut (*zone).lock);
        page_reporting_drain(prdev, sgl, (*prdev).capacity, err == 0);
        next = page;
        if err != 0 { break; }
    }
    if !next.is_null() && !list_entry_is_head(next, list, &mut (*next).lru) && !list_is_first(&mut (*next).lru, list) { list_rotate_to_front(&mut (*next).lru, list); }
    spin_unlock_irq(&mut (*zone).lock); 0
}

unsafe fn page_reporting_process_zone(prdev: *mut page_reporting_dev_info, sgl: *mut scatterlist, zone: *mut zone) -> c_int {
    let watermark = low_wmark_pages(zone) + ((*prdev).capacity as u64 * page_reporting_order as u64);
    if !zone_watermark_ok(zone, 0, watermark, 0, ALLOC_CMA) { return 0; }
    let mut offset = (*prdev).capacity;
    let mut order = page_reporting_order;
    while order < NR_PAGE_ORDERS { let mut mt = 0; while mt < MIGRATE_TYPES { if !is_migrate_isolate(mt) { let e = page_reporting_cycle(prdev, zone, order, mt, sgl, &mut offset); if e != 0 { return e; } } mt += 1; } order += 1; }
    let leftover = (*prdev).capacity - offset;
    if leftover != 0 { let tail = sgl.add(offset as usize); let err = match (*prdev).report { Some(f) => f(prdev, tail, leftover), None => 0 }; spin_lock_irq(&mut (*zone).lock); page_reporting_drain(prdev, tail, leftover, err == 0); spin_unlock_irq(&mut (*zone).lock); return err; }
    0
}

unsafe extern "C" fn page_reporting_process(work: *mut work_struct) {
    let d_work = to_delayed_work(work); let prdev = container_of(d_work); let state = PAGE_REPORTING_ACTIVE;
    atomic_set(&mut (*prdev).state, state);
    let sgl = kmalloc_objs(core::mem::size_of::<scatterlist>(), (*prdev).capacity);
    if !sgl.is_null() { sg_init_table(sgl, (*prdev).capacity); /* for_each_zone(zone) */ kfree(sgl); }
    let old = atomic_cmpxchg(&mut (*prdev).state, state, PAGE_REPORTING_IDLE);
    if old == PAGE_REPORTING_REQUESTED { page_reporting_schedule_work(prdev); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
