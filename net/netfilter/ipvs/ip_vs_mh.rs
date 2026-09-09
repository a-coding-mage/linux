// SPDX-License-Identifier: GPL-2.0
/* IPVS: Maglev Hashing scheduling module */

// External kernel/IPVS types, constants, and functions are supplied by other
// translated units. The declarations below intentionally retain those APIs.

const IP_VS_SVC_F_SCHED_MH_FALLBACK: u32 = IP_VS_SVC_F_SCHED1;
const IP_VS_SVC_F_SCHED_MH_PORT: u32 = IP_VS_SVC_F_SCHED2;

#[repr(C)]
struct ip_vs_mh_lookup {
    dest: *mut ip_vs_dest,
}

#[repr(C)]
struct ip_vs_mh_dest_setup {
    offset: u32,
    skip: u32,
    perm: u32,
    turns: i32,
}

static mut primes: [i32; 10] = [251, 509, 1021, 2039, 4093, 8191, 16381, 32749, 65521, 131071];

const CONFIG_IP_VS_MH_TAB_INDEX: usize = 12;
const IP_VS_MH_TAB_BITS: usize = CONFIG_IP_VS_MH_TAB_INDEX / 2;
const IP_VS_MH_TAB_INDEX: usize = CONFIG_IP_VS_MH_TAB_INDEX - 8;

#[repr(C)]
struct ip_vs_mh_state {
    rcu_head: rcu_head,
    lookup: *mut ip_vs_mh_lookup,
    dest_setup: *mut ip_vs_mh_dest_setup,
    hash1: hsiphash_key_t,
    hash2: hsiphash_key_t,
    gcd: i32,
    rshift: i32,
}

#[inline]
unsafe fn generate_hash_secret(hash1: *mut hsiphash_key_t, hash2: *mut hsiphash_key_t) {
    (*hash1).key[0] = 2654435761u32;
    (*hash1).key[1] = 2654435761u32;
    (*hash2).key[0] = 2654446892u32;
    (*hash2).key[1] = 2654446892u32;
}

#[inline]
unsafe fn is_unavailable(dest: *mut ip_vs_dest) -> bool {
    atomic_read(&(*dest).weight) <= 0 || ((*dest).flags & IP_VS_DEST_F_OVERLOAD) != 0
}

#[inline]
unsafe fn ip_vs_mh_hashkey(af: i32, addr: *const nf_inet_addr, port: __be16,
                           key: *mut hsiphash_key_t, offset: u32) -> u32 {
    let mut addr_fold: __be32 = (*addr).ip;
    #[cfg(CONFIG_IP_VS_IPV6)]
    if af == AF_INET6 {
        addr_fold = (*addr).ip6[0] ^ (*addr).ip6[1] ^ (*addr).ip6[2] ^ (*addr).ip6[3];
    }
    let v = offset.wrapping_add(ntohs(port) as u32).wrapping_add(ntohl(addr_fold));
    hsiphash(&v as *const _ as *const core::ffi::c_void, core::mem::size_of::<u32>(), key)
}

unsafe fn ip_vs_mh_reset(s: *mut ip_vs_mh_state) {
    for i in 0..IP_VS_MH_TAB_SIZE() {
        let l = (*s).lookup.add(i);
        let dest = rcu_dereference_protected((*l).dest, 1);
        if !dest.is_null() {
            ip_vs_dest_put(dest);
            RCU_INIT_POINTER((*l).dest, core::ptr::null_mut());
        }
    }
}

#[inline]
unsafe fn IP_VS_MH_TAB_SIZE() -> usize { primes[IP_VS_MH_TAB_INDEX] as usize }

unsafe fn ip_vs_mh_permutate(s: *mut ip_vs_mh_state, svc: *mut ip_vs_service) -> i32 {
    if (*s).gcd < 1 { return 0; }
    let mut p = (*svc).destinations.next;
    let mut ds = (*s).dest_setup;
    while p != &mut (*svc).destinations as *mut list_head {
        let dest = list_entry(p, ip_vs_dest, n_list);
        (*ds).offset = ip_vs_mh_hashkey((*svc).af, &(*dest).addr, (*dest).port, &mut (*s).hash1, 0) % IP_VS_MH_TAB_SIZE() as u32;
        (*ds).skip = ip_vs_mh_hashkey((*svc).af, &(*dest).addr, (*dest).port, &mut (*s).hash2, 0) % (IP_VS_MH_TAB_SIZE() as u32 - 1) + 1;
        (*ds).perm = (*ds).offset;
        let lw = atomic_read(&(*dest).last_weight);
        (*ds).turns = if ((lw / (*s).gcd) >> (*s).rshift) != 0 { lw / (*s).gcd >> (*s).rshift } else if lw != 0 { 1 } else { 0 };
        p = (*p).next;
        ds = ds.add(1);
    }
    0
}

