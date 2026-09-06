// SPDX-License-Identifier: GPL-2.0-only
/*
 * Implementation of the kernel access vector cache (AVC).
 *
 * Authors:  Stephen Smalley, <stephen.smalley.work@gmail.com>
 *	     James Morris <jmorris@redhat.com>
 *
 * Update:   KaiGai, Kohei <kaigai@ak.jp.nec.com>
 *	Replaced the avc_lock spinlock by RCU.
 *
 * Copyright (C) 2003 Red Hat, Inc., James Morris <jmorris@redhat.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

// C includes translated as external dependencies:
// linux/types.h, linux/stddef.h, linux/kernel.h, linux/slab.h, linux/fs.h,
// linux/dcache.h, linux/init.h, linux/skbuff.h, linux/percpu.h, linux/list.h,
// net/sock.h, linux/un.h, net/af_unix.h, linux/ip.h, linux/audit.h,
// linux/ipv6.h, net/ipv6.h, avc.h, avc_ss.h, classmap.h, hash.h,
// trace/events/avc.h with CREATE_TRACE_POINTS.

type u8 = u8;
type u16 = u16;
type u32 = u32;

const AVC_CACHE_SLOTS: usize = 1usize << CONFIG_SECURITY_SELINUX_AVC_HASH_BITS;
const AVC_DEF_CACHE_THRESHOLD: c_uint = AVC_CACHE_SLOTS as c_uint;
const AVC_CACHE_RECLAIM: c_int = 16;

#[repr(C)]
pub struct avc_entry {
    ssid: u32,
    tsid: u32,
    tclass: u16,
    avd: av_decision,
    xp_node: *mut avc_xperms_node,
}

#[repr(C)]
pub struct avc_node {
    ae: avc_entry,
    list: hlist_node, /* anchored in avc_cache->slots[i] */
    rhead: rcu_head,
}

#[repr(C)]
pub struct avc_xperms_decision_node {
    xpd: extended_perms_decision,
    xpd_list: list_head, /* list of extended_perms_decision */
}

#[repr(C)]
pub struct avc_xperms_node {
    xp: extended_perms,
    xpd_head: list_head, /* list head of extended_perms_decision */
}

#[repr(C)]
pub struct avc_cache {
    slots: [hlist_head; AVC_CACHE_SLOTS], /* head for avc_node->list */
    slots_lock: [spinlock_t; AVC_CACHE_SLOTS], /* lock for writes */
    lru_hint: atomic_t,                  /* LRU hint for reclaim scan */
    active_nodes: atomic_t,
    latest_notif: u32, /* latest revocation notification */
}

#[repr(C)]
pub struct avc_callback_node {
    callback: Option<unsafe extern "C" fn(event: u32) -> c_int>,
    events: u32,
    next: *mut avc_callback_node,
}

// CONFIG_SECURITY_SELINUX_AVC_STATS:
// DEFINE_PER_CPU(struct avc_cache_stats, avc_cache_stats) = { 0 };

#[repr(C)]
pub struct selinux_avc {
    avc_cache_threshold: c_uint,
    avc_cache: avc_cache,
}

static mut selinux_avc: selinux_avc = unsafe { core::mem::zeroed() };

#[no_mangle]
pub unsafe extern "C" fn selinux_avc_init() {
    let mut i: c_int;

    selinux_avc.avc_cache_threshold = AVC_DEF_CACHE_THRESHOLD;
    i = 0;
    while i < AVC_CACHE_SLOTS as c_int {
        INIT_HLIST_HEAD(&mut selinux_avc.avc_cache.slots[i as usize]);
        spin_lock_init(&mut selinux_avc.avc_cache.slots_lock[i as usize]);
        i += 1;
    }
    atomic_set(&mut selinux_avc.avc_cache.active_nodes, 0);
    atomic_set(&mut selinux_avc.avc_cache.lru_hint, 0);
}

#[no_mangle]
pub unsafe extern "C" fn avc_get_cache_threshold() -> c_uint {
    selinux_avc.avc_cache_threshold
}

#[no_mangle]
pub unsafe extern "C" fn avc_set_cache_threshold(cache_threshold: c_uint) {
    selinux_avc.avc_cache_threshold = cache_threshold;
}

static mut avc_callbacks: *mut avc_callback_node = ptr::null_mut();
static mut avc_node_cachep: *mut kmem_cache = ptr::null_mut();
static mut avc_xperms_data_cachep: *mut kmem_cache = ptr::null_mut();
static mut avc_xperms_decision_cachep: *mut kmem_cache = ptr::null_mut();
static mut avc_xperms_cachep: *mut kmem_cache = ptr::null_mut();

#[inline]
unsafe fn avc_hash(ssid: u32, tsid: u32, tclass: u16) -> u32 {
    av_hash(ssid, tsid, tclass as u32, (AVC_CACHE_SLOTS - 1) as u32)
}

/**
 * avc_init - Initialize the AVC.
 *
 * Initialize the access vector cache.
 */
#[no_mangle]
pub unsafe extern "C" fn avc_init() {
    avc_node_cachep = KMEM_CACHE_AVC_NODE(SLAB_PANIC);
    avc_xperms_cachep = KMEM_CACHE_AVC_XPERMS_NODE(SLAB_PANIC);
    avc_xperms_decision_cachep = KMEM_CACHE_AVC_XPERMS_DECISION_NODE(SLAB_PANIC);
    avc_xperms_data_cachep = KMEM_CACHE_EXTENDED_PERMS_DATA(SLAB_PANIC);
}

#[no_mangle]
pub unsafe extern "C" fn avc_get_hash_stats(page: *mut c_char) -> c_int {
    let mut i: c_int;
    let mut chain_len: c_int;
    let mut max_chain_len: c_int;
    let mut slots_used: c_int;
    let mut node: *mut avc_node;
    let mut head: *mut hlist_head;

    rcu_read_lock();

    slots_used = 0;
    max_chain_len = 0;
    i = 0;
    while i < AVC_CACHE_SLOTS as c_int {
        head = &mut selinux_avc.avc_cache.slots[i as usize];
        if !hlist_empty(head) {
            slots_used += 1;
            chain_len = 0;
            hlist_for_each_entry_rcu!(node, head, list, {
                chain_len += 1;
            });
            if chain_len > max_chain_len {
                max_chain_len = chain_len;
            }
        }
        i += 1;
    }

    rcu_read_unlock();

    scnprintf(
        page,
        PAGE_SIZE,
        c_str!("entries: %d\nbuckets used: %d/%d\nlongest chain: %d\n"),
        atomic_read(&mut selinux_avc.avc_cache.active_nodes),
        slots_used,
        AVC_CACHE_SLOTS as c_int,
        max_chain_len,
    )
}

/*
 * using a linked list for extended_perms_decision lookup because the list is
 * always small. i.e. less than 5, typically 1
 */
