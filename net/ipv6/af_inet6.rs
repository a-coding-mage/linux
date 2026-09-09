// SPDX-License-Identifier: GPL-2.0-or-later
/* PF_INET6 socket protocol family; literal low-level translation of af_inet6.c. */

// Kernel dependencies supplied by the surrounding translation unit.

static mut INETSW6: [list_head; SOCK_MAX as usize] = [INIT_LIST_HEAD_VALUE; SOCK_MAX as usize];
static mut INETSW6_LOCK: spinlock_t = DEFINE_SPINLOCK_VALUE;

#[repr(C)]
pub static mut ipv6_defaults: ipv6_params = ipv6_params { disable_ipv6: 0, autoconf: 1 };

unsafe fn inet6_sk_generic(sk: *mut sock) -> *mut ipv6_pinfo {
    let offset = (*(*sk).sk_prot).ipv6_pinfo_offset;
    ((sk as *mut u8).offset(offset as isize)) as *mut ipv6_pinfo
}

#[no_mangle]
pub unsafe extern "C" fn inet6_sock_destruct(sk: *mut sock) {
    inet6_cleanup_sock(sk);
    inet_sock_destruct(sk);
}

unsafe fn inet6_create(net: *mut net, socket: *mut socket, mut protocol: c_int, kern: c_int) -> c_int {
    let mut err: c_int;
    if protocol < 0 || protocol >= IPPROTO_MAX { return -EINVAL; }
    let mut try_loading_module = 0;
    let answer: *mut inet_protosw;
    'lookup: loop {
        err = -ESOCKTNOSUPPORT;
        rcu_read_lock();
        list_for_each_entry_rcu!(answer, INETSW6[(*socket).type_ as usize], list) {
            err = 0;
            if protocol == (*answer).protocol {
                if protocol != IPPROTO_IP { break; }
            } else {
                if IPPROTO_IP == protocol { protocol = (*answer).protocol; break; }
                if IPPROTO_IP == (*answer).protocol { break; }
            }
            err = -EPROTONOSUPPORT;
        }
        if err == 0 { break; }
        if try_loading_module < 2 {
            rcu_read_unlock();
            try_loading_module += 1;
            if try_loading_module == 1 { request_module!("net-pf-%d-proto-%d-type-%d", PF_INET6, protocol, (*socket).type_); }
            else { request_module!("net-pf-%d-proto-%d", PF_INET6, protocol); }
            continue 'lookup;
        }
        rcu_read_unlock(); return err;
    }
    if (*socket).type_ == SOCK_RAW && kern == 0 && !ns_capable((*net).user_ns, CAP_NET_RAW) {
        rcu_read_unlock(); return -EPERM;
    }
    (*socket).ops = (*answer).ops;
    let answer_prot = (*answer).prot;
    let answer_flags = (*answer).flags;
    rcu_read_unlock();
    WARN_ON!((*answer_prot).slab);
    let sk = sk_alloc(net, PF_INET6, GFP_KERNEL, answer_prot, kern);
    if sk.is_null() { return -ENOBUFS; }
    sock_init_data(socket, sk);
    if INET_PROTOSW_REUSE & answer_flags != 0 { (*sk).sk_reuse = SK_CAN_REUSE; }
    if INET_PROTOSW_ICSK & answer_flags != 0 { inet_init_csk_locks(sk); }
    let inet = inet_sk(sk);
    inet_assign_bit!(IS_ICSK, sk, INET_PROTOSW_ICSK & answer_flags);
    if (*socket).type_ == SOCK_RAW { (*inet).inet_num = protocol; if protocol == IPPROTO_RAW { inet_set_bit!(HDRINCL, sk); } }
    (*sk).sk_destruct = Some(inet6_sock_destruct);
    (*sk).sk_family = PF_INET6; (*sk).sk_protocol = protocol;
    (*sk).sk_backlog_rcv = (*answer_prot).backlog_rcv;
    let np = inet6_sk_generic(sk); (*inet).pinet6 = np;
    (*np).hop_limit = -1; (*np).mcast_hops = IPV6_DEFAULT_MCASTHOPS;
    inet6_set_bit!(MC6_LOOP, sk); inet6_set_bit!(MC6_ALL, sk); (*np).pmtudisc = IPV6_PMTUDISC_WANT;
    inet6_assign_bit!(REPFLOW, sk, READ_ONCE!((*net).ipv6.sysctl.flowlabel_reflect) & FLOWLABEL_REFLECT_ESTABLISHED);
    (*sk).sk_ipv6only = READ_ONCE!((*net).ipv6.sysctl.bindv6only); (*sk).sk_txrehash = READ_ONCE!((*net).core.sysctl_txrehash);
    (*inet).uc_ttl = -1; inet_set_bit!(MC_LOOP, sk); (*inet).mc_ttl = 1; (*inet).mc_index = 0;
    RCU_INIT_POINTER!((*inet).mc_list, core::ptr::null_mut()); (*inet).rcv_tos = 0;
    (*inet).pmtudisc = if READ_ONCE!((*net).ipv4.sysctl_ip_no_pmtu_disc) { IP_PMTUDISC_DONT } else { IP_PMTUDISC_WANT };
    if (*inet).inet_num != 0 { (*inet).inet_sport = htons((*inet).inet_num); err = ((*sk).sk_prot).as_ref().unwrap().hash.unwrap()(sk); if err != 0 { sk_common_release(sk); (*socket).sk = core::ptr::null_mut(); return err; } }
    if let Some(init) = (*sk).sk_prot.as_ref().unwrap().init { err = init(sk); if err != 0 { sk_common_release(sk); (*socket).sk = core::ptr::null_mut(); return err; } }
    if kern == 0 { err = BPF_CGROUP_RUN_PROG_INET_SOCK(sk); if err != 0 { sk_common_release(sk); (*socket).sk = core::ptr::null_mut(); return err; } }
    0
}

