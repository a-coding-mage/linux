/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2016 Qualcomm Atheros, Inc
 *
 * Based on net/sched/sch_fq_codel.c
 */

// Translated from the C header.  Types, constants, helpers, and callbacks
// supplied by net/fq.h remain external dependencies.

unsafe fn __fq_adjust_removal(
    fq: *mut fq,
    flow: *mut fq_flow,
    packets: c_uint,
    bytes: c_uint,
    truesize: c_uint,
) {
    let tin = (*flow).tin;
    (*tin).backlog_bytes -= bytes;
    (*tin).backlog_packets -= packets;
    (*flow).backlog -= bytes;
    (*fq).backlog -= packets;
    (*fq).memory_usage -= truesize;
    if (*flow).backlog != 0 { return; }
    if flow == &mut (*tin).default_flow {
        list_del_init(&mut (*tin).tin_list);
        return;
    }
    let idx = flow.offset_from((*fq).flows);
    __clear_bit(idx, (*fq).flows_bitmap);
}

unsafe fn fq_adjust_removal(fq: *mut fq, flow: *mut fq_flow, skb: *mut sk_buff) {
    __fq_adjust_removal(fq, flow, 1, (*skb).len, (*skb).truesize);
}

unsafe fn fq_flow_dequeue(fq: *mut fq, flow: *mut fq_flow) -> *mut sk_buff {
    lockdep_assert_held(&(*fq).lock);
    let skb = __skb_dequeue(&mut (*flow).queue);
    if skb.is_null() { return core::ptr::null_mut(); }
    fq_adjust_removal(fq, flow, skb);
    skb
}

unsafe fn fq_flow_drop(fq: *mut fq, flow: *mut fq_flow, free_func: fq_skb_free_t) -> c_int {
    let mut packets: c_uint = 0;
    let mut bytes: c_uint = 0;
    let mut truesize: c_uint = 0;
    let tin = (*flow).tin;
    lockdep_assert_held(&(*fq).lock);
    let pending = min_t(32i32, (skb_queue_len(&(*flow).queue) / 2) as c_int);
    loop {
        let skb = __skb_dequeue(&mut (*flow).queue);
        if skb.is_null() || packets >= pending as c_uint { break; }
        packets += 1;
        bytes += (*skb).len;
        truesize += (*skb).truesize;
        free_func(fq, tin, flow, skb);
    }
    __fq_adjust_removal(fq, flow, packets, bytes, truesize);
    packets as c_int
}

unsafe fn fq_tin_dequeue(fq: *mut fq, tin: *mut fq_tin, dequeue_func: fq_tin_dequeue_t) -> *mut sk_buff {
    lockdep_assert_held(&(*fq).lock);
    loop {
        let mut head = &mut (*tin).new_flows;
        if list_empty(head) {
            head = &mut (*tin).old_flows;
            if list_empty(head) { return core::ptr::null_mut(); }
        }
        let flow = list_first_entry(head, fq_flow, flowchain);
        if (*flow).deficit <= 0 {
            (*flow).deficit += (*fq).quantum;
            list_move_tail(&mut (*flow).flowchain, &mut (*tin).old_flows);
            continue;
        }
        let skb = dequeue_func(fq, tin, flow);
        if skb.is_null() {
            if core::ptr::eq(head, &(*tin).new_flows) && !list_empty(&(*tin).old_flows) {
                list_move_tail(&mut (*flow).flowchain, &mut (*tin).old_flows);
            } else {
                list_del_init(&mut (*flow).flowchain);
                (*flow).tin = core::ptr::null_mut();
            }
            continue;
        }
        (*flow).deficit -= (*skb).len as _;
        (*tin).tx_bytes += (*skb).len;
        (*tin).tx_packets += 1;
        return skb;
    }
}

unsafe fn fq_flow_idx(fq: *mut fq, skb: *mut sk_buff) -> u32 {
    reciprocal_scale(skb_get_hash(skb), (*fq).flows_cnt)
}

unsafe fn fq_flow_classify(fq: *mut fq, tin: *mut fq_tin, idx: u32, _skb: *mut sk_buff) -> *mut fq_flow {
    lockdep_assert_held(&(*fq).lock);
    let mut flow = &mut *(*fq).flows.add(idx as usize) as *mut fq_flow;
    if !(*flow).tin.is_null() && (*flow).tin != tin {
        flow = &mut (*tin).default_flow;
        (*tin).collisions += 1;
        (*fq).collisions += 1;
    }
    if (*flow).tin.is_null() { (*tin).flows += 1; }
    flow
}

unsafe fn fq_find_fattest_flow(fq: *mut fq) -> *mut fq_flow {
    let mut flow = core::ptr::null_mut();
    let mut len = 0u32;
    for i in for_each_set_bit((*fq).flows_bitmap, (*fq).flows_cnt) {
        let cur = (*fq).flows.add(i as usize);
        if (*cur).backlog > len { flow = cur; len = (*cur).backlog; }
    }
    list_for_each_entry!(tin, (*fq).tin_backlog, tin_list, fq_tin, {
        if tin.default_flow.backlog > len { flow = &mut tin.default_flow; len = tin.default_flow.backlog; }
    });
    flow
}

