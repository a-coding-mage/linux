// SPDX-License-Identifier: GPL-2.0
/* Copyright Amazon.com Inc. or its affiliates. */

/* Translated from C. External BPF/kernel definitions come from the original
 * includes: vmlinux.h, bpf_helpers.h, bpf_endian.h, bpf_tracing_net.h,
 * bpf_kfuncs.h, test_siphash.h, test_tcp_custom_syncookie.h, and bpf_misc.h.
 */

type __u8 = u8;
type __u16 = u16;
type __u32 = u32;
type __u64 = u64;
type __be16 = u16;
type __be32 = u32;
type __sum16 = u16;
type __wsum = u32;
type u32 = u32;
type u64 = u64;
type s64 = i64;

const MAX_PACKET_OFF: __u32 = 0xffff;

/* Hash is calculated for each client and split into ISN and TS.
 *
 *       MSB                                   LSB
 * ISN:  | 31 ... 8 | 7 6 |   5 |    4 | 3 2 1 0 |
 *       |   Hash_1 | MSS | ECN | SACK |  WScale |
 *
 * TS:   | 31 ... 8 |          7 ... 0           |
 *       |   Random |           Hash_2           |
 */
const COOKIE_BITS: u32 = 8;
const COOKIE_MASK: __u32 = (((1 as __u32) << COOKIE_BITS) - 1);

const BPF_SYNCOOKIE_WSCALE_MASK: u32 = (1 << 4) - 1;
const BPF_SYNCOOKIE_SACK: u32 = 1 << 4;
const BPF_SYNCOOKIE_ECN: u32 = 1 << 5;

const MSS_LOCAL_IPV4: __u16 = 65495;
const MSS_LOCAL_IPV6: __u16 = 65476;

static msstab4: [__u16; 4] = [
    536,
    1300,
    1460,
    MSS_LOCAL_IPV4,
];

static msstab6: [__u16; 4] = [
    1280 - 60, /* IPV6_MIN_MTU - 60 */
    1480 - 60,
    9000 - 60,
    MSS_LOCAL_IPV6,
];

#[repr(C)]
struct siphash_key_t {
    key: [u64; 2],
}

static mut test_key_siphash: siphash_key_t = siphash_key_t {
    key: [0x0706050403020100u64, 0x0f0e0d0c0b0a0908u64],
};

#[repr(C)]
struct tcp_syncookie {
    skb: *mut __sk_buff,
    data: *mut core::ffi::c_void,
    data_end: *mut core::ffi::c_void,
    eth: *mut ethhdr,
    ipv4: *mut iphdr,
    ipv6: *mut ipv6hdr,
    tcp: *mut tcphdr,
    ptr32: *mut __be32,
    attrs: bpf_tcp_req_attrs,
    off: u32,
    cookie: u32,
    first: u64,
}

static mut handled_syn: bool = false;
static mut handled_ack: bool = false;

unsafe fn tcp_load_headers(ctx: *mut tcp_syncookie) -> i32 {
    (*ctx).data = (*(*ctx).skb).data as isize as *mut core::ffi::c_void;
    (*ctx).data_end = (*(*ctx).skb).data_end as isize as *mut core::ffi::c_void;
    (*ctx).eth = (*(*ctx).skb).data as isize as *mut ethhdr;

    if (*ctx).eth.add(1) as *mut core::ffi::c_void > (*ctx).data_end {
        return -1;
    }

    match bpf_ntohs((*(*ctx).eth).h_proto) as u32 {
        ETH_P_IP => {
            (*ctx).ipv4 = (*ctx).eth.add(1) as *mut iphdr;

            if (*ctx).ipv4.add(1) as *mut core::ffi::c_void > (*ctx).data_end {
                return -1;
            }

            if (*(*ctx).ipv4).ihl != (core::mem::size_of::<iphdr>() / 4) as __u8 {
                return -1;
            }

            if (*(*ctx).ipv4).version != 4 {
                return -1;
            }

            if (*(*ctx).ipv4).protocol != IPPROTO_TCP as __u8 {
                return -1;
            }

            (*ctx).tcp = (*ctx).ipv4.add(1) as *mut tcphdr;
        }
        ETH_P_IPV6 => {
            (*ctx).ipv6 = (*ctx).eth.add(1) as *mut ipv6hdr;

            if (*ctx).ipv6.add(1) as *mut core::ffi::c_void > (*ctx).data_end {
                return -1;
            }

            if (*(*ctx).ipv6).version != 6 {
                return -1;
            }

            if (*(*ctx).ipv6).nexthdr != NEXTHDR_TCP as __u8 {
                return -1;
            }

            (*ctx).tcp = (*ctx).ipv6.add(1) as *mut tcphdr;
        }
        _ => return -1,
    }

    if (*ctx).tcp.add(1) as *mut core::ffi::c_void > (*ctx).data_end {
        return -1;
    }

    0
}

