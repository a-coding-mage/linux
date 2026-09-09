// SPDX-License-Identifier: GPL-2.0-only
/*
 * MCE event pool management in MCE context
 *
 * Copyright (C) 2015 Intel Corp.
 * Author: Chen, Gong <gong.chen@linux.intel.com>
 */

/* Dependencies are supplied by the surrounding kernel translation. */

/*
 * printk() is not safe in MCE context. This is a lock-less memory allocator
 * used to save error information organized in a lock-less list.
 *
 * This memory pool is only to be used to save MCE records in MCE context.
 * MCE events are rare, so a fixed size memory pool should be enough.
 * Allocate on a sliding scale based on number of CPUs.
 */
const MCE_MIN_ENTRIES: i32 = 80;
const MCE_PER_CPU: i32 = 2;

static mut mce_evt_pool: *mut gen_pool = core::ptr::null_mut();
static mut mce_event_llist: llist_head = LLIST_HEAD_INIT;

/*
 * Compare the record "t" with each of the records on list "l" to see if
 * an equivalent one is present in the list.
 */
unsafe fn is_duplicate_mce_record(
    t: *mut mce_evt_llist,
    l: *mut mce_evt_llist,
) -> bool {
    let err1: *mut mce_hw_err = &mut (*t).err;
    let mut node: *mut mce_evt_llist;

    llist_for_each_entry!(node, &mut (*l).llnode, llnode);
    while !node.is_null() {
        let err2: *mut mce_hw_err = &mut (*node).err;

        if !mce_cmp(&(*err1).m, &(*err2).m) {
            return true;
        }
        node = llist_for_each_entry_next!(node, llnode);
    }
    false
}

/*
 * The system has panicked - we'd like to peruse the list of MCE records
 * that have been queued, but not seen by anyone yet.  The list is in
 * reverse time order, so we need to reverse it. While doing that we can
 * also drop duplicate records (these were logged because some banks are
 * shared between cores or by all threads on a socket).
 */
unsafe fn mce_gen_pool_prepare_records() -> *mut llist_node {
    let head: *mut llist_node;
    let mut new_head: llist_head = LLIST_HEAD_INIT;
    let mut node: *mut mce_evt_llist;
    let mut t: *mut mce_evt_llist;

    head = llist_del_all(&mut mce_event_llist);
    if head.is_null() {
        return core::ptr::null_mut();
    }

    /* squeeze out duplicates while reversing order */
    llist_for_each_entry_safe!(node, t, head, llnode);
    while !node.is_null() {
        if !is_duplicate_mce_record(node, t) {
            llist_add(&mut (*node).llnode, &mut new_head);
        }
        node = llist_for_each_entry_safe_next!(node, t, llnode);
    }

    new_head.first
}

unsafe fn mce_gen_pool_process(__unused: *mut work_struct) {
    let mut node: *mut mce_evt_llist;
    let mut tmp: *mut mce_evt_llist;
    let mut head: *mut llist_node;
    let mut mce: *mut mce;

    head = llist_del_all(&mut mce_event_llist);
    if head.is_null() {
        return;
    }

    head = llist_reverse_order(head);
    llist_for_each_entry_safe!(node, tmp, head, llnode);
    while !node.is_null() {
        mce = &mut (*node).err.m;
        blocking_notifier_call_chain(&mut x86_mce_decoder_chain, 0, mce);
        gen_pool_free(mce_evt_pool, node as usize, core::mem::size_of::<mce_evt_llist>());
        node = llist_for_each_entry_safe_next!(node, tmp, llnode);
    }
}

unsafe fn mce_gen_pool_empty() -> bool {
    llist_empty(&mce_event_llist)
}

unsafe fn mce_gen_pool_add(err: *mut mce_hw_err) -> bool {
    let mut node: *mut mce_evt_llist;

    if filter_mce(&mut (*err).m) {
        return false;
    }

    if mce_evt_pool.is_null() {
        return false;
    }

    node = gen_pool_alloc(mce_evt_pool, core::mem::size_of::<mce_evt_llist>()) as *mut mce_evt_llist;
    if node.is_null() {
        pr_warn_ratelimited!("MCE records pool full!\n");
        return false;
    }

    core::ptr::copy_nonoverlapping(err, &mut (*node).err, 1);
    llist_add(&mut (*node).llnode, &mut mce_event_llist);

    true
}

unsafe fn mce_gen_pool_create() -> bool {
    let mut mce_numrecords: i32;
    let mut mce_poolsz: i32;
    let order: i32;
    let gpool: *mut gen_pool;
    let mce_pool: *mut core::ffi::c_void;

    order = order_base_2(core::mem::size_of::<mce_evt_llist>());
    gpool = gen_pool_create(order, -1);
    if gpool.is_null() {
        return false;
    }

    mce_numrecords = max(MCE_MIN_ENTRIES, num_possible_cpus() * MCE_PER_CPU);
    mce_poolsz = mce_numrecords * (1_i32 << order);
    mce_pool = kmalloc(mce_poolsz as usize, GFP_KERNEL);
    if mce_pool.is_null() {
        gen_pool_destroy(gpool);
        return false;
    }

    if gen_pool_add(gpool, mce_pool as usize, mce_poolsz as usize, -1) != 0 {
        gen_pool_destroy(gpool);
        kfree(mce_pool);
        return false;
    }

    mce_evt_pool = gpool;

    true
}

unsafe fn mce_gen_pool_init() -> bool {
    /* Just init mce_gen_pool once. */
    if !mce_evt_pool.is_null() {
        return true;
    }

    mce_gen_pool_create()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
