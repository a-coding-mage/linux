// SPDX-License-Identifier: GPL-2.0-or-later
/* RAW sockets for IPv6 — translated from raw.c. */

// Linux kernel headers and configuration supplied by other translation units.

const ICMPV6_HDRLEN: usize = 4;

extern "C" {
    static mut raw_v6_hashinfo: raw_hashinfo;
}

#[no_mangle]
pub unsafe extern "C" fn raw_v6_match(
    net: *mut net, sk: *const sock, num: u16,
    loc_addr: *const in6_addr, rmt_addr: *const in6_addr,
    dif: i32, sdif: i32,
) -> bool {
    if (*inet_sk(sk)).inet_num != num ||
       !net_eq(sock_net(sk), net) ||
       (!ipv6_addr_any(&(*sk).sk_v6_daddr) &&
        !ipv6_addr_equal(&(*sk).sk_v6_daddr, rmt_addr)) ||
       !raw_sk_bound_dev_eq(net, (*sk).sk_bound_dev_if, dif, sdif) {
        return false;
    }
    if ipv6_addr_any(&(*sk).sk_v6_rcv_saddr) ||
       ipv6_addr_equal(&(*sk).sk_v6_rcv_saddr, loc_addr) ||
       (ipv6_addr_is_multicast(loc_addr) && inet6_mc_check(sk, loc_addr, rmt_addr)) {
        return true;
    }
    false
}

unsafe fn icmpv6_filter(sk: *const sock, skb: *mut sk_buff) -> i32 {
    if !pskb_may_pull(skb, ICMPV6_HDRLEN) { return 1; }
    let hdr = (*skb).data as *const icmp6hdr;
    let typ = (*hdr).icmp6_type as usize;
    let data = &raw6_sk(sk).filter.data[0] as *const u32;
    ((*data.add(typ >> 5) & (1u32 << (typ & 31))) != 0) as i32
}

#[cfg(CONFIG_IPV6_MIP6)]
type mh_filter_t = unsafe extern "C" fn(*mut sock, *mut sk_buff) -> i32;
#[cfg(CONFIG_IPV6_MIP6)]
static mut mh_filter: *mut mh_filter_t = core::ptr::null_mut();

#[cfg(CONFIG_IPV6_MIP6)]
pub unsafe extern "C" fn rawv6_mh_filter_register(filter: mh_filter_t) -> i32 {
    mh_filter = Box::into_raw(Box::new(filter)); 0
}
#[cfg(CONFIG_IPV6_MIP6)]
pub unsafe extern "C" fn rawv6_mh_filter_unregister(_filter: mh_filter_t) -> i32 {
    mh_filter = core::ptr::null_mut(); synchronize_rcu(); 0
}

unsafe fn ipv6_raw_deliver(skb: *mut sk_buff, nexthdr: i32) -> bool {
    let netp = dev_net((*skb).dev);
    let ip6h = ipv6_hdr(skb);
    let hash = raw_hashfunc(netp, nexthdr);
    let hlist = &mut (*raw_v6_hashinfo).ht[hash as usize];
    let mut delivered = false;
    rcu_read_lock();
    let mut sk = sk_for_each_rcu(hlist);
    while !sk.is_null() {
        let mut filtered: i32;
        if !raw_v6_match(netp, sk, nexthdr as u16, &(*ip6h).daddr, &(*ip6h).saddr,
                         inet6_iif(skb), inet6_sdif(skb)) { sk = sk_next_rcu(sk); continue; }
        if atomic_read(&(*sk).sk_rmem_alloc) >= READ_ONCE((*sk).sk_rcvbuf) {
            sk_drops_inc(sk); sk = sk_next_rcu(sk); continue;
        }
        delivered = true;
        filtered = match nexthdr {
            IPPROTO_ICMPV6 => icmpv6_filter(sk, skb),
            #[cfg(CONFIG_IPV6_MIP6)]
            IPPROTO_MH => if !mh_filter.is_null() { (**mh_filter)(sk, skb) } else { 0 },
            _ => 0,
        };
        if filtered < 0 { break; }
        if filtered == 0 {
            let clone = skb_clone(skb, GFP_ATOMIC);
            if !clone.is_null() { rawv6_rcv(sk, clone); }
        }
        sk = sk_next_rcu(sk);
    }
    rcu_read_unlock(); delivered
}

