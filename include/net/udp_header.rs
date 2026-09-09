/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Definitions for the UDP module, translated from udp.h. */

#[repr(C)]
pub struct udp_skb_cb {
    pub header: udp_skb_cb_header,
}

#[repr(C)]
pub union udp_skb_cb_header {
    pub h4: inet_skb_parm,
    // CONFIG_IPV6: pub h6: inet6_skb_parm,
}

#[inline]
pub unsafe fn UDP_SKB_CB(__skb: *mut sk_buff) -> *mut udp_skb_cb {
    (*__skb).cb.as_mut_ptr() as *mut udp_skb_cb
}

#[repr(C, align(2))]
pub struct udp_hslot {
    pub head: udp_hslot_head,
    pub count: c_int,
    pub lock: spinlock_t,
}

#[repr(C)]
pub union udp_hslot_head {
    pub head: hlist_head,
    pub nulls_head: hlist_nulls_head,
}

#[repr(C, align(2))]
pub struct udp_hslot_main {
    pub hslot: udp_hslot,
    // !CONFIG_BASE_SMALL: pub hash4_cnt: u32,
}

#[inline]
pub unsafe fn UDP_HSLOT_MAIN(__hslot: *mut udp_hslot) -> *mut udp_hslot_main {
    __hslot as *mut udp_hslot_main
}

#[repr(C)]
pub struct udp_table {
    pub hash: *mut udp_hslot,
    pub hash2: *mut udp_hslot_main,
    // !CONFIG_BASE_SMALL: pub hash4: *mut udp_hslot,
    pub mask: c_uint,
    pub log: c_uint,
}

pub static mut udp_table: udp_table;

#[inline]
pub unsafe fn udp_hashslot(table: *mut udp_table, net: *const net, num: c_uint) -> *mut udp_hslot {
    (*table).hash.add(udp_hashfn(net, num, (*table).mask) as usize)
}

#[inline]
pub unsafe fn udp_hashslot2(table: *mut udp_table, hash: c_uint) -> *mut udp_hslot {
    &mut (*(*table).hash2.add((hash & (*table).mask) as usize)).hslot
}

// CONFIG_BASE_SMALL selects the stub implementations below; otherwise use the full versions.
#[cfg(CONFIG_BASE_SMALL)]
pub unsafe fn udp_table_hash4_init(_table: *mut udp_table) {}
#[cfg(CONFIG_BASE_SMALL)]
pub unsafe fn udp_hashslot4(_table: *mut udp_table, _hash: c_uint) -> *mut udp_hslot { BUILD_BUG!(); core::ptr::null_mut() }
#[cfg(CONFIG_BASE_SMALL)]
pub unsafe fn udp_hashed4(_sk: *const sock) -> bool { false }
#[cfg(CONFIG_BASE_SMALL)]
pub unsafe fn udp_hash4_slot_size() -> c_uint { 0 }
#[cfg(CONFIG_BASE_SMALL)]
pub unsafe fn udp_has_hash4(_hslot2: *const udp_hslot) -> bool { false }
#[cfg(CONFIG_BASE_SMALL)]
pub unsafe fn udp_hash4_inc(_hslot2: *mut udp_hslot) {}
#[cfg(CONFIG_BASE_SMALL)]
pub unsafe fn udp_hash4_dec(_hslot2: *mut udp_hslot) {}

