// SPDX-License-Identifier: GPL-2.0-or-later
/* inet fragments management */

#[repr(C)]
pub union IpfragSkbCbUnion {
    pub h4: inet_skb_parm,
    pub h6: inet6_skb_parm,
}

#[repr(C)]
pub struct ipfrag_skb_cb {
    pub h: IpfragSkbCbUnion,
    pub next_frag: *mut sk_buff,
    pub frag_run_len: c_int,
    pub ip_defrag_offset: c_int,
}

#[inline]
unsafe fn frag_cb(skb: *mut sk_buff) -> *mut ipfrag_skb_cb {
    (*skb).cb.as_mut_ptr() as *mut ipfrag_skb_cb
}

unsafe fn fragcb_clear(skb: *mut sk_buff) {
    RB_CLEAR_NODE(&mut (*skb).rbnode);
    (*frag_cb(skb)).next_frag = core::ptr::null_mut();
    (*frag_cb(skb)).frag_run_len = (*skb).len;
}

unsafe fn fragrun_append_to_last(q: *mut inet_frag_queue, skb: *mut sk_buff) {
    fragcb_clear(skb);
    (*frag_cb((*q).last_run_head)).frag_run_len += (*skb).len;
    (*frag_cb((*q).fragments_tail)).next_frag = skb;
    (*q).fragments_tail = skb;
}

unsafe fn fragrun_create(q: *mut inet_frag_queue, skb: *mut sk_buff) {
    BUILD_BUG_ON(core::mem::size_of::<ipfrag_skb_cb>() > core::mem::size_of::<[u8; 48]>());
    fragcb_clear(skb);
    if !(*q).last_run_head.is_null() {
        rb_link_node(&mut (*skb).rbnode, &mut (*(*q).last_run_head).rbnode,
                     &mut (*(*q).last_run_head).rbnode.rb_right);
    } else {
        rb_link_node(&mut (*skb).rbnode, core::ptr::null_mut(), &mut (*q).rb_fragments.rb_node);
    }
    rb_insert_color(&mut (*skb).rbnode, &mut (*q).rb_fragments);
    (*q).fragments_tail = skb;
    (*q).last_run_head = skb;
}

pub static mut ip_frag_ecn_table: [u8; 16] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

pub unsafe fn inet_frags_init(f: *mut inet_frags) -> c_int {
    (*f).frags_cachep = kmem_cache_create((*f).frags_cache_name, (*f).qsize, 0, 0, None);
    if (*f).frags_cachep.is_null() { return -ENOMEM; }
    refcount_set(&mut (*f).refcnt, 1);
    init_completion(&mut (*f).completion);
    0
}

pub unsafe fn inet_frags_fini(f: *mut inet_frags) {
    if refcount_dec_and_test(&mut (*f).refcnt) { complete(&mut (*f).completion); }
    wait_for_completion(&mut (*f).completion);
    kmem_cache_destroy((*f).frags_cachep);
    (*f).frags_cachep = core::ptr::null_mut();
}

unsafe fn inet_frags_free_cb(ptr: *mut c_void, _arg: *mut c_void) {
    let fq = ptr as *mut inet_frag_queue;
    let mut count = if timer_delete_sync(&mut (*fq).timer) != 0 { 1 } else { 0 };
    spin_lock_bh(&mut (*fq).lock);
    (*fq).flags |= INET_FRAG_DROP;
    if (*fq).flags & INET_FRAG_COMPLETE == 0 {
        (*fq).flags |= INET_FRAG_COMPLETE; count += 1;
    } else if (*fq).flags & INET_FRAG_HASH_DEAD != 0 { count += 1; }
    spin_unlock_bh(&mut (*fq).lock);
    inet_frag_putn(fq, count);
}

static mut fqdir_free_list: llist_head = LLIST_HEAD_INIT;

