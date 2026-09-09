// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * IPVS:        Destination Hashing scheduling module
 *
 * Authors:     Wensong Zhang <wensong@gnuchina.org>
 *
 *              Inspired by the consistent hashing scheduler patch from
 *              Thomas Proell <proellt@gmx.de>
 *
 * Changes:
 */

/*
 * The dh algorithm is to select server by the hash key of destination IP
 * address. The pseudo code is as follows:
 *
 *       n <- servernode[dest_ip];
 *       if (n is dead) OR
 *          (n is overloaded) OR (n.weight <= 0) then
 *                 return NULL;
 *
 *       return n;
 *
 * Notes that servernode is a 256-bucket hash table that maps the hash
 * index derived from packet destination IP address to the current server
 * array. If the dh scheduler is used in cache cluster, it is good to
 * combine it with cache_bypass feature. When the statically assigned
 * server is dead or overloaded, the load balancer can bypass the cache
 * server and send requests to the original server directly.
 */

// C dependencies: linux/ip.h, linux/slab.h, linux/module.h, linux/kernel.h,
// linux/skbuff.h, linux/hash.h, and net/ip_vs.h.

#[repr(C)]
pub struct ip_vs_dh_bucket {
    pub dest: *mut ip_vs_dest, /* real server (cache) */
}

// CONFIG_IP_VS_DH_TAB_BITS defaults to 8 when not supplied by the build.
pub const IP_VS_DH_TAB_BITS: usize = CONFIG_IP_VS_DH_TAB_BITS;
pub const IP_VS_DH_TAB_SIZE: usize = 1usize << IP_VS_DH_TAB_BITS;
pub const IP_VS_DH_TAB_MASK: usize = IP_VS_DH_TAB_SIZE - 1;

#[repr(C)]
pub struct ip_vs_dh_state {
    pub buckets: [ip_vs_dh_bucket; IP_VS_DH_TAB_SIZE],
    pub rcu_head: rcu_head,
}

#[inline]
unsafe fn ip_vs_dh_hashkey(af: i32, addr: *const nf_inet_addr) -> u32 {
    let mut addr_fold: u32 = (*addr).ip;

    // CONFIG_IP_VS_IPV6 conditionally includes IPv6 address folding.
    #[cfg(CONFIG_IP_VS_IPV6)]
    if af == AF_INET6 {
        addr_fold = (*addr).ip6[0]
            ^ (*addr).ip6[1]
            ^ (*addr).ip6[2]
            ^ (*addr).ip6[3];
    }
    hash_32(ntohl(addr_fold), IP_VS_DH_TAB_BITS as u32)
}

#[inline]
unsafe fn ip_vs_dh_get(
    af: i32,
    s: *mut ip_vs_dh_state,
    addr: *const nf_inet_addr,
) -> *mut ip_vs_dest {
    (*s).buckets[ip_vs_dh_hashkey(af, addr) as usize].dest
}

unsafe fn ip_vs_dh_reassign(s: *mut ip_vs_dh_state, svc: *mut ip_vs_service) -> i32 {
    let mut p = &mut (*svc).destinations as *mut list_head;
    let empty = list_empty(p);

    for i in 0..IP_VS_DH_TAB_SIZE {
        let b = &mut (*s).buckets[i];
        let dest = b.dest;
        if !dest.is_null() {
            ip_vs_dest_put(dest);
        }
        if empty {
            b.dest = core::ptr::null_mut();
        } else {
            if p == &mut (*svc).destinations as *mut list_head {
                p = (*p).next;
            }
            let dest = list_entry(p, ip_vs_dest, n_list);
            ip_vs_dest_hold(dest);
            b.dest = dest;
            p = (*p).next;
        }
    }
    0
}

unsafe fn ip_vs_dh_flush(s: *mut ip_vs_dh_state) {
    for i in 0..IP_VS_DH_TAB_SIZE {
        let b = &mut (*s).buckets[i];
        let dest = b.dest;
        if !dest.is_null() {
            ip_vs_dest_put(dest);
            b.dest = core::ptr::null_mut();
        }
    }
}

