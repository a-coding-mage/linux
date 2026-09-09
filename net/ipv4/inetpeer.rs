// SPDX-License-Identifier: GPL-2.0
/*
 * INETPEER - A storage for permanent information about peers
 *
 * Translated from inetpeer.c. Kernel-provided types, constants, functions,
 * and macros referenced below are supplied by the surrounding translation.
 */

static mut peer_cachep: *mut kmem_cache = core::ptr::null_mut();
static mut inetpeer_hash_key: siphash_aligned_key_t = siphash_aligned_key_t { _unused: [] };

unsafe fn inetpeer_addr_hash(a: *const inetpeer_addr) -> u64 {
    net_get_random_once(
        &mut inetpeer_hash_key as *mut _ as *mut core::ffi::c_void,
        core::mem::size_of::<siphash_aligned_key_t>(),
    );

    if (*a).family == AF_INET {
        return siphash_2u32((*a).a4.addr as u32, (*a).a4.vif, &inetpeer_hash_key);
    }
    siphash_4u32(
        (*a).a6.s6_addr32[0] as u32,
        (*a).a6.s6_addr32[1] as u32,
        (*a).a6.s6_addr32[2] as u32,
        (*a).a6.s6_addr32[3] as u32,
        &inetpeer_hash_key,
    )
}

unsafe fn inetpeer_entry_cmp(
    dhash: u64,
    daddr: *const inetpeer_addr,
    p: *const inet_peer,
) -> i32 {
    if dhash < (*p).hash { return -1; }
    if dhash > (*p).hash { return 1; }
    inetpeer_addr_cmp(daddr, &(*p).daddr)
}

pub unsafe fn inet_peer_base_init(bp: *mut inet_peer_base) {
    (*bp).rb_root = RB_ROOT;
    seqlock_init(&mut (*bp).lock);
    (*bp).total = 0;
}

pub const PEER_MAX_GC: usize = 32;
pub static mut inet_peer_threshold: i32 = 0;
pub static mut inet_peer_minttl: i32 = 120 * HZ;
pub static mut inet_peer_maxttl: i32 = 10 * 60 * HZ;

pub unsafe fn inet_initpeers() {
    let nr_entries = div64_ul(
        (totalram_pages() as u64) << PAGE_SHIFT,
        100 * L1_CACHE_ALIGN(core::mem::size_of::<inet_peer>()),
    );
    inet_peer_threshold = clamp_val(nr_entries, 4096, 65536 + 128);
    peer_cachep = KMEM_CACHE(inet_peer, SLAB_HWCACHE_ALIGN | SLAB_PANIC);
}

unsafe fn lookup(
    daddr: *const inetpeer_addr, dhash: u64, base: *mut inet_peer_base,
    seq: u32, gc_stack: *mut *mut inet_peer, gc_cnt: *mut u32,
    parent_p: *mut *mut rb_node, pp_p: *mut *mut *mut rb_node,
) -> *mut inet_peer {
    let mut pp = &mut (*base).rb_root.rb_node as *mut *mut rb_node;
    let mut parent: *mut rb_node = core::ptr::null_mut();
    loop {
        let next = rcu_dereference_raw(*pp);
        if next.is_null() { break; }
        parent = next;
        let p = rb_entry(parent, inet_peer, rb_node);
        let cmp = inetpeer_entry_cmp(dhash, daddr, p);
        if cmp == 0 {
            let now = jiffies;
            if READ_ONCE((*p).dtime) != now { WRITE_ONCE((*p).dtime, now); }
            return p;
        }
        if !gc_stack.is_null() {
            if *gc_cnt < PEER_MAX_GC as u32 { *gc_stack.add(*gc_cnt as usize) = p; *gc_cnt += 1; }
        } else if unlikely(read_seqretry(&(*base).lock, seq)) { break; }
        pp = if cmp == -1 { &mut (*next).rb_left } else { &mut (*next).rb_right };
    }
    *parent_p = parent; *pp_p = pp; core::ptr::null_mut()
}