#[cfg(not(CONFIG_BASE_SMALL))]
pub unsafe fn udp_table_hash4_init(table: *mut udp_table) {
    (*table).hash4 = ((*table).hash2.add(((*table).mask + 1) as usize)) as *mut udp_hslot;
    for i in 0..=(*table).mask {
        (*(*table).hash2.add(i as usize)).hash4_cnt = 0;
        INIT_HLIST_NULLS_HEAD(&mut (*(*table).hash4.add(i as usize)).nulls_head, i);
        (*(*table).hash4.add(i as usize)).count = 0;
        spin_lock_init(&mut (*(*table).hash4.add(i as usize)).lock);
    }
}
#[cfg(not(CONFIG_BASE_SMALL))]
pub unsafe fn udp_hashslot4(table: *mut udp_table, hash: c_uint) -> *mut udp_hslot { (*table).hash4.add((hash & (*table).mask) as usize) }
#[cfg(not(CONFIG_BASE_SMALL))]
pub unsafe fn udp_hashed4(sk: *const sock) -> bool { !hlist_nulls_unhashed(&(*udp_sk(sk)).udp_lrpa_node) }
#[cfg(not(CONFIG_BASE_SMALL))]
pub unsafe fn udp_hash4_slot_size() -> usize { core::mem::size_of::<udp_hslot>() }
#[cfg(not(CONFIG_BASE_SMALL))]
pub unsafe fn udp_has_hash4(hslot2: *const udp_hslot) -> bool { (*UDP_HSLOT_MAIN(hslot2 as *mut _)).hash4_cnt != 0 }
#[cfg(not(CONFIG_BASE_SMALL))]
pub unsafe fn udp_hash4_inc(hslot2: *mut udp_hslot) { (*UDP_HSLOT_MAIN(hslot2)).hash4_cnt += 1; }
#[cfg(not(CONFIG_BASE_SMALL))]
pub unsafe fn udp_hash4_dec(hslot2: *mut udp_hslot) { (*UDP_HSLOT_MAIN(hslot2)).hash4_cnt -= 1; }

pub static mut udp_prot: proto;
// DECLARE_PER_CPU(int, udp_memory_per_cpu_fw_alloc);
pub static mut sysctl_udp_mem: [c_long; 3];
pub static mut sysctl_udp_rmem_min: c_int;
pub static mut sysctl_udp_wmem_min: c_int;

#[inline]
pub unsafe fn __udp_lib_checksum_complete(skb: *mut sk_buff) -> __sum16 { __skb_checksum_complete(skb) }
#[inline]
pub unsafe fn udp_lib_checksum_complete(skb: *mut sk_buff) -> c_int { (!skb_csum_unnecessary(skb) && __udp_lib_checksum_complete(skb) != 0) as c_int }

#[inline]
pub unsafe fn udp_csum_outgoing(sk: *mut sock, skb: *mut sk_buff) -> __wsum {
    let mut csum = csum_partial(skb_transport_header(skb), core::mem::size_of::<udphdr>(), 0);
    skb_queue_walk!(&mut (*sk).sk_write_queue, skb, { csum = csum_add(csum, (*skb).csum); });
    csum
}

#[inline]
pub unsafe fn udp_csum(mut skb: *mut sk_buff) -> __wsum {
    let mut csum = csum_partial(skb_transport_header(skb), core::mem::size_of::<udphdr>(), (*skb).csum);
    while !skb.is_null() { csum = csum_add(csum, (*skb).csum); skb = (*skb).next; }
    csum
}

#[inline]
pub unsafe fn udp_v4_check(len: c_int, saddr: __be32, daddr: __be32, base: __wsum) -> __sum16 { csum_tcpudp_magic(saddr, daddr, len, IPPROTO_UDP, base) }
pub fn udp_set_csum(nocheck: bool, skb: *mut sk_buff, saddr: __be32, daddr: __be32, len: c_int);

#[inline]
pub unsafe fn udp_csum_pull_header(skb: *mut sk_buff) {
    if !(*skb).csum_valid && (*skb).ip_summed == CHECKSUM_NONE { (*skb).csum = csum_partial((*skb).data, core::mem::size_of::<udphdr>(), (*skb).csum); }
    skb_pull_rcsum(skb, core::mem::size_of::<udphdr>());
}

pub type udp_lookup_t = Option<unsafe extern "C" fn(*const sk_buff, __be16, __be16) -> *mut sock>;
pub fn udp_v6_early_demux(skb: *mut sk_buff);
pub fn udpv6_rcv(skb: *mut sk_buff) -> c_int;
pub fn udpv6_sendmsg(sk: *mut sock, msg: *mut msghdr, len: usize) -> c_int;
pub fn udpv6_recvmsg(sk: *mut sock, msg: *mut msghdr, len: usize, flags: c_int) -> c_int;
pub fn __udp_gso_segment(gso_skb: *mut sk_buff, features: netdev_features_t, is_ipv6: bool) -> *mut sk_buff;