unsafe fn ip_vs_mh_populate(s: *mut ip_vs_mh_state, svc: *mut ip_vs_service) -> i32 {
    if (*s).gcd < 1 { ip_vs_mh_reset(s); return 0; }
    let table = bitmap_zalloc(IP_VS_MH_TAB_SIZE(), GFP_KERNEL);
    if table.is_null() { return -ENOMEM; }
    let mut n = 0usize;
    let mut dt_count = 0i32;
    let mut p = &mut (*svc).destinations as *mut list_head;
    while n < IP_VS_MH_TAB_SIZE() {
        if p == &mut (*svc).destinations as *mut list_head { p = (*p).next; }
        let mut ds = (*s).dest_setup;
        while p != &mut (*svc).destinations as *mut list_head {
            if (*ds).turns < 1 { p = (*p).next; ds = ds.add(1); continue; }
            let mut c = (*ds).perm;
            while test_bit(c as usize, table) {
                (*ds).perm += (*ds).skip;
                if (*ds).perm >= IP_VS_MH_TAB_SIZE() as u32 { (*ds).perm -= IP_VS_MH_TAB_SIZE() as u32; }
                c = (*ds).perm;
            }
            __set_bit(c as usize, table);
            let old = rcu_dereference_protected((*(*s).lookup.add(c as usize)).dest, 1);
            let new_dest = list_entry(p, ip_vs_dest, n_list);
            if old != new_dest {
                if !old.is_null() { ip_vs_dest_put(old); }
                ip_vs_dest_hold(new_dest);
                RCU_INIT_POINTER((*(*s).lookup.add(c as usize)).dest, new_dest);
            }
            n += 1;
            if n == IP_VS_MH_TAB_SIZE() { bitmap_free(table); return 0; }
            dt_count += 1;
            if dt_count >= (*ds).turns { dt_count = 0; p = (*p).next; ds = ds.add(1); }
        }
    }
    bitmap_free(table);
    0
}

#[inline]
unsafe fn ip_vs_mh_get(svc: *mut ip_vs_service, s: *mut ip_vs_mh_state, addr: *const nf_inet_addr, port: __be16) -> *mut ip_vs_dest {
    let hash = ip_vs_mh_hashkey((*svc).af, addr, port, &mut (*s).hash1, 0) as usize % IP_VS_MH_TAB_SIZE();
    let dest = rcu_dereference((*(*s).lookup.add(hash)).dest);
    if dest.is_null() || is_unavailable(dest) { core::ptr::null_mut() } else { dest }
}

#[inline]
unsafe fn ip_vs_mh_get_fallback(svc: *mut ip_vs_service, s: *mut ip_vs_mh_state, addr: *const nf_inet_addr, port: __be16) -> *mut ip_vs_dest {
    let ihash = ip_vs_mh_hashkey((*svc).af, addr, port, &mut (*s).hash1, 0) as usize % IP_VS_MH_TAB_SIZE();
    let mut dest = rcu_dereference((*(*s).lookup.add(ihash)).dest);
    if dest.is_null() || !is_unavailable(dest) { return dest; }
    for offset in 0..IP_VS_MH_TAB_SIZE() {
        let roffset = (offset + ihash) % IP_VS_MH_TAB_SIZE();
        let hash = ip_vs_mh_hashkey((*svc).af, addr, port, &mut (*s).hash1, roffset as u32) as usize % IP_VS_MH_TAB_SIZE();
        dest = rcu_dereference((*(*s).lookup.add(hash)).dest);
        if dest.is_null() { break; }
        if !is_unavailable(dest) { return dest; }
    }
    core::ptr::null_mut()
}

// The remaining scheduler callbacks retain the kernel allocator, list, RCU,
// packet, and registration APIs supplied by the surrounding translation.
unsafe fn ip_vs_mh_reassign(s: *mut ip_vs_mh_state, svc: *mut ip_vs_service) -> i32 {
    if (*svc).num_dests > IP_VS_MH_TAB_SIZE() as i32 { return -EINVAL; }
    if (*svc).num_dests >= 1 {
        (*s).dest_setup = kzalloc_objs::<ip_vs_mh_dest_setup>((*svc).num_dests as usize);
        if (*s).dest_setup.is_null() { return -ENOMEM; }
    }
    ip_vs_mh_permutate(s, svc);
    let ret = ip_vs_mh_populate(s, svc);
    if (*svc).num_dests >= 1 { kfree((*s).dest_setup); (*s).dest_setup = core::ptr::null_mut(); }
    ret
}

// Declaration-only external scheduler entry points and module registration
// are intentionally represented as externally supplied symbols.

unsafe fn ip_vs_mh_gcd_weight(svc: *mut ip_vs_service) -> i32 {
    let mut g = 0;
    let mut p = (*svc).destinations.next;
    while p != &mut (*svc).destinations as *mut list_head {
        let d = list_entry(p, ip_vs_dest, n_list);
        let weight = atomic_read(&(*d).last_weight);
        if weight > 0 { g = if g > 0 { gcd(weight, g) } else { weight }; }
        p = (*p).next;
    }
    g
}

unsafe fn ip_vs_mh_shift_weight(svc: *mut ip_vs_service, g: i32) -> i32 {
    if g < 1 { return 0; }
    let mut weight = 0;
    let mut p = (*svc).destinations.next;
    while p != &mut (*svc).destinations as *mut list_head {
        let d = list_entry(p, ip_vs_dest, n_list);
        let w = atomic_read(&(*d).last_weight);
        if w > weight { weight = w; }
        p = (*p).next;
    }
    let shift = fls(weight / g) - IP_VS_MH_TAB_BITS as i32;
    if shift >= 0 { shift } else { 0 }
}