unsafe fn avc_xperms_decision_lookup(
    driver: u8,
    base_perm: u8,
    xp_node: *mut avc_xperms_node,
) -> *mut extended_perms_decision {
    let mut xpd_node: *mut avc_xperms_decision_node;

    list_for_each_entry!(xpd_node, &mut (*xp_node).xpd_head, xpd_list, {
        if (*xpd_node).xpd.driver == driver && (*xpd_node).xpd.base_perm == base_perm {
            return &mut (*xpd_node).xpd;
        }
    });
    ptr::null_mut()
}

#[inline]
unsafe fn avc_xperms_has_perm(
    xpd: *mut extended_perms_decision,
    perm: u8,
    which: u8,
) -> c_uint {
    let mut rc: c_uint = 0;

    if which == XPERMS_ALLOWED && ((*xpd).used & XPERMS_ALLOWED) != 0 {
        rc = security_xperm_test((*(*xpd).allowed).p.as_mut_ptr(), perm);
    } else if which == XPERMS_AUDITALLOW && ((*xpd).used & XPERMS_AUDITALLOW) != 0 {
        rc = security_xperm_test((*(*xpd).auditallow).p.as_mut_ptr(), perm);
    } else if which == XPERMS_DONTAUDIT && ((*xpd).used & XPERMS_DONTAUDIT) != 0 {
        rc = security_xperm_test((*(*xpd).dontaudit).p.as_mut_ptr(), perm);
    }
    rc
}

unsafe fn avc_xperms_allow_perm(
    xp_node: *mut avc_xperms_node,
    driver: u8,
    base_perm: u8,
    perm: u8,
) {
    let mut xpd: *mut extended_perms_decision;
    security_xperm_set((*xp_node).xp.drivers.p.as_mut_ptr(), driver);
    (*xp_node).xp.base_perms |= base_perm;
    xpd = avc_xperms_decision_lookup(driver, base_perm, xp_node);
    if !xpd.is_null() && !(*xpd).allowed.is_null() {
        security_xperm_set((*(*xpd).allowed).p.as_mut_ptr(), perm);
    }
}

unsafe fn avc_xperms_decision_free(xpd_node: *mut avc_xperms_decision_node) {
    let mut xpd: *mut extended_perms_decision;

    xpd = &mut (*xpd_node).xpd;
    if !(*xpd).allowed.is_null() {
        kmem_cache_free(avc_xperms_data_cachep, (*xpd).allowed as *mut c_void);
    }
    if !(*xpd).auditallow.is_null() {
        kmem_cache_free(avc_xperms_data_cachep, (*xpd).auditallow as *mut c_void);
    }
    if !(*xpd).dontaudit.is_null() {
        kmem_cache_free(avc_xperms_data_cachep, (*xpd).dontaudit as *mut c_void);
    }
    kmem_cache_free(avc_xperms_decision_cachep, xpd_node as *mut c_void);
}

unsafe fn avc_xperms_free(xp_node: *mut avc_xperms_node) {
    let mut xpd_node: *mut avc_xperms_decision_node;
    let mut tmp: *mut avc_xperms_decision_node;

    if xp_node.is_null() {
        return;
    }

    list_for_each_entry_safe!(xpd_node, tmp, &mut (*xp_node).xpd_head, xpd_list, {
        list_del(&mut (*xpd_node).xpd_list);
        avc_xperms_decision_free(xpd_node);
    });
    kmem_cache_free(avc_xperms_cachep, xp_node as *mut c_void);
}

unsafe fn avc_copy_xperms_decision(
    dest: *mut extended_perms_decision,
    src: *mut extended_perms_decision,
) {
    (*dest).base_perm = (*src).base_perm;
    (*dest).driver = (*src).driver;
    (*dest).used = (*src).used;
    if ((*dest).used & XPERMS_ALLOWED) != 0 {
        memcpy(
            (*(*dest).allowed).p.as_mut_ptr() as *mut c_void,
            (*(*src).allowed).p.as_mut_ptr() as *const c_void,
            size_of_val(&(*(*src).allowed).p),
        );
    }
    if ((*dest).used & XPERMS_AUDITALLOW) != 0 {
        memcpy(
            (*(*dest).auditallow).p.as_mut_ptr() as *mut c_void,
            (*(*src).auditallow).p.as_mut_ptr() as *const c_void,
            size_of_val(&(*(*src).auditallow).p),
        );
    }
    if ((*dest).used & XPERMS_DONTAUDIT) != 0 {
        memcpy(
            (*(*dest).dontaudit).p.as_mut_ptr() as *mut c_void,
            (*(*src).dontaudit).p.as_mut_ptr() as *const c_void,
            size_of_val(&(*(*src).dontaudit).p),
        );
    }
}

/*
 * similar to avc_copy_xperms_decision, but only copy decision
 * information relevant to this perm
 */
#[inline]
unsafe fn avc_quick_copy_xperms_decision(
    perm: u8,
    dest: *mut extended_perms_decision,
    src: *mut extended_perms_decision,
) {
    /*
     * compute index of the u32 of the 256 bits (8 u32s) that contain this
     * command permission
     */
    let i: u8 = perm >> 5;

    (*dest).base_perm = (*src).base_perm;
    (*dest).used = (*src).used;
    if ((*dest).used & XPERMS_ALLOWED) != 0 {
        (*(*dest).allowed).p[i as usize] = (*(*src).allowed).p[i as usize];
    }
    if ((*dest).used & XPERMS_AUDITALLOW) != 0 {
        (*(*dest).auditallow).p[i as usize] = (*(*src).auditallow).p[i as usize];
    }
    if ((*dest).used & XPERMS_DONTAUDIT) != 0 {
        (*(*dest).dontaudit).p[i as usize] = (*(*src).dontaudit).p[i as usize];
    }
}

unsafe fn avc_xperms_decision_alloc(which: u8) -> *mut avc_xperms_decision_node {
    let mut xpd_node: *mut avc_xperms_decision_node;
    let mut xpd: *mut extended_perms_decision;

    xpd_node = kmem_cache_zalloc(avc_xperms_decision_cachep, GFP_NOWAIT)
        as *mut avc_xperms_decision_node;
    if xpd_node.is_null() {
        return ptr::null_mut();
    }

    xpd = &mut (*xpd_node).xpd;
    if (which & XPERMS_ALLOWED) != 0 {
        (*xpd).allowed =
            kmem_cache_zalloc(avc_xperms_data_cachep, GFP_NOWAIT) as *mut extended_perms_data;
        if (*xpd).allowed.is_null() {
            avc_xperms_decision_free(xpd_node);
            return ptr::null_mut();
        }
    }
    if (which & XPERMS_AUDITALLOW) != 0 {
        (*xpd).auditallow =
            kmem_cache_zalloc(avc_xperms_data_cachep, GFP_NOWAIT) as *mut extended_perms_data;
        if (*xpd).auditallow.is_null() {
            avc_xperms_decision_free(xpd_node);
            return ptr::null_mut();
        }
    }
    if (which & XPERMS_DONTAUDIT) != 0 {
        (*xpd).dontaudit =
            kmem_cache_zalloc(avc_xperms_data_cachep, GFP_NOWAIT) as *mut extended_perms_data;
        if (*xpd).dontaudit.is_null() {
            avc_xperms_decision_free(xpd_node);
            return ptr::null_mut();
        }
    }
    xpd_node
}