unsafe fn tcp_reload_headers(ctx: *mut tcp_syncookie) -> i32 {
    /* Without volatile,
     * R3 32-bit pointer arithmetic prohibited
     */
    let data_len: volatile_u64 = volatile_u64 {
        value: ((*(*ctx).skb).data_end - (*(*ctx).skb).data) as u64,
    };

    if (*(*ctx).tcp).doff < (core::mem::size_of::<tcphdr>() / 4) as __u8 {
        return -1;
    }

    /* Needed to calculate csum and parse TCP options. */
    if bpf_skb_change_tail(
        (*ctx).skb,
        data_len.value + 60 - ((*(*ctx).tcp).doff as u64) * 4,
        0,
    ) != 0
    {
        return -1;
    }

    (*ctx).data = (*(*ctx).skb).data as isize as *mut core::ffi::c_void;
    (*ctx).data_end = (*(*ctx).skb).data_end as isize as *mut core::ffi::c_void;
    (*ctx).eth = (*(*ctx).skb).data as isize as *mut ethhdr;
    if !(*ctx).ipv4.is_null() {
        (*ctx).ipv4 = (*ctx).eth.add(1) as *mut iphdr;
        (*ctx).ipv6 = core::ptr::null_mut();
        (*ctx).tcp = (*ctx).ipv4.add(1) as *mut tcphdr;
    } else {
        (*ctx).ipv4 = core::ptr::null_mut();
        (*ctx).ipv6 = (*ctx).eth.add(1) as *mut ipv6hdr;
        (*ctx).tcp = (*ctx).ipv6.add(1) as *mut tcphdr;
    }

    if ((*ctx).tcp as *mut __u8).add(60) as *mut core::ffi::c_void > (*ctx).data_end {
        return -1;
    }

    0
}

unsafe fn tcp_v4_csum(ctx: *mut tcp_syncookie, csum: __wsum) -> __sum16 {
    csum_tcpudp_magic(
        (*(*ctx).ipv4).saddr,
        (*(*ctx).ipv4).daddr,
        ((*(*ctx).tcp).doff as u32) * 4,
        IPPROTO_TCP,
        csum,
    )
}

unsafe fn tcp_v6_csum(ctx: *mut tcp_syncookie, csum: __wsum) -> __sum16 {
    csum_ipv6_magic(
        &mut (*(*ctx).ipv6).saddr,
        &mut (*(*ctx).ipv6).daddr,
        ((*(*ctx).tcp).doff as u32) * 4,
        IPPROTO_TCP,
        csum,
    )
}

unsafe fn tcp_validate_header(ctx: *mut tcp_syncookie) -> i32 {
    let mut csum: s64;

    if tcp_reload_headers(ctx) != 0 {
        return -1;
    }

    csum = bpf_csum_diff(
        core::ptr::null_mut(),
        0,
        (*ctx).tcp as *mut core::ffi::c_void,
        ((*(*ctx).tcp).doff as u32) * 4,
        0,
    );
    if csum < 0 {
        return -1;
    }

    if !(*ctx).ipv4.is_null() {
        /* check tcp_v4_csum(csum) is 0 if not on lo. */

        csum = bpf_csum_diff(
            core::ptr::null_mut(),
            0,
            (*ctx).ipv4 as *mut core::ffi::c_void,
            ((*(*ctx).ipv4).ihl as u32) * 4,
            0,
        );
        if csum < 0 {
            return -1;
        }

        if csum_fold(csum) != 0 {
            return -1;
        }
    } else if !(*ctx).ipv6.is_null() {
        /* check tcp_v6_csum(csum) is 0 if not on lo. */
    }

    0
}

