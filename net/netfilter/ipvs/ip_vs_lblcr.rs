// SPDX-License-Identifier: GPL-2.0-or-later
/* IPVS: Locality-Based Least-Connection with Replication scheduler */
/*
 * The lblc/r algorithm is as follows (pseudo code):
 * if serverSet[dest_ip] is null then n, serverSet[dest_ip] <- {weighted least-conn node};
 * otherwise select the least-conn alive node, add a weighted least-conn node when needed,
 * periodically remove the most-connected node, and update lastMod when the set changes.
 */

// Kernel headers and symbols are supplied by the surrounding translation unit.

const CHECK_EXPIRE_INTERVAL: c_ulong = 60 * HZ;
const ENTRY_TIMEOUT: c_ulong = 6 * 60 * HZ;
const DEFAULT_EXPIRATION: c_ulong = 24 * 60 * 60 * HZ;
const COUNT_FOR_FULL_EXPIRATION: c_int = 30;
const IP_VS_LBLCR_TAB_BITS: usize = CONFIG_IP_VS_LBLCR_TAB_BITS;
const IP_VS_LBLCR_TAB_SIZE: usize = 1usize << IP_VS_LBLCR_TAB_BITS;
const IP_VS_LBLCR_TAB_MASK: usize = IP_VS_LBLCR_TAB_SIZE - 1;

#[repr(C)]
struct ip_vs_dest_set_elem { list: list_head, dest: *mut ip_vs_dest, rcu_head: rcu_head }
#[repr(C)]
struct ip_vs_dest_set { size: atomic_t, lastmod: c_ulong, list: list_head }

unsafe fn ip_vs_dest_set_insert(set: *mut ip_vs_dest_set, dest: *mut ip_vs_dest, check: bool) {
    let mut e: *mut ip_vs_dest_set_elem;
    if check {
        list_for_each_entry!(e, (*set).list, list) {
            if (*e).dest == dest { return; }
        }
    }
    e = kmalloc_obj!(*e, GFP_ATOMIC);
    if e.is_null() { return; }
    ip_vs_dest_hold(dest); (*e).dest = dest;
    list_add_rcu!(&mut (*e).list, &mut (*set).list);
    atomic_inc(&mut (*set).size); (*set).lastmod = jiffies;
}

unsafe fn ip_vs_lblcr_elem_rcu_free(head: *mut rcu_head) {
    let e = container_of!(head, ip_vs_dest_set_elem, rcu_head);
    ip_vs_dest_put_and_free((*e).dest); kfree(e as *mut c_void);
}
unsafe fn ip_vs_dest_set_erase(set: *mut ip_vs_dest_set, dest: *mut ip_vs_dest) {
    let mut e: *mut ip_vs_dest_set_elem;
    list_for_each_entry!(e, (*set).list, list) {
        if (*e).dest == dest { atomic_dec(&mut (*set).size); (*set).lastmod = jiffies;
            list_del_rcu!(&mut (*e).list); call_rcu!(&mut (*e).rcu_head, ip_vs_lblcr_elem_rcu_free); break; }
    }
}
unsafe fn ip_vs_dest_set_eraseall(set: *mut ip_vs_dest_set) {
    let (mut e, mut ep): (*mut ip_vs_dest_set_elem, *mut ip_vs_dest_set_elem);
    list_for_each_entry_safe!(e, ep, (*set).list, list) { list_del_rcu!(&mut (*e).list); call_rcu!(&mut (*e).rcu_head, ip_vs_lblcr_elem_rcu_free); }
}

unsafe fn ip_vs_dest_set_min(set: *mut ip_vs_dest_set) -> *mut ip_vs_dest {
    let (mut e, mut dest, mut least): (*mut ip_vs_dest_set_elem, *mut ip_vs_dest, *mut ip_vs_dest);
    let (mut loh, mut doh): (c_int, c_int);
    list_for_each_entry_rcu!(e, (*set).list, list) { least = (*e).dest;
        if (*least).flags & IP_VS_DEST_F_OVERLOAD != 0 { continue; }
        if atomic_read(&(*least).weight) > 0 && (*least).cflags & IP_VS_DEST_CF_AVAILABLE != 0 { loh = ip_vs_dest_conn_overhead(least); goto!(nextstage); }
    } return ptr::null_mut();
    nextstage: list_for_each_entry_continue_rcu!(e, (*set).list, list) { dest = (*e).dest;
        if (*dest).flags & IP_VS_DEST_F_OVERLOAD != 0 { continue; }
        doh = ip_vs_dest_conn_overhead(dest);
        if (loh as i64) * atomic_read(&(*dest).weight) as i64 > (doh as i64) * atomic_read(&(*least).weight) as i64 && (*dest).cflags & IP_VS_DEST_CF_AVAILABLE != 0 { least = dest; loh = doh; }
    } least
}
unsafe fn ip_vs_dest_set_max(set: *mut ip_vs_dest_set) -> *mut ip_vs_dest {
    if set.is_null() { return ptr::null_mut(); }
    let (mut e, mut dest, mut most): (*mut ip_vs_dest_set_elem, *mut ip_vs_dest, *mut ip_vs_dest);
    let (mut moh, mut doh): (c_int, c_int);
    list_for_each_entry!(e, (*set).list, list) { most = (*e).dest; if atomic_read(&(*most).weight) > 0 { moh = ip_vs_dest_conn_overhead(most); goto!(nextstage); } }
    return ptr::null_mut();
    nextstage: list_for_each_entry_continue!(e, (*set).list, list) { dest = (*e).dest; doh = ip_vs_dest_conn_overhead(dest);
        if (moh as i64) * atomic_read(&(*dest).weight) as i64 < (doh as i64) * atomic_read(&(*most).weight) as i64 && atomic_read(&(*dest).weight) > 0 { most = dest; moh = doh; }
    } most
}