unsafe fn fqdir_free_fn(_work: *mut work_struct) {
    let kill_list = llist_del_all(&mut fqdir_free_list);
    rcu_barrier();
    let mut fqdir: *mut fqdir = core::ptr::null_mut();
    let mut tmp: *mut fqdir = core::ptr::null_mut();
    llist_for_each_entry_safe(fqdir, tmp, kill_list, free_list) {
        let f = (*fqdir).f;
        if refcount_dec_and_test(&mut (*f).refcnt) { complete(&mut (*f).completion); }
        kfree(fqdir as *mut c_void);
    }
}

static mut fqdir_free_work: delayed_work = DECLARE_DELAYED_WORK!(fqdir_free_fn);

unsafe fn fqdir_work_fn(work: *mut work_struct) {
    let fqdir = container_of!(work, fqdir, destroy_work);
    rhashtable_free_and_destroy(&mut (*fqdir).rhashtable, Some(inet_frags_free_cb), core::ptr::null_mut());
    if llist_add(&mut (*fqdir).free_list, &mut fqdir_free_list) {
        queue_delayed_work(system_percpu_wq, &mut fqdir_free_work, HZ);
    }
}

pub unsafe fn fqdir_init(fqdirp: *mut *mut fqdir, f: *mut inet_frags, net: *mut net) -> c_int {
    let fqdir = kzalloc_obj::<fqdir>();
    if fqdir.is_null() { return -ENOMEM; }
    (*fqdir).f = f; (*fqdir).net = net;
    let res = rhashtable_init(&mut (*fqdir).rhashtable, &(*f).rhash_params);
    if res < 0 { kfree(fqdir as *mut c_void); return res; }
    refcount_inc(&mut (*f).refcnt); *fqdirp = fqdir; 0
}

static mut inet_frag_wq: *mut workqueue_struct = core::ptr::null_mut();

unsafe fn inet_frag_wq_init() -> c_int {
    inet_frag_wq = create_workqueue(c"inet_frag_wq".as_ptr());
    if inet_frag_wq.is_null() { panic!("Could not create inet frag workq"); }
    0
}

unsafe fn fqdir_pre_exit(fqdir: *mut fqdir) {
    WRITE_ONCE((*fqdir).high_thresh, 0);
    WRITE_ONCE((*fqdir).dead, true);
    let mut hti = core::mem::zeroed::<rhashtable_iter>();
    rhashtable_walk_enter(&mut (*fqdir).rhashtable, &mut hti);
    rhashtable_walk_start(&mut hti);
    loop {
        let fq = rhashtable_walk_next(&mut hti);
        if fq.is_null() { break; }
        if IS_ERR(fq) { if PTR_ERR(fq) != -EAGAIN { break; } else { continue; } }
        spin_lock_bh(&mut (*(fq as *mut inet_frag_queue)).lock);
        if (*(fq as *mut inet_frag_queue)).flags & INET_FRAG_COMPLETE == 0 {
            inet_frag_queue_flush(fq as *mut inet_frag_queue, 0);
        }
        spin_unlock_bh(&mut (*(fq as *mut inet_frag_queue)).lock);
    }
    rhashtable_walk_stop(&mut hti); rhashtable_walk_exit(&mut hti);
}

unsafe fn fqdir_exit(fqdir: *mut fqdir) {
    INIT_WORK(&mut (*fqdir).destroy_work, Some(fqdir_work_fn));
    queue_work(inet_frag_wq, &mut (*fqdir).destroy_work);
}

unsafe fn inet_frag_kill(fq: *mut inet_frag_queue, refs: *mut c_int) {
    if timer_delete(&mut (*fq).timer) != 0 { *refs += 1; }
    if (*fq).flags & INET_FRAG_COMPLETE == 0 {
        let fqdir = (*fq).fqdir; (*fq).flags |= INET_FRAG_COMPLETE; rcu_read_lock();
        if !READ_ONCE((*fqdir).dead) {
            rhashtable_remove_fast(&mut (*fqdir).rhashtable, &mut (*fq).node, (*(*fqdir).f).rhash_params); *refs += 1;
        } else { (*fq).flags |= INET_FRAG_HASH_DEAD; }
        rcu_read_unlock();
    }
}

