// SPDX-License-Identifier: GPL-2.0-or-later
/* Rust translation of the Linux kernel IPv4 ping socket implementation. */

// Kernel headers and build-time configuration are supplied by the surrounding
// crate; CONFIG_IPV6 and CONFIG_PROC_FS conditionals remain represented below.

#[repr(C)]
pub struct ping_table {
    pub hash: [hlist_head; PING_HTABLE_SIZE],
    pub lock: spinlock_t,
}

static mut ping_table: ping_table = ping_table { hash: [hlist_head {}; PING_HTABLE_SIZE], lock: spinlock_t {} };
pub static mut pingv6_ops: pingv6_ops = pingv6_ops {};

#[inline]
unsafe fn ping_hashfn(net: *const net, num: u32, mask: u32) -> u32 {
    let res = (num.wrapping_add(net_hash_mix(net))) & mask;
    pr_debug!("hash({}) = {}\n", num, res);
    res
}

#[inline]
unsafe fn ping_hashslot(table: *mut ping_table, net: *const net, num: u32) -> *mut hlist_head {
    (*table).hash.as_mut_ptr().add(ping_hashfn(net, num, PING_HTABLE_MASK) as usize)
}

pub unsafe fn ping_get_port(sk: *mut sock, ident: u16) -> i32 {
    let net = sock_net(sk);
    let isk = inet_sk(sk);
    let mut hlist: *mut hlist_head;
    let mut sk2: *mut sock = core::ptr::null_mut();
    spin_lock(&mut ping_table.lock);
    if ident == 0 {
        let mut result: u16 = (*net).ipv4.ping_port_rover.wrapping_add(1);
        let mut i: u32 = 0;
        while i < (1u32 << 16) {
            if result == 0 { i += 1; result = result.wrapping_add(1); continue; }
            hlist = ping_hashslot(&mut ping_table, net, result as u32);
            sk_for_each!(sk2, hlist, {
                if !net_eq(sock_net(sk2), net) && continue;
                let isk2 = inet_sk(sk2);
                if (*isk2).inet_num == result { break 'next_port; }
            });
            (*net).ipv4.ping_port_rover = result; (*isk).inet_num = result; break;
            'next_port: { result = result.wrapping_add(1); i += 1; }
        }
        if i >= (1u32 << 16) { spin_unlock(&mut ping_table.lock); return -EADDRINUSE; }
    } else {
        hlist = ping_hashslot(&mut ping_table, net, ident as u32);
        sk_for_each!(sk2, hlist, {
            if !net_eq(sock_net(sk2), net) && continue;
            let isk2 = inet_sk(sk2);
            if (*isk2).inet_num == ident && sk2 != sk && (!(*sk2).sk_reuse || !(*sk).sk_reuse) {
                spin_unlock(&mut ping_table.lock); return -EADDRINUSE;
            }
        });
    }
    pr_debug!("found port/ident = {}\n", ident);
    (*isk).inet_num = ident;
    if sk_unhashed(sk) { sk_add_node_rcu(sk, hlist); sock_set_flag(sk, SOCK_RCU_FREE); sock_prot_inuse_add(net, (*sk).sk_prot, 1); }
    spin_unlock(&mut ping_table.lock); 0
}

pub unsafe fn ping_unhash(sk: *mut sock) {
    let isk = inet_sk(sk);
    pr_debug!("ping_unhash(isk={:p},isk->num={})\n", isk, (*isk).inet_num);
    spin_lock(&mut ping_table.lock);
    if sk_del_node_init_rcu(sk) { WRITE_ONCE!((*isk).inet_num, 0); (*isk).inet_sport = 0; sock_prot_inuse_add(sock_net(sk), (*sk).sk_prot, -1); }
    spin_unlock(&mut ping_table.lock);
}

unsafe fn ping_lookup(net: *mut net, skb: *mut sk_buff, ident: u16) -> *mut sock {
    let hslot = ping_hashslot(&mut ping_table, net, ident as u32);
    let mut sk: *mut sock = core::ptr::null_mut();
    let protocol = (*skb).protocol;
    if protocol != htons(ETH_P_IP) && protocol != htons(ETH_P_IPV6) { return core::ptr::null_mut(); }
    sk_for_each_rcu!(sk, hslot, {
        if !net_eq(sock_net(sk), net) && continue;
        let isk = inet_sk(sk);
        if READ_ONCE!((*isk).inet_num) != ident { continue; }
        let dif = if protocol == htons(ETH_P_IP) { inet_iif(skb) } else { inet6_iif(skb) };
        let sdif = if protocol == htons(ETH_P_IP) { inet_sdif(skb) } else { inet6_sdif(skb) };
        let bound = READ_ONCE!((*sk).sk_bound_dev_if);
        if protocol == htons(ETH_P_IP) && (*sk).sk_family == AF_INET {
            let addr = READ_ONCE!((*isk).inet_rcv_saddr);
            if addr != 0 && addr != (*ip_hdr(skb)).daddr { continue; }
        } else if protocol == htons(ETH_P_IPV6) && (*sk).sk_family == AF_INET6 {
            if !ipv6_addr_any(&(*sk).sk_v6_rcv_saddr) && !ipv6_addr_equal(&(*sk).sk_v6_rcv_saddr, &(*ipv6_hdr(skb)).daddr) { continue; }
        } else { continue; }
        if bound != 0 && bound != dif && bound != sdif { continue; }
        break;
    });
    sk
}

unsafe fn inet_get_ping_group_range_net(net: *mut net, low: *mut kgid_t, high: *mut kgid_t) {
    let data = (*net).ipv4.ping_group_range.range.as_ptr();
    let mut seq;
    loop { seq = read_seqbegin(&(*net).ipv4.ping_group_range.lock); *low = *data; *high = *data.add(1); if !read_seqretry(&(*net).ipv4.ping_group_range.lock, seq) { break; } }
}

pub unsafe fn ping_init_sock(sk: *mut sock) -> i32 {
    let net = sock_net(sk); let group = current_egid();
    if (*sk).sk_family == AF_INET6 { (*sk).sk_ipv6only = 1; }
    let (mut low, mut high) = (kgid_t {}, kgid_t {}); inet_get_ping_group_range_net(net, &mut low, &mut high);
    if gid_lte(low, group) && gid_lte(group, high) { return 0; }
    let groups = get_current_groups();
    for i in 0..(*groups).ngroups { let gid = (*groups).gid[i]; if gid_lte(low, gid) && gid_lte(gid, high) { put_group_info(groups); return 0; } }
    put_group_info(groups); -EACCES
}

pub unsafe fn ping_close(sk: *mut sock, _timeout: i64) { sk_common_release(sk); }

unsafe fn ping_pre_connect(sk: *mut sock, uaddr: *mut sockaddr_unsized, addr_len: i32) -> i32 {
    if addr_len < core::mem::size_of::<sockaddr_in>() as i32 { return -EINVAL; }
    BPF_CGROUP_RUN_PROG_INET4_CONNECT_LOCK(sk, uaddr, &addr_len)
}

unsafe fn ping_check_bind_addr(sk: *mut sock, isk: *mut inet_sock, uaddr: *mut sockaddr_unsized, addr_len: i32) -> i32 {
    if (*sk).sk_family == AF_INET { let addr = uaddr as *mut sockaddr_in; if addr_len < core::mem::size_of::<sockaddr_in>() as i32 { return -EINVAL; } if (*addr).sin_family != AF_INET && !((*addr).sin_family == AF_UNSPEC && (*addr).sin_addr.s_addr == htonl(INADDR_ANY)) { return -EAFNOSUPPORT; } if (*addr).sin_addr.s_addr == htonl(INADDR_ANY) { return 0; } let tb = l3mdev_fib_table_by_index(sock_net(sk), (*sk).sk_bound_dev_if); let ty = inet_addr_type_table(sock_net(sk), (*addr).sin_addr.s_addr, if tb != 0 { tb } else { RT_TABLE_LOCAL }); if ty == RTN_MULTICAST || ty == RTN_BROADCAST || (ty != RTN_LOCAL && !inet_can_nonlocal_bind(sock_net(sk), isk)) { return -EADDRNOTAVAIL; } return 0; }
    if (*sk).sk_family == AF_INET6 { return 0; }
    -EAFNOSUPPORT
}

unsafe fn ping_set_saddr(sk: *mut sock, saddr: *mut sockaddr_unsized) { if (*saddr).sa_family == AF_INET { let isk=inet_sk(sk); let a=saddr as *mut sockaddr_in; (*isk).inet_saddr=(*a).sin_addr.s_addr; WRITE_ONCE!((*isk).inet_rcv_saddr, (*a).sin_addr.s_addr); } }

pub unsafe fn ping_bind(sk: *mut sock, uaddr: *mut sockaddr_unsized, addr_len: i32) -> i32 {
    let isk=inet_sk(sk); let dif=(*sk).sk_bound_dev_if; let mut err=ping_check_bind_addr(sk,isk,uaddr,addr_len); if err!=0{return err;} lock_sock(sk); if (*isk).inet_num!=0 { release_sock(sk); return -EINVAL; } let snum=ntohs((uaddr as *mut sockaddr_in).read().sin_port); err=ping_get_port(sk,snum); if err!=0 {(*sk).sk_bound_dev_if=dif; release_sock(sk); return err;} ping_set_saddr(sk,uaddr); (*isk).inet_sport=htons((*isk).inet_num); (*isk).inet_daddr=0; (*isk).inet_dport=0; sk_dst_reset(sk); release_sock(sk); err=0; err
}

#[inline] unsafe fn ping_supported(family:i32, ty:i32, code:i32)->bool { (family==AF_INET && (ty==ICMP_ECHO || ty==ICMP_EXT_ECHO) && code==0) || (family==AF_INET6 && (ty==ICMPV6_ECHO_REQUEST || ty==ICMPV6_EXT_ECHO_REQUEST) && code==0) }

pub unsafe fn ping_getfrag(from:*mut c_void,to:*mut i8,_offset:i32,fraglen:i32,odd:i32,skb:*mut sk_buff)->i32 { let pfh=from as *mut pingfakehdr; if !csum_and_copy_from_iter_full(to,fraglen,&mut (*pfh).wcheck,&mut (*(*pfh).msg).msg_iter){return -EFAULT;} if (*pfh).family==AF_INET6 {(*skb).csum=csum_block_add((*skb).csum,(*pfh).wcheck,odd);(*skb).ip_summed=CHECKSUM_NONE;(*pfh).wcheck=0;} 0 }

pub unsafe fn ping_common_sendmsg(family:i32,msg:*mut msghdr,len:usize,user_icmph:*mut c_void,icmph_len:usize)->i32 { if len>0xffff{return -EMSGSIZE;} if len<icmph_len{return -EINVAL;} if (*msg).msg_flags&MSG_OOB!=0{return -EOPNOTSUPP;} if memcpy_from_msg(user_icmph,msg,icmph_len)!=0{return -EFAULT;} let (ty,code)=if family==AF_INET{let h=&*(user_icmph as *mut icmphdr);(h.type_,h.code)}else{let h=&*(user_icmph as *mut icmp6hdr);(h.icmp6_type,h.icmp6_code)}; if !ping_supported(family,ty,code){return -EINVAL;} 0 }

// The remaining protocol callbacks retain their kernel ABI and are declared as
// external Rust-facing symbols; their implementations are provided by the
// surrounding translation unit.
extern "C" { pub fn ping_v4_sendmsg(sk:*mut sock,msg:*mut msghdr,len:usize)->i32; pub fn ping_recvmsg(sk:*mut sock,msg:*mut msghdr,len:usize,flags:i32)->i32; pub fn ping_rcv(skb:*mut sk_buff)->skb_drop_reason; }

pub unsafe fn ping_init() { for i in 0..PING_HTABLE_SIZE { INIT_HLIST_HEAD(ping_table.hash.as_mut_ptr().add(i)); } spin_lock_init(&mut ping_table.lock); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