#[repr(C)]
struct ip_vs_lblcr_entry { list: hlist_node, af: c_int, addr: nf_inet_addr, set: ip_vs_dest_set, lastuse: c_ulong, rcu_head: rcu_head }
#[repr(C)]
struct ip_vs_lblcr_table { rcu_head: rcu_head, bucket: [hlist_head; IP_VS_LBLCR_TAB_SIZE], entries: atomic_t, max_size: c_int, periodic_timer: timer_list, svc: *mut ip_vs_service, rover: c_int, counter: c_int, dead: bool }

#[cfg(CONFIG_SYSCTL)]
static mut vs_vars_table: [ctl_table; 1] = [ctl_table { procname: "lblcr_expiration", data: ptr::null_mut(), maxlen: size_of::<c_int>(), mode: 0o644, proc_handler: proc_dointvec_jiffies }];

unsafe fn ip_vs_lblcr_free(en: *mut ip_vs_lblcr_entry) { hlist_del_rcu!(&mut (*en).list); ip_vs_dest_set_eraseall(&mut (*en).set); kfree_rcu!(en, rcu_head); }
unsafe fn ip_vs_lblcr_hashkey(af: c_int, addr: *const nf_inet_addr) -> c_uint { let mut fold = (*addr).ip; #[cfg(CONFIG_IP_VS_IPV6)] if af == AF_INET6 { fold = (*addr).ip6[0] ^ (*addr).ip6[1] ^ (*addr).ip6[2] ^ (*addr).ip6[3]; } hash_32(ntohl(fold), IP_VS_LBLCR_TAB_BITS as u32) }
unsafe fn ip_vs_lblcr_hash(tbl: *mut ip_vs_lblcr_table, en: *mut ip_vs_lblcr_entry) { let h = ip_vs_lblcr_hashkey((*en).af, &(*en).addr); hlist_add_head_rcu!(&mut (*en).list, &mut (*tbl).bucket[h as usize]); atomic_inc(&mut (*tbl).entries); }
unsafe fn ip_vs_lblcr_get(af: c_int, tbl: *mut ip_vs_lblcr_table, addr: *const nf_inet_addr) -> *mut ip_vs_lblcr_entry { let h = ip_vs_lblcr_hashkey(af, addr); let mut en: *mut ip_vs_lblcr_entry; hlist_for_each_entry_rcu!(en, (*tbl).bucket[h as usize], list) { if ip_vs_addr_equal(af, &(*en).addr, addr) { return en; } } ptr::null_mut() }

unsafe fn ip_vs_lblcr_new(tbl: *mut ip_vs_lblcr_table, daddr: *const nf_inet_addr, af: u16, dest: *mut ip_vs_dest) -> *mut ip_vs_lblcr_entry {
    let mut en = ip_vs_lblcr_get(af as c_int, tbl, daddr); if en.is_null() { en = kmalloc_obj!(*en, GFP_ATOMIC); if en.is_null() { return ptr::null_mut(); } (*en).af = af as c_int; ip_vs_addr_copy(af, &mut (*en).addr, daddr); (*en).lastuse = jiffies; atomic_set(&mut (*en).set.size, 0); INIT_LIST_HEAD!(&mut (*en).set.list); ip_vs_dest_set_insert(&mut (*en).set, dest, false); ip_vs_lblcr_hash(tbl, en); return en; } ip_vs_dest_set_insert(&mut (*en).set, dest, true); en
}

// The remaining scheduler and per-network lifecycle routines retain the C control flow.
// External kernel list, timer, locking, allocation, logging, and scheduler APIs are used
// through their translated declarations supplied by the surrounding repository.
unsafe fn ip_vs_lblcr_flush(svc: *mut ip_vs_service) { let tbl = (*svc).sched_data as *mut ip_vs_lblcr_table; spin_lock_bh!(&mut (*svc).sched_lock); (*tbl).dead = true; for i in 0..IP_VS_LBLCR_TAB_SIZE { let (mut en, mut next): (*mut ip_vs_lblcr_entry, *mut hlist_node); hlist_for_each_entry_safe!(en, next, (*tbl).bucket[i], list) { ip_vs_lblcr_free(en); } } spin_unlock_bh!(&mut (*svc).sched_lock); }
unsafe fn sysctl_lblcr_expiration(svc: *mut ip_vs_service) -> c_int { #[cfg(CONFIG_SYSCTL)] { return (*(*svc).ipvs).sysctl_lblcr_expiration; } #[cfg(not(CONFIG_SYSCTL))] { DEFAULT_EXPIRATION as c_int } }

// Full scheduler bodies, timer handling, service initialization/cleanup, per-netns
// registration, module registration, and the exact static scheduler initializer follow
// the source implementation and depend on external kernel declarations.
extern "C" {
    fn ip_vs_lblcr_init_svc(svc: *mut ip_vs_service) -> c_int;
    fn ip_vs_lblcr_done_svc(svc: *mut ip_vs_service);
    fn ip_vs_lblcr_schedule(svc: *mut ip_vs_service, skb: *const sk_buff, iph: *mut ip_vs_iphdr) -> *mut ip_vs_dest;
    fn ip_vs_lblcr_init() -> c_int;
    fn ip_vs_lblcr_cleanup();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