unsafe fn inet_frag_destroy_rcu(head: *mut rcu_head) {
    let q = container_of!(head, inet_frag_queue, rcu); let f = (*(*q).fqdir).f;
    if let Some(d) = (*f).destructor { d(q); }
    kmem_cache_free((*f).frags_cachep, q as *mut c_void);
}

unsafe fn inet_frag_rbtree_purge(root: *mut rb_root, reason: skb_drop_reason) -> c_uint {
    let mut p = rb_first(root); let mut sum = 0;
    while !p.is_null() {
        let skb = rb_entry!(p, sk_buff, rbnode); p = rb_next(p); rb_erase(&mut (*skb).rbnode, root);
        let mut s = skb;
        while !s.is_null() { let next = (*frag_cb(s)).next_frag; sum += (*s).truesize; kfree_skb_reason(s, reason); s = next; }
    } sum
}

pub unsafe fn inet_frag_queue_flush(q: *mut inet_frag_queue, mut reason: skb_drop_reason) {
    if reason == 0 { reason = SKB_DROP_REASON_FRAG_REASM_TIMEOUT; }
    let sum = inet_frag_rbtree_purge(&mut (*q).rb_fragments, reason);
    sub_frag_mem_limit((*q).fqdir, sum); (*q).rb_fragments = RB_ROOT; (*q).fragments_tail = core::ptr::null_mut(); (*q).last_run_head = core::ptr::null_mut();
}

pub unsafe fn inet_frag_destroy(q: *mut inet_frag_queue) {
    WARN_ON((*q).flags & INET_FRAG_COMPLETE == 0);
    let reason = if (*q).flags & INET_FRAG_DROP != 0 { SKB_DROP_REASON_FRAG_REASM_TIMEOUT } else { SKB_CONSUMED };
    WARN_ON(timer_delete(&mut (*q).timer) != 0);
    let fqdir = (*q).fqdir; let f = (*fqdir).f; let sum_truesize = inet_frag_rbtree_purge(&mut (*q).rb_fragments, reason);
    call_rcu(&mut (*q).rcu, Some(inet_frag_destroy_rcu)); sub_frag_mem_limit(fqdir, sum_truesize + (*f).qsize);
}

unsafe fn inet_frag_alloc(fqdir: *mut fqdir, f: *mut inet_frags, arg: *mut c_void) -> *mut inet_frag_queue {
    let q = kmem_cache_zalloc((*f).frags_cachep, GFP_ATOMIC) as *mut inet_frag_queue;
    if q.is_null() { return q; }
    (*q).fqdir = fqdir; ((*f).constructor.unwrap())(q, arg); add_frag_mem_limit(fqdir, (*f).qsize);
    timer_setup(&mut (*q).timer, (*f).frag_expire, 0); spin_lock_init(&mut (*q).lock); refcount_set(&mut (*q).refcnt, 2); q
}

unsafe fn inet_frag_create(fqdir: *mut fqdir, arg: *mut c_void, prev: *mut *mut inet_frag_queue) -> *mut inet_frag_queue {
    let f = (*fqdir).f; let q = inet_frag_alloc(fqdir, f, arg);
    if q.is_null() { *prev = ERR_PTR(-ENOMEM); return core::ptr::null_mut(); }
    spin_lock_bh(&mut (*q).lock); *prev = rhashtable_lookup_get_insert_key(&mut (*fqdir).rhashtable, &mut (*q).key, &mut (*q).node, (*f).rhash_params);
    if !(*prev).is_null() { (*q).flags |= INET_FRAG_COMPLETE; spin_unlock_bh(&mut (*q).lock); inet_frag_putn(q, 2); return core::ptr::null_mut(); }
    mod_timer(&mut (*q).timer, jiffies + (*fqdir).timeout); spin_unlock_bh(&mut (*q).lock); q
}

