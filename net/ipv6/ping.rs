// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * INET An implementation of the TCP/IP protocol suite for the LINUX
 * operating system. INET is implemented using the BSD Socket interface.
 *
 * "Ping" sockets
 *
 * Based on ipv4/ping.c code.
 *
 * Authors: Lorenzo Colitti (IPv6 support)
 *          Vasiliy Kulikov / Openwall (IPv4 implementation, for Linux 2.6),
 *          Pavel Kankovsky (IPv4 implementation, for Linux 2.4.32)
 */

// Compatibility glue so we can support IPv6 when it's compiled as a module.
unsafe extern "C" {
    fn ipv6_recv_error(sk: *mut sock, msg: *mut msghdr, len: c_int) -> c_int;
    fn ip6_datagram_recv_common_ctl(sk: *mut sock, msg: *mut msghdr, skb: *mut sk_buff);
    fn ip6_datagram_recv_specific_ctl(sk: *mut sock, msg: *mut msghdr, skb: *mut sk_buff);
    fn icmpv6_err_convert(ty: u8, code: u8, err: *mut c_int) -> c_int;
    fn ipv6_icmp_error(sk: *mut sock, skb: *mut sk_buff, err: c_int, port: __be16,
                       info: u32, payload: *mut u8);
    fn ipv6_chk_addr(net: *mut net, addr: *const in6_addr,
                     dev: *const net_device, strict: c_int) -> c_int;
}

unsafe fn dummy_ipv6_recv_error(_sk: *mut sock, _msg: *mut msghdr, _len: c_int) -> c_int {
    -EAFNOSUPPORT
}

unsafe fn dummy_ip6_datagram_recv_ctl(_sk: *mut sock, _msg: *mut msghdr, _skb: *mut sk_buff) {}

unsafe fn dummy_icmpv6_err_convert(_ty: u8, _code: u8, _err: *mut c_int) -> c_int {
    -EAFNOSUPPORT
}

unsafe fn dummy_ipv6_icmp_error(_sk: *mut sock, _skb: *mut sk_buff, _err: c_int,
                                _port: __be16, _info: u32, _payload: *mut u8) {}

unsafe fn dummy_ipv6_chk_addr(_net: *mut net, _addr: *const in6_addr,
                              _dev: *const net_device, _strict: c_int) -> c_int {
    0
}

unsafe fn ping_v6_pre_connect(sk: *mut sock, uaddr: *mut sockaddr_unsized,
                              addr_len: c_int) -> c_int {
    /* This check is replicated from __ip6_datagram_connect() and intended to
     * prevent BPF program called below from accessing bytes that are out of
     * the bound specified by user in addr_len.
     */
    if addr_len < SIN6_LEN_RFC2133 {
        return -EINVAL;
    }
    BPF_CGROUP_RUN_PROG_INET6_CONNECT_LOCK(sk, uaddr, &addr_len)
}

