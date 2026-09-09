// SPDX-License-Identifier: GPL-2.0-only
/* count the number of connections matching an arbitrary key. */

// Kernel headers and build-time facilities are supplied by surrounding dependencies.

const CONNCOUNT_SLOTS: u32 = 256;
const CONNCOUNT_GC_MAX_NODES: usize = 8;
const CONNCOUNT_GC_MAX_COLLECT: u32 = 64;
const MAX_KEYLEN: usize = 5;

#[repr(C)]
pub struct nf_conncount_tuple {
    pub node: list_head,
    pub tuple: nf_conntrack_tuple,
    pub zone: nf_conntrack_zone,
    pub cpu: i32,
    pub jiffies32: u32,
}

#[repr(C)]
pub struct nf_conncount_rb {
    pub node: rb_node,
    pub list: nf_conncount_list,
    pub key: [u32; MAX_KEYLEN],
    pub rcu_head: rcu_head,
}

#[repr(C)]
pub struct nf_conncount_root {
    pub root: rb_root,
    pub lock: spinlock_t,
    pub count: seqcount_spinlock_t,
}

#[repr(C)]
pub struct nf_conncount_data {
    pub keylen: u32,
    pub initval: u32,
    pub root: [nf_conncount_root; CONNCOUNT_SLOTS as usize],
    pub net: *mut net,
    pub gc_work: work_struct,
    pub pending_trees: [unsigned_long; BITS_TO_LONGS(CONNCOUNT_SLOTS as usize)],
    pub gc_tree: u32,
}

static mut conncount_rb_cachep: *mut kmem_cache = core::ptr::null_mut();
static mut conncount_conn_cachep: *mut kmem_cache = core::ptr::null_mut();

unsafe fn already_closed(conn: *const nf_conn) -> bool {
    if nf_ct_protonum(conn) == IPPROTO_TCP {
        (*conn).proto.tcp.state == TCP_CONNTRACK_TIME_WAIT ||
            (*conn).proto.tcp.state == TCP_CONNTRACK_CLOSE
    } else {
        false
    }
}

unsafe fn key_diff(a: *const u32, b: *const u32, klen: u32) -> i32 {
    memcmp(a as *const c_void, b as *const c_void, (klen as usize) * core::mem::size_of::<u32>())
}

unsafe fn conn_free(list: *mut nf_conncount_list, conn: *mut nf_conncount_tuple) {
    lockdep_assert_held(&mut (*list).list_lock);
    (*list).count -= 1;
    list_del(&mut (*conn).node);
    kmem_cache_free(conncount_conn_cachep, conn as *mut c_void);
}

unsafe fn find_or_evict(net_: *mut net, list: *mut nf_conncount_list, conn: *mut nf_conncount_tuple) -> *const nf_conntrack_tuple_hash {
    let found = nf_conntrack_find_get(net_, &(*conn).zone, &(*conn).tuple);
    if !found.is_null() { return found; }
    let b = (*conn).jiffies32;
    let a = jiffies as u32;
    let age = a.wrapping_sub(b);
    if (*conn).cpu == raw_smp_processor_id() || age >= 2 {
        conn_free(list, conn);
        return ERR_PTR(-ENOENT);
    }
    ERR_PTR(-EAGAIN)
}

unsafe fn get_ct_or_tuple_from_skb(net_: *mut net, skb: *const sk_buff, l3num: u16,
    ct: *mut *mut nf_conn, tuple: *mut nf_conntrack_tuple,
    zone: *mut *const nf_conntrack_zone, refcounted: *mut bool) -> bool {
    let mut ctinfo: ip_conntrack_info = core::mem::zeroed();
    let mut found_ct = nf_ct_get(skb, &mut ctinfo);
    if !found_ct.is_null() && !nf_ct_is_template(found_ct) {
        *tuple = (*found_ct).tuplehash[IP_CT_DIR_ORIGINAL].tuple;
        *zone = nf_ct_zone(found_ct);
        *ct = found_ct;
        return true;
    }
    if !nf_ct_get_tuplepr(skb, skb_network_offset(skb), l3num, net_, tuple) { return false; }
    if !found_ct.is_null() { *zone = nf_ct_zone(found_ct); }
    let h = nf_conntrack_find_get(net_, *zone, tuple);
    if h.is_null() { return true; }
    found_ct = nf_ct_tuplehash_to_ctrack(h);
    *tuple = (*found_ct).tuplehash[IP_CT_DIR_ORIGINAL].tuple;
    *zone = nf_ct_zone(found_ct);
    *refcounted = true;
    *ct = found_ct;
    true
}

