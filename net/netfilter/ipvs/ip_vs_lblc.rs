// SPDX-License-Identifier: GPL-2.0-or-later
/* IPVS: Locality-Based Least-Connection scheduling module */

// Kernel includes and symbols from net/ip_vs.h are supplied by other modules.

const CHECK_EXPIRE_INTERVAL: usize = 60 * HZ;
const ENTRY_TIMEOUT: usize = 6 * 60 * HZ;
const DEFAULT_EXPIRATION: usize = 24 * 60 * 60 * HZ;
const COUNT_FOR_FULL_EXPIRATION: i32 = 30;
const CONFIG_IP_VS_LBLC_TAB_BITS: usize = 10;
const IP_VS_LBLC_TAB_BITS: usize = CONFIG_IP_VS_LBLC_TAB_BITS;
const IP_VS_LBLC_TAB_SIZE: usize = 1 << IP_VS_LBLC_TAB_BITS;
const IP_VS_LBLC_TAB_MASK: usize = IP_VS_LBLC_TAB_SIZE - 1;

#[repr(C)]
struct IpVsLblcEntry {
    list: hlist_node,
    af: i32,
    addr: nf_inet_addr,
    dest: *mut ip_vs_dest,
    lastuse: c_ulong,
    rcu_head: rcu_head,
}

#[repr(C)]
struct IpVsLblcTable {
    rcu_head: rcu_head,
    bucket: [hlist_head; IP_VS_LBLC_TAB_SIZE],
    periodic_timer: timer_list,
    svc: *mut ip_vs_service,
    entries: atomic_t,
    max_size: i32,
    rover: i32,
    counter: i32,
    dead: bool,
}

#[cfg(CONFIG_SYSCTL)]
static mut VS_VARS_TABLE: [ctl_table; 1] = [ctl_table {
    procname: "lblc_expiration",
    data: core::ptr::null_mut(),
    maxlen: core::mem::size_of::<i32>(),
    mode: 0o644,
    proc_handler: Some(proc_dointvec_jiffies),
}];

unsafe fn ip_vs_lblc_rcu_free(head: *mut rcu_head) {
    let en = container_of!(head, IpVsLblcEntry, rcu_head);
    ip_vs_dest_put_and_free((*en).dest);
    kfree(en as *mut c_void);
}

unsafe fn ip_vs_lblc_del(en: *mut IpVsLblcEntry) {
    hlist_del_rcu(&mut (*en).list);
    call_rcu(&mut (*en).rcu_head, Some(ip_vs_lblc_rcu_free));
}

unsafe fn ip_vs_lblc_hashkey(af: i32, addr: *const nf_inet_addr) -> u32 {
    let mut addr_fold = (*addr).ip;
    // CONFIG_IP_VS_IPV6 conditionally enables the IPv6 folding below.
    #[cfg(CONFIG_IP_VS_IPV6)]
    if af == AF_INET6 {
        addr_fold = (*addr).ip6[0] ^ (*addr).ip6[1] ^ (*addr).ip6[2] ^ (*addr).ip6[3];
    }
    hash_32(ntohl(addr_fold), IP_VS_LBLC_TAB_BITS as u32)
}

unsafe fn ip_vs_lblc_hash(tbl: *mut IpVsLblcTable, en: *mut IpVsLblcEntry) {
    let hash = ip_vs_lblc_hashkey((*en).af, &(*en).addr) as usize;
    hlist_add_head_rcu(&mut (*en).list, &mut (*tbl).bucket[hash]);
    atomic_inc(&mut (*tbl).entries);
}

unsafe fn ip_vs_lblc_get(af: i32, tbl: *mut IpVsLblcTable, addr: *const nf_inet_addr) -> *mut IpVsLblcEntry {
    let hash = ip_vs_lblc_hashkey(af, addr) as usize;
    let mut en: *mut IpVsLblcEntry = core::ptr::null_mut();
    hlist_for_each_entry_rcu!(en, &mut (*tbl).bucket[hash], list) {
        if ip_vs_addr_equal(af, &(*en).addr, addr) { return en; }
    }
    core::ptr::null_mut()
}