unsafe fn ping_v6_sendmsg(sk: *mut sock, msg: *mut msghdr, len: size_t) -> c_int {
    let inet = inet_sk(sk);
    let np = inet6_sk(sk);
    let mut user_icmph: icmp6hdr = core::mem::zeroed();
    let mut addr_type: c_int;
    let mut daddr: *mut in6_addr;
    let mut oif: c_int = 0;
    let mut fl6: flowi6 = core::mem::zeroed();
    let mut err: c_int;
    let mut dst: *mut dst_entry;
    let mut rt: *mut rt6_info;
    let mut pfh: pingfakehdr = core::mem::zeroed();
    let mut ipc6: ipcm6_cookie = core::mem::zeroed();

    err = ping_common_sendmsg(AF_INET6, msg, len, &mut user_icmph as *mut _ as *mut _,
                               core::mem::size_of::<icmp6hdr>());
    if err != 0 { return err; }

    core::ptr::write_bytes(&mut fl6 as *mut _, 0, 1);
    if !(*msg).msg_name.is_null() {
        let u = (*msg).msg_name as *mut sockaddr_in6;
        if (*msg).msg_namelen < core::mem::size_of::<sockaddr_in6>() as c_int { return -EINVAL; }
        if (*u).sin6_family != AF_INET6 { return -EAFNOSUPPORT; }
        daddr = &mut (*u).sin6_addr;
        if inet6_test_bit(SNDFLOW, sk) { fl6.flowlabel = (*u).sin6_flowinfo & IPV6_FLOWINFO_MASK; }
        if __ipv6_addr_needs_scope_id(ipv6_addr_type(daddr)) { oif = (*u).sin6_scope_id; }
    } else {
        if (*sk).sk_state != TCP_ESTABLISHED { return -EDESTADDRREQ; }
        daddr = &mut (*sk).sk_v6_daddr;
        fl6.flowlabel = (*np).flow_label;
    }
    if oif == 0 { oif = (*sk).sk_bound_dev_if; }
    if oif == 0 { oif = (*np).sticky_pktinfo.ipi6_ifindex; }
    if oif == 0 && ipv6_addr_is_multicast(daddr) != 0 { oif = READ_ONCE((*np).mcast_oif); }
    else if oif == 0 { oif = READ_ONCE((*np).ucast_oif); }
    addr_type = ipv6_addr_type(daddr);
    if (__ipv6_addr_needs_scope_id(addr_type) != 0 && oif == 0) ||
       (addr_type & IPV6_ADDR_MAPPED) != 0 ||
       (oif != 0 && (*sk).sk_bound_dev_if != 0 && oif != (*sk).sk_bound_dev_if &&
        l3mdev_master_ifindex_by_index(sock_net(sk), oif) != (*sk).sk_bound_dev_if) { return -EINVAL; }
    ipcm6_init_sk(&mut ipc6, sk);
    fl6.flowi6_oif = oif;
    if (*msg).msg_controllen != 0 {
        let mut opt: ipv6_txoptions = core::mem::zeroed();
        opt.tot_len = core::mem::size_of::<ipv6_txoptions>() as c_int;
        ipc6.opt = &mut opt;
        err = ip6_datagram_send_ctl(sock_net(sk), sk, msg, &mut fl6, &mut ipc6);
        if err < 0 { return err; }
        ipc6.opt = core::ptr::null_mut();
    }
    fl6.flowi6_proto = IPPROTO_ICMPV6;
    fl6.saddr = (*np).saddr; fl6.daddr = *daddr; fl6.flowi6_mark = ipc6.sockc.mark;
    fl6.flowi6_uid = sk_uid(sk); fl6.fl6_icmp_type = user_icmph.icmp6_type;
    fl6.fl6_icmp_code = user_icmph.icmp6_code;
    security_sk_classify_flow(sk, flowi6_to_flowi_common(&mut fl6));
    fl6.flowlabel = ip6_make_flowinfo(ipc6.tclass, fl6.flowlabel);
    dst = ip6_sk_dst_lookup_flow(sk, &mut fl6, daddr, false);
    if IS_ERR(dst) { return PTR_ERR(dst); }
    rt = dst_rt6_info(dst);
    if fl6.flowi6_oif == 0 && ipv6_addr_is_multicast(&mut fl6.daddr) != 0 { fl6.flowi6_oif = READ_ONCE((*np).mcast_oif); }
    else if fl6.flowi6_oif == 0 { fl6.flowi6_oif = READ_ONCE((*np).ucast_oif); }
    pfh.icmph.ty = user_icmph.icmp6_type; pfh.icmph.code = user_icmph.icmp6_code;
    pfh.icmph.checksum = 0; pfh.icmph.un_.echo.id = (*inet).inet_sport;
    pfh.icmph.un_.echo.sequence = user_icmph.icmp6_sequence; pfh.msg = msg;
    pfh.wcheck = 0; pfh.family = AF_INET6;
    if ipc6.hlimit < 0 { ipc6.hlimit = ip6_sk_dst_hoplimit(np, &mut fl6, dst); }
    lock_sock(sk);
    err = ip6_append_data(sk, ping_getfrag, &mut pfh, len, core::mem::size_of::<icmp6hdr>(),
                          &mut ipc6, &mut fl6, rt, MSG_DONTWAIT);
    if err != 0 { ICMP6_INC_STATS(sock_net(sk), (*rt).rt6i_idev, ICMP6_MIB_OUTERRORS); ip6_flush_pending_frames(sk); }
    else { icmpv6_push_pending_frames(sk, &mut fl6, &mut pfh.icmph, len); }
    release_sock(sk); dst_release(dst);
    if err != 0 { return err; }
    len as c_int
}