unsafe fn next(ctx: *mut tcp_syncookie, sz: __u32) -> *mut core::ffi::c_void {
    let off: __u64 = (*ctx).off as __u64;
    let data: *mut __u8;

    /* Verifier forbids access to packet when offset exceeds MAX_PACKET_OFF */
    if off > (MAX_PACKET_OFF - sz) as __u64 {
        return core::ptr::null_mut();
    }

    data = ((*ctx).data as *mut __u8).add(off as usize);
    barrier_var(data);
    if data.add(sz as usize) as *mut core::ffi::c_void >= (*ctx).data_end {
        return core::ptr::null_mut();
    }

    (*ctx).off += sz;
    data as *mut core::ffi::c_void
}

unsafe fn tcp_parse_option(_index: __u32, ctx: *mut tcp_syncookie) -> i32 {
    let opcode: *mut __u8;
    let opsize: *mut __u8;
    let wscale: *mut __u8;
    let tsval: *mut __u32;
    let tsecr: *mut __u32;
    let mss: *mut __u16;
    let off: __u32;

    off = (*ctx).off;
    opcode = next(ctx, 1) as *mut __u8;
    if opcode.is_null() {
        return 1;
    }

    if *opcode == TCPOPT_EOL as __u8 {
        return 1;
    }

    if *opcode == TCPOPT_NOP as __u8 {
        return 0;
    }

    opsize = next(ctx, 1) as *mut __u8;
    if opsize.is_null() {
        return 1;
    }

    if *opsize < 2 {
        return 1;
    }

    match *opcode as u32 {
        TCPOPT_MSS => {
            mss = next(ctx, 2) as *mut __u16;
            if *opsize == TCPOLEN_MSS as __u8 && (*(*ctx).tcp).syn != 0 && !mss.is_null() {
                (*ctx).attrs.mss = get_unaligned_be16(mss);
            }
        }
        TCPOPT_WINDOW => {
            wscale = next(ctx, 1) as *mut __u8;
            if *opsize == TCPOLEN_WINDOW as __u8 && (*(*ctx).tcp).syn != 0 && !wscale.is_null() {
                (*ctx).attrs.wscale_ok = 1;
                (*ctx).attrs.snd_wscale = *wscale as u32;
            }
        }
        TCPOPT_TIMESTAMP => {
            tsval = next(ctx, 4) as *mut __u32;
            tsecr = next(ctx, 4) as *mut __u32;
            if *opsize == TCPOLEN_TIMESTAMP as __u8 && !tsval.is_null() && !tsecr.is_null() {
                (*ctx).attrs.rcv_tsval = get_unaligned_be32(tsval);
                (*ctx).attrs.rcv_tsecr = get_unaligned_be32(tsecr);

                if (*(*ctx).tcp).syn != 0 && (*ctx).attrs.rcv_tsecr != 0 {
                    (*ctx).attrs.tstamp_ok = 0;
                } else {
                    (*ctx).attrs.tstamp_ok = 1;
                }
            }
        }
        TCPOPT_SACK_PERM => {
            if *opsize == TCPOLEN_SACK_PERM as __u8 && (*(*ctx).tcp).syn != 0 {
                (*ctx).attrs.sack_ok = 1;
            }
        }
        _ => {}
    }

    (*ctx).off = off + (*opsize as u32);
    0
}

unsafe fn tcp_parse_options(ctx: *mut tcp_syncookie) {
    (*ctx).off = ((*ctx).tcp.add(1) as *mut __u8).offset_from((*ctx).data as *mut __u8) as u32;

    bpf_loop(40, Some(tcp_parse_option), ctx as *mut core::ffi::c_void, 0);
}

