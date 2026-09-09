/*
 * lib/test_parman.c - Test module for parman
 * Copyright (c) 2017 Mellanox Technologies. All rights reserved.
 * Copyright (c) 2017 Jiri Pirko <jiri@mellanox.com>
 *
 * Translated from C to Rust.
 */

// Kernel headers and build-time module macros are supplied by the surrounding
// kernel/Rust environment.

const TEST_PARMAN_PRIO_SHIFT: usize = 7;
const TEST_PARMAN_PRIO_COUNT: usize = 1usize << TEST_PARMAN_PRIO_SHIFT;
const TEST_PARMAN_PRIO_MASK: usize = TEST_PARMAN_PRIO_COUNT - 1;
const TEST_PARMAN_ITEM_SHIFT: usize = 13;
const TEST_PARMAN_ITEM_COUNT: usize = 1usize << TEST_PARMAN_ITEM_SHIFT;
const TEST_PARMAN_ITEM_MASK: usize = TEST_PARMAN_ITEM_COUNT - 1;
const TEST_PARMAN_BASE_SHIFT: usize = 8;
const TEST_PARMAN_BASE_COUNT: usize = 1usize << TEST_PARMAN_BASE_SHIFT;
const TEST_PARMAN_RESIZE_STEP_SHIFT: usize = 7;
const TEST_PARMAN_RESIZE_STEP_COUNT: usize = 1usize << TEST_PARMAN_RESIZE_STEP_SHIFT;
const TEST_PARMAN_BULK_MAX_SHIFT: usize = 2 + TEST_PARMAN_RESIZE_STEP_SHIFT;
const TEST_PARMAN_BULK_MAX_COUNT: usize = 1usize << TEST_PARMAN_BULK_MAX_SHIFT;
const TEST_PARMAN_BULK_MAX_MASK: usize = TEST_PARMAN_BULK_MAX_COUNT - 1;
const TEST_PARMAN_RUN_BUDGET: usize = TEST_PARMAN_ITEM_COUNT * 256;

#[repr(C)]
pub struct parman_prio { _private: [u8; 0] }
#[repr(C)]
pub struct parman_item { pub index: usize }
#[repr(C)]
pub struct parman { _private: [u8; 0] }
#[repr(C)]
pub struct parman_ops {
    pub base_count: usize,
    pub resize_step: usize,
    pub resize: Option<unsafe extern "C" fn(*mut core::ffi::c_void, usize) -> i32>,
    pub r#move: Option<unsafe extern "C" fn(*mut core::ffi::c_void, usize, usize, usize)>,
    pub algo: i32,
}
#[repr(C)]
pub struct rnd_state { _private: [u8; 0] }

