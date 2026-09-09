// SPDX-License-Identifier: GPL-2.0
/*
 * Block device elevator/IO-scheduler.
 *
 * Direct Rust translation of elevator.c. Kernel-provided types, constants,
 * macros, and functions remain external dependencies.
 */

static mut ELV_LIST_LOCK: spinlock_t = spinlock_t::new();
static mut ELV_LIST: list_head = list_head::new();

#[inline]
unsafe fn rq_hash_key(rq: *mut request) -> sector_t {
    blk_rq_pos(rq) + blk_rq_sectors(rq)
}

unsafe fn elv_iosched_allow_bio_merge(rq: *mut request, bio: *mut bio) -> bool {
    let q = (*rq).q;
    let e = (*q).elevator;
    if let Some(f) = (*(*e).type_).ops.allow_merge {
        return f(q, rq, bio);
    }
    true
}

pub unsafe fn elv_bio_merge_ok(rq: *mut request, bio: *mut bio) -> bool {
    if !blk_rq_merge_ok(rq, bio) { return false; }
    if !elv_iosched_allow_bio_merge(rq, bio) { return false; }
    true
}

unsafe fn elevator_match(e: *const elevator_type, name: *const c_char) -> bool {
    !strcmp((*e).elevator_name, name) ||
        ((*e).elevator_alias != core::ptr::null() && !strcmp((*e).elevator_alias, name))
}

unsafe fn __elevator_find(name: *const c_char) -> *mut elevator_type {
    let mut e: *mut elevator_type;
    list_for_each_entry!(e, &mut ELV_LIST, list, {
        if elevator_match(e, name) { return e; }
    });
    core::ptr::null_mut()
}

unsafe fn elevator_find_get(name: *const c_char) -> *mut elevator_type {
    spin_lock(&mut ELV_LIST_LOCK);
    let mut e = __elevator_find(name);
    if !e.is_null() && !elevator_tryget(e) { e = core::ptr::null_mut(); }
    spin_unlock(&mut ELV_LIST_LOCK);
    e
}

pub unsafe fn elevator_alloc(q: *mut request_queue, e: *mut elevator_type,
                             res: *mut elevator_resources) -> *mut elevator_queue {
    let eq = kzalloc_node(core::mem::size_of::<elevator_queue>(), GFP_KERNEL, (*q).node)
        as *mut elevator_queue;
    if eq.is_null() { return core::ptr::null_mut(); }
    __elevator_get(e);
    (*eq).type_ = e;
    kobject_init(&mut (*eq).kobj, &ELV_KTYPE);
    mutex_init(&mut (*eq).sysfs_lock);
    hash_init!((*eq).hash);
    (*eq).et = (*res).et;
    (*eq).elevator_data = (*res).data;
    eq
}

unsafe fn elevator_release(kobj: *mut kobject) {
    let e = container_of!(kobj, elevator_queue, kobj);
    elevator_put((*e).type_);
    kfree(e as *mut c_void);
}

unsafe fn elevator_exit(q: *mut request_queue) {
    let e = (*q).elevator;
    lockdep_assert_held!(&(*q).elevator_lock);
    ioc_clear_queue(q);
    mutex_lock(&mut (*e).sysfs_lock);
    blk_mq_exit_sched(q, e);
    mutex_unlock(&mut (*e).sysfs_lock);
}

#[inline]
unsafe fn __elv_rqhash_del(rq: *mut request) {
    hash_del!(&mut (*rq).hash);
    (*rq).rq_flags &= !RQF_HASHED;
}

pub unsafe fn elv_rqhash_del(_q: *mut request_queue, rq: *mut request) {
    if ELV_ON_HASH(rq) { __elv_rqhash_del(rq); }
}

pub unsafe fn elv_rqhash_add(q: *mut request_queue, rq: *mut request) {
    let e = (*q).elevator;
    BUG_ON!(ELV_ON_HASH(rq));
    hash_add!((*e).hash, &mut (*rq).hash, rq_hash_key(rq));
    (*rq).rq_flags |= RQF_HASHED;
}

pub unsafe fn elv_rqhash_reposition(q: *mut request_queue, rq: *mut request) {
    __elv_rqhash_del(rq);
    elv_rqhash_add(q, rq);
}

pub unsafe fn elv_rqhash_find(q: *mut request_queue, offset: sector_t) -> *mut request {
    let e = (*q).elevator;
    let mut rq: *mut request;
    let mut next: *mut hlist_node;
    hash_for_each_possible_safe!((*e).hash, rq, next, hash, offset, {
        BUG_ON!(!ELV_ON_HASH(rq));
        if !rq_mergeable(rq) { __elv_rqhash_del(rq); continue; }
        if rq_hash_key(rq) == offset { return rq; }
    });
    core::ptr::null_mut()
}