unsafe fn tcp_validate_sysctl(ctx: *mut tcp_syncookie) -> i32 {
    if ((!(*ctx).ipv4.is_null()) && (*ctx).attrs.mss != MSS_LOCAL_IPV4)
        || ((!(*ctx).ipv6.is_null()) && (*ctx).attrs.mss != MSS_LOCAL_IPV6)
    {
        return -1;
    }

    if (*ctx).attrs.wscale_ok == 0
        || (*ctx).attrs.snd_wscale == 0
        || (*ctx).attrs.snd_wscale >= BPF_SYNCOOKIE_WSCALE_MASK
    {
        return -1;
    }

    if (*ctx).attrs.tstamp_ok == 0 {
        return -1;
    }

    if (*ctx).attrs.sack_ok == 0 {
        return -1;
    }

    if (*(*ctx).tcp).ece == 0 || (*(*ctx).tcp).cwr == 0 {
        return -1;
    }

    0
}

unsafe fn tcp_prepare_cookie(ctx: *mut tcp_syncookie) {
    let seq: u32 = bpf_ntohl((*(*ctx).tcp).seq);
    let mut first: u64 = 0;
    let second: u64;
    let mut mssind: i32 = 0;
    let mut hash: u32;

    if !(*ctx).ipv4.is_null() {
        mssind = (msstab4.len() - 1) as i32;
        while mssind != 0 {
            if (*ctx).attrs.mss >= msstab4[mssind as usize] {
                break;
            }
            mssind -= 1;
        }

        (*ctx).attrs.mss = msstab4[mssind as usize];

        first = ((*(*ctx).ipv4).saddr as u64) << 32 | ((*(*ctx).ipv4).daddr as u64);
    } else if !(*ctx).ipv6.is_null() {
        mssind = (msstab6.len() - 1) as i32;
        while mssind != 0 {
            if (*ctx).attrs.mss >= msstab6[mssind as usize] {
                break;
            }
            mssind -= 1;
        }

        (*ctx).attrs.mss = msstab6[mssind as usize];

        first = ((*(*ctx).ipv6).saddr.in6_u.u6_addr8[0] as u64) << 32
            | ((*(*ctx).ipv6).daddr.in6_u.u6_addr32[0] as u64);
    }

    second = (seq as u64) << 32
        | ((*(*ctx).tcp).source as u64) << 16
        | ((*(*ctx).tcp).dest as u64);
    hash = siphash_2u64(first, second, &mut test_key_siphash);

    if (*ctx).attrs.tstamp_ok != 0 {
        (*ctx).attrs.rcv_tsecr = bpf_get_prandom_u32();
        (*ctx).attrs.rcv_tsecr &= !COOKIE_MASK;
        (*ctx).attrs.rcv_tsecr |= hash & COOKIE_MASK;
    }

    hash &= !COOKIE_MASK;
    hash |= (mssind as u32) << 6;

    if (*ctx).attrs.wscale_ok != 0 {
        hash |= (*ctx).attrs.snd_wscale & BPF_SYNCOOKIE_WSCALE_MASK;
    }

    if (*ctx).attrs.sack_ok != 0 {
        hash |= BPF_SYNCOOKIE_SACK;
    }

    if (*ctx).attrs.tstamp_ok != 0 && (*(*ctx).tcp).ece != 0 && (*(*ctx).tcp).cwr != 0 {
        hash |= BPF_SYNCOOKIE_ECN;
    }

    (*ctx).cookie = hash;
}

