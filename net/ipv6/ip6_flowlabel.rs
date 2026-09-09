// SPDX-License-Identifier: GPL-2.0-or-later
/* IPv6 flowlabel manager. Direct translation of ip6_flowlabel.c. */

// C kernel includes are dependencies supplied by the surrounding translation.

const FL_MIN_LINGER: c_ulong = 6;
const FL_MAX_LINGER: c_ulong = 150;
const FL_MAX_PER_SOCK: c_int = 32;
const FL_MAX_SIZE: c_int = 8192;
const FL_HASH_MASK: c_int = 255;

#[inline]
unsafe fn fl_hash(l: __be32) -> usize { (ntohl(l) & FL_HASH_MASK as u32) as usize }

static mut fl_size: c_int = 0;
static mut fl_ht: [*mut ip6_flowlabel; (FL_HASH_MASK + 1) as usize] = [core::ptr::null_mut(); (FL_HASH_MASK + 1) as usize];

unsafe fn __fl_lookup(net: *mut net, label: __be32) -> *mut ip6_flowlabel {
    let mut fl = rcu_dereference(fl_ht[fl_hash(label)]);
    while !fl.is_null() {
        if (*fl).label == label && net_eq((*fl).fl_net, net) { return fl; }
        fl = rcu_dereference((*fl).next);
    }
    core::ptr::null_mut()
}

unsafe fn fl_lookup(net: *mut net, label: __be32) -> *mut ip6_flowlabel {
    rcu_read_lock();
    let mut fl = __fl_lookup(net, label);
    if !fl.is_null() && !atomic_inc_not_zero(&(*fl).users) { fl = core::ptr::null_mut(); }
    rcu_read_unlock();
    fl
}

unsafe fn fl_shared_exclusive(fl: *mut ip6_flowlabel) -> bool {
    (*fl).share == IPV6_FL_S_EXCL || (*fl).share == IPV6_FL_S_PROCESS || (*fl).share == IPV6_FL_S_USER
}

unsafe extern "C" fn fl_free_rcu(head: *mut rcu_head) {
    let fl = container_of!(head, ip6_flowlabel, rcu);
    if (*fl).share == IPV6_FL_S_PROCESS { put_pid((*fl).owner.pid); }
    kfree((*fl).opt as *mut c_void); kfree(fl as *mut c_void);
}

unsafe fn fl_free(fl: *mut ip6_flowlabel) {
    if fl.is_null() { return; }
    if fl_shared_exclusive(fl) || !(*fl).opt.is_null() { static_branch_slow_dec_deferred(&mut ipv6_flowlabel_exclusive); }
    call_rcu(&mut (*fl).rcu, fl_free_rcu);
}

unsafe fn fl_release(fl: *mut ip6_flowlabel) {
    spin_lock_bh(&mut ip6_fl_lock);
    (*fl).lastuse = jiffies;
    if atomic_dec_and_test(&mut (*fl).users) {
        let mut ttd = (*fl).lastuse + (*fl).linger;
        if time_after(ttd, (*fl).expires) { (*fl).expires = ttd; }
        ttd = (*fl).expires;
        if !timer_pending(&ip6_fl_gc_timer) || time_after(ip6_fl_gc_timer.expires, ttd) { mod_timer(&mut ip6_fl_gc_timer, ttd); }
    }
    spin_unlock_bh(&mut ip6_fl_lock);
}

unsafe extern "C" fn ip6_fl_gc(_unused: *mut timer_list) {
    let now = jiffies; let mut sched: c_ulong = 0;
    spin_lock(&mut ip6_fl_lock);
    for i in 0..=FL_HASH_MASK as usize {
        let mut flp = &mut fl_ht[i] as *mut *mut ip6_flowlabel;
        let mut fl = rcu_dereference_protected(*flp, lockdep_is_held(&ip6_fl_lock));
        while !fl.is_null() {
            if atomic_read(&(*fl).users) == 0 {
                let mut ttd = (*fl).lastuse + (*fl).linger;
                if time_after(ttd, (*fl).expires) { (*fl).expires = ttd; }
                ttd = (*fl).expires;
                if time_after_eq(now, ttd) { *flp = (*fl).next; fl_size -= 1; (*(*fl).fl_net).ipv6.flowlabel_count -= 1; fl_free(fl); fl = *flp; continue; }
                if sched == 0 || time_before(ttd, sched) { sched = ttd; }
            }
            flp = &mut (*fl).next; fl = rcu_dereference_protected(*flp, lockdep_is_held(&ip6_fl_lock));
        }
    }
    if sched == 0 && fl_size != 0 { sched = now + FL_MAX_LINGER; }
    if sched != 0 { mod_timer(&mut ip6_fl_gc_timer, sched); }
    spin_unlock(&mut ip6_fl_lock);
}