pub unsafe extern "C" fn raw6_local_deliver(skb: *mut sk_buff, nexthdr: i32) -> bool {
    ipv6_raw_deliver(skb, nexthdr)
}

unsafe fn rawv6_bind(sk: *mut sock, uaddr: *mut sockaddr_unsized, addr_len: i32) -> i32 {
    let inet = inet_sk(sk); let np = inet6_sk(sk); let addr = uaddr as *mut sockaddr_in6;
    if addr_len < SIN6_LEN_RFC2133 || (*addr).sin6_family != AF_INET6 { return -EINVAL; }
    let addr_type = ipv6_addr_type(&(*addr).sin6_addr);
    if addr_type == IPV6_ADDR_MAPPED { return -EADDRNOTAVAIL; }
    lock_sock(sk); let mut err = -EINVAL;
    if (*sk).sk_state != TCP_CLOSE { release_sock(sk); return err; }
    rcu_read_lock();
    if addr_type != IPV6_ADDR_ANY {
        let mut dev: *mut net_device = core::ptr::null_mut();
        if __ipv6_addr_needs_scope_id(addr_type) {
            if addr_len >= core::mem::size_of::<sockaddr_in6>() as i32 && (*addr).sin6_scope_id != 0 {
                (*sk).sk_bound_dev_if = (*addr).sin6_scope_id;
            }
            if (*sk).sk_bound_dev_if == 0 { rcu_read_unlock(); release_sock(sk); return err; }
        }
        if (*sk).sk_bound_dev_if != 0 {
            err = -ENODEV; dev = dev_get_by_index_rcu(sock_net(sk), (*sk).sk_bound_dev_if);
            if dev.is_null() { rcu_read_unlock(); release_sock(sk); return err; }
        }
        let v4addr = LOOPBACK4_IPV6;
        if (addr_type & IPV6_ADDR_MULTICAST) == 0 && !ipv6_can_nonlocal_bind(sock_net(sk), inet) {
            err = -EADDRNOTAVAIL;
            if !ipv6_chk_addr(sock_net(sk), &(*addr).sin6_addr, dev, 0) {
                rcu_read_unlock(); release_sock(sk); return err;
            }
        }
        (*inet).inet_rcv_saddr = v4addr; (*inet).inet_saddr = v4addr;
    } else { (*inet).inet_rcv_saddr = 0; (*inet).inet_saddr = 0; }
    (*sk).sk_v6_rcv_saddr = (*addr).sin6_addr;
    if (addr_type & IPV6_ADDR_MULTICAST) == 0 { (*np).saddr = (*addr).sin6_addr; }
    err = 0; rcu_read_unlock(); release_sock(sk); err
}

extern "C" {
    fn rawv6_rcv(sk: *mut sock, skb: *mut sk_buff) -> i32;
    fn rawv6_sendmsg(sk: *mut sock, msg: *mut msghdr, len: usize) -> i32;
    fn rawv6_recvmsg(sk: *mut sock, msg: *mut msghdr, len: usize, flags: i32) -> i32;
    fn rawv6_close(sk: *mut sock, timeout: i64);
    fn raw6_destroy(sk: *mut sock);
    fn rawv6_init_sk(sk: *mut sock) -> i32;
    fn rawv6_setsockopt(sk: *mut sock, level: i32, optname: i32,
                        optval: sockptr_t, optlen: u32) -> i32;
    fn rawv6_getsockopt(sk: *mut sock, level: i32, optname: i32,
                        optval: *mut core::ffi::c_char, optlen: *mut i32) -> i32;
    fn rawv6_ioctl(sk: *mut sock, cmd: i32, karg: *mut i32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