unsafe fn tcp_write_options(ctx: *mut tcp_syncookie) {
    (*ctx).ptr32 = (*ctx).tcp.add(1) as *mut __be32;

    *(*ctx).ptr32 = bpf_htonl((TCPOPT_MSS << 24) | (TCPOLEN_MSS << 16) | ((*ctx).attrs.mss as u32));
    (*ctx).ptr32 = (*ctx).ptr32.add(1);

    if (*ctx).attrs.wscale_ok != 0 {
        *(*ctx).ptr32 = bpf_htonl(
            (TCPOPT_NOP << 24)
                | (TCPOPT_WINDOW << 16)
                | (TCPOLEN_WINDOW << 8)
                | (*ctx).attrs.snd_wscale,
        );
        (*ctx).ptr32 = (*ctx).ptr32.add(1);
    }

    if (*ctx).attrs.tstamp_ok != 0 {
        if (*ctx).attrs.sack_ok != 0 {
            *(*ctx).ptr32 = bpf_htonl(
                (TCPOPT_SACK_PERM << 24)
                    | (TCPOLEN_SACK_PERM << 16)
                    | (TCPOPT_TIMESTAMP << 8)
                    | TCPOLEN_TIMESTAMP,
            );
        } else {
            *(*ctx).ptr32 = bpf_htonl(
                (TCPOPT_NOP << 24)
                    | (TCPOPT_NOP << 16)
                    | (TCPOPT_TIMESTAMP << 8)
                    | TCPOLEN_TIMESTAMP,
            );
        }
        (*ctx).ptr32 = (*ctx).ptr32.add(1);

        *(*ctx).ptr32 = bpf_htonl((*ctx).attrs.rcv_tsecr);
        (*ctx).ptr32 = (*ctx).ptr32.add(1);
        *(*ctx).ptr32 = bpf_htonl((*ctx).attrs.rcv_tsval);
        (*ctx).ptr32 = (*ctx).ptr32.add(1);
    } else if (*ctx).attrs.sack_ok != 0 {
        *(*ctx).ptr32 = bpf_htonl(
            (TCPOPT_NOP << 24)
                | (TCPOPT_NOP << 16)
                | (TCPOPT_SACK_PERM << 8)
                | TCPOLEN_SACK_PERM,
        );
        (*ctx).ptr32 = (*ctx).ptr32.add(1);
    }
}

unsafe fn tcp_handle_syn(ctx: *mut tcp_syncookie) -> i32 {
    let mut csum: s64;

    if tcp_validate_header(ctx) != 0 {
        return TC_ACT_SHOT;
    }

    tcp_parse_options(ctx);

    if tcp_validate_sysctl(ctx) != 0 {
        return TC_ACT_SHOT;
    }

    tcp_prepare_cookie(ctx);
    tcp_write_options(ctx);

    core::mem::swap(&mut (*(*ctx).tcp).source, &mut (*(*ctx).tcp).dest);
    (*(*ctx).tcp).check = 0;
    (*(*ctx).tcp).ack_seq = bpf_htonl(bpf_ntohl((*(*ctx).tcp).seq) + 1);
    (*(*ctx).tcp).seq = bpf_htonl((*ctx).cookie);
    (*(*ctx).tcp).doff = (((*ctx).ptr32 as isize - (*ctx).tcp as isize) >> 2) as __u8;
    (*(*ctx).tcp).ack = 1;
    if (*ctx).attrs.tstamp_ok == 0 || (*(*ctx).tcp).ece == 0 || (*(*ctx).tcp).cwr == 0 {
        (*(*ctx).tcp).ece = 0;
    }
    (*(*ctx).tcp).cwr = 0;

    csum = bpf_csum_diff(
        core::ptr::null_mut(),
        0,
        (*ctx).tcp as *mut core::ffi::c_void,
        ((*(*ctx).tcp).doff as u32) * 4,
        0,
    );
    if csum < 0 {
        return TC_ACT_SHOT;
    }

    if !(*ctx).ipv4.is_null() {
        core::mem::swap(&mut (*(*ctx).ipv4).saddr, &mut (*(*ctx).ipv4).daddr);
        (*(*ctx).tcp).check = tcp_v4_csum(ctx, csum as __wsum);

        (*(*ctx).ipv4).check = 0;
        (*(*ctx).ipv4).tos = 0;
        (*(*ctx).ipv4).tot_len = bpf_htons(((*ctx).ptr32 as isize - (*ctx).ipv4 as isize) as u16);
        (*(*ctx).ipv4).id = 0;
        (*(*ctx).ipv4).ttl = 64;

        csum = bpf_csum_diff(
            core::ptr::null_mut(),
            0,
            (*ctx).ipv4 as *mut core::ffi::c_void,
            core::mem::size_of::<iphdr>() as u32,
            0,
        );
        if csum < 0 {
            return TC_ACT_SHOT;
        }

        (*(*ctx).ipv4).check = csum_fold(csum);
    } else if !(*ctx).ipv6.is_null() {
        core::mem::swap(&mut (*(*ctx).ipv6).saddr, &mut (*(*ctx).ipv6).daddr);
        (*(*ctx).tcp).check = tcp_v6_csum(ctx, csum as __wsum);

        *(*ctx).ipv6.cast::<__be32>() = bpf_htonl(0x60000000);
        (*(*ctx).ipv6).payload_len =
            bpf_htons(((*ctx).ptr32 as isize - (*ctx).tcp as isize) as u16);
        (*(*ctx).ipv6).hop_limit = 64;
    }

    swap_array((*(*ctx).eth).h_source.as_mut_ptr(), (*(*ctx).eth).h_dest.as_mut_ptr());

    if bpf_skb_change_tail((*ctx).skb, ((*ctx).ptr32 as isize - (*ctx).eth as isize) as u64, 0) != 0 {
        return TC_ACT_SHOT;
    }

    bpf_redirect((*(*ctx).skb).ifindex, 0)
}