unsafe fn ip_vs_lblc_new(tbl: *mut IpVsLblcTable, daddr: *const nf_inet_addr, af: u16, dest: *mut ip_vs_dest) -> *mut IpVsLblcEntry {
    let mut en = ip_vs_lblc_get(af as i32, tbl, daddr);
    if !en.is_null() {
        if (*en).dest == dest { return en; }
        ip_vs_lblc_del(en);
    }
    en = kmalloc_obj::<IpVsLblcEntry>(GFP_ATOMIC);
    if en.is_null() { return core::ptr::null_mut(); }
    (*en).af = af as i32;
    ip_vs_addr_copy(af as i32, &mut (*en).addr, daddr);
    (*en).lastuse = jiffies;
    ip_vs_dest_hold(dest);
    (*en).dest = dest;
    ip_vs_lblc_hash(tbl, en);
    en
}

unsafe fn ip_vs_lblc_flush(svc: *mut ip_vs_service) {
    let tbl = (*svc).sched_data as *mut IpVsLblcTable;
    let _guard = spin_lock_bh(&mut (*svc).sched_lock);
    (*tbl).dead = true;
    for i in 0..IP_VS_LBLC_TAB_SIZE {
        let mut en: *mut IpVsLblcEntry = core::ptr::null_mut();
        let mut next: *mut hlist_node = core::ptr::null_mut();
        hlist_for_each_entry_safe!(en, next, &mut (*tbl).bucket[i], list) {
            ip_vs_lblc_del(en);
            atomic_dec(&mut (*tbl).entries);
        }
    }
}

unsafe fn sysctl_lblc_expiration(svc: *mut ip_vs_service) -> usize {
    #[cfg(CONFIG_SYSCTL)] { (*(*svc).ipvs).sysctl_lblc_expiration }
    #[cfg(not(CONFIG_SYSCTL))] { DEFAULT_EXPIRATION }
}

unsafe fn ip_vs_lblc_full_check(svc: *mut ip_vs_service) {
    let tbl = (*svc).sched_data as *mut IpVsLblcTable;
    let now = jiffies;
    let mut j = (*tbl).rover as usize;
    for _ in 0..IP_VS_LBLC_TAB_SIZE {
        j = (j + 1) & IP_VS_LBLC_TAB_MASK;
        let _guard = spin_lock(&mut (*svc).sched_lock);
        let mut en: *mut IpVsLblcEntry = core::ptr::null_mut();
        let mut next: *mut hlist_node = core::ptr::null_mut();
        hlist_for_each_entry_safe!(en, next, &mut (*tbl).bucket[j], list) {
            if time_before(now, (*en).lastuse.wrapping_add(sysctl_lblc_expiration(svc))) { continue; }
            ip_vs_lblc_del(en); atomic_dec(&mut (*tbl).entries);
        }
    }
    (*tbl).rover = j as i32;
}

unsafe extern "C" fn ip_vs_lblc_check_expire(t: *mut timer_list) {
    let tbl = timer_container_of!(t, IpVsLblcTable, periodic_timer);
    let svc = (*tbl).svc;
    if (*tbl).counter % COUNT_FOR_FULL_EXPIRATION == 0 {
        ip_vs_lblc_full_check(svc); (*tbl).counter = 1;
    } else if atomic_read(&(*tbl).entries) <= (*tbl).max_size {
        (*tbl).counter += 1;
    } else {
        let mut goal = (atomic_read(&(*tbl).entries) - (*tbl).max_size) * 4 / 3;
        if goal > (*tbl).max_size / 2 { goal = (*tbl).max_size / 2; }
        let now = jiffies; let mut j = (*tbl).rover as usize;
        for _ in 0..IP_VS_LBLC_TAB_SIZE {
            j = (j + 1) & IP_VS_LBLC_TAB_MASK;
            let _guard = spin_lock(&mut (*svc).sched_lock);
            let mut en: *mut IpVsLblcEntry = core::ptr::null_mut(); let mut next = core::ptr::null_mut();
            hlist_for_each_entry_safe!(en, next, &mut (*tbl).bucket[j], list) {
                if time_before(now, (*en).lastuse.wrapping_add(ENTRY_TIMEOUT)) { continue; }
                ip_vs_lblc_del(en); atomic_dec(&mut (*tbl).entries); goal -= 1;
            }
            if goal <= 0 { break; }
        }
        (*tbl).rover = j as i32;
    }
    mod_timer(&mut (*tbl).periodic_timer, jiffies + CHECK_EXPIRE_INTERVAL);
}