pub unsafe fn elv_rb_add(root: *mut rb_root, rq: *mut request) {
    let mut p = &mut (*root).rb_node as *mut *mut rb_node;
    let mut parent: *mut rb_node = core::ptr::null_mut();
    while !(*p).is_null() {
        parent = *p;
        let r = rb_entry!(parent, request, rb_node);
        if blk_rq_pos(rq) < blk_rq_pos(r) { p = &mut (**p).rb_left; }
        else { p = &mut (**p).rb_right; }
    }
    rb_link_node(&mut (*rq).rb_node, parent, p);
    rb_insert_color(&mut (*rq).rb_node, root);
}

pub unsafe fn elv_rb_del(root: *mut rb_root, rq: *mut request) {
    BUG_ON!(RB_EMPTY_NODE!(&(*rq).rb_node));
    rb_erase(&mut (*rq).rb_node, root);
    RB_CLEAR_NODE!(&mut (*rq).rb_node);
}

pub unsafe fn elv_rb_find(root: *mut rb_root, sector: sector_t) -> *mut request {
    let mut n = (*root).rb_node;
    while !n.is_null() {
        let rq = rb_entry!(n, request, rb_node);
        if sector < blk_rq_pos(rq) { n = (*n).rb_left; }
        else if sector > blk_rq_pos(rq) { n = (*n).rb_right; }
        else { return rq; }
    }
    core::ptr::null_mut()
}

pub unsafe fn elv_merge(q: *mut request_queue, req: *mut *mut request,
                        bio: *mut bio) -> elv_merge {
    let e = (*q).elevator;
    if blk_queue_nomerges(q) || !bio_mergeable(bio) { return ELEVATOR_NO_MERGE; }
    if !(*q).last_merge.is_null() && elv_bio_merge_ok((*q).last_merge, bio) {
        let ret = blk_try_merge((*q).last_merge, bio);
        if ret != ELEVATOR_NO_MERGE { *req = (*q).last_merge; return ret; }
    }
    if blk_queue_noxmerges(q) { return ELEVATOR_NO_MERGE; }
    let rq = elv_rqhash_find(q, (*bio).bi_iter.bi_sector);
    if !rq.is_null() && elv_bio_merge_ok(rq, bio) {
        *req = rq;
        if blk_discard_mergable(rq) { return ELEVATOR_DISCARD_MERGE; }
        return ELEVATOR_BACK_MERGE;
    }
    if let Some(f) = (*(*e).type_).ops.request_merge { return f(q, req, bio); }
    ELEVATOR_NO_MERGE
}

pub unsafe fn elv_attempt_insert_merge(q: *mut request_queue, mut rq: *mut request,
                                      free: *mut list_head) -> bool {
    if blk_queue_nomerges(q) { return false; }
    if !(*q).last_merge.is_null() && blk_attempt_req_merge(q, (*q).last_merge, rq) {
        list_add(&mut (*rq).queuelist, free); return true;
    }
    if blk_queue_noxmerges(q) { return false; }
    let mut ret = false;
    loop {
        let r = elv_rqhash_find(q, blk_rq_pos(rq));
        if r.is_null() || !blk_attempt_req_merge(q, r, rq) { break; }
        list_add(&mut (*rq).queuelist, free); ret = true; rq = r;
    }
    ret
}

pub unsafe fn elv_merged_request(q: *mut request_queue, rq: *mut request, ty: elv_merge) {
    let e = (*q).elevator;
    if let Some(f) = (*(*e).type_).ops.request_merged { f(q, rq, ty); }
    if ty == ELEVATOR_BACK_MERGE { elv_rqhash_reposition(q, rq); }
    (*q).last_merge = rq;
}

pub unsafe fn elv_merge_requests(q: *mut request_queue, rq: *mut request, next: *mut request) {
    let e = (*q).elevator;
    if let Some(f) = (*(*e).type_).ops.requests_merged { f(q, rq, next); }
    elv_rqhash_reposition(q, rq); (*q).last_merge = rq;
}

pub unsafe fn elv_latter_request(q: *mut request_queue, rq: *mut request) -> *mut request {
    if let Some(f) = (*(*(*q).elevator).type_).ops.next_request { return f(q, rq); }
    core::ptr::null_mut()
}

pub unsafe fn elv_former_request(q: *mut request_queue, rq: *mut request) -> *mut request {
    if let Some(f) = (*(*(*q).elevator).type_).ops.former_request { return f(q, rq); }
    core::ptr::null_mut()
}

pub unsafe fn elv_rb_former_request(_q: *mut request_queue, rq: *mut request) -> *mut request {
    let n = rb_prev(&(*rq).rb_node); if !n.is_null() { return rb_entry_rq(n); }
    core::ptr::null_mut()
}