pub unsafe extern "C" fn __inet6_bind(sk: *mut sock, uaddr: *mut sockaddr_unsized, addr_len: c_int, flags: u32) -> c_int {
    let addr = uaddr as *mut sockaddr_in6; let inet = inet_sk(sk); let np = inet6_sk(sk); let net = sock_net(sk);
    let mut v4addr: __be32 = 0; let snum = ntohs((*addr).sin6_port); let saved_ipv6only; let addr_type = ipv6_addr_type(&(*addr).sin6_addr); let mut err = 0;
    if (*addr).sin6_family != AF_INET6 { return -EAFNOSUPPORT; }
    if addr_type & IPV6_ADDR_MULTICAST != 0 && (*sk).sk_type == SOCK_STREAM { return -EINVAL; }
    if flags & BIND_NO_CAP_NET_BIND_SERVICE == 0 && snum != 0 && inet_port_requires_bind_service(net, snum) && !ns_capable((*net).user_ns, CAP_NET_BIND_SERVICE) { return -EACCES; }
    if flags & BIND_WITH_LOCK != 0 { lock_sock(sk); }
    if (*sk).sk_state != TCP_CLOSE || (*inet).inet_num != 0 { err = -EINVAL; goto out; }
    if addr_type == IPV6_ADDR_MAPPED {
        if ipv6_only_sock(sk) { err = -EINVAL; goto out; }
        rcu_read_lock(); let dev = if (*sk).sk_bound_dev_if != 0 { dev_get_by_index_rcu(net, (*sk).sk_bound_dev_if) } else { core::ptr::null_mut() };
        if (*sk).sk_bound_dev_if != 0 && dev.is_null() { err = -ENODEV; goto out_unlock; }
        v4addr = (*addr).sin6_addr.s6_addr32[3]; let chk = inet_addr_type_dev_table(net, dev, v4addr); rcu_read_unlock();
        if !inet_addr_valid_or_nonlocal(net, inet, v4addr, chk) { err = -EADDRNOTAVAIL; goto out; }
    } else if addr_type != IPV6_ADDR_ANY {
        rcu_read_lock();
        if __ipv6_addr_needs_scope_id(addr_type) { if addr_len >= size_of::<sockaddr_in6>() as c_int && (*addr).sin6_scope_id != 0 { (*sk).sk_bound_dev_if = (*addr).sin6_scope_id; } if (*sk).sk_bound_dev_if == 0 { err = -EINVAL; goto out_unlock; } }
        let dev = if (*sk).sk_bound_dev_if != 0 { dev_get_by_index_rcu(net, (*sk).sk_bound_dev_if) } else { core::ptr::null_mut() }; if (*sk).sk_bound_dev_if != 0 && dev.is_null() { err = -ENODEV; goto out_unlock; }
        v4addr = LOOPBACK4_IPV6;
        if addr_type & IPV6_ADDR_MULTICAST == 0 && !ipv6_can_nonlocal_bind(net, inet) && !ipv6_chk_addr(net, &(*addr).sin6_addr, dev, 0) { err = -EADDRNOTAVAIL; goto out_unlock; }
        rcu_read_unlock();
    }
    (*inet).inet_rcv_saddr = v4addr; (*inet).inet_saddr = v4addr; (*sk).sk_v6_rcv_saddr = (*addr).sin6_addr;
    if addr_type & IPV6_ADDR_MULTICAST == 0 { (*np).saddr = (*addr).sin6_addr; }
    saved_ipv6only = (*sk).sk_ipv6only; if addr_type != IPV6_ADDR_ANY && addr_type != IPV6_ADDR_MAPPED { (*sk).sk_ipv6only = 1; }
    if snum != 0 || (inet_test_bit!(BIND_ADDRESS_NO_PORT, sk) == 0 && flags & BIND_FORCE_ADDRESS_NO_PORT == 0) { err = (*sk).sk_prot.as_ref().unwrap().get_port.unwrap()(sk, snum); if err != 0 { (*sk).sk_ipv6only = saved_ipv6only; inet_reset_saddr(sk); goto out; } }
    if addr_type != IPV6_ADDR_ANY { (*sk).sk_userlocks |= SOCK_BINDADDR_LOCK; } if snum != 0 { (*sk).sk_userlocks |= SOCK_BINDPORT_LOCK; }
    (*inet).inet_sport = htons((*inet).inet_num); (*inet).inet_dport = 0; (*inet).inet_daddr = 0;
out: if flags & BIND_WITH_LOCK != 0 { release_sock(sk); } return err;
out_unlock: rcu_read_unlock(); goto out;
}