unsafe fn tcp_validate_cookie(ctx: *mut tcp_syncookie) -> i32 {
    let cookie: u32 = bpf_ntohl((*(*ctx).tcp).ack_seq) - 1;
    let seq: u32 = bpf_ntohl((*(*ctx).tcp).seq) - 1;
    let mut first: u64 = 0;
    let second: u64;
    let mut mssind: i32;
    let mut hash: u32;

    if !(*ctx).ipv4.is_null() {
        first = ((*(*ctx).ipv4).saddr as u64) << 32 | ((*(*ctx).ipv4).daddr as u64);
    } else if !(*ctx).ipv6.is_null() {
        first = ((*(*ctx).ipv6).saddr.in6_u.u6_addr8[0] as u64) << 32
            | ((*(*ctx).ipv6).daddr.in6_u.u6_addr32[0] as u64);
    }

    second = (seq as u64) << 32
        | ((*(*ctx).tcp).source as u64) << 16
        | ((*(*ctx).tcp).dest as u64);
    hash = siphash_2u64(first, second, &mut test_key_siphash);

    if (*ctx).attrs.tstamp_ok != 0 {
        hash -= (*ctx).attrs.rcv_tsecr & COOKIE_MASK;
    } else {
        hash &= !COOKIE_MASK;
    }

    hash -= cookie & !COOKIE_MASK;
    if hash != 0 {
        return -1;
    }

    mssind = ((cookie & (3 << 6)) >> 6) as i32;
    if !(*ctx).ipv4.is_null() {
        (*ctx).attrs.mss = msstab4[mssind as usize];
    } else {
        (*ctx).attrs.mss = msstab6[mssind as usize];
    }

    (*ctx).attrs.snd_wscale = cookie & BPF_SYNCOOKIE_WSCALE_MASK;
    (*ctx).attrs.rcv_wscale = (*ctx).attrs.snd_wscale;
    (*ctx).attrs.wscale_ok = ((*ctx).attrs.snd_wscale == BPF_SYNCOOKIE_WSCALE_MASK) as u32;
    (*ctx).attrs.sack_ok = cookie & BPF_SYNCOOKIE_SACK;
    (*ctx).attrs.ecn_ok = cookie & BPF_SYNCOOKIE_ECN;

    0
}