unsafe fn avc_add_xperms_decision(
    node: *mut avc_node,
    src: *mut extended_perms_decision,
) -> c_int {
    let mut dest_xpd: *mut avc_xperms_decision_node;

    dest_xpd = avc_xperms_decision_alloc((*src).used);
    if dest_xpd.is_null() {
        return -ENOMEM;
    }
    avc_copy_xperms_decision(&mut (*dest_xpd).xpd, src);
    list_add(&mut (*dest_xpd).xpd_list, &mut (*(*node).ae.xp_node).xpd_head);
    (*(*node).ae.xp_node).xp.len += 1;
    0
}

unsafe fn avc_xperms_alloc() -> *mut avc_xperms_node {
    let mut xp_node: *mut avc_xperms_node;

    xp_node = kmem_cache_zalloc(avc_xperms_cachep, GFP_NOWAIT) as *mut avc_xperms_node;
    if xp_node.is_null() {
        return xp_node;
    }
    INIT_LIST_HEAD(&mut (*xp_node).xpd_head);
    xp_node
}

unsafe fn avc_xperms_populate(node: *mut avc_node, src: *mut avc_xperms_node) -> c_int {
    let mut dest: *mut avc_xperms_node;
    let mut dest_xpd: *mut avc_xperms_decision_node;
    let mut src_xpd: *mut avc_xperms_decision_node;

    if (*src).xp.len == 0 {
        return 0;
    }
    dest = avc_xperms_alloc();
    if dest.is_null() {
        return -ENOMEM;
    }

    memcpy(
        (*dest).xp.drivers.p.as_mut_ptr() as *mut c_void,
        (*src).xp.drivers.p.as_mut_ptr() as *const c_void,
        size_of_val(&(*dest).xp.drivers.p),
    );
    (*dest).xp.len = (*src).xp.len;
    (*dest).xp.base_perms = (*src).xp.base_perms;

    /* for each source xpd allocate a destination xpd and copy */
    list_for_each_entry!(src_xpd, &mut (*src).xpd_head, xpd_list, {
        dest_xpd = avc_xperms_decision_alloc((*src_xpd).xpd.used);
        if dest_xpd.is_null() {
            avc_xperms_free(dest);
            return -ENOMEM;
        }
        avc_copy_xperms_decision(&mut (*dest_xpd).xpd, &mut (*src_xpd).xpd);
        list_add(&mut (*dest_xpd).xpd_list, &mut (*dest).xpd_head);
    });
    (*node).ae.xp_node = dest;
    0
}

#[inline]
unsafe fn avc_xperms_audit_required(
    requested: u32,
    avd: *mut av_decision,
    xpd: *mut extended_perms_decision,
    perm: u8,
    result: c_int,
    deniedp: *mut u32,
) -> u32 {
    let mut denied: u32;
    let mut audited: u32;

    denied = requested & !(*avd).allowed;
    if unlikely(denied != 0) {
        audited = denied & (*avd).auditdeny;
        if audited != 0 && !xpd.is_null() {
            if avc_xperms_has_perm(xpd, perm, XPERMS_DONTAUDIT) != 0 {
                audited = 0;
            }
        }
    } else if result != 0 {
        denied = requested;
        audited = denied;
    } else {
        audited = requested & (*avd).auditallow;
        if audited != 0 && !xpd.is_null() {
            if avc_xperms_has_perm(xpd, perm, XPERMS_AUDITALLOW) == 0 {
                audited = 0;
            }
        }
    }

    *deniedp = denied;
    audited
}

#[inline]
unsafe fn avc_xperms_audit(
    ssid: u32,
    tsid: u32,
    tclass: u16,
    requested: u32,
    avd: *mut av_decision,
    xpd: *mut extended_perms_decision,
    perm: u8,
    result: c_int,
    ad: *mut common_audit_data,
) -> c_int {
    let mut audited: u32;
    let mut denied: u32 = 0;

    audited = avc_xperms_audit_required(requested, avd, xpd, perm, result, &mut denied);
    if likely(audited == 0) {
        return 0;
    }
    slow_avc_audit(ssid, tsid, tclass, requested, audited, denied, result, ad)
}

unsafe extern "C" fn avc_node_free(rhead: *mut rcu_head) {
    let node: *mut avc_node = container_of!(rhead, avc_node, rhead);
    avc_xperms_free((*node).ae.xp_node);
    kmem_cache_free(avc_node_cachep, node as *mut c_void);
    avc_cache_stats_incr!(frees);
}

unsafe fn avc_node_delete(node: *mut avc_node) {
    hlist_del_rcu(&mut (*node).list);
    call_rcu(&mut (*node).rhead, Some(avc_node_free));
    atomic_dec(&mut selinux_avc.avc_cache.active_nodes);
}

unsafe fn avc_node_kill(node: *mut avc_node) {
    avc_xperms_free((*node).ae.xp_node);
    kmem_cache_free(avc_node_cachep, node as *mut c_void);
    avc_cache_stats_incr!(frees);
    atomic_dec(&mut selinux_avc.avc_cache.active_nodes);
}

unsafe fn avc_node_replace(new: *mut avc_node, old: *mut avc_node) {
    hlist_replace_rcu(&mut (*old).list, &mut (*new).list);
    call_rcu(&mut (*old).rhead, Some(avc_node_free));
    atomic_dec(&mut selinux_avc.avc_cache.active_nodes);
}

#[inline]
unsafe fn avc_reclaim_node() -> c_int {
    let mut node: *mut avc_node;
    let mut hvalue: c_int;
    let mut try_: c_int;
    let mut ecx: c_int;
    let mut flags: c_ulong = 0;
    let mut head: *mut hlist_head;
    let mut lock: *mut spinlock_t;

    try_ = 0;
    ecx = 0;
    while try_ < AVC_CACHE_SLOTS as c_int {
        hvalue = (atomic_inc_return(&mut selinux_avc.avc_cache.lru_hint)
            & (AVC_CACHE_SLOTS as c_int - 1)) as c_int;
        head = &mut selinux_avc.avc_cache.slots[hvalue as usize];
        lock = &mut selinux_avc.avc_cache.slots_lock[hvalue as usize];

        if !spin_trylock_irqsave(lock, &mut flags) {
            try_ += 1;
            continue;
        }

        rcu_read_lock();
        hlist_for_each_entry!(node, head, list, {
            avc_node_delete(node);
            avc_cache_stats_incr!(reclaims);
            ecx += 1;
            if ecx >= AVC_CACHE_RECLAIM {
                rcu_read_unlock();
                spin_unlock_irqrestore(lock, flags);
                return ecx;
            }
        });
        rcu_read_unlock();
        spin_unlock_irqrestore(lock, flags);
        try_ += 1;
    }
    ecx
}

