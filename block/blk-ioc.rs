// SPDX-License-Identifier: GPL-2.0
/* Functions related to io context handling */

// Dependencies supplied by the surrounding kernel translation unit.

static mut iocontext_cachep: *mut kmem_cache = core::ptr::null_mut();

#[cfg(CONFIG_BLK_ICQ)]
unsafe fn get_io_context(ioc: *mut io_context) {
    BUG_ON(atomic_long_read(&(*ioc).refcount) <= 0);
    atomic_long_inc(&(*ioc).refcount);
}

#[cfg(CONFIG_BLK_ICQ)]
unsafe fn ioc_exit_icq(icq: *mut io_cq) {
    let et = (*(*icq).q).elevator.type_;
    if (*icq).flags & ICQ_EXITED != 0 { return; }
    if ((*et).ops.exit_icq).is_some() { ((*et).ops.exit_icq.unwrap())(icq); }
    (*icq).flags |= ICQ_EXITED;
}

#[cfg(CONFIG_BLK_ICQ)]
unsafe fn ioc_exit_icqs(ioc: *mut io_context) {
    let mut icq: *mut io_cq;
    spin_lock_irq(&mut (*ioc).lock);
    hlist_for_each_entry!(icq, (*ioc).icq_list, ioc_node, { ioc_exit_icq(icq); });
    spin_unlock_irq(&mut (*ioc).lock);
}

#[cfg(CONFIG_BLK_ICQ)]
unsafe fn ioc_destroy_icq(icq: *mut io_cq) {
    let ioc = (*icq).ioc;
    let q = (*icq).q;
    let et = (*(*q).elevator).type_;
    lockdep_assert_held(&(*ioc).lock);
    lockdep_assert_held(&(*q).queue_lock);
    if (*icq).flags & ICQ_DESTROYED != 0 { return; }
    radix_tree_delete(&mut (*ioc).icq_tree, (*icq).q.id);
    hlist_del_init(&mut (*icq).ioc_node);
    list_del_init(&mut (*icq).q_node);
    if rcu_access_pointer((*ioc).icq_hint) == icq {
        rcu_assign_pointer((*ioc).icq_hint, core::ptr::null_mut());
    }
    ioc_exit_icq(icq);
    (*icq).__rcu_icq_cache = (*et).icq_cache;
    (*icq).flags |= ICQ_DESTROYED;
    kfree_rcu!(icq, __rcu_head);
}

#[cfg(CONFIG_BLK_ICQ)]
unsafe fn ioc_release_fn(work: *mut work_struct) {
    let ioc = container_of!(work, io_context, release_work);
    spin_lock_irq(&mut (*ioc).lock);
    while !hlist_empty(&(*ioc).icq_list) {
        let icq = hlist_entry!((*ioc).icq_list.first, io_cq, ioc_node);
        let q = (*icq).q;
        if spin_trylock(&mut (*q).queue_lock) {
            ioc_destroy_icq(icq);
            spin_unlock(&mut (*q).queue_lock);
        } else {
            rcu_read_lock();
            spin_unlock(&mut (*ioc).lock);
            spin_lock(&mut (*q).queue_lock);
            spin_lock(&mut (*ioc).lock);
            ioc_destroy_icq(icq);
            spin_unlock(&mut (*q).queue_lock);
            rcu_read_unlock();
        }
    }
    spin_unlock_irq(&mut (*ioc).lock);
    kmem_cache_free(iocontext_cachep, ioc);
}

#[cfg(CONFIG_BLK_ICQ)]
unsafe fn ioc_delay_free(ioc: *mut io_context) -> bool {
    let mut flags: ulong = 0;
    spin_lock_irqsave(&mut (*ioc).lock, &mut flags);
    if !hlist_empty(&(*ioc).icq_list) {
        queue_work(system_power_efficient_wq, &mut (*ioc).release_work);
        spin_unlock_irqrestore(&mut (*ioc).lock, flags);
        return true;
    }
    spin_unlock_irqrestore(&mut (*ioc).lock, flags);
    false
}