extern "C" {
    fn krealloc(ptr: *mut core::ffi::c_void, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn memset(dst: *mut core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void;
    fn memmove(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    fn prandom_seed_state(state: *mut rnd_state, seed: u64);
    fn prandom_u32_state(state: *mut rnd_state) -> u32;
    fn parman_create(ops: *const parman_ops, priv_: *mut core::ffi::c_void) -> *mut parman;
    fn parman_destroy(parman: *mut parman);
    fn parman_prio_init(parman: *mut parman, prio: *mut parman_prio, priority: usize);
    fn parman_prio_fini(prio: *mut parman_prio);
    fn parman_item_add(parman: *mut parman, prio: *mut parman_prio, item: *mut parman_item) -> i32;
    fn parman_item_remove(parman: *mut parman, prio: *mut parman_prio, item: *mut parman_item);
}

#[repr(C)]
struct test_parman_prio { parman_prio: parman_prio, priority: usize }
#[repr(C)]
struct test_parman_item { parman_item: parman_item, prio: *mut test_parman_prio, used: bool }
#[repr(C)]
struct test_parman {
    parman: *mut parman,
    prio_array: *mut *mut test_parman_item,
    prio_array_limit: usize,
    prios: [test_parman_prio; TEST_PARMAN_PRIO_COUNT],
    items: [test_parman_item; TEST_PARMAN_ITEM_COUNT],
    rnd: rnd_state,
    run_budget: usize,
    bulk_budget: usize,
    bulk_noop: bool,
    used_items: u32,
}

const fn item_ptrs_size(count: usize) -> usize { core::mem::size_of::<*mut test_parman_item>() * count }

unsafe extern "C" fn test_parman_resize(priv_: *mut core::ffi::c_void, new_count: usize) -> i32 {
    let tp = priv_ as *mut test_parman;
    let prio_array = krealloc((*tp).prio_array as *mut core::ffi::c_void, item_ptrs_size(new_count), 0) as *mut *mut test_parman_item;
    if new_count == 0 { return 0; }
    if prio_array.is_null() { return -12; }
    let old_count = (*tp).prio_array_limit;
    if new_count > old_count { memset(prio_array.add(old_count) as *mut core::ffi::c_void, 0, item_ptrs_size(new_count - old_count)); }
    (*tp).prio_array = prio_array;
    (*tp).prio_array_limit = new_count;
    0
}

unsafe extern "C" fn test_parman_move(priv_: *mut core::ffi::c_void, from_index: usize, to_index: usize, count: usize) {
    let tp = priv_ as *mut test_parman;
    let a = (*tp).prio_array;
    memmove(a.add(to_index) as *mut core::ffi::c_void, a.add(from_index) as *const core::ffi::c_void, item_ptrs_size(count));
    memset(a.add(from_index) as *mut core::ffi::c_void, 0, item_ptrs_size(count));
}

static test_parman_lsort_ops: parman_ops = parman_ops { base_count: TEST_PARMAN_BASE_COUNT, resize_step: TEST_PARMAN_RESIZE_STEP_COUNT, resize: Some(test_parman_resize), r#move: Some(test_parman_move), algo: 0 };

unsafe fn test_parman_rnd_init(tp: *mut test_parman) { prandom_seed_state(&mut (*tp).rnd, 3141592653589793238u64); }
unsafe fn test_parman_rnd_get(tp: *mut test_parman) -> u32 { prandom_u32_state(&mut (*tp).rnd) }

unsafe fn test_parman_priority_gen(tp: *mut test_parman) -> usize {
    loop {
        let priority = test_parman_rnd_get(tp) as usize;
        if priority == 0 { continue; }
        let mut ok = true;
        for i in 0..TEST_PARMAN_PRIO_COUNT { if (*tp).prios[i].priority == priority { ok = false; break; } }
        if ok { return priority; }
    }
}

unsafe fn test_parman_prios_init(tp: *mut test_parman) { for i in 0..TEST_PARMAN_PRIO_COUNT { (*tp).prios[i].priority = test_parman_priority_gen(tp); parman_prio_init((*tp).parman, &mut (*tp).prios[i].parman_prio, (*tp).prios[i].priority); } }
unsafe fn test_parman_prios_fini(tp: *mut test_parman) { for i in 0..TEST_PARMAN_PRIO_COUNT { parman_prio_fini(&mut (*tp).prios[i].parman_prio); } }
unsafe fn test_parman_items_init(tp: *mut test_parman) { for i in 0..TEST_PARMAN_ITEM_COUNT { let n = (test_parman_rnd_get(tp) as usize) & TEST_PARMAN_PRIO_MASK; (*tp).items[i].prio = &mut (*tp).prios[n]; } }
unsafe fn test_parman_items_fini(tp: *mut test_parman) { for i in 0..TEST_PARMAN_ITEM_COUNT { if (*tp).items[i].used { let it = &mut (*tp).items[i]; parman_item_remove((*tp).parman, &mut (*it.prio).parman_prio, &mut it.parman_item); } } }

unsafe fn test_parman_create(ops: *const parman_ops) -> *mut test_parman {
    let tp = Box::into_raw(Box::new(core::mem::zeroed::<test_parman>()));
    if test_parman_resize(tp as *mut _, TEST_PARMAN_BASE_COUNT) != 0 { kfree(tp as *mut _); return core::ptr::null_mut(); }
    (*tp).parman = parman_create(ops, tp as *mut _);
    if (*tp).parman.is_null() { test_parman_resize(tp as *mut _, 0); kfree(tp as *mut _); return core::ptr::null_mut(); }
    test_parman_rnd_init(tp); test_parman_prios_init(tp); test_parman_items_init(tp);
    (*tp).run_budget = TEST_PARMAN_RUN_BUDGET;
    tp
}

unsafe fn test_parman_destroy(tp: *mut test_parman) {
    test_parman_items_fini(tp); test_parman_prios_fini(tp); parman_destroy((*tp).parman); test_parman_resize(tp as *mut _, 0); kfree(tp as *mut _);
}

unsafe fn test_parman_run_check_budgets(tp: *mut test_parman) -> bool {
    if (*tp).run_budget == 0 { return false; }
    (*tp).run_budget = (*tp).run_budget.wrapping_sub(1);
    if (*tp).bulk_budget != 0 { (*tp).bulk_budget = (*tp).bulk_budget.wrapping_sub(1); return true; }
    (*tp).bulk_budget = (test_parman_rnd_get(tp) as usize) & TEST_PARMAN_BULK_MAX_MASK;
    (*tp).bulk_noop = (test_parman_rnd_get(tp) & 1) != 0; true
}

unsafe fn test_parman_run(tp: *mut test_parman) -> i32 {
    let mut i = test_parman_rnd_get(tp) as usize;
    while test_parman_run_check_budgets(tp) {
        let item = &mut (*tp).items[i & TEST_PARMAN_ITEM_MASK]; i = i.wrapping_add(1);
        if (*tp).bulk_noop { continue; }
        if !item.used {
            let err = parman_item_add((*tp).parman, &mut (*item.prio).parman_prio, &mut item.parman_item);
            if err != 0 { return err; }
            *(*tp).prio_array.add(item.parman_item.index) = item; (*tp).used_items += 1;
        } else {
            *(*tp).prio_array.add(item.parman_item.index) = core::ptr::null_mut();
            parman_item_remove((*tp).parman, &mut (*item.prio).parman_prio, &mut item.parman_item); (*tp).used_items -= 1;
        }
        item.used = !item.used;
    } 0
}

unsafe fn test_parman_check_array(tp: *mut test_parman, gaps_allowed: bool) -> i32 {
    if (*tp).prio_array_limit < TEST_PARMAN_BASE_COUNT { return -22; }
    let mut used = 0u32; let mut last_unused = 0usize; let mut last_priority = 0usize;
    for i in 0..(*tp).prio_array_limit {
        let item = *(*tp).prio_array.add(i); if item.is_null() { last_unused += 1; continue; }
        if last_unused != 0 && !gaps_allowed { return -22; } last_unused = 0; used += 1;
        if (*(*item).prio).priority < last_priority { return -22; } last_priority = (*(*item).prio).priority;
        if (*item).parman_item.index != i { return -22; }
    }
    if used != (*tp).used_items || last_unused >= TEST_PARMAN_RESIZE_STEP_COUNT { return -22; } 0
}

unsafe fn test_parman_lsort() -> i32 {
    let tp = test_parman_create(&test_parman_lsort_ops); if tp.is_null() { return -12; }
    let mut err = test_parman_run(tp); if err == 0 { err = test_parman_check_array(tp, false); } test_parman_destroy(tp); err
}

unsafe fn test_parman_init() -> i32 { test_parman_lsort() }
unsafe fn test_parman_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