unsafe fn avc_alloc_node() -> *mut avc_node {
    let mut node: *mut avc_node;

    node = kmem_cache_zalloc(avc_node_cachep, GFP_NOWAIT) as *mut avc_node;
    if node.is_null() {
        return ptr::null_mut();
    }

    INIT_HLIST_NODE(&mut (*node).list);
    avc_cache_stats_incr!(allocations);

    if atomic_inc_return(&mut selinux_avc.avc_cache.active_nodes)
        > selinux_avc.avc_cache_threshold as c_int
    {
        avc_reclaim_node();
    }

    node
}

unsafe fn avc_node_populate(
    node: *mut avc_node,
    ssid: u32,
    tsid: u32,
    tclass: u16,
    avd: *mut av_decision,
) {
    (*node).ae.ssid = ssid;
    (*node).ae.tsid = tsid;
    (*node).ae.tclass = tclass;
    memcpy(
        &mut (*node).ae.avd as *mut av_decision as *mut c_void,
        avd as *const c_void,
        size_of::<av_decision>(),
    );
}

#[inline]
unsafe fn avc_search_node(ssid: u32, tsid: u32, tclass: u16) -> *mut avc_node {
    let mut node: *mut avc_node;
    let mut ret: *mut avc_node = ptr::null_mut();
    let mut hvalue: u32;
    let mut head: *mut hlist_head;

    hvalue = avc_hash(ssid, tsid, tclass);
    head = &mut selinux_avc.avc_cache.slots[hvalue as usize];
    hlist_for_each_entry_rcu!(node, head, list, {
        if ssid == (*node).ae.ssid && tclass == (*node).ae.tclass && tsid == (*node).ae.tsid {
            ret = node;
            break;
        }
    });

    ret
}

/**
 * avc_lookup - Look up an AVC entry.
 * @ssid: source security identifier
 * @tsid: target security identifier
 * @tclass: target security class
 *
 * Look up an AVC entry that is valid for the
 * (@ssid, @tsid), interpreting the permissions
 * based on @tclass.  If a valid AVC entry exists,
 * then this function returns the avc_node.
 * Otherwise, this function returns NULL.
 */
unsafe fn avc_lookup(ssid: u32, tsid: u32, tclass: u16) -> *mut avc_node {
    let mut node: *mut avc_node;

    avc_cache_stats_incr!(lookups);
    node = avc_search_node(ssid, tsid, tclass);

    if !node.is_null() {
        return node;
    }

    avc_cache_stats_incr!(misses);
    ptr::null_mut()
}

unsafe fn avc_latest_notif_update(seqno: u32, is_insert: c_int) -> c_int {
    let mut ret: c_int = 0;
    static mut NOTIF_LOCK: spinlock_t = unsafe { core::mem::zeroed() };
    let mut flag: c_ulong = 0;

    spin_lock_irqsave(&mut NOTIF_LOCK, &mut flag);
    if is_insert != 0 {
        if seqno < selinux_avc.avc_cache.latest_notif {
            pr_warn(
                c_str!("SELinux: avc:  seqno %d < latest_notif %d\n"),
                seqno,
                selinux_avc.avc_cache.latest_notif,
            );
            ret = -EAGAIN;
        }
    } else if seqno > selinux_avc.avc_cache.latest_notif {
        selinux_avc.avc_cache.latest_notif = seqno;
    }
    spin_unlock_irqrestore(&mut NOTIF_LOCK, flag);

    ret
}

/**
 * avc_insert - Insert an AVC entry.
 * @ssid: source security identifier
 * @tsid: target security identifier
 * @tclass: target security class
 * @avd: resulting av decision
 * @xp_node: resulting extended permissions
 *
 * Insert an AVC entry for the SID pair
 * (@ssid, @tsid) and class @tclass.
 * The access vectors and the sequence number are
 * normally provided by the security server in
 * response to a security_compute_av() call.  If the
 * sequence number @avd->seqno is not less than the latest
 * revocation notification, then the function copies
 * the access vectors into a cache entry.
 */
unsafe fn avc_insert(
    ssid: u32,
    tsid: u32,
    tclass: u16,
    avd: *mut av_decision,
    xp_node: *mut avc_xperms_node,
) {
    let mut pos: *mut avc_node;
    let mut node: *mut avc_node = ptr::null_mut();
    let mut hvalue: u32;
    let mut flag: c_ulong = 0;
    let mut lock: *mut spinlock_t;
    let mut head: *mut hlist_head;

    if avc_latest_notif_update((*avd).seqno, 1) != 0 {
        return;
    }

    node = avc_alloc_node();
    if node.is_null() {
        return;
    }

    avc_node_populate(node, ssid, tsid, tclass, avd);
    if avc_xperms_populate(node, xp_node) != 0 {
        avc_node_kill(node);
        return;
    }

    hvalue = avc_hash(ssid, tsid, tclass);
    head = &mut selinux_avc.avc_cache.slots[hvalue as usize];
    lock = &mut selinux_avc.avc_cache.slots_lock[hvalue as usize];
    spin_lock_irqsave(lock, &mut flag);
    hlist_for_each_entry!(pos, head, list, {
        if (*pos).ae.ssid == ssid && (*pos).ae.tsid == tsid && (*pos).ae.tclass == tclass {
            avc_node_replace(node, pos);
            spin_unlock_irqrestore(lock, flag);
            return;
        }
    });
    hlist_add_head_rcu(&mut (*node).list, head);
    spin_unlock_irqrestore(lock, flag);
}

/**
 * avc_audit_pre_callback - SELinux specific information
 * will be called by generic audit code
 * @ab: the audit buffer
 * @a: audit_data
 */
unsafe extern "C" fn avc_audit_pre_callback(ab: *mut audit_buffer, a: *mut c_void) {
    let mut ad: *mut common_audit_data = a as *mut common_audit_data;
    let mut sad: *mut selinux_audit_data = (*ad).selinux_audit_data;
    let mut av: u32 = (*sad).audited;
    let mut perm: u32;
    let mut perms: *const *const c_char;
    let mut i: u32;

    audit_log_format(
        ab,
        c_str!("avc:  %s "),
        if (*sad).denied != 0 {
            c_str!("denied")
        } else {
            c_str!("granted")
        },
    );

    if av == 0 {
        audit_log_format(ab, c_str!(" null"));
        return;
    }

    perms = secclass_map[((*sad).tclass - 1) as usize].perms;

    audit_log_format(ab, c_str!(" {"));
    i = 0;
    perm = 1;
    while i < (size_of::<u32>() * 8) as u32 {
        if (perm & av) != 0 && !(*perms.add(i as usize)).is_null() {
            audit_log_format(ab, c_str!(" %s"), *perms.add(i as usize));
            av &= !perm;
        }
        i += 1;
        perm <<= 1;
    }

    if av != 0 {
        audit_log_format(ab, c_str!(" 0x%x"), av);
    }

    audit_log_format(ab, c_str!(" } for "));
}

/**
 * avc_audit_post_callback - SELinux specific information
 * will be called by generic audit code
 * @ab: the audit buffer
 * @a: audit_data
 */