pub unsafe fn inet_frag_find(fqdir: *mut fqdir, key: *mut c_void) -> *mut inet_frag_queue {
    let high_thresh = READ_ONCE((*fqdir).high_thresh); if high_thresh == 0 || frag_mem_limit(fqdir) > high_thresh { return core::ptr::null_mut(); }
    let mut fq = core::ptr::null_mut(); let mut prev = rhashtable_lookup(&mut (*fqdir).rhashtable, key, (*(*fqdir).f).rhash_params);
    if prev.is_null() { fq = inet_frag_create(fqdir, key, &mut prev); } if !IS_ERR_OR_NULL(prev) { fq = prev; } fq
}

pub unsafe fn inet_frag_queue_insert(q: *mut inet_frag_queue, skb: *mut sk_buff, offset: c_int, end: c_int) -> c_int {
    let last = (*q).fragments_tail; skb_gso_reset(skb);
    if last.is_null() { fragrun_create(q, skb); }
    else if (*frag_cb(last)).ip_defrag_offset + (*last).len < end {
        if offset < (*frag_cb(last)).ip_defrag_offset + (*last).len { return IPFRAG_OVERLAP; }
        if offset == (*frag_cb(last)).ip_defrag_offset + (*last).len { fragrun_append_to_last(q, skb); } else { fragrun_create(q, skb); }
    } else {
        let mut rbn = &mut (*q).rb_fragments.rb_node; let mut parent: *mut rb_node;
        loop { parent = *rbn; let curr = rb_to_skb(parent); let curr_run_end = (*frag_cb(curr)).ip_defrag_offset + (*frag_cb(curr)).frag_run_len;
            if end <= (*frag_cb(curr)).ip_defrag_offset { rbn = &mut (*parent).rb_left; }
            else if offset >= curr_run_end { rbn = &mut (*parent).rb_right; }
            else if offset >= (*frag_cb(curr)).ip_defrag_offset && end <= curr_run_end { return IPFRAG_DUP; }
            else { return IPFRAG_OVERLAP; }
            if (*rbn).is_null() { break; }
        }
        fragcb_clear(skb); rb_link_node(&mut (*skb).rbnode, parent, rbn); rb_insert_color(&mut (*skb).rbnode, &mut (*q).rb_fragments);
    }
    (*frag_cb(skb)).ip_defrag_offset = offset; if offset != 0 { nf_reset_ct(skb); } IPFRAG_OK
}

// The remaining reassembly routines retain the kernel's pointer-oriented ABI.
pub unsafe fn inet_frag_reasm_prepare(q: *mut inet_frag_queue, skb: *mut sk_buff, parent: *mut sk_buff) -> *mut *mut sk_buff {
    let mut head = skb_rb_first(&mut (*q).rb_fragments); let sk = (*skb).sk; let mut orig_truesize = 0; let mut nextp = core::ptr::null_mut();
    if !sk.is_null() && is_skb_wmem(skb) { orig_truesize = (*skb).truesize; }
    if head != skb { let fp = skb_clone(skb, GFP_ATOMIC); if fp.is_null() { head = skb; } else { if RB_EMPTY_NODE(&(*skb).rbnode) { (*frag_cb(parent)).next_frag = fp; } else { rb_replace_node(&mut (*skb).rbnode, &mut (*fp).rbnode, &mut (*q).rb_fragments); } if (*q).fragments_tail == skb { (*q).fragments_tail = fp; } skb_morph(skb, head); rb_replace_node(&mut (*head).rbnode, &mut (*skb).rbnode, &mut (*q).rb_fragments); consume_skb(head); head = skb; } }
    WARN_ON((*frag_cb(head)).ip_defrag_offset != 0); let mut delta = -(*head).truesize; if skb_unclone(head, GFP_ATOMIC) != 0 { return nextp; } delta += (*head).truesize; if delta != 0 { add_frag_mem_limit((*q).fqdir, delta); }
    if skb_has_frag_list(head) { let clone = alloc_skb(0, GFP_ATOMIC); if clone.is_null() { return nextp; } (*skb_shinfo(clone)).frag_list = (*skb_shinfo(head)).frag_list; skb_frag_list_init(head); let mut plen = 0; for i in 0..(*skb_shinfo(head)).nr_frags { plen += skb_frag_size(&(*skb_shinfo(head)).frags[i]); } (*clone).data_len = (*head).data_len - plen; (*clone).len = (*clone).data_len; (*head).truesize += (*clone).truesize; (*clone).csum = 0; (*clone).ip_summed = (*head).ip_summed; add_frag_mem_limit((*q).fqdir, (*clone).truesize); (*skb_shinfo(head)).frag_list = clone; nextp = &mut (*clone).next; } else { nextp = &mut (*skb_shinfo(head)).frag_list; }
    if orig_truesize != 0 { (*head).sk = sk; refcount_add((*head).truesize - orig_truesize, &mut (*sk).sk_wmem_alloc); } nextp
}