unsafe fn tcp_handle_ack(ctx: *mut tcp_syncookie) -> i32 {
    let mut tuple: bpf_sock_tuple = core::mem::zeroed();
    let skc: *mut bpf_sock;
    let mut ret: i32 = TC_ACT_OK;
    let sk: *mut sock;
    let tuple_size: u32;

    if !(*ctx).ipv4.is_null() {
        tuple.ipv4.saddr = (*(*ctx).ipv4).saddr;
        tuple.ipv4.daddr = (*(*ctx).ipv4).daddr;
        tuple.ipv4.sport = (*(*ctx).tcp).source;
        tuple.ipv4.dport = (*(*ctx).tcp).dest;
        tuple_size = core::mem::size_of_val(&tuple.ipv4) as u32;
    } else if !(*ctx).ipv6.is_null() {
        core::ptr::copy_nonoverlapping(
            &mut (*(*ctx).ipv6).saddr as *mut in6_addr as *mut u8,
            tuple.ipv6.saddr.as_mut_ptr(),
            core::mem::size_of_val(&tuple.ipv6.saddr),
        );
        core::ptr::copy_nonoverlapping(
            &mut (*(*ctx).ipv6).daddr as *mut in6_addr as *mut u8,
            tuple.ipv6.daddr.as_mut_ptr(),
            core::mem::size_of_val(&tuple.ipv6.daddr),
        );
        tuple.ipv6.sport = (*(*ctx).tcp).source;
        tuple.ipv6.dport = (*(*ctx).tcp).dest;
        tuple_size = core::mem::size_of_val(&tuple.ipv6) as u32;
    } else {
        return ret;
    }

    skc = bpf_skc_lookup_tcp((*ctx).skb, &mut tuple, tuple_size, -1, 0);
    if skc.is_null() {
        return ret;
    }

    if (*skc).state != TCP_LISTEN {
        bpf_sk_release(skc);
        return ret;
    }

    sk = bpf_skc_to_tcp_sock(skc) as *mut sock;
    if sk.is_null() {
        ret = TC_ACT_SHOT;
        bpf_sk_release(skc);
        return ret;
    }

    if tcp_validate_header(ctx) != 0 {
        ret = TC_ACT_SHOT;
        bpf_sk_release(skc);
        return ret;
    }

    tcp_parse_options(ctx);

    if tcp_validate_cookie(ctx) != 0 {
        ret = TC_ACT_SHOT;
        bpf_sk_release(skc);
        return ret;
    }

    ret = bpf_sk_assign_tcp_reqsk(
        (*ctx).skb,
        sk,
        &mut (*ctx).attrs,
        core::mem::size_of_val(&(*ctx).attrs) as u32,
    );
    if ret < 0 {
        ret = TC_ACT_SHOT;
    }

    bpf_sk_release(skc);
    ret
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn tcp_custom_syncookie(skb: *mut __sk_buff) -> i32 {
    let mut ctx: tcp_syncookie = core::mem::zeroed();
    ctx.skb = skb;

    if tcp_load_headers(&mut ctx) != 0 {
        return TC_ACT_OK;
    }

    if (*ctx.tcp).rst != 0 {
        return TC_ACT_OK;
    }

    if (*ctx.tcp).syn != 0 {
        if (*ctx.tcp).ack != 0 {
            return TC_ACT_OK;
        }

        handled_syn = true;

        return tcp_handle_syn(&mut ctx);
    }

    handled_ack = true;

    tcp_handle_ack(&mut ctx)
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
struct volatile_u64 {
    value: u64,
}

#[repr(C)]
struct __sk_buff {
    data: u32,
    data_end: u32,
    ifindex: u32,
}

#[repr(C)]
struct ethhdr {
    h_dest: [u8; 6],
    h_source: [u8; 6],
    h_proto: __be16,
}

#[repr(C)]
struct iphdr {
    ihl: __u8,
    version: __u8,
    tos: __u8,
    tot_len: __be16,
    id: __be16,
    protocol: __u8,
    ttl: __u8,
    check: __sum16,
    saddr: __be32,
    daddr: __be32,
}

#[repr(C)]
union in6_addr_union {
    u6_addr8: [__u8; 16],
    u6_addr32: [__be32; 4],
}

#[repr(C)]
struct in6_addr {
    in6_u: in6_addr_union,
}

#[repr(C)]
struct ipv6hdr {
    version: __u8,
    nexthdr: __u8,
    payload_len: __be16,
    hop_limit: __u8,
    saddr: in6_addr,
    daddr: in6_addr,
}

#[repr(C)]
struct tcphdr {
    source: __be16,
    dest: __be16,
    seq: __be32,
    ack_seq: __be32,
    doff: __u8,
    rst: __u8,
    syn: __u8,
    ack: __u8,
    ece: __u8,
    cwr: __u8,
    check: __sum16,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct bpf_tcp_req_attrs {
    mss: u32,
    wscale_ok: u32,
    snd_wscale: u32,
    rcv_wscale: u32,
    rcv_tsval: u32,
    rcv_tsecr: u32,
    tstamp_ok: u32,
    sack_ok: u32,
    ecn_ok: u32,
}

#[repr(C)]
struct bpf_sock_tuple_ipv4 {
    saddr: __be32,
    daddr: __be32,
    sport: __be16,
    dport: __be16,
}

#[repr(C)]
struct bpf_sock_tuple_ipv6 {
    saddr: [__u8; 16],
    daddr: [__u8; 16],
    sport: __be16,
    dport: __be16,
}

#[repr(C)]
union bpf_sock_tuple {
    ipv4: bpf_sock_tuple_ipv4,
    ipv6: bpf_sock_tuple_ipv6,
}

#[repr(C)]
struct bpf_sock {
    state: u32,
}

#[repr(C)]
struct sock {
    _private: [u8; 0],
}

const ETH_P_IP: u32 = 0x0800;
const ETH_P_IPV6: u32 = 0x86DD;
const IPPROTO_TCP: u32 = 6;
const NEXTHDR_TCP: u32 = 6;
const TCPOPT_EOL: u32 = 0;
const TCPOPT_NOP: u32 = 1;
const TCPOPT_MSS: u32 = 2;
const TCPOPT_WINDOW: u32 = 3;
const TCPOPT_SACK_PERM: u32 = 4;
const TCPOPT_TIMESTAMP: u32 = 8;
const TCPOLEN_MSS: u32 = 4;
const TCPOLEN_WINDOW: u32 = 3;
const TCPOLEN_SACK_PERM: u32 = 2;
const TCPOLEN_TIMESTAMP: u32 = 10;
const TC_ACT_OK: i32 = 0;
const TC_ACT_SHOT: i32 = 2;
const TCP_LISTEN: u32 = 10;

extern "C" {
    fn bpf_ntohs(x: __be16) -> u16;
    fn bpf_ntohl(x: __be32) -> u32;
    fn bpf_htons(x: u16) -> __be16;
    fn bpf_htonl(x: u32) -> __be32;
    fn bpf_skb_change_tail(skb: *mut __sk_buff, len: u64, flags: u64) -> i32;
    fn csum_tcpudp_magic(saddr: __be32, daddr: __be32, len: u32, proto: u32, csum: __wsum) -> __sum16;
    fn csum_ipv6_magic(saddr: *mut in6_addr, daddr: *mut in6_addr, len: u32, proto: u32, csum: __wsum) -> __sum16;
    fn bpf_csum_diff(from: *mut core::ffi::c_void, from_size: u32, to: *mut core::ffi::c_void, to_size: u32, seed: __wsum) -> s64;
    fn csum_fold(csum: s64) -> __sum16;
    fn barrier_var<T>(var: T);
    fn bpf_loop(nr_loops: u32, callback_fn: Option<unsafe fn(__u32, *mut tcp_syncookie) -> i32>, callback_ctx: *mut core::ffi::c_void, flags: u64) -> i32;
    fn get_unaligned_be16(p: *mut __u16) -> u32;
    fn get_unaligned_be32(p: *mut __u32) -> u32;
    fn siphash_2u64(first: u64, second: u64, key: *mut siphash_key_t) -> u32;
    fn bpf_get_prandom_u32() -> u32;
    fn swap_array(a: *mut u8, b: *mut u8);
    fn bpf_redirect(ifindex: u32, flags: u64) -> i32;
    fn bpf_skc_lookup_tcp(skb: *mut __sk_buff, tuple: *mut bpf_sock_tuple, tuple_size: u32, netns: i64, flags: u64) -> *mut bpf_sock;
    fn bpf_skc_to_tcp_sock(skc: *mut bpf_sock) -> *mut core::ffi::c_void;
    fn bpf_sk_assign_tcp_reqsk(skb: *mut __sk_buff, sk: *mut sock, attrs: *mut bpf_tcp_req_attrs, attrs__sz: u32) -> i32;
    fn bpf_sk_release(sock: *mut bpf_sock);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