unsafe fn inet_peer_gc(base: *mut inet_peer_base, gc_stack: *mut *mut inet_peer, gc_cnt: u32) {
    let peer_threshold = READ_ONCE(inet_peer_threshold);
    let peer_maxttl = READ_ONCE(inet_peer_maxttl);
    let peer_minttl = READ_ONCE(inet_peer_minttl);
    let ttl = if (*base).total >= peer_threshold { 0 } else {
        peer_maxttl - (peer_maxttl - peer_minttl) / HZ * (*base).total / peer_threshold * HZ
    };
    for i in 0..gc_cnt as usize {
        let p = *gc_stack.add(i);
        if !p.is_null() {
            let delta = (jiffies as u32).wrapping_sub(READ_ONCE((*p).dtime));
            if delta < ttl as u32 || !refcount_dec_if_one(&(*p).refcnt) { *gc_stack.add(i) = core::ptr::null_mut(); }
        }
    }
    for i in 0..gc_cnt as usize { let p = *gc_stack.add(i); if !p.is_null() { rb_erase(&mut (*p).rb_node, &mut (*base).rb_root); (*base).total -= 1; kfree_rcu(p, rcu); } }
}

pub unsafe fn inet_getpeer(base: *mut inet_peer_base, daddr: *const inetpeer_addr) -> *mut inet_peer {
    let dhash = inetpeer_addr_hash(daddr); let mut gc_stack: [*mut inet_peer; PEER_MAX_GC] = [core::ptr::null_mut(); PEER_MAX_GC];
    let mut parent = core::ptr::null_mut(); let mut pp = core::ptr::null_mut(); let mut gc_cnt = 0; let seq = read_seqbegin(&(*base).lock);
    let mut p = lookup(daddr, dhash, base, seq, core::ptr::null_mut(), &mut gc_cnt, &mut parent, &mut pp);
    if !p.is_null() && !read_seqretry(&(*base).lock, seq) { return p; }
    parent = core::ptr::null_mut(); write_seqlock_bh(&mut (*base).lock); gc_cnt = 0;
    p = lookup(daddr, dhash, base, seq, gc_stack.as_mut_ptr(), &mut gc_cnt, &mut parent, &mut pp);
    if p.is_null() { p = kmem_cache_alloc(peer_cachep, GFP_ATOMIC); if !p.is_null() {
        (*p).daddr = *daddr; (*p).hash = dhash; (*p).dtime = jiffies as u32; refcount_set(&mut (*p).refcnt, 1); atomic_set(&mut (*p).rid, 0); (*p).metrics[RTAX_LOCK - 1] = INETPEER_METRICS_NEW; (*p).rate_tokens = 0; (*p).n_redirects = 0; (*p).rate_last = jiffies - 60 * HZ;
        rb_link_node(&mut (*p).rb_node, parent, pp); rb_insert_color(&mut (*p).rb_node, &mut (*base).rb_root); (*base).total += 1;
    }}
    if gc_cnt != 0 { inet_peer_gc(base, gc_stack.as_mut_ptr(), gc_cnt); } write_sequnlock_bh(&mut (*base).lock); p
}

pub unsafe fn inet_putpeer(p: *mut inet_peer) { if refcount_dec_and_test(&mut (*p).refcnt) { kfree_rcu(p, rcu); } }

pub const XRLIM_BURST_FACTOR: usize = 6;
pub unsafe fn inet_peer_xrlim_allow(peer: *mut inet_peer, timeout: usize) -> bool {
    if peer.is_null() { return true; }
    let otoken = READ_ONCE((*peer).rate_tokens); let mut token = otoken; let now = jiffies; let delta = now - READ_ONCE((*peer).rate_last);
    if delta != 0 { WRITE_ONCE((*peer).rate_last, now); token += delta; if token > XRLIM_BURST_FACTOR * timeout { token = XRLIM_BURST_FACTOR * timeout; } }
    let rc = if token >= timeout { token -= timeout; true } else { false }; if token != otoken { WRITE_ONCE((*peer).rate_tokens, token); } rc
}

pub unsafe fn inetpeer_invalidate_tree(base: *mut inet_peer_base) {
    let mut p = rb_first(&(*base).rb_root);
    while !p.is_null() { let peer = rb_entry(p, inet_peer, rb_node); p = rb_next(p); rb_erase(&mut (*peer).rb_node, &mut (*base).rb_root); inet_putpeer(peer); cond_resched(); }
    (*base).total = 0;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