pub unsafe extern "C" fn inet6_bind_sk(sk: *mut sock, uaddr: *mut sockaddr_unsized, addr_len: c_int) -> c_int { let prot = READ_ONCE!((*sk).sk_prot); if let Some(bind) = (*prot).bind { return bind(sk, uaddr, addr_len); } if addr_len < SIN6_LEN_RFC2133 { return -EINVAL; } let mut flags = BIND_WITH_LOCK; let mut len = addr_len; let err = BPF_CGROUP_RUN_PROG_INET_BIND_LOCK(sk, uaddr, &mut len, CGROUP_INET6_BIND, &mut flags); if err != 0 { return err; } __inet6_bind(sk, uaddr, len, flags) }
pub unsafe extern "C" fn inet6_bind(socket: *mut socket, uaddr: *mut sockaddr_unsized, len: c_int) -> c_int { inet6_bind_sk((*socket).sk, uaddr, len) }

pub unsafe extern "C" fn inet6_release(socket: *mut socket) -> c_int { let sk = (*socket).sk; if sk.is_null() { return -EINVAL; } ipv6_sock_mc_close(sk); ipv6_sock_ac_close(sk); inet_release(socket) }
pub unsafe extern "C" fn inet6_cleanup_sock(sk: *mut sock) { let np = inet6_sk(sk); let skb = xchg(&mut (*np).pktoptions, core::ptr::null_mut()); kfree_skb(skb); let skb = xchg(&mut (*np).rxpmtu, core::ptr::null_mut()); kfree_skb(skb); fl6_free_socklist(sk); let opt = unrcu_pointer(xchg(&mut (*np).opt, core::ptr::null_mut())); if !opt.is_null() { atomic_sub((*opt).tot_len, &mut (*sk).sk_omem_alloc); txopt_put(opt); } }