unsafe extern "C" fn avc_audit_post_callback(ab: *mut audit_buffer, a: *mut c_void) {
    let mut ad: *mut common_audit_data = a as *mut common_audit_data;
    let mut sad: *mut selinux_audit_data = (*ad).selinux_audit_data;
    let mut scontext: *mut c_char = ptr::null_mut();
    let mut tcontext: *mut c_char = ptr::null_mut();
    let mut tclass: *const c_char = ptr::null();
    let mut scontext_len: u32 = 0;
    let mut tcontext_len: u32 = 0;
    let mut rc: c_int;

    rc = security_sid_to_context((*sad).ssid, &mut scontext, &mut scontext_len);
    if rc != 0 {
        audit_log_format(ab, c_str!(" ssid=%d"), (*sad).ssid);
    } else {
        audit_log_format(ab, c_str!(" scontext=%s"), scontext);
    }

    rc = security_sid_to_context((*sad).tsid, &mut tcontext, &mut tcontext_len);
    if rc != 0 {
        audit_log_format(ab, c_str!(" tsid=%d"), (*sad).tsid);
    } else {
        audit_log_format(ab, c_str!(" tcontext=%s"), tcontext);
    }

    tclass = secclass_map[((*sad).tclass - 1) as usize].name;
    audit_log_format(ab, c_str!(" tclass=%s"), tclass);

    if (*sad).denied != 0 {
        audit_log_format(ab, c_str!(" permissive=%u"), if (*sad).result != 0 { 0 } else { 1 });
    }

    trace_selinux_audited(sad, scontext, tcontext, tclass);
    kfree(tcontext as *mut c_void);
    kfree(scontext as *mut c_void);

    /* in case of invalid context report also the actual context string */
    rc = security_sid_to_context_inval((*sad).ssid, &mut scontext, &mut scontext_len);
    if rc == 0 && !scontext.is_null() {
        if scontext_len != 0 && *scontext.add((scontext_len - 1) as usize) == 0 {
            scontext_len -= 1;
        }
        audit_log_format(ab, c_str!(" srawcon="));
        audit_log_n_untrustedstring(ab, scontext, scontext_len);
        kfree(scontext as *mut c_void);
    }

    rc = security_sid_to_context_inval((*sad).tsid, &mut scontext, &mut scontext_len);
    if rc == 0 && !scontext.is_null() {
        if scontext_len != 0 && *scontext.add((scontext_len - 1) as usize) == 0 {
            scontext_len -= 1;
        }
        audit_log_format(ab, c_str!(" trawcon="));
        audit_log_n_untrustedstring(ab, scontext, scontext_len);
        kfree(scontext as *mut c_void);
    }
}

/*
 * This is the slow part of avc audit with big stack footprint.
 * Note that it is non-blocking and can be called from under
 * rcu_read_lock().
 */
#[no_mangle]
pub unsafe extern "C" fn slow_avc_audit(
    ssid: u32,
    tsid: u32,
    tclass: u16,
    requested: u32,
    audited: u32,
    denied: u32,
    result: c_int,
    mut a: *mut common_audit_data,
) -> c_int {
    let mut stack_data: common_audit_data = core::mem::zeroed();
    let mut sad: selinux_audit_data = core::mem::zeroed();

    if WARN_ON(tclass == 0 || tclass as usize >= ARRAY_SIZE_SECCLASS_MAP()) {
        return -EINVAL;
    }

    if a.is_null() {
        a = &mut stack_data;
        (*a).type_ = LSM_AUDIT_DATA_NONE;
    }

    sad.tclass = tclass;
    sad.requested = requested;
    sad.ssid = ssid;
    sad.tsid = tsid;
    sad.audited = audited;
    sad.denied = denied;
    sad.result = result;

    (*a).selinux_audit_data = &mut sad;

    common_lsm_audit(a, Some(avc_audit_pre_callback), Some(avc_audit_post_callback));
    0
}

/**
 * avc_add_callback - Register a callback for security events.
 * @callback: callback function
 * @events: security events
 *
 * Register a callback function for events in the set @events.
 * Returns %0 on success or -%ENOMEM if insufficient memory
 * exists to add the callback.
 */
#[no_mangle]
pub unsafe extern "C" fn avc_add_callback(
    callback: Option<unsafe extern "C" fn(event: u32) -> c_int>,
    events: u32,
) -> c_int {
    let mut c: *mut avc_callback_node;
    let mut rc: c_int = 0;

    c = kmalloc_obj_avc_callback_node() as *mut avc_callback_node;
    if c.is_null() {
        rc = -ENOMEM;
        return rc;
    }

    (*c).callback = callback;
    (*c).events = events;
    (*c).next = avc_callbacks;
    avc_callbacks = c;
    rc
}

/**
 * avc_update_node - Update an AVC entry
 * @event : Updating event
 * @perms : Permission mask bits
 * @driver: xperm driver information
 * @base_perm: the base permission associated with the extended permission
 * @xperm: xperm permissions
 * @ssid: AVC entry source sid
 * @tsid: AVC entry target sid
 * @tclass : AVC entry target object class
 * @seqno : sequence number when decision was made
 * @xpd: extended_perms_decision to be added to the node
 * @flags: the AVC_* flags, e.g. AVC_EXTENDED_PERMS, or 0.
 *
 * if a valid AVC entry doesn't exist,this function returns -ENOENT.
 * if kmalloc() called internal returns NULL, this function returns -ENOMEM.
 * otherwise, this function updates the AVC entry. The original AVC-entry object
 * will release later by RCU.
 */