#[cfg(CONFIG_BLK_ICQ)]
pub unsafe fn ioc_clear_queue(q: *mut request_queue) {
    spin_lock_irq(&mut (*q).queue_lock);
    while !list_empty(&(*q).icq_list) {
        let icq = list_first_entry!((*q).icq_list, io_cq, q_node);
        spin_lock(&mut (*(*icq).ioc).lock);
        ioc_destroy_icq(icq);
        spin_unlock(&mut (*(*icq).ioc).lock);
    }
    spin_unlock_irq(&mut (*q).queue_lock);
}

#[cfg(not(CONFIG_BLK_ICQ))]
unsafe fn ioc_exit_icqs(_ioc: *mut io_context) {}
#[cfg(not(CONFIG_BLK_ICQ))]
unsafe fn ioc_delay_free(_ioc: *mut io_context) -> bool { false }

pub unsafe fn put_io_context(ioc: *mut io_context) {
    BUG_ON(atomic_long_read(&(*ioc).refcount) <= 0);
    if atomic_long_dec_and_test(&mut (*ioc).refcount) && !ioc_delay_free(ioc) {
        kmem_cache_free(iocontext_cachep, ioc);
    }
}

pub unsafe fn exit_io_context(task: *mut task_struct) {
    task_lock(task);
    let ioc = (*task).io_context;
    (*task).io_context = core::ptr::null_mut();
    task_unlock(task);
    if atomic_dec_and_test(&mut (*ioc).active_ref) {
        ioc_exit_icqs(ioc);
        put_io_context(ioc);
    }
}

unsafe fn alloc_io_context(gfp_flags: gfp_t, node: int) -> *mut io_context {
    let ioc = kmem_cache_alloc_node(iocontext_cachep, gfp_flags | __GFP_ZERO, node) as *mut io_context;
    if ioc.is_null() { return core::ptr::null_mut(); }
    atomic_long_set(&mut (*ioc).refcount, 1);
    atomic_set(&mut (*ioc).active_ref, 1);
    #[cfg(CONFIG_BLK_ICQ)] {
        spin_lock_init(&mut (*ioc).lock);
        INIT_RADIX_TREE!(&mut (*ioc).icq_tree, GFP_ATOMIC);
        INIT_HLIST_HEAD!(&mut (*ioc).icq_list);
        INIT_WORK!(&mut (*ioc).release_work, ioc_release_fn);
    }
    (*ioc).ioprio = IOPRIO_DEFAULT;
    ioc
}

pub unsafe fn set_task_ioprio(task: *mut task_struct, ioprio: int) -> int {
    let cred = current_cred();
    rcu_read_lock();
    let tcred = __task_cred(task);
    if !uid_eq((*tcred).uid, (*cred).euid) && !uid_eq((*tcred).uid, (*cred).uid) && !capable(CAP_SYS_NICE) {
        rcu_read_unlock(); return -EPERM;
    }
    rcu_read_unlock();
    let err = security_task_setioprio(task, ioprio);
    if err != 0 { return err; }
    task_lock(task);
    if (*task).io_context.is_null() {
        task_unlock(task);
        let ioc = alloc_io_context(GFP_ATOMIC, NUMA_NO_NODE);
        if ioc.is_null() { return -ENOMEM; }
        task_lock(task);
        if (*task).flags & PF_EXITING != 0 {
            kmem_cache_free(iocontext_cachep, ioc); goto out;
        }
        if !(*task).io_context.is_null() { kmem_cache_free(iocontext_cachep, ioc); }
        else { (*task).io_context = ioc; }
    }
    (*(*task).io_context).ioprio = ioprio;
out:
    task_unlock(task); 0
}