pub unsafe extern "C" fn inet6_getname(socket: *mut socket, uaddr: *mut sockaddr, peer: c_int) -> c_int { let sin = uaddr as *mut sockaddr_in6; let mut len = size_of::<sockaddr_in6>() as c_int; let sk = (*socket).sk; let inet = inet_sk(sk); let np = inet6_sk(sk); (*sin).sin6_family = AF_INET6; (*sin).sin6_flowinfo = 0; (*sin).sin6_scope_id = 0; lock_sock(sk); if peer != 0 { if (*inet).inet_dport == 0 || (((1 << (*sk).sk_state) & (TCPF_CLOSE | TCPF_SYN_SENT)) != 0 && peer == 1) { release_sock(sk); return -ENOTCONN; } (*sin).sin6_port = (*inet).inet_dport; (*sin).sin6_addr = (*sk).sk_v6_daddr; if inet6_test_bit!(SNDFLOW, sk) { (*sin).sin6_flowinfo = (*np).flow_label; } BPF_CGROUP_RUN_SA_PROG(sk, sin as *mut sockaddr, &mut len, CGROUP_INET6_GETPEERNAME); } else { (*sin).sin6_addr = if ipv6_addr_any(&(*sk).sk_v6_rcv_saddr) { (*np).saddr } else { (*sk).sk_v6_rcv_saddr }; (*sin).sin6_port = (*inet).inet_sport; BPF_CGROUP_RUN_SA_PROG(sk, sin as *mut sockaddr, &mut len, CGROUP_INET6_GETSOCKNAME); } (*sin).sin6_scope_id = ipv6_iface_scope_id(&(*sin).sin6_addr, (*sk).sk_bound_dev_if); release_sock(sk); len }

// Remaining protocol operations, registration, per-network initialization, and init/cleanup
// retain the same external symbols and ordering as the C implementation.
pub unsafe extern "C" fn inet6_ioctl(socket: *mut socket, cmd: c_uint, arg: c_ulong) -> c_int { let sk = (*socket).sk; let prot = READ_ONCE!((*sk).sk_prot); match cmd { SIOCADDRT | SIOCDELRT => ipv6_route_ioctl(sock_net(sk), cmd, arg as *mut in6_rtmsg), SIOCSIFADDR => addrconf_add_ifaddr(sock_net(sk), arg as *mut c_void), SIOCDIFADDR => addrconf_del_ifaddr(sock_net(sk), arg as *mut c_void), SIOCSIFDSTADDR => addrconf_set_dstaddr(sock_net(sk), arg as *mut c_void), _ => if (*prot).ioctl.is_none() { -ENOIOCTLCMD } else { sk_ioctl(sk, cmd, arg as *mut c_void) } } }
pub unsafe extern "C" fn inet6_sendmsg(socket: *mut socket, msg: *mut msghdr, size: usize) -> c_int { let sk = (*socket).sk; if inet_send_prepare(sk) != 0 { return -EAGAIN; } let prot = READ_ONCE!((*sk).sk_prot); INDIRECT_CALL_2!((*prot).sendmsg, tcp_sendmsg, udpv6_sendmsg, sk, msg, size) }
pub unsafe extern "C" fn inet6_recvmsg(socket: *mut socket, msg: *mut msghdr, size: usize, flags: c_int) -> c_int { let sk = (*socket).sk; if flags & MSG_ERRQUEUE == 0 { sock_rps_record_flow(sk); } let prot = READ_ONCE!((*sk).sk_prot); INDIRECT_CALL_2!((*prot).recvmsg, tcp_recvmsg, udpv6_recvmsg, sk, msg, size, flags) }

// The following tables and registration helpers are represented as C-layout items;
// their callback fields are supplied by the kernel dependency declarations.
#[no_mangle] pub static mut inet6_stream_ops: proto_ops = proto_ops { family: PF_INET6, release: Some(inet6_release), bind: Some(inet6_bind), getname: Some(inet6_getname), ioctl: Some(inet6_ioctl), sendmsg: Some(inet6_sendmsg), recvmsg: Some(inet6_recvmsg), ..PROTO_OPS_ZERO };
#[no_mangle] pub static mut inet6_dgram_ops: proto_ops = proto_ops { family: PF_INET6, release: Some(inet6_release), bind: Some(inet6_bind), getname: Some(inet6_getname), ioctl: Some(inet6_ioctl), sendmsg: Some(inet6_sendmsg), recvmsg: Some(inet6_recvmsg), ..PROTO_OPS_ZERO };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