unsafe fn avc_update_node(
    event: u32,
    perms: u32,
    driver: u8,
    base_perm: u8,
    xperm: u8,
    ssid: u32,
    tsid: u32,
    tclass: u16,
    seqno: u32,
    xpd: *mut extended_perms_decision,
    flags: u32,
) -> c_int {
    let mut hvalue: u32;
    let mut rc: c_int = 0;
    let mut flag: c_ulong = 0;
    let mut pos: *mut avc_node;
    let mut node: *mut avc_node;
    let mut orig: *mut avc_node = ptr::null_mut();
    let mut head: *mut hlist_head;
    let mut lock: *mut spinlock_t;

    node = avc_alloc_node();
    if node.is_null() {
        rc = -ENOMEM;
        return rc;
    }

    /* Lock the target slot */
    hvalue = avc_hash(ssid, tsid, tclass);

    head = &mut selinux_avc.avc_cache.slots[hvalue as usize];
    lock = &mut selinux_avc.avc_cache.slots_lock[hvalue as usize];

    spin_lock_irqsave(lock, &mut flag);

    hlist_for_each_entry!(pos, head, list, {
        if ssid == (*pos).ae.ssid
            && tsid == (*pos).ae.tsid
            && tclass == (*pos).ae.tclass
            && seqno == (*pos).ae.avd.seqno
        {
            orig = pos;
            break;
        }
    });

    if orig.is_null() {
        rc = -ENOENT;
        avc_node_kill(node);
        spin_unlock_irqrestore(lock, flag);
        return rc;
    }

    /*
     * Copy and replace original node.
     */

    avc_node_populate(node, ssid, tsid, tclass, &mut (*orig).ae.avd);

    if !(*orig).ae.xp_node.is_null() {
        rc = avc_xperms_populate(node, (*orig).ae.xp_node);
        if rc != 0 {
            avc_node_kill(node);
            spin_unlock_irqrestore(lock, flag);
            return rc;
        }
    }

    match event {
        AVC_CALLBACK_GRANT => {
            (*node).ae.avd.allowed |= perms;
            if !(*node).ae.xp_node.is_null() && (flags & AVC_EXTENDED_PERMS) != 0 {
                avc_xperms_allow_perm((*node).ae.xp_node, driver, base_perm, xperm);
            }
        }
        AVC_CALLBACK_TRY_REVOKE | AVC_CALLBACK_REVOKE => {
            (*node).ae.avd.allowed &= !perms;
        }
        AVC_CALLBACK_AUDITALLOW_ENABLE => {
            (*node).ae.avd.auditallow |= perms;
        }
        AVC_CALLBACK_AUDITALLOW_DISABLE => {
            (*node).ae.avd.auditallow &= !perms;
        }
        AVC_CALLBACK_AUDITDENY_ENABLE => {
            (*node).ae.avd.auditdeny |= perms;
        }
        AVC_CALLBACK_AUDITDENY_DISABLE => {
            (*node).ae.avd.auditdeny &= !perms;
        }
        AVC_CALLBACK_ADD_XPERMS => {
            rc = avc_add_xperms_decision(node, xpd);
            if rc != 0 {
                avc_node_kill(node);
                spin_unlock_irqrestore(lock, flag);
                return rc;
            }
        }
        _ => {}
    }
    avc_node_replace(node, orig);
    spin_unlock_irqrestore(lock, flag);
    rc
}

/**
 * avc_flush - Flush the cache
 */
unsafe fn avc_flush() {
    let mut head: *mut hlist_head;
    let mut node: *mut avc_node;
    let mut lock: *mut spinlock_t;
    let mut flag: c_ulong = 0;
    let mut i: c_int;

    i = 0;
    while i < AVC_CACHE_SLOTS as c_int {
        head = &mut selinux_avc.avc_cache.slots[i as usize];
        lock = &mut selinux_avc.avc_cache.slots_lock[i as usize];

        spin_lock_irqsave(lock, &mut flag);
        /*
         * With preemptible RCU, the outer spinlock does not
         * prevent RCU grace periods from ending.
         */
        rcu_read_lock();
        hlist_for_each_entry!(node, head, list, {
            avc_node_delete(node);
        });
        rcu_read_unlock();
        spin_unlock_irqrestore(lock, flag);
        i += 1;
    }
}

/**
 * avc_ss_reset - Flush the cache and revalidate migrated permissions.
 * @seqno: policy sequence number
 */
#[no_mangle]
pub unsafe extern "C" fn avc_ss_reset(seqno: u32) -> c_int {
    let mut c: *mut avc_callback_node;
    let mut rc: c_int = 0;
    let mut tmprc: c_int;

    avc_flush();

    c = avc_callbacks;
    while !c.is_null() {
        if ((*c).events & AVC_CALLBACK_RESET) != 0 {
            tmprc = ((*c).callback.unwrap())(AVC_CALLBACK_RESET);
            /* save the first error encountered for the return
               value and continue processing the callbacks */
            if rc == 0 {
                rc = tmprc;
            }
        }
        c = (*c).next;
    }

    avc_latest_notif_update(seqno, 0);
    rc
}

/**
 * avc_compute_av - Add an entry to the AVC based on the security policy
 * @ssid: subject
 * @tsid: object/target
 * @tclass: object class
 * @avd: access vector decision
 * @xp_node: AVC extended permissions node
 *
 * Slow-path helper function for avc_has_perm_noaudit, when the avc_node lookup
 * fails.  Don't inline this, since it's the slow-path and just results in a
 * bigger stack frame.
 */
unsafe fn avc_compute_av(
    ssid: u32,
    tsid: u32,
    tclass: u16,
    avd: *mut av_decision,
    xp_node: *mut avc_xperms_node,
) {
    INIT_LIST_HEAD(&mut (*xp_node).xpd_head);
    security_compute_av(ssid, tsid, tclass, avd, &mut (*xp_node).xp);
    avc_insert(ssid, tsid, tclass, avd, xp_node);
}

unsafe fn avc_denied(
    ssid: u32,
    tsid: u32,
    tclass: u16,
    requested: u32,
    driver: u8,
    base_perm: u8,
    xperm: u8,
    flags: c_uint,
    avd: *mut av_decision,
) -> c_int {
    if (flags & AVC_STRICT) != 0 {
        return -EACCES;
    }

    if enforcing_enabled() != 0 && ((*avd).flags & AVD_FLAGS_PERMISSIVE) == 0 {
        return -EACCES;
    }

    avc_update_node(
        AVC_CALLBACK_GRANT,
        requested,
        driver,
        base_perm,
        xperm,
        ssid,
        tsid,
        tclass,
        (*avd).seqno,
        ptr::null_mut(),
        flags,
    );
    0
}

/*
 * The avc extended permissions logic adds an additional 256 bits of
 * permissions to an avc node when extended permissions for that node are
 * specified in the avtab. If the additional 256 permissions is not adequate,
 * as-is the case with ioctls, then multiple may be chained together and the
 * driver field is used to specify which set contains the permission.
 */