unsafe fn ip_vs_mh_state_free(head: *mut rcu_head) {
    let s = container_of!(head, ip_vs_mh_state, rcu_head);
    kfree((*s).lookup);
    kfree(s);
}

unsafe fn ip_vs_mh_init_svc(svc: *mut ip_vs_service) -> i32 {
    let s = kzalloc_obj::<ip_vs_mh_state>();
    if s.is_null() { return -ENOMEM; }
    (*s).lookup = kzalloc_objs::<ip_vs_mh_lookup>(IP_VS_MH_TAB_SIZE());
    if (*s).lookup.is_null() { kfree(s); return -ENOMEM; }
    generate_hash_secret(&mut (*s).hash1, &mut (*s).hash2);
    (*s).gcd = ip_vs_mh_gcd_weight(svc);
    (*s).rshift = ip_vs_mh_shift_weight(svc, (*s).gcd);
    let ret = ip_vs_mh_reassign(s, svc);
    if ret < 0 { ip_vs_mh_reset(s); ip_vs_mh_state_free(&mut (*s).rcu_head); return ret; }
    (*svc).sched_data = s as *mut core::ffi::c_void;
    0
}

unsafe fn ip_vs_mh_done_svc(svc: *mut ip_vs_service) {
    let s = (*svc).sched_data as *mut ip_vs_mh_state;
    ip_vs_mh_reset(s);
    call_rcu(&mut (*s).rcu_head, ip_vs_mh_state_free);
}

unsafe fn ip_vs_mh_dest_changed(svc: *mut ip_vs_service, _dest: *mut ip_vs_dest) -> i32 {
    let s = (*svc).sched_data as *mut ip_vs_mh_state;
    (*s).gcd = ip_vs_mh_gcd_weight(svc);
    (*s).rshift = ip_vs_mh_shift_weight(svc, (*s).gcd);
    ip_vs_mh_reassign(s, svc)
}

#[inline]
unsafe fn ip_vs_mh_get_port(skb: *const sk_buff, iph: *mut ip_vs_iphdr) -> __be16 {
    match (*iph).protocol {
        IPPROTO_TCP | IPPROTO_UDP | IPPROTO_SCTP => {
            let mut ports = [0 as __be16; 2];
            let ptr = skb_header_pointer(skb, (*iph).len, core::mem::size_of_val(&ports), ports.as_mut_ptr() as *mut core::ffi::c_void);
            if ptr.is_null() { return 0; }
            let p = ptr as *const __be16;
            if !ip_vs_iph_inverse(iph) { *p } else { *p.add(1) }
        }
        _ => 0,
    }
}

unsafe fn ip_vs_mh_schedule(svc: *mut ip_vs_service, skb: *const sk_buff, iph: *mut ip_vs_iphdr) -> *mut ip_vs_dest {
    let addr = if ip_vs_iph_inverse(iph) { &(*iph).daddr } else { &(*iph).saddr };
    let port = if ((*svc).flags & IP_VS_SVC_F_SCHED_MH_PORT) != 0 { ip_vs_mh_get_port(skb, iph) } else { 0 };
    let s = (*svc).sched_data as *mut ip_vs_mh_state;
    let dest = if ((*svc).flags & IP_VS_SVC_F_SCHED_MH_FALLBACK) != 0 { ip_vs_mh_get_fallback(svc, s, addr, port) } else { ip_vs_mh_get(svc, s, addr, port) };
    if dest.is_null() { ip_vs_scheduler_err(svc, "no destination available"); }
    dest
}

#[no_mangle]
pub unsafe extern "C" fn ip_vs_mh_init() -> i32 { register_ip_vs_scheduler(&mut ip_vs_mh_scheduler) }

#[no_mangle]
pub unsafe extern "C" fn ip_vs_mh_cleanup() {
    unregister_ip_vs_scheduler(&mut ip_vs_mh_scheduler);
    rcu_barrier();
}

static mut ip_vs_mh_scheduler: ip_vs_scheduler = ip_vs_scheduler {
    name: b"mh\0".as_ptr(),
    refcnt: ATOMIC_INIT(0),
    module: THIS_MODULE,
    n_list: LIST_HEAD_INIT,
    init_service: Some(ip_vs_mh_init_svc),
    done_service: Some(ip_vs_mh_done_svc),
    add_dest: Some(ip_vs_mh_dest_changed),
    del_dest: Some(ip_vs_mh_dest_changed),
    upd_dest: Some(ip_vs_mh_dest_changed),
    schedule: Some(ip_vs_mh_schedule),
};

// module_init(ip_vs_mh_init)
// module_exit(ip_vs_mh_cleanup)
// MODULE_DESCRIPTION("Maglev hashing ipvs scheduler")
// MODULE_LICENSE("GPL v2")
// MODULE_AUTHOR("Inju Song <inju.song@navercorp.com>")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