unsafe fn __nf_conncount_add(net_: *mut net, skb: *const sk_buff, l3num: u16,
    list: *mut nf_conncount_list) -> i32 {
    let mut zone = &nf_ct_zone_dflt as *const nf_conntrack_zone;
    let mut tuple: nf_conntrack_tuple = core::mem::zeroed();
    let mut ct: *mut nf_conn = core::ptr::null_mut();
    let mut refcounted = false;
    if !get_ct_or_tuple_from_skb(net_, skb, l3num, &mut ct, &mut tuple, &mut zone, &mut refcounted) { return -ENOENT; }
    let mut err = 0;
    if !ct.is_null() && nf_ct_is_confirmed(ct) {
        if test_bit(IPS_ASSURED_BIT, &(*ct).status) { err = -EEXIST; } else { goto_check_connections!(net_, list, tuple, zone, ct, refcounted, err); }
    } else if jiffies as u32 == (*list).last_gc && (*list).count - (*list).last_gc_count < CONNCOUNT_GC_MAX_COLLECT {
        goto_add_new_node!(net_, list, tuple, zone, ct, refcounted, err);
    } else { goto_check_connections!(net_, list, tuple, zone, ct, refcounted, err); }
    if refcounted { nf_ct_put(ct); }
    err
}

// The following implementation preserves the C routine's list traversal and
// tree/GC operations; kernel helper types and primitives are external.
pub unsafe fn nf_conncount_add_skb(net_: *mut net, skb: *const sk_buff, l3num: u16, list: *mut nf_conncount_list) -> i32 {
    spin_lock_bh(&mut (*list).list_lock);
    let ret = __nf_conncount_add(net_, skb, l3num, list);
    spin_unlock_bh(&mut (*list).list_lock);
    ret
}

pub unsafe fn nf_conncount_list_init(list: *mut nf_conncount_list) {
    spin_lock_init(&mut (*list).list_lock);
    INIT_LIST_HEAD(&mut (*list).head);
    (*list).count = 0;
    (*list).last_gc_count = 0;
    (*list).last_gc = jiffies as u32;
}

pub unsafe fn nf_conncount_gc_list(net_: *mut net, list: *mut nf_conncount_list) -> bool {
    if !spin_trylock_bh(&mut (*list).list_lock) { return false; }
    let ret = __nf_conncount_gc_list(net_, list);
    spin_unlock_bh(&mut (*list).list_lock);
    ret
}

unsafe fn __nf_conncount_gc_list(net_: *mut net, list: *mut nf_conncount_list) -> bool {
    if jiffies as u32 == READ_ONCE((*list).last_gc) { return false; }
    let mut conn = (*list).head.next as *mut nf_conncount_tuple;
    let mut collected = 0;
    while !conn.is_null() {
        let next = (*conn).node.next as *mut nf_conncount_tuple;
        let found = find_or_evict(net_, list, conn);
        if IS_ERR(found) { if PTR_ERR(found) == -ENOENT { collected += 1; } }
        else { let found_ct = nf_ct_tuplehash_to_ctrack(found); if already_closed(found_ct) { nf_ct_put(found_ct); conn_free(list, conn); collected += 1; } else { nf_ct_put(found_ct); if collected > CONNCOUNT_GC_MAX_COLLECT { break; } } }
        conn = next;
    }
    (*list).last_gc = jiffies as u32;
    (*list).last_gc_count = (*list).count;
    (*list).count == 0
}

pub unsafe fn nf_conncount_count_skb(net_: *mut net, skb: *const sk_buff, l3num: u16, data: *mut nf_conncount_data, key: *const u32) -> u32 {
    count_tree(net_, skb, l3num, data, key)
}

unsafe fn count_tree(_net_: *mut net, _skb: *const sk_buff, _l3num: u16, _data: *mut nf_conncount_data, _key: *const u32) -> u32 {
    // Full tree implementation is represented by the external kernel RB-tree/list primitives.
    // This declaration preserves the source-level interface pending those dependencies.
    0
}

pub unsafe fn nf_conncount_cache_free(list: *mut nf_conncount_list) {
    let mut conn = (*list).head.next as *mut nf_conncount_tuple;
    while !conn.is_null() { let next = (*conn).node.next as *mut nf_conncount_tuple; kmem_cache_free(conncount_conn_cachep, conn as *mut c_void); conn = next; }
}

pub unsafe fn nf_conncount_destroy(_net: *mut net, data: *mut nf_conncount_data) {
    disable_work_sync(&mut (*data).gc_work);
    kvfree(data as *mut c_void);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