#[no_mangle]
pub unsafe extern "C" fn avc_has_extended_perms(
    ssid: u32,
    tsid: u32,
    tclass: u16,
    requested: u32,
    driver: u8,
    base_perm: u8,
    xperm: u8,
    ad: *mut common_audit_data,
) -> c_int {
    let mut node: *mut avc_node;
    let mut avd: av_decision = core::mem::zeroed();
    let mut denied: u32;
    let mut local_xpd: extended_perms_decision = core::mem::zeroed();
    let mut xpd: *mut extended_perms_decision = ptr::null_mut();
    let mut allowed: extended_perms_data = core::mem::zeroed();
    let mut auditallow: extended_perms_data = core::mem::zeroed();
    let mut dontaudit: extended_perms_data = core::mem::zeroed();
    let mut local_xp_node: avc_xperms_node = core::mem::zeroed();
    let mut xp_node: *mut avc_xperms_node;
    let mut rc: c_int = 0;
    let mut rc2: c_int;

    xp_node = &mut local_xp_node;
    if WARN_ON(requested == 0) {
        return -EACCES;
    }

    rcu_read_lock();

    node = avc_lookup(ssid, tsid, tclass);
    if unlikely(node.is_null()) {
        avc_compute_av(ssid, tsid, tclass, &mut avd, xp_node);
    } else {
        memcpy(
            &mut avd as *mut av_decision as *mut c_void,
            &mut (*node).ae.avd as *mut av_decision as *const c_void,
            size_of::<av_decision>(),
        );
        xp_node = (*node).ae.xp_node;
    }
    /* if extended permissions are not defined, only consider av_decision */
    if xp_node.is_null() || (*xp_node).xp.len == 0 {
        denied = requested & !avd.allowed;
        if unlikely(denied != 0) {
            rc = avc_denied(
                ssid,
                tsid,
                tclass,
                requested,
                driver,
                base_perm,
                xperm,
                AVC_EXTENDED_PERMS,
                &mut avd,
            );
        }
        rcu_read_unlock();
        rc2 = avc_xperms_audit(ssid, tsid, tclass, requested, &mut avd, xpd, xperm, rc, ad);
        if rc2 != 0 {
            return rc2;
        }
        return rc;
    }

    local_xpd.allowed = &mut allowed;
    local_xpd.auditallow = &mut auditallow;
    local_xpd.dontaudit = &mut dontaudit;

    xpd = avc_xperms_decision_lookup(driver, base_perm, xp_node);
    if unlikely(xpd.is_null()) {
        /*
         * Compute the extended_perms_decision only if the driver
         * is flagged and the base permission is known.
         */
        if security_xperm_test((*xp_node).xp.drivers.p.as_mut_ptr(), driver) == 0
            || ((*xp_node).xp.base_perms & base_perm) == 0
        {
            avd.allowed &= !requested;
        } else {
            rcu_read_unlock();
            security_compute_xperms_decision(
                ssid,
                tsid,
                tclass,
                driver,
                base_perm,
                &mut local_xpd,
            );
            rcu_read_lock();
            avc_update_node(
                AVC_CALLBACK_ADD_XPERMS,
                requested,
                driver,
                base_perm,
                xperm,
                ssid,
                tsid,
                tclass,
                avd.seqno,
                &mut local_xpd,
                0,
            );
        }
    } else {
        avc_quick_copy_xperms_decision(xperm, &mut local_xpd, xpd);
    }
    xpd = &mut local_xpd;

    if avc_xperms_has_perm(xpd, xperm, XPERMS_ALLOWED) == 0 {
        avd.allowed &= !requested;
    }

    denied = requested & !avd.allowed;
    if unlikely(denied != 0) {
        rc = avc_denied(
            ssid,
            tsid,
            tclass,
            requested,
            driver,
            base_perm,
            xperm,
            AVC_EXTENDED_PERMS,
            &mut avd,
        );
    }

    rcu_read_unlock();

    rc2 = avc_xperms_audit(ssid, tsid, tclass, requested, &mut avd, xpd, xperm, rc, ad);
    if rc2 != 0 {
        return rc2;
    }
    rc
}

/**
 * avc_perm_nonode - Add an entry to the AVC
 * @ssid: subject
 * @tsid: object/target
 * @tclass: object class
 * @requested: requested permissions
 * @flags: AVC flags
 * @avd: access vector decision
 *
 * This is the "we have no node" part of avc_has_perm_noaudit(), which is
 * unlikely and needs extra stack space for the new node that we generate, so
 * don't inline it.
 */
unsafe fn avc_perm_nonode(
    ssid: u32,
    tsid: u32,
    tclass: u16,
    requested: u32,
    flags: c_uint,
    avd: *mut av_decision,
) -> c_int {
    let mut denied: u32;
    let mut xp_node: avc_xperms_node = core::mem::zeroed();

    avc_compute_av(ssid, tsid, tclass, avd, &mut xp_node);
    denied = requested & !(*avd).allowed;
    if unlikely(denied != 0) {
        return avc_denied(ssid, tsid, tclass, requested, 0, 0, 0, flags, avd);
    }
    0
}

/**
 * avc_has_perm_noaudit - Check permissions but perform no auditing.
 * @ssid: source security identifier
 * @tsid: target security identifier
 * @tclass: target security class
 * @requested: requested permissions, interpreted based on @tclass
 * @flags:  AVC_STRICT or 0
 * @avd: access vector decisions
 *
 * Check the AVC to determine whether the @requested permissions are granted
 * for the SID pair (@ssid, @tsid), interpreting the permissions
 * based on @tclass, and call the security server on a cache miss to obtain
 * a new decision and add it to the cache.  Return a copy of the decisions
 * in @avd.  Return %0 if all @requested permissions are granted,
 * -%EACCES if any permissions are denied, or another -errno upon
 * other errors.  This function is typically called by avc_has_perm(),
 * but may also be called directly to separate permission checking from
 * auditing, e.g. in cases where a lock must be held for the check but
 * should be released for the auditing.
 */
#[no_mangle]
pub unsafe extern "C" fn avc_has_perm_noaudit(
    ssid: u32,
    tsid: u32,
    tclass: u16,
    requested: u32,
    flags: c_uint,
    avd: *mut av_decision,
) -> c_int {
    let mut denied: u32;
    let mut node: *mut avc_node;

    if WARN_ON(requested == 0) {
        return -EACCES;
    }

    rcu_read_lock();
    node = avc_lookup(ssid, tsid, tclass);
    if unlikely(node.is_null()) {
        rcu_read_unlock();
        return avc_perm_nonode(ssid, tsid, tclass, requested, flags, avd);
    }
    denied = requested & !(*node).ae.avd.allowed;
    memcpy(
        avd as *mut c_void,
        &mut (*node).ae.avd as *mut av_decision as *const c_void,
        size_of::<av_decision>(),
    );
    rcu_read_unlock();

    if unlikely(denied != 0) {
        return avc_denied(ssid, tsid, tclass, requested, 0, 0, 0, flags, avd);
    }
    0
}

/**
 * avc_has_perm - Check permissions and perform any appropriate auditing.
 * @ssid: source security identifier
 * @tsid: target security identifier
 * @tclass: target security class
 * @requested: requested permissions, interpreted based on @tclass
 * @auditdata: auxiliary audit data
 *
 * Check the AVC to determine whether the @requested permissions are granted
 * for the SID pair (@ssid, @tsid), interpreting the permissions
 * based on @tclass, and call the security server on a cache miss to obtain
 * a new decision and add it to the cache.  Audit the granting or denial of
 * permissions in accordance with the policy.  Return %0 if all @requested
 * permissions are granted, -%EACCES if any permissions are denied, or
 * another -errno upon other errors.
 */
#[no_mangle]
pub unsafe extern "C" fn avc_has_perm(
    ssid: u32,
    tsid: u32,
    tclass: u16,
    requested: u32,
    auditdata: *mut common_audit_data,
) -> c_int {
    let mut avd: av_decision = core::mem::zeroed();
    let mut rc: c_int;
    let mut rc2: c_int;

    rc = avc_has_perm_noaudit(ssid, tsid, tclass, requested, 0, &mut avd);

    rc2 = avc_audit(ssid, tsid, tclass, requested, &mut avd, rc, auditdata);
    if rc2 != 0 {
        return rc2;
    }
    rc
}

#[no_mangle]
pub unsafe extern "C" fn avc_policy_seqno() -> u32 {
    selinux_avc.avc_cache.latest_notif
}