static mut pingv6_prot: proto = proto {
    name: b"PINGv6\0".as_ptr() as *const _, owner: THIS_MODULE, init: Some(ping_init_sock),
    close: Some(ping_close), pre_connect: Some(ping_v6_pre_connect), connect: Some(ip6_datagram_connect_v6_only),
    disconnect: Some(__udp_disconnect), setsockopt: Some(ipv6_setsockopt), getsockopt: Some(ipv6_getsockopt),
    sendmsg: Some(ping_v6_sendmsg), recvmsg: Some(ping_recvmsg), bind: Some(ping_bind),
    backlog_rcv: Some(ping_queue_rcv_skb), unhash: Some(ping_unhash), get_port: Some(ping_get_port),
    put_port: Some(ping_unhash), obj_size: core::mem::size_of::<raw6_sock>(),
    ipv6_pinfo_offset: core::mem::offset_of!(raw6_sock, inet6),
};

static mut pingv6_protosw: inet_protosw = inet_protosw {
    type_: SOCK_DGRAM, protocol: IPPROTO_ICMPV6, prot: &raw mut pingv6_prot,
    ops: &inet6_sockraw_ops, flags: INET_PROTOSW_REUSE,
};

#[cfg(CONFIG_PROC_FS)]
unsafe fn ping_v6_seq_start(seq: *mut seq_file, pos: *mut loff_t) -> *mut core::ffi::c_void { ping_seq_start(seq, pos, AF_INET6) }

#[cfg(CONFIG_PROC_FS)]
unsafe fn ping_v6_seq_show(seq: *mut seq_file, v: *mut core::ffi::c_void) -> c_int {
    if v == SEQ_START_TOKEN { seq_puts(seq, IPV6_SEQ_DGRAM_HEADER); }
    else { let bucket = (*( (*seq).private as *mut ping_iter_state)).bucket; let inet = inet_sk(v as *mut sock); ip6_dgram_sock_seq_show(seq, v, ntohs((*inet).inet_sport), ntohs((*inet).inet_dport), bucket); }
    0
}

#[cfg(CONFIG_PROC_FS)]
static ping_v6_seq_ops: seq_operations = seq_operations { start: Some(ping_v6_seq_start), show: Some(ping_v6_seq_show), next: Some(ping_seq_next), stop: Some(ping_seq_stop) };

#[cfg(CONFIG_PROC_FS)]
unsafe fn ping_v6_proc_init_net(net: *mut net) -> c_int { if proc_create_net(b"icmp6\0".as_ptr() as *const _, 0o444, (*net).proc_net, &ping_v6_seq_ops, core::mem::size_of::<ping_iter_state>()).is_null() { -ENOMEM } else { 0 } }

#[cfg(CONFIG_PROC_FS)]
unsafe fn ping_v6_proc_exit_net(net: *mut net) { remove_proc_entry(b"icmp6\0".as_ptr() as *const _, (*net).proc_net); }

#[cfg(CONFIG_PROC_FS)]
static mut ping_v6_net_ops: pernet_operations = pernet_operations { init: Some(ping_v6_proc_init_net), exit: Some(ping_v6_proc_exit_net) };

unsafe fn pingv6_init() -> c_int {
    #[cfg(CONFIG_PROC_FS)] { let ret = register_pernet_subsys(&raw mut ping_v6_net_ops); if ret != 0 { return ret; } }
    pingv6_ops.ipv6_recv_error = Some(ipv6_recv_error); pingv6_ops.ip6_datagram_recv_common_ctl = Some(ip6_datagram_recv_common_ctl);
    pingv6_ops.ip6_datagram_recv_specific_ctl = Some(ip6_datagram_recv_specific_ctl); pingv6_ops.icmpv6_err_convert = Some(icmpv6_err_convert);
    pingv6_ops.ipv6_icmp_error = Some(ipv6_icmp_error); pingv6_ops.ipv6_chk_addr = Some(ipv6_chk_addr);
    inet6_register_protosw(&raw mut pingv6_protosw)
}

/* This never gets called because it's not possible to unload the ipv6 module,
 * but just in case.
 */
unsafe fn pingv6_exit() {
    pingv6_ops.ipv6_recv_error = Some(dummy_ipv6_recv_error); pingv6_ops.ip6_datagram_recv_common_ctl = Some(dummy_ip6_datagram_recv_ctl);
    pingv6_ops.ip6_datagram_recv_specific_ctl = Some(dummy_ip6_datagram_recv_ctl); pingv6_ops.icmpv6_err_convert = Some(dummy_icmpv6_err_convert);
    pingv6_ops.ipv6_icmp_error = Some(dummy_ipv6_icmp_error); pingv6_ops.ipv6_chk_addr = Some(dummy_ipv6_chk_addr);
    #[cfg(CONFIG_PROC_FS)] unregister_pernet_subsys(&raw mut ping_v6_net_ops);
    inet6_unregister_protosw(&raw mut pingv6_protosw);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
