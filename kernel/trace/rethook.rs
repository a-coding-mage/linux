// SPDX-License-Identifier: GPL-2.0
// C dependencies are supplied by the surrounding kernel translation.

/* Return hook list (shadow stack by list) */

pub unsafe fn rethook_flush_task(tk: *mut task_struct) {
    let mut rhn: *mut rethook_node;
    let mut node: *mut llist_node = __llist_del_all(&mut (*tk).rethooks);
    while !node.is_null() {
        rhn = container_of(node, rethook_node, llist);
        node = (*node).next;
        preempt_disable();
        rethook_recycle(rhn);
        preempt_enable();
    }
}

unsafe fn rethook_free_rcu(head: *mut rcu_head) {
    let rh: *mut rethook = container_of(head, rethook, rcu);
    objpool_fini(&mut (*rh).pool);
}

pub unsafe fn rethook_stop(rh: *mut rethook) {
    rcu_assign_pointer(&mut (*rh).handler, core::ptr::null_mut());
}

pub unsafe fn rethook_free(rh: *mut rethook) {
    rethook_stop(rh);
    call_rcu(&mut (*rh).rcu, rethook_free_rcu);
}

unsafe fn rethook_init_node(nod: *mut core::ffi::c_void, context: *mut core::ffi::c_void) -> i32 {
    let node = nod as *mut rethook_node;
    (*node).rethook = context as *mut rethook;
    0
}

unsafe fn rethook_fini_pool(_head: *mut objpool_head, context: *mut core::ffi::c_void) -> i32 {
    kfree(context);
    0
}

#[inline]
unsafe fn rethook_get_handler(rh: *mut rethook) -> rethook_handler_t {
    rcu_dereference_check((*rh).handler, rcu_read_lock_any_held())
}

pub unsafe fn rethook_alloc(data: *mut core::ffi::c_void, handler: rethook_handler_t,
                             size: i32, num: i32) -> *mut rethook {
    if handler.is_none() || num <= 0 || size < core::mem::size_of::<rethook_node>() as i32 {
        return ERR_PTR(-EINVAL);
    }
    let rh = kzalloc_obj::<rethook>();
    if rh.is_null() { return ERR_PTR(-ENOMEM); }
    (*rh).data = data;
    rcu_assign_pointer(&mut (*rh).handler, handler);
    if objpool_init(&mut (*rh).pool, num, size, GFP_KERNEL, rh as *mut _,
                    rethook_init_node, rethook_fini_pool) != 0 {
        kfree(rh as *mut _);
        return ERR_PTR(-ENOMEM);
    }
    rh
}

unsafe fn free_rethook_node_rcu(head: *mut rcu_head) {
    let node: *mut rethook_node = container_of(head, rethook_node, rcu);
    let rh = (*node).rethook;
    objpool_drop(node as *mut _, &mut (*rh).pool);
}

pub unsafe fn rethook_recycle(node: *mut rethook_node) {
    let handler = rethook_get_handler((*node).rethook);
    if likely(handler.is_some()) {
        objpool_push(node as *mut _, &mut (*(*node).rethook).pool);
    } else {
        call_rcu(&mut (*node).rcu, free_rethook_node_rcu);
    }
}

pub unsafe fn rethook_try_get(rh: *mut rethook) -> *mut rethook_node {
    let handler = rethook_get_handler(rh);
    if unlikely(handler.is_none()) { return core::ptr::null_mut(); }
    // CONFIG_FTRACE_VALIDATE_RCU_IS_WATCHING or CONFIG_KPROBE_EVENTS_ON_NOTRACE:
    // the caller must run in an RCU-available context.
    objpool_pop(&mut (*rh).pool) as *mut rethook_node
}

pub unsafe fn rethook_hook(node: *mut rethook_node, regs: *mut pt_regs, mcount: bool) {
    arch_rethook_prepare(node, regs, mcount);
    __llist_add(&mut (*node).llist, &mut (*current).rethooks);
}

unsafe fn __rethook_find_ret_addr(tsk: *mut task_struct, cur: *mut *mut llist_node) -> usize {
    let mut node = if (*cur).is_null() { (*tsk).rethooks.first } else { (**cur).next };
    while !node.is_null() {
        let rh = container_of(node, rethook_node, llist);
        if (*rh).ret_addr != arch_rethook_trampoline as usize {
            *cur = node;
            return (*rh).ret_addr;
        }
        node = (*node).next;
    }
    0
}

pub unsafe fn rethook_find_ret_addr(tsk: *mut task_struct, frame: usize,
                                    cur: *mut *mut llist_node) -> usize {
    if cur.is_null() { return 0; }
    if tsk != current && task_is_running(tsk) { return 0; }
    let mut ret;
    loop {
        ret = __rethook_find_ret_addr(tsk, cur);
        if ret == 0 { break; }
        let rhn = container_of(*cur, rethook_node, llist);
        if (*rhn).frame == frame { break; }
    }
    ret
}

pub unsafe fn arch_rethook_fixup_return(_regs: *mut pt_regs, _correct_ret_addr: usize) {}

pub unsafe fn rethook_trampoline_handler(regs: *mut pt_regs, frame: usize) -> usize {
    let mut node: *mut llist_node = core::ptr::null_mut();
    let correct_ret_addr = __rethook_find_ret_addr(current, &mut node);
    if correct_ret_addr == 0 { BUG_ON(1); }
    instruction_pointer_set(regs, correct_ret_addr);
    preempt_disable_notrace();
    let mut first = (*current).rethooks.first;
    while !first.is_null() {
        let rhn = container_of(first, rethook_node, llist);
        if (*rhn).frame != frame { break; }
        let handler = rethook_get_handler((*rhn).rethook);
        if let Some(handler) = handler {
            handler(rhn, (*(*rhn).rethook).data, correct_ret_addr, regs);
        }
        if first == node { break; }
        first = (*first).next;
    }
    arch_rethook_fixup_return(regs, correct_ret_addr);
    first = (*current).rethooks.first;
    (*current).rethooks.first = (*node).next;
    (*node).next = core::ptr::null_mut();
    while !first.is_null() {
        let rhn = container_of(first, rethook_node, llist);
        first = (*first).next;
        rethook_recycle(rhn);
    }
    preempt_enable_notrace();
    correct_ret_addr
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