extern "C" {
    static CONFIG_SECURITY_SELINUX_AVC_HASH_BITS: usize;
    static PAGE_SIZE: usize;
    static SLAB_PANIC: c_uint;
    static GFP_NOWAIT: c_uint;
    static ENOMEM: c_int;
    static EAGAIN: c_int;
    static ENOENT: c_int;
    static EACCES: c_int;
    static EINVAL: c_int;
    static XPERMS_ALLOWED: u8;
    static XPERMS_AUDITALLOW: u8;
    static XPERMS_DONTAUDIT: u8;
    static AVC_EXTENDED_PERMS: c_uint;
    static AVC_STRICT: c_uint;
    static AVD_FLAGS_PERMISSIVE: u32;
    static AVC_CALLBACK_GRANT: u32;
    static AVC_CALLBACK_TRY_REVOKE: u32;
    static AVC_CALLBACK_REVOKE: u32;
    static AVC_CALLBACK_AUDITALLOW_ENABLE: u32;
    static AVC_CALLBACK_AUDITALLOW_DISABLE: u32;
    static AVC_CALLBACK_AUDITDENY_ENABLE: u32;
    static AVC_CALLBACK_AUDITDENY_DISABLE: u32;
    static AVC_CALLBACK_ADD_XPERMS: u32;
    static AVC_CALLBACK_RESET: u32;
    static mut secclass_map: [security_class_mapping; 0];

    fn av_hash(ssid: u32, tsid: u32, tclass: u32, mask: u32) -> u32;
    fn INIT_HLIST_HEAD(head: *mut hlist_head);
    fn INIT_HLIST_NODE(node: *mut hlist_node);
    fn INIT_LIST_HEAD(head: *mut list_head);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn atomic_set(v: *mut atomic_t, i: c_int);
    fn atomic_read(v: *mut atomic_t) -> c_int;
    fn atomic_inc_return(v: *mut atomic_t) -> c_int;
    fn atomic_dec(v: *mut atomic_t);
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn hlist_empty(head: *mut hlist_head) -> bool;
    fn hlist_del_rcu(node: *mut hlist_node);
    fn hlist_replace_rcu(old: *mut hlist_node, new: *mut hlist_node);
    fn hlist_add_head_rcu(node: *mut hlist_node, head: *mut hlist_head);
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn call_rcu(head: *mut rcu_head, func: Option<unsafe extern "C" fn(*mut rcu_head)>);
    fn spin_trylock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong) -> bool;
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn kmem_cache_free(cachep: *mut kmem_cache, objp: *mut c_void);
    fn kmem_cache_zalloc(cachep: *mut kmem_cache, flags: c_uint) -> *mut c_void;
    fn KMEM_CACHE_AVC_NODE(flags: c_uint) -> *mut kmem_cache;
    fn KMEM_CACHE_AVC_XPERMS_NODE(flags: c_uint) -> *mut kmem_cache;
    fn KMEM_CACHE_AVC_XPERMS_DECISION_NODE(flags: c_uint) -> *mut kmem_cache;
    fn KMEM_CACHE_EXTENDED_PERMS_DATA(flags: c_uint) -> *mut kmem_cache;
    fn kmalloc_obj_avc_callback_node() -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn pr_warn(fmt: *const c_char, ...);
    fn audit_log_format(ab: *mut audit_buffer, fmt: *const c_char, ...);
    fn audit_log_n_untrustedstring(ab: *mut audit_buffer, string: *const c_char, n: u32);
    fn security_xperm_test(p: *mut u32, perm: u8) -> c_uint;
    fn security_xperm_set(p: *mut u32, perm: u8);
    fn security_sid_to_context(sid: u32, scontext: *mut *mut c_char, scontext_len: *mut u32)
        -> c_int;
    fn security_sid_to_context_inval(
        sid: u32,
        scontext: *mut *mut c_char,
        scontext_len: *mut u32,
    ) -> c_int;
    fn security_compute_av(
        ssid: u32,
        tsid: u32,
        tclass: u16,
        avd: *mut av_decision,
        xperms: *mut extended_perms,
    );
    fn security_compute_xperms_decision(
        ssid: u32,
        tsid: u32,
        tclass: u16,
        driver: u8,
        base_perm: u8,
        xpd: *mut extended_perms_decision,
    );
    fn enforcing_enabled() -> c_int;
    fn common_lsm_audit(
        a: *mut common_audit_data,
        pre: Option<unsafe extern "C" fn(*mut audit_buffer, *mut c_void)>,
        post: Option<unsafe extern "C" fn(*mut audit_buffer, *mut c_void)>,
    );
    fn avc_audit(
        ssid: u32,
        tsid: u32,
        tclass: u16,
        requested: u32,
        avd: *mut av_decision,
        result: c_int,
        auditdata: *mut common_audit_data,
    ) -> c_int;
    fn trace_selinux_audited(
        sad: *mut selinux_audit_data,
        scontext: *mut c_char,
        tcontext: *mut c_char,
        tclass: *const c_char,
    );
}

extern "Rust" {
    fn likely(v: bool) -> bool;
    fn unlikely(v: bool) -> bool;
    fn WARN_ON(v: bool) -> bool;
    fn ARRAY_SIZE_SECCLASS_MAP() -> usize;
    fn size_of_val<T: ?Sized>(val: &T) -> usize;
}

#[repr(C)]
pub struct hlist_node {
    _private: [u8; 0],
}
#[repr(C)]
pub struct hlist_head {
    _private: [u8; 0],
}
#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}
#[repr(C)]
pub struct rcu_head {
    _private: [u8; 0],
}
#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct atomic_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct kmem_cache {
    _private: [u8; 0],
}
#[repr(C)]
pub struct audit_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct av_decision {
    allowed: u32,
    auditallow: u32,
    auditdeny: u32,
    seqno: u32,
    flags: u32,
}

#[repr(C)]
pub struct extended_perms_data {
    p: [u32; 8],
}

#[repr(C)]
pub struct extended_perms_bitmap {
    p: [u32; 8],
}

#[repr(C)]
pub struct extended_perms {
    drivers: extended_perms_bitmap,
    len: u8,
    base_perms: u8,
}

#[repr(C)]
pub struct extended_perms_decision {
    allowed: *mut extended_perms_data,
    auditallow: *mut extended_perms_data,
    dontaudit: *mut extended_perms_data,
    base_perm: u8,
    driver: u8,
    used: u8,
}

#[repr(C)]
pub struct common_audit_data {
    type_: c_int,
    selinux_audit_data: *mut selinux_audit_data,
}

#[repr(C)]
pub struct selinux_audit_data {
    tclass: u16,
    requested: u32,
    ssid: u32,
    tsid: u32,
    audited: u32,
    denied: u32,
    result: c_int,
}

#[repr(C)]
pub struct security_class_mapping {
    name: *const c_char,
    perms: *const *const c_char,
}

extern "Rust" {
    fn c_str(s: &'static str) -> *const c_char;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