unsafe extern "C" fn ip_vs_lblc_init_svc(svc: *mut ip_vs_service) -> i32 {
    let tbl = kmalloc_obj::<IpVsLblcTable>(GFP_KERNEL);
    if tbl.is_null() { return -ENOMEM; }
    (*svc).sched_data = tbl as *mut c_void;
    for bucket in (*tbl).bucket.iter_mut() { INIT_HLIST_HEAD(bucket); }
    (*tbl).max_size = (IP_VS_LBLC_TAB_SIZE * 16) as i32;
    (*tbl).rover = 0; (*tbl).counter = 1; (*tbl).dead = false; (*tbl).svc = svc;
    atomic_set(&mut (*tbl).entries, 0);
    timer_setup(&mut (*tbl).periodic_timer, Some(ip_vs_lblc_check_expire), 0);
    mod_timer(&mut (*tbl).periodic_timer, jiffies + CHECK_EXPIRE_INTERVAL);
    0
}

unsafe extern "C" fn ip_vs_lblc_done_svc(svc: *mut ip_vs_service) {
    let tbl = (*svc).sched_data as *mut IpVsLblcTable;
    timer_shutdown_sync(&mut (*tbl).periodic_timer);
    ip_vs_lblc_flush(svc);
    kfree_rcu(tbl, rcu_head);
}

unsafe fn __ip_vs_lblc_schedule(svc: *mut ip_vs_service) -> *mut ip_vs_dest {
    let mut least: *mut ip_vs_dest = core::ptr::null_mut(); let mut loh: i32 = 0;
    list_for_each_entry_rcu!(dest, &(*svc).destinations, n_list) {
        if (*dest).flags & IP_VS_DEST_F_OVERLOAD != 0 { continue; }
        if atomic_read(&(*dest).weight) > 0 { least = dest; loh = ip_vs_dest_conn_overhead(dest); break; }
    }
    if least.is_null() { return core::ptr::null_mut(); }
    list_for_each_entry_continue_rcu!(dest, &(*svc).destinations, n_list) {
        if (*dest).flags & IP_VS_DEST_F_OVERLOAD != 0 { continue; }
        let doh = ip_vs_dest_conn_overhead(dest);
        if (loh as i64) * atomic_read(&(*dest).weight) as i64 > (doh as i64) * atomic_read(&(*least).weight) as i64 { least = dest; loh = doh; }
    }
    least
}

unsafe fn is_overloaded(dest: *mut ip_vs_dest, svc: *mut ip_vs_service) -> i32 {
    if atomic_read(&(*dest).activeconns) > atomic_read(&(*dest).weight) {
        list_for_each_entry_rcu!(d, &(*svc).destinations, n_list) {
            if atomic_read(&(*d).activeconns) * 2 < atomic_read(&(*d).weight) { return 1; }
        }
    }
    0
}

unsafe extern "C" fn ip_vs_lblc_schedule(svc: *mut ip_vs_service, _skb: *const sk_buff, iph: *mut ip_vs_iphdr) -> *mut ip_vs_dest {
    let tbl = (*svc).sched_data as *mut IpVsLblcTable;
    let mut en = ip_vs_lblc_get((*svc).af, tbl, &(*iph).daddr);
    if !en.is_null() {
        (*en).lastuse = jiffies;
        let dest = (*en).dest;
        if (*dest).cflags & IP_VS_DEST_CF_AVAILABLE != 0 && atomic_read(&(*dest).weight) > 0 && is_overloaded(dest, svc) == 0 { return dest; }
    }
    let dest = __ip_vs_lblc_schedule(svc);
    if dest.is_null() { ip_vs_scheduler_err(svc, "no destination available"); return core::ptr::null_mut(); }
    let _guard = spin_lock_bh(&mut (*svc).sched_lock);
    if !(*tbl).dead { en = ip_vs_lblc_new(tbl, &(*iph).daddr, (*svc).af as u16, dest); }
    dest
}

