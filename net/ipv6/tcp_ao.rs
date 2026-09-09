// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * INET		An implementation of the TCP Authentication Option (TCP-AO).
 *		See RFC5925.
 *
 * Authors:	Dmitry Safonov <dima@arista.com>
 *		Francesco Ruggeri <fruggeri@arista.com>
 *		Salam Noureddine <noureddine@arista.com>
 */

#[repr(C, packed)]
struct KdfInputBlock {
    counter: u8,
    label: [u8; 6],
    ctx: tcp6_ao_context,
    outlen: __be16,
}

unsafe fn tcp_v6_ao_calc_key(
    mkt: *mut tcp_ao_key,
    key: *mut u8,
    saddr: *const in6_addr,
    daddr: *const in6_addr,
    sport: __be16,
    dport: __be16,
    sisn: __be32,
    disn: __be32,
) {
    let input = KdfInputBlock {
        counter: 1,
        label: *b"TCP-AO",
        ctx: tcp6_ao_context {
            saddr: *saddr,
            daddr: *daddr,
            sport,
            dport,
            sisn,
            disn,
        },
        outlen: htons(tcp_ao_digest_size(mkt) * 8), /* in bits */
    };

    tcp_ao_calc_traffic_key(mkt, key, &input as *const _ as *const _,
                            core::mem::size_of::<KdfInputBlock>());
}

pub unsafe extern "C" fn tcp_v6_ao_calc_key_skb(
    mkt: *mut tcp_ao_key,
    key: *mut u8,
    skb: *const sk_buff,
    sisn: __be32,
    disn: __be32,
) {
    let iph: *const ipv6hdr = ipv6_hdr(skb);
    let th: *const tcphdr = tcp_hdr(skb);

    tcp_v6_ao_calc_key(mkt, key, &(*iph).saddr, &(*iph).daddr,
                       (*th).source, (*th).dest, sisn, disn);
}

pub unsafe extern "C" fn tcp_v6_ao_calc_key_sk(
    mkt: *mut tcp_ao_key,
    key: *mut u8,
    sk: *const sock,
    sisn: __be32,
    disn: __be32,
    send: bool,
) {
    if send {
        tcp_v6_ao_calc_key(mkt, key, &(*sk).sk_v6_rcv_saddr,
                           &(*sk).sk_v6_daddr, htons((*sk).sk_num),
                           (*sk).sk_dport, sisn, disn);
    } else {
        tcp_v6_ao_calc_key(mkt, key, &(*sk).sk_v6_daddr,
                           &(*sk).sk_v6_rcv_saddr, (*sk).sk_dport,
                           htons((*sk).sk_num), disn, sisn);
    }
}

pub unsafe extern "C" fn tcp_v6_ao_calc_key_rsk(
    mkt: *mut tcp_ao_key,
    key: *mut u8,
    req: *mut request_sock,
) {
    let ireq: *mut inet_request_sock = inet_rsk(req);

    tcp_v6_ao_calc_key(mkt, key, &(*ireq).ir_v6_loc_addr,
                       &(*ireq).ir_v6_rmt_addr, htons((*ireq).ir_num),
                       (*ireq).ir_rmt_port, htonl((*tcp_rsk(req)).snt_isn),
                       htonl((*tcp_rsk(req)).rcv_isn));
}

pub unsafe extern "C" fn tcp_v6_ao_lookup(
    sk: *const sock,
    addr_sk: *mut sock,
    sndid: c_int,
    rcvid: c_int,
) -> *mut tcp_ao_key {
    let l3index = l3mdev_master_ifindex_by_index(sock_net(sk), (*addr_sk).sk_bound_dev_if);
    let addr: *mut in6_addr = &mut (*addr_sk).sk_v6_daddr;

    tcp_ao_do_lookup(sk, l3index, addr as *mut tcp_ao_addr, AF_INET6, sndid, rcvid)
}

pub unsafe extern "C" fn tcp_v6_ao_lookup_rsk(
    sk: *const sock,
    req: *mut request_sock,
    sndid: c_int,
    rcvid: c_int,
) -> *mut tcp_ao_key {
    let ireq: *mut inet_request_sock = inet_rsk(req);
    let addr: *mut in6_addr = &mut (*ireq).ir_v6_rmt_addr;
    let l3index = l3mdev_master_ifindex_by_index(sock_net(sk), (*ireq).ir_iif);

    tcp_ao_do_lookup(sk, l3index, addr as *mut tcp_ao_addr, AF_INET6, sndid, rcvid)
}

pub unsafe extern "C" fn tcp_v6_ao_hash_pseudoheader(
    mac_ctx: *mut tcp_ao_mac_ctx,
    daddr: *const in6_addr,
    saddr: *const in6_addr,
    nbytes: c_int,
) {
    /* 1. TCP pseudo-header (RFC2460) */
    let phdr = tcp6_pseudohdr {
        saddr: *saddr,
        daddr: *daddr,
        len: cpu_to_be32(nbytes as u32),
        protocol: cpu_to_be32(IPPROTO_TCP),
    };

    tcp_ao_mac_update(mac_ctx, &phdr as *const _ as *const _,
                      core::mem::size_of::<tcp6_pseudohdr>());
}

pub unsafe extern "C" fn tcp_v6_ao_hash_skb(
    ao_hash: *mut c_char,
    key: *mut tcp_ao_key,
    sk: *const sock,
    skb: *const sk_buff,
    tkey: *const u8,
    hash_offset: c_int,
    sne: u32,
) -> c_int {
    tcp_ao_hash_skb(AF_INET6, ao_hash, key, sk, skb, tkey, hash_offset, sne)
}

pub unsafe extern "C" fn tcp_v6_parse_ao(
    sk: *mut sock,
    cmd: c_int,
    optval: sockptr_t,
    optlen: c_int,
) -> c_int {
    tcp_parse_ao(sk, cmd, AF_INET6, optval, optlen)
}

pub unsafe extern "C" fn tcp_v6_ao_synack_hash(
    ao_hash: *mut c_char,
    ao_key: *mut tcp_ao_key,
    req: *mut request_sock,
    skb: *const sk_buff,
    hash_offset: c_int,
    sne: u32,
) -> c_int {
    let mut tkey_buf = [0u8; TCP_AO_MAX_TRAFFIC_KEY_LEN as usize];

    tcp_v6_ao_calc_key_rsk(ao_key, tkey_buf.as_mut_ptr(), req);

    tcp_ao_hash_skb(AF_INET6, ao_hash, ao_key, req_to_sk(req), skb,
                    tkey_buf.as_ptr(), hash_offset, sne)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