unsafe fn ip_vs_dh_init_svc(svc: *mut ip_vs_service) -> i32 {
    /* allocate the DH table for this service */
    let s = kzalloc_obj::<ip_vs_dh_state>();
    if s.is_null() {
        return -ENOMEM;
    }

    (*svc).sched_data = s as *mut core::ffi::c_void;
    IP_VS_DBG!(6, "DH hash table (memory=%zdbytes) allocated for current service\n",
        core::mem::size_of::<ip_vs_dh_bucket>() * IP_VS_DH_TAB_SIZE);

    /* assign the hash buckets with current dests */
    ip_vs_dh_reassign(s, svc);
    0
}

unsafe fn ip_vs_dh_done_svc(svc: *mut ip_vs_service) {
    let s = (*svc).sched_data as *mut ip_vs_dh_state;
    /* got to clean up hash buckets here */
    ip_vs_dh_flush(s);
    /* release the table itself */
    kfree_rcu(s, rcu_head);
    IP_VS_DBG!(6, "DH hash table (memory=%zdbytes) released\n",
        core::mem::size_of::<ip_vs_dh_bucket>() * IP_VS_DH_TAB_SIZE);
}

unsafe fn ip_vs_dh_dest_changed(
    svc: *mut ip_vs_service,
    _dest: *mut ip_vs_dest,
) -> i32 {
    let s = (*svc).sched_data as *mut ip_vs_dh_state;
    /* assign the hash buckets with the updated service */
    ip_vs_dh_reassign(s, svc);
    0
}

/* If the dest flags is set with IP_VS_DEST_F_OVERLOAD, consider overloaded. */
#[inline]
unsafe fn is_overloaded(dest: *mut ip_vs_dest) -> i32 {
    (*dest).flags & IP_VS_DEST_F_OVERLOAD
}

/* Destination hashing scheduling */
unsafe fn ip_vs_dh_schedule(
    svc: *mut ip_vs_service,
    _skb: *const sk_buff,
    iph: *mut ip_vs_iphdr,
) -> *mut ip_vs_dest {
    IP_VS_DBG!(6, "%s(): Scheduling...\n", "ip_vs_dh_schedule");

    let s = (*svc).sched_data as *mut ip_vs_dh_state;
    let dest = ip_vs_dh_get((*svc).af, s, &(*iph).daddr);
    if dest.is_null()
        || ((*dest).cflags & IP_VS_DEST_CF_AVAILABLE) == 0
        || atomic_read(&(*dest).weight) <= 0
        || is_overloaded(dest) != 0
    {
        ip_vs_scheduler_err(svc, "no destination available");
        return core::ptr::null_mut();
    }

    IP_VS_DBG_BUF!(6, "DH: destination IP address %s --> server %s:%d\n",
        IP_VS_DBG_ADDR((*svc).af, &(*iph).daddr),
        IP_VS_DBG_ADDR((*dest).af, &(*dest).addr),
        ntohs((*dest).port));
    dest
}

#[no_mangle]
pub static mut ip_vs_dh_scheduler: ip_vs_scheduler = ip_vs_scheduler {
    name: "dh",
    refcnt: ATOMIC_INIT(0),
    module: THIS_MODULE,
    n_list: LIST_HEAD_INIT,
    init_service: Some(ip_vs_dh_init_svc),
    done_service: Some(ip_vs_dh_done_svc),
    add_dest: Some(ip_vs_dh_dest_changed),
    del_dest: Some(ip_vs_dh_dest_changed),
    schedule: Some(ip_vs_dh_schedule),
};

unsafe fn ip_vs_dh_init() -> i32 {
    register_ip_vs_scheduler(&mut ip_vs_dh_scheduler)
}

unsafe fn ip_vs_dh_cleanup() {
    unregister_ip_vs_scheduler(&mut ip_vs_dh_scheduler);
    synchronize_rcu();
}

// module_init(ip_vs_dh_init); module_exit(ip_vs_dh_cleanup);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("ipvs destination hashing scheduler");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