unsafe fn fq_tin_enqueue(fq: *mut fq, tin: *mut fq_tin, idx: u32, skb: *mut sk_buff, free_func: fq_skb_free_t) {
    lockdep_assert_held(&(*fq).lock);
    let flow = fq_flow_classify(fq, tin, idx, skb);
    if (*flow).backlog == 0 {
        if flow != &mut (*tin).default_flow { __set_bit(idx, (*fq).flows_bitmap); }
        else if list_empty(&(*tin).tin_list) { list_add(&mut (*tin).tin_list, &mut (*fq).tin_backlog); }
    }
    (*flow).tin = tin;
    skb_list_walk_safe!(skb, next, {
        skb_mark_not_on_list(skb);
        (*flow).backlog += (*skb).len;
        (*tin).backlog_bytes += (*skb).len;
        (*tin).backlog_packets += 1;
        (*fq).memory_usage += (*skb).truesize;
        (*fq).backlog += 1;
        __skb_queue_tail(&mut (*flow).queue, skb);
    });
    if list_empty(&(*flow).flowchain) {
        (*flow).deficit = (*fq).quantum;
        list_add_tail(&mut (*flow).flowchain, &mut (*tin).new_flows);
    }
    let mut oom = (*fq).memory_usage > (*fq).memory_limit;
    while (*fq).backlog > (*fq).limit || oom {
        let fattest = fq_find_fattest_flow(fq);
        if fattest.is_null() { return; }
        if fq_flow_drop(fq, fattest, free_func) == 0 { return; }
        (*(*fattest).tin).overlimit += 1;
        (*fq).overlimit += 1;
        if oom { (*fq).overmemory += 1; oom = (*fq).memory_usage > (*fq).memory_limit; }
    }
}

unsafe fn fq_flow_filter(fq: *mut fq, flow: *mut fq_flow, filter_func: fq_skb_filter_t, filter_data: *mut c_void, free_func: fq_skb_free_t) {
    let tin = (*flow).tin;
    lockdep_assert_held(&(*fq).lock);
    skb_queue_walk_safe!(&mut (*flow).queue, skb, tmp, {
        if filter_func(fq, tin, flow, skb, filter_data) {
            __skb_unlink(skb, &mut (*flow).queue);
            fq_adjust_removal(fq, flow, skb);
            free_func(fq, tin, flow, skb);
        }
    });
}

unsafe fn fq_tin_filter(fq: *mut fq, tin: *mut fq_tin, filter_func: fq_skb_filter_t, filter_data: *mut c_void, free_func: fq_skb_free_t) {
    lockdep_assert_held(&(*fq).lock);
    list_for_each_entry!(flow, (*tin).new_flows, flowchain, fq_flow, { fq_flow_filter(fq, flow, filter_func, filter_data, free_func); });
    list_for_each_entry!(flow, (*tin).old_flows, flowchain, fq_flow, { fq_flow_filter(fq, flow, filter_func, filter_data, free_func); });
}

unsafe fn fq_flow_reset(fq: *mut fq, flow: *mut fq_flow, free_func: fq_skb_free_t) {
    let tin = (*flow).tin;
    while let Some(skb) = (!fq_flow_dequeue(fq, flow).is_null()).then(|| fq_flow_dequeue(fq, flow)) { free_func(fq, tin, flow, skb); }
    if !list_empty(&(*flow).flowchain) {
        list_del_init(&mut (*flow).flowchain);
        if list_empty(&(*tin).new_flows) && list_empty(&(*tin).old_flows) { list_del_init(&mut (*tin).tin_list); }
    }
    (*flow).tin = core::ptr::null_mut();
    WARN_ON_ONCE((*flow).backlog != 0);
}

unsafe fn fq_tin_reset(fq: *mut fq, tin: *mut fq_tin, free_func: fq_skb_free_t) {
    loop {
        let head = if !list_empty(&(*tin).new_flows) { &mut (*tin).new_flows } else if !list_empty(&(*tin).old_flows) { &mut (*tin).old_flows } else { break };
        let flow = list_first_entry(head, fq_flow, flowchain);
        fq_flow_reset(fq, flow, free_func);
    }
    WARN_ON_ONCE(!list_empty(&(*tin).tin_list));
    WARN_ON_ONCE((*tin).backlog_bytes != 0);
    WARN_ON_ONCE((*tin).backlog_packets != 0);
}

unsafe fn fq_flow_init(flow: *mut fq_flow) { INIT_LIST_HEAD(&mut (*flow).flowchain); __skb_queue_head_init(&mut (*flow).queue); }
unsafe fn fq_tin_init(tin: *mut fq_tin) { INIT_LIST_HEAD(&mut (*tin).new_flows); INIT_LIST_HEAD(&mut (*tin).old_flows); INIT_LIST_HEAD(&mut (*tin).tin_list); fq_flow_init(&mut (*tin).default_flow); }

unsafe fn fq_init(fq: *mut fq, flows_cnt: c_int) -> c_int {
    memset(fq, 0, core::mem::size_of::<fq>());
    spin_lock_init(&mut (*fq).lock); INIT_LIST_HEAD(&mut (*fq).tin_backlog);
    (*fq).flows_cnt = max_t(flows_cnt as u32, 1); (*fq).quantum = 300; (*fq).limit = 8192; (*fq).memory_limit = 16 << 20;
    (*fq).flows = kvzalloc_objs((*fq).flows, (*fq).flows_cnt);
    if (*fq).flows.is_null() { return -12; }
    (*fq).flows_bitmap = bitmap_zalloc((*fq).flows_cnt, GFP_KERNEL);
    if (*fq).flows_bitmap.is_null() { kvfree((*fq).flows); (*fq).flows = core::ptr::null_mut(); return -12; }
    for i in 0..(*fq).flows_cnt { fq_flow_init((*fq).flows.add(i as usize)); }
    0
}

unsafe fn fq_reset(fq: *mut fq, free_func: fq_skb_free_t) {
    for i in 0..(*fq).flows_cnt { fq_flow_reset(fq, (*fq).flows.add(i as usize), free_func); }
    kvfree((*fq).flows); (*fq).flows = core::ptr::null_mut(); bitmap_free((*fq).flows_bitmap); (*fq).flows_bitmap = core::ptr::null_mut();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