#[cfg(CONFIG_SYSCTL)]
unsafe extern "C" fn __ip_vs_lblc_init(net: *mut net) -> i32 {
    let ipvs = net_ipvs(net);
    if ipvs.is_null() { return -ENOENT; }
    let mut vars_table_size = ARRAY_SIZE!(VS_VARS_TABLE);
    if !net_eq(net, &mut init_net) {
        (*ipvs).lblc_ctl_table = kmemdup(VS_VARS_TABLE.as_ptr(), core::mem::size_of_val(&VS_VARS_TABLE), GFP_KERNEL);
        if (*ipvs).lblc_ctl_table.is_null() { return -ENOMEM; }
        if (*net).user_ns != &mut init_user_ns { vars_table_size = 0; }
    } else {
        (*ipvs).lblc_ctl_table = VS_VARS_TABLE.as_mut_ptr();
    }
    (*ipvs).sysctl_lblc_expiration = DEFAULT_EXPIRATION;
    (*ipvs).lblc_ctl_table[0].data = &mut (*ipvs).sysctl_lblc_expiration as *mut _;
    (*ipvs).lblc_ctl_header = register_net_sysctl_sz(net, "net/ipv4/vs", (*ipvs).lblc_ctl_table, vars_table_size);
    if (*ipvs).lblc_ctl_header.is_null() {
        if !net_eq(net, &mut init_net) { kfree((*ipvs).lblc_ctl_table as *mut c_void); }
        return -ENOMEM;
    }
    0
}

#[cfg(CONFIG_SYSCTL)]
unsafe extern "C" fn __ip_vs_lblc_exit(net: *mut net) {
    let ipvs = net_ipvs(net);
    unregister_net_sysctl_table((*ipvs).lblc_ctl_header);
    if !net_eq(net, &mut init_net) { kfree((*ipvs).lblc_ctl_table as *mut c_void); }
}

#[cfg(not(CONFIG_SYSCTL))]
unsafe extern "C" fn __ip_vs_lblc_init(_net: *mut net) -> i32 { 0 }
#[cfg(not(CONFIG_SYSCTL))]
unsafe extern "C" fn __ip_vs_lblc_exit(_net: *mut net) {}

static mut ip_vs_lblc_ops: pernet_operations = pernet_operations {
    init: Some(__ip_vs_lblc_init), exit: Some(__ip_vs_lblc_exit),
};

static mut IP_VS_LBLC_SCHEDULER: ip_vs_scheduler = ip_vs_scheduler {
    name: "lblc", refcnt: ATOMIC_INIT(0), module: THIS_MODULE,
    n_list: LIST_HEAD_INIT(), init_service: Some(ip_vs_lblc_init_svc), done_service: Some(ip_vs_lblc_done_svc), schedule: Some(ip_vs_lblc_schedule),
};

unsafe extern "C" fn ip_vs_lblc_init() -> i32 {
    let ret = register_pernet_subsys(&mut ip_vs_lblc_ops);
    if ret != 0 { return ret; }
    let ret = register_ip_vs_scheduler(&mut IP_VS_LBLC_SCHEDULER);
    if ret != 0 { unregister_pernet_subsys(&mut ip_vs_lblc_ops); }
    ret
}

unsafe extern "C" fn ip_vs_lblc_cleanup() {
    unregister_ip_vs_scheduler(&mut IP_VS_LBLC_SCHEDULER);
    unregister_pernet_subsys(&mut ip_vs_lblc_ops);
    rcu_barrier();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