#[inline]
pub unsafe fn udp_lib_init_sock(sk: *mut sock) -> c_int {
    let up = udp_sk(sk); (*sk).sk_drop_counters = &mut (*up).drop_counters; skb_queue_head_init(&mut (*up).reader_queue); INIT_HLIST_NODE(&mut (*up).tunnel_list); (*up).forward_threshold = (*sk).sk_rcvbuf >> 2; set_bit(SOCK_CUSTOM_SOCKOPT, &mut (*(*sk).sk_socket).flags);
    (*up).udp_prod_queue = kzalloc_objs!((*up).udp_prod_queue, nr_node_ids); if (*up).udp_prod_queue.is_null() { return -ENOMEM; }
    for i in 0..nr_node_ids { init_llist_head(&mut (*(*up).udp_prod_queue.add(i as usize)).ll_root); } 0
}
#[inline] pub unsafe fn udp_drops_inc(sk: *mut sock) { numa_drop_add(&mut (*udp_sk(sk)).drop_counters, 1); }
#[inline] pub unsafe fn udp_lib_hash(_sk: *mut sock) -> c_int { BUG!(); 0 }
pub fn udp_lib_unhash(sk: *mut sock);
pub fn udp_lib_rehash(sk: *mut sock, new_hash: u16, new_hash4: u16);
pub fn udp_ehashfn(net: *const net, laddr: __be32, lport: __u16, faddr: __be32, fport: __be16) -> u32;
#[inline] pub unsafe fn udp_lib_close(sk: *mut sock, _timeout: c_long) { sk_common_release(sk); }

#[cfg(CONFIG_BASE_SMALL)] #[inline] pub unsafe fn udp_lib_hash4(_sk: *mut sock, _hash: u16) {}
#[cfg(CONFIG_BASE_SMALL)] #[inline] pub unsafe fn udp4_hash4(_sk: *mut sock) {}
#[cfg(not(CONFIG_BASE_SMALL))] pub fn udp_lib_hash4(sk: *mut sock, hash: u16);
#[cfg(not(CONFIG_BASE_SMALL))] pub fn udp4_hash4(sk: *mut sock);
pub fn udp_lib_get_port(sk: *mut sock, snum: c_ushort, hash2_nulladdr: c_uint) -> c_int;
pub fn udp_flow_hashrnd() -> u32;

#[inline]
pub unsafe fn udp_flow_src_port(net: *mut net, skb: *mut sk_buff, mut min: c_int, mut max: c_int, use_eth: bool) -> __be16 {
    if min >= max { inet_get_local_port_range(net, &mut min, &mut max); }
    let mut hash = skb_get_hash(skb);
    if hash == 0 { hash = if use_eth { jhash((*skb).data, 2 * ETH_ALEN, (*skb).protocol as u32) } else { udp_flow_hashrnd() }; }
    hash ^= hash << 16; htons(reciprocal_scale(hash, (max - min + 1) as u32) + min as u32)
}
#[inline] pub unsafe fn udp_rqueue_get(sk: *mut sock) -> c_int { sk_rmem_alloc_get(sk) - READ_ONCE!((*udp_sk(sk)).forward_deficit) }
#[inline] pub unsafe fn udp_sk_bound_dev_eq(net: *const net, bound_dev_if: c_int, dif: c_int, sdif: c_int) -> bool { inet_bound_dev_eq(true, bound_dev_if, dif, sdif) }

pub fn udp_destruct_common(sk: *mut sock); pub fn skb_consume_udp(sk: *mut sock, skb: *mut sk_buff, len: c_int); pub fn __udp_enqueue_schedule_skb(sk: *mut sock, skb: *mut sk_buff) -> c_int; pub fn udp_skb_destructor(sk: *mut sock, skb: *mut sk_buff);
pub fn __skb_recv_udp(sk: *mut sock, flags: c_uint, off: *mut c_int, err: *mut c_int) -> *mut sk_buff;
#[inline] pub unsafe fn skb_recv_udp(sk: *mut sock, flags: c_uint, err: *mut c_int) -> *mut sk_buff { let mut off = 0; __skb_recv_udp(sk, flags, &mut off, err) }