pub unsafe fn elv_rb_latter_request(_q: *mut request_queue, rq: *mut request) -> *mut request {
    let n = rb_next(&(*rq).rb_node); if !n.is_null() { return rb_entry_rq(n); }
    core::ptr::null_mut()
}

unsafe fn elv_register_queue(q: *mut request_queue, e: *mut elevator_queue, uevent: bool) -> c_int {
    let mut error = kobject_add(&mut (*e).kobj, &mut (*(*q).disk).queue_kobj, "iosched\0".as_ptr() as *const c_char);
    if error == 0 {
        let mut attr = (*(*e).type_).elevator_attrs;
        if !attr.is_null() {
            while !(*attr).attr.name.is_null() {
                if sysfs_create_file(&mut (*e).kobj, &mut (*attr).attr) != 0 { break; }
                attr = attr.add(1);
            }
        }
        if uevent { kobject_uevent(&mut (*e).kobj, KOBJ_ADD); }
        blk_mq_sched_reg_debugfs(q);
        set_bit(ELEVATOR_FLAG_REGISTERED, &mut (*e).flags);
    }
    error
}

unsafe fn elv_unregister_queue(q: *mut request_queue, e: *mut elevator_queue) {
    if !e.is_null() && test_and_clear_bit(ELEVATOR_FLAG_REGISTERED, &mut (*e).flags) {
        kobject_uevent(&mut (*e).kobj, KOBJ_REMOVE);
        kobject_del(&mut (*e).kobj);
        blk_mq_sched_unreg_debugfs(q);
    }
}

pub unsafe fn elv_register(e: *mut elevator_type) -> c_int {
    if WARN_ON_ONCE!((*e).ops.finish_request.is_none()) { return -EINVAL; }
    if WARN_ON_ONCE!((*e).ops.insert_requests.is_none() || (*e).ops.dispatch_request.is_none()) { return -EINVAL; }
    if (*e).icq_size != 0 {
        if WARN_ON!((*e).icq_size < core::mem::size_of::<io_cq>()) || WARN_ON!((*e).icq_align < core::mem::align_of::<io_cq>()) { return -EINVAL; }
        snprintf!((*e).icq_cache_name.as_mut_ptr(), (*e).icq_cache_name.len(), "%s_io_cq", (*e).elevator_name);
        (*e).icq_cache = kmem_cache_create((*e).icq_cache_name.as_ptr(), (*e).icq_size, (*e).icq_align, 0, None);
        if (*e).icq_cache.is_null() { return -ENOMEM; }
    }
    spin_lock(&mut ELV_LIST_LOCK);
    if !__elevator_find((*e).elevator_name).is_null() { spin_unlock(&mut ELV_LIST_LOCK); kmem_cache_destroy((*e).icq_cache); return -EBUSY; }
    list_add_tail(&mut (*e).list, &mut ELV_LIST);
    spin_unlock(&mut ELV_LIST_LOCK);
    printk!(KERN_INFO "io scheduler %s registered\n", (*e).elevator_name);
    0
}

pub unsafe fn elv_unregister(e: *mut elevator_type) {
    spin_lock(&mut ELV_LIST_LOCK); list_del_init(&mut (*e).list); spin_unlock(&mut ELV_LIST_LOCK);
    if !(*e).icq_cache.is_null() { rcu_barrier(); kmem_cache_destroy((*e).icq_cache); (*e).icq_cache = core::ptr::null_mut(); }
}

pub unsafe fn elv_update_nr_hw_queues(q: *mut request_queue, ctx: *mut elv_change_ctx) {
    let set = (*q).tag_set; let mut ret = -ENODEV;
    WARN_ON_ONCE!((*q).mq_freeze_depth == 0);
    if !(*ctx).type_.is_null() && !blk_queue_dying(q) && blk_queue_registered(q) {
        mutex_lock(&mut (*q).elevator_lock); ret = elevator_switch(q, ctx); mutex_unlock(&mut (*q).elevator_lock);
    }
    blk_mq_unfreeze_queue_nomemrestore(q);
    if ret == 0 { WARN_ON_ONCE!(elevator_change_done(q, ctx) != 0); }
    if (*ctx).new_.is_null() { blk_mq_free_sched_res(&mut (*ctx).res, (*ctx).type_, set); }
}

pub unsafe fn elevator_set_none(q: *mut request_queue) {
    let mut ctx = elv_change_ctx::zeroed(); (*ctx.name.as_mut_ptr()) = b'n' as c_char;
    let err = elevator_change(q, &mut ctx);
    if err < 0 { pr_warn!("%s: set none elevator failed %d\n", __func__, err); }
}

unsafe fn elevator_setup(_str: *mut c_char) -> c_int {
    pr_warn!("Kernel parameter elevator= does not have any effect anymore.\nPlease use sysfs to set IO scheduler for individual devices.\n");
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