pub unsafe fn __copy_io(clone_flags: u64, tsk: *mut task_struct) -> int {
    let ioc = (*current).io_context;
    if clone_flags & CLONE_IO != 0 {
        atomic_inc(&mut (*ioc).active_ref); (*tsk).io_context = ioc;
    } else if ioprio_valid((*ioc).ioprio) {
        (*tsk).io_context = alloc_io_context(GFP_KERNEL, NUMA_NO_NODE);
        if (*tsk).io_context.is_null() { return -ENOMEM; }
        (*(*tsk).io_context).ioprio = (*ioc).ioprio;
    }
    0
}

#[cfg(CONFIG_BLK_ICQ)]
pub unsafe fn ioc_lookup_icq(q: *mut request_queue) -> *mut io_cq {
    let ioc = (*current).io_context;
    rcu_read_lock();
    let mut icq = rcu_dereference((*ioc).icq_hint);
    if !(icq.is_null() || (*icq).q != q) { rcu_read_unlock(); return icq; }
    icq = radix_tree_lookup(&(*ioc).icq_tree, (*q).id);
    if !icq.is_null() && (*icq).q == q { rcu_assign_pointer((*ioc).icq_hint, icq); }
    else { icq = core::ptr::null_mut(); }
    rcu_read_unlock(); icq
}

#[cfg(CONFIG_BLK_ICQ)]
unsafe fn ioc_create_icq(q: *mut request_queue) -> *mut io_cq {
    let ioc = (*current).io_context;
    let et = (*(*q).elevator).type_;
    let mut icq = kmem_cache_alloc_node((*et).icq_cache, GFP_ATOMIC | __GFP_ZERO, (*q).node) as *mut io_cq;
    if icq.is_null() || radix_tree_maybe_preload(GFP_ATOMIC) < 0 { if !icq.is_null() { kmem_cache_free((*et).icq_cache, icq); } return core::ptr::null_mut(); }
    (*icq).ioc = ioc; (*icq).q = q; INIT_LIST_HEAD!(&mut (*icq).q_node); INIT_HLIST_NODE!(&mut (*icq).ioc_node);
    spin_lock_irq(&mut (*q).queue_lock); spin_lock(&mut (*ioc).lock);
    if radix_tree_insert(&mut (*ioc).icq_tree, (*q).id, icq) == 0 {
        hlist_add_head!(&mut (*icq).ioc_node, &mut (*ioc).icq_list); list_add!(&mut (*icq).q_node, &mut (*q).icq_list);
        if ((*et).ops.init_icq).is_some() { ((*et).ops.init_icq.unwrap())(icq); }
    } else { kmem_cache_free((*et).icq_cache, icq); icq = ioc_lookup_icq(q); if icq.is_null() { printk!(KERN_ERR, "cfq: icq link failed!\n"); } }
    spin_unlock(&mut (*ioc).lock); spin_unlock_irq(&mut (*q).queue_lock); radix_tree_preload_end(); icq
}

#[cfg(CONFIG_BLK_ICQ)]
pub unsafe fn ioc_find_get_icq(q: *mut request_queue) -> *mut io_cq {
    let mut ioc = (*current).io_context; let mut icq = core::ptr::null_mut();
    if ioc.is_null() { ioc = alloc_io_context(GFP_ATOMIC, (*q).node); if ioc.is_null() { return core::ptr::null_mut(); } task_lock(current); if !(*current).io_context.is_null() { kmem_cache_free(iocontext_cachep, ioc); ioc = (*current).io_context; } else { (*current).io_context = ioc; } get_io_context(ioc); task_unlock(current); }
    else { get_io_context(ioc); icq = ioc_lookup_icq(q); }
    if icq.is_null() { icq = ioc_create_icq(q); if icq.is_null() { put_io_context(ioc); return core::ptr::null_mut(); } } icq
}

unsafe fn blk_ioc_init() -> int {
    iocontext_cachep = kmem_cache_create!("blkdev_ioc", core::mem::size_of::<io_context>(), 0, SLAB_PANIC, None);
    0
}
subsys_initcall!(blk_ioc_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