pub fn udp_v4_early_demux(skb: *mut sk_buff) -> skb_drop_reason; pub fn udp_sk_rx_dst_set(sk: *mut sock, dst: *mut dst_entry) -> bool; pub fn udp_err(skb: *mut sk_buff, info: u32) -> c_int; pub fn udp_abort(sk: *mut sock, err: c_int) -> c_int; pub fn udp_sendmsg(sk: *mut sock, msg: *mut msghdr, len: usize) -> c_int; pub fn udp_recvmsg(sk: *mut sock, msg: *mut msghdr, len: usize, flags: c_int) -> c_int; pub fn udp_splice_eof(sock: *mut socket); pub fn udp_push_pending_frames(sk: *mut sock) -> c_int; pub fn udp_flush_pending_frames(sk: *mut sock); pub fn udp_cmsg_send(sk: *mut sock, msg: *mut msghdr, gso_size: *mut u16) -> c_int; pub fn udp4_hwcsum(skb: *mut sk_buff, src: __be32, dst: __be32); pub fn udp_rcv(skb: *mut sk_buff) -> c_int; pub fn udp_ioctl(sk: *mut sock, cmd: c_int, karg: *mut c_int) -> c_int; pub fn udp_pre_connect(sk: *mut sock, uaddr: *mut sockaddr_unsized, addr_len: c_int) -> c_int; pub fn __udp_disconnect(sk: *mut sock, flags: c_int) -> c_int; pub fn udp_disconnect(sk: *mut sock, flags: c_int) -> c_int; pub fn udp_poll(file: *mut file, sock: *mut socket, wait: *mut poll_table) -> __poll_t;
pub fn skb_udp_tunnel_segment(skb: *mut sk_buff, features: netdev_features_t, is_ipv6: bool) -> *mut sk_buff; pub fn udp_lib_getsockopt(sk: *mut sock, level: c_int, optname: c_int, opt: *mut sockopt_t) -> c_int; pub fn udp_lib_setsockopt(sk: *mut sock, level: c_int, optname: c_int, optval: sockptr_t, optlen: c_uint, push_pending_frames: Option<unsafe extern "C" fn(*mut sock) -> c_int>) -> c_int;

#[repr(C)] pub struct udp_dev_scratch { pub _tsize_state: u32, /* 64-bit builds: */ pub len: u16, pub is_linear: bool, pub csum_unnecessary: bool }
#[inline] pub unsafe fn udp_skb_scratch(skb: *mut sk_buff) -> *mut udp_dev_scratch { &mut (*skb).dev_scratch as *mut _ as *mut udp_dev_scratch }
#[inline] pub unsafe fn udp_skb_len(skb: *mut sk_buff) -> c_uint { (*udp_skb_scratch(skb)).len as c_uint }
#[inline] pub unsafe fn udp_skb_csum_unnecessary(skb: *mut sk_buff) -> bool { (*udp_skb_scratch(skb)).csum_unnecessary }
#[inline] pub unsafe fn udp_skb_is_linear(skb: *mut sk_buff) -> bool { (*udp_skb_scratch(skb)).is_linear }
#[inline] pub unsafe fn copy_linear_skb(skb: *mut sk_buff, len: c_int, off: c_int, to: *mut iov_iter) -> c_int { if copy_to_iter_full!((*skb).data.add(off as usize), len, to) { 0 } else { -EFAULT } }

/* SNMP statistics macros are represented by their original external operations. */
#[inline] pub unsafe fn __UDP_INC_STATS(net: *mut net, field: c_int) { __SNMP_INC_STATS!((*net).mib.udp_statistics, field); }
#[inline] pub unsafe fn UDP_INC_STATS(net: *mut net, field: c_int) { SNMP_INC_STATS!((*net).mib.udp_statistics, field); }
#[inline] pub unsafe fn __UDP6_INC_STATS(net: *mut net, field: c_int) { __SNMP_INC_STATS!((*net).mib.udp_stats_in6, field); }
#[inline] pub unsafe fn UDP6_INC_STATS(net: *mut net, field: c_int) { SNMP_INC_STATS!((*net).mib.udp_stats_in6, field); }

#[repr(C)] pub struct udp_seq_afinfo { pub family: sa_family_t }
#[repr(C)] pub struct udp_iter_state { pub p: seq_net_private, pub bucket: c_int }
pub fn udp_seq_start(seq: *mut seq_file, pos: *mut loff_t) -> *mut c_void; pub fn udp_seq_next(seq: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void; pub fn udp_seq_stop(seq: *mut seq_file, v: *mut c_void);
pub fn udp4_proc_init() -> c_int; pub fn udp4_proc_exit();
pub fn udpv4_offload_init() -> c_int; pub fn udp_init();
pub fn udp_encap_enable(); pub fn udp_encap_disable(); pub fn udpv6_encap_enable();
pub fn udp_bpf_update_proto(sk: *mut sock, psock: *mut sk_psock, restore: bool) -> c_int;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