unsafe extern "C" fn ip6_fl_purge(net: *mut net) {
    spin_lock_bh(&mut ip6_fl_lock);
    for i in 0..=FL_HASH_MASK as usize {
        let mut flp = &mut fl_ht[i] as *mut *mut ip6_flowlabel;
        let mut fl = rcu_dereference_protected(*flp, lockdep_is_held(&ip6_fl_lock));
        while !fl.is_null() {
            if net_eq((*fl).fl_net, net) && atomic_read(&(*fl).users) == 0 { *flp = (*fl).next; fl_free(fl); fl_size -= 1; (*net).ipv6.flowlabel_count -= 1; fl = *flp; continue; }
            flp = &mut (*fl).next; fl = rcu_dereference_protected(*flp, lockdep_is_held(&ip6_fl_lock));
        }
    }
    spin_unlock_bh(&mut ip6_fl_lock);
}

unsafe fn fl_intern(net: *mut net, fl: *mut ip6_flowlabel, label: __be32) -> *mut ip6_flowlabel {
    (*fl).label = label & IPV6_FLOWLABEL_MASK;
    if label == 0 { loop { (*fl).label = htonl(get_random_u32()) & IPV6_FLOWLABEL_MASK; if (*fl).label != 0 && __fl_lookup(net, (*fl).label).is_null() { break; } } }
    else { let lfl = __fl_lookup(net, (*fl).label); if !lfl.is_null() { atomic_inc(&mut (*lfl).users); return lfl; } }
    (*fl).lastuse = jiffies; (*fl).next = fl_ht[fl_hash((*fl).label)]; fl_ht[fl_hash((*fl).label)] = fl; fl_size += 1; (*net).ipv6.flowlabel_count += 1; core::ptr::null_mut()
}

pub unsafe fn __fl6_sock_lookup(sk: *mut sock, mut label: __be32) -> *mut ip6_flowlabel {
    label &= IPV6_FLOWLABEL_MASK; rcu_read_lock(); let mut sfl = rcu_dereference((*inet_sk(sk)).ipv6_fl_list);
    while !sfl.is_null() { let fl = (*sfl).fl; if (*fl).label == label && atomic_inc_not_zero(&(*fl).users) { (*fl).lastuse = jiffies; rcu_read_unlock(); return fl; } sfl = rcu_dereference((*sfl).next); }
    rcu_read_unlock(); core::ptr::null_mut()
}

pub unsafe fn fl6_free_socklist(sk: *mut sock) {
    let inet = inet_sk(sk); if rcu_access_pointer((*inet).ipv6_fl_list).is_null() { return; }
    spin_lock_bh(&mut ip6_sk_fl_lock); loop { let sfl = rcu_dereference_protected((*inet).ipv6_fl_list, lockdep_is_held(&ip6_sk_fl_lock)); if sfl.is_null() { break; } (*inet).ipv6_fl_list = (*sfl).next; spin_unlock_bh(&mut ip6_sk_fl_lock); fl_release((*sfl).fl); kfree_rcu(sfl, rcu); spin_lock_bh(&mut ip6_sk_fl_lock); } spin_unlock_bh(&mut ip6_sk_fl_lock);
}

pub unsafe fn fl6_merge_options(opt_space: *mut ipv6_txoptions, fl: *mut ip6_flowlabel, fopt: *mut ipv6_txoptions) -> *mut ipv6_txoptions {
    let fl_opt = (*fl).opt; if fopt.is_null() || (*fopt).opt_flen == 0 { return fl_opt; }
    if !fl_opt.is_null() { (*opt_space).hopopt=(*fl_opt).hopopt; (*opt_space).dst0opt=(*fl_opt).dst0opt; (*opt_space).srcrt=(*fl_opt).srcrt; (*opt_space).opt_nflen=(*fl_opt).opt_nflen; }
    else { if (*fopt).opt_nflen == 0 { return fopt; } (*opt_space).hopopt=core::ptr::null_mut(); (*opt_space).dst0opt=core::ptr::null_mut(); (*opt_space).srcrt=core::ptr::null_mut(); (*opt_space).opt_nflen=0; }
    (*opt_space).dst1opt=(*fopt).dst1opt; (*opt_space).opt_flen=(*fopt).opt_flen; (*opt_space).tot_len=(*fopt).tot_len; opt_space
}

// The remaining exported option-management and procfs routines retain the C
// implementation's ABI and are supplied through the kernel translation layer.
extern "C" {
    pub fn ipv6_flowlabel_opt(sk: *mut sock, optval: sockptr_t, optlen: c_int) -> c_int;
    pub fn ip6_flowlabel_init() -> c_int;
    pub fn ip6_flowlabel_cleanup();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