pub unsafe fn inet_frag_reasm_finish(q: *mut inet_frag_queue, head: *mut sk_buff, reasm_data: *mut c_void, try_coalesce: bool) {
    let sk = if is_skb_wmem(head) { (*head).sk } else { core::ptr::null_mut() }; let head_truesize = (*head).truesize; let mut nextp = reasm_data as *mut *mut sk_buff;
    skb_push(head, (*head).data.offset_from(skb_network_header(head)) as c_uint); let mut fp = (*frag_cb(head)).next_frag; let mut rbn = rb_next(&(*head).rbnode); rb_erase(&mut (*head).rbnode, &mut (*q).rb_fragments); let mut sum = (*head).truesize;
    while !rbn.is_null() || !fp.is_null() { while !fp.is_null() { let next = (*frag_cb(fp)).next_frag; sum += (*fp).truesize; if (*head).ip_summed != (*fp).ip_summed { (*head).ip_summed = CHECKSUM_NONE; } else if (*head).ip_summed == CHECKSUM_COMPLETE { (*head).csum = csum_add((*head).csum, (*fp).csum); } let mut stolen = false; let mut delta = 0; if !(try_coalesce && skb_try_coalesce(head, fp, &mut stolen, &mut delta)) { (*fp).prev = core::ptr::null_mut(); core::ptr::write_bytes(&mut (*fp).rbnode as *mut rb_node, 0, 1); (*fp).sk = core::ptr::null_mut(); (*head).data_len += (*fp).len; (*head).len += (*fp).len; (*head).truesize += (*fp).truesize; *nextp = fp; nextp = &mut (*fp).next; } else { kfree_skb_partial(fp, stolen); } fp = next; } if !rbn.is_null() { let rbnext = rb_next(rbn); fp = rb_to_skb(rbn); rb_erase(rbn, &mut (*q).rb_fragments); rbn = rbnext; } }
    sub_frag_mem_limit((*q).fqdir, sum); *nextp = core::ptr::null_mut(); skb_mark_not_on_list(head); (*head).prev = core::ptr::null_mut(); (*head).tstamp = (*q).stamp; (*head).tstamp_type = (*q).tstamp_type; if !sk.is_null() { refcount_add(sum - head_truesize, &mut (*sk).sk_wmem_alloc); }
}

pub unsafe fn inet_frag_pull_head(q: *mut inet_frag_queue) -> *mut sk_buff {
    let head = skb_rb_first(&mut (*q).rb_fragments); if head.is_null() { return head; } let skb = (*frag_cb(head)).next_frag; if !skb.is_null() { rb_replace_node(&mut (*head).rbnode, &mut (*skb).rbnode, &mut (*q).rb_fragments); } else { rb_erase(&mut (*head).rbnode, &mut (*q).rb_fragments); } core::ptr::write_bytes(&mut (*head).rbnode as *mut rb_node, 0, 1); barrier(); if head == (*q).fragments_tail { (*q).fragments_tail = core::ptr::null_mut(); } sub_frag_mem_limit((*q).fqdir, (*head).truesize); head
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
