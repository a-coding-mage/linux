// SPDX-License-Identifier: GPL-2.0
// Translated from testing/selftests/bpf/progs/bpf_flow.c.
// C include dependencies are expected to provide the BPF/kernel types,
// constants, helper functions, and section/map attributes referenced below.

pub const FLOW_CONTINUE_SADDR: u32 = 0x7f00007f; /* 127.0.0.127 */

/* These are the identifiers of the BPF programs that will be used in tail
 * calls. Name is limited to 16 characters, with the terminating character and
 * bpf_func_ above, we have only 6 to work with, anything after will be cropped.
 */
pub const IP: u32 = 0;
pub const IPV6: u32 = 1;
pub const IPV6OP: u32 = 2; /* Destination/Hop-by-Hop Options IPv6 Ext. Header */
pub const IPV6FR: u32 = 3; /* Fragmentation IPv6 Extension Header */
pub const MPLS: u32 = 4;
pub const VLAN: u32 = 5;
pub const MAX_PROG: u32 = 6;

pub const IP_MF: u16 = 0x2000;
pub const IP_OFFSET: u16 = 0x1FFF;
pub const IP6_MF: u16 = 0x0001;
pub const IP6_OFFSET: u16 = 0xFFF8;

#[repr(C)]
pub struct vlan_hdr {
    pub h_vlan_TCI: __be16,
    pub h_vlan_encapsulated_proto: __be16,
}

#[repr(C)]
pub struct gre_hdr {
    pub flags: __be16,
    pub proto: __be16,
}

#[repr(C)]
pub struct frag_hdr {
    pub nexthdr: __u8,
    pub reserved: __u8,
    pub frag_off: __be16,
    pub identification: __be32,
}

// SEC(".maps")
// struct {
//     __uint(type, BPF_MAP_TYPE_PROG_ARRAY);
//     __uint(max_entries, MAX_PROG);
//     __uint(key_size, sizeof(__u32));
//     __uint(value_size, sizeof(__u32));
// } jmp_table;
extern "C" {
    pub static mut jmp_table: core::ffi::c_void;
}

// SEC(".maps")
// struct {
//     __uint(type, BPF_MAP_TYPE_HASH);
//     __uint(max_entries, 1024);
//     __type(key, __u32);
//     __type(value, struct bpf_flow_keys);
// } last_dissection;
extern "C" {
    pub static mut last_dissection: core::ffi::c_void;
}

#[inline(always)]
unsafe fn export_flow_keys(keys: *mut bpf_flow_keys, ret: i32) -> i32 {
    let key: __u32 = (((*keys).sport as __u32) << 16) | ((*keys).dport as __u32);
    let mut val: bpf_flow_keys = core::mem::zeroed();

    core::ptr::copy_nonoverlapping(
        keys as *const u8,
        &mut val as *mut bpf_flow_keys as *mut u8,
        core::mem::size_of_val(&val),
    );
    bpf_map_update_elem(
        core::ptr::addr_of_mut!(last_dissection) as *mut _,
        &key as *const __u32 as *const core::ffi::c_void,
        &val as *const bpf_flow_keys as *const core::ffi::c_void,
        BPF_ANY as __u64,
    );
    ret
}

pub const IPV6_FLOWLABEL_MASK: __be32 = __bpf_constant_htonl(0x000FFFFF);

#[inline]
unsafe fn ip6_flowlabel(hdr: *const ipv6hdr) -> __be32 {
    (*(hdr as *const __be32)) & IPV6_FLOWLABEL_MASK
}

#[inline(always)]
unsafe fn bpf_flow_dissect_get_header(
    skb: *mut __sk_buff,
    hdr_size: __u16,
    buffer: *mut core::ffi::c_void,
) -> *mut core::ffi::c_void {
    let data_end = (*skb).data_end as usize as *mut core::ffi::c_void;
    let data = (*skb).data as usize as *mut core::ffi::c_void;
    let thoff: __u16 = (*(*skb).flow_keys).thoff;
    let hdr: *mut __u8;

    /* Verifies this variable offset does not overflow */
    if thoff > ((__u16::MAX) - hdr_size) {
        return core::ptr::null_mut();
    }

    hdr = (data as *mut __u8).add(thoff as usize);
    if hdr.add(hdr_size as usize) <= data_end as *mut __u8 {
        return hdr as *mut core::ffi::c_void;
    }

    if bpf_skb_load_bytes(skb, thoff as __u32, buffer, hdr_size as __u32) != 0 {
        return core::ptr::null_mut();
    }

    buffer
}

/* Dispatches on ETHERTYPE */
#[inline(always)]
unsafe fn parse_eth_proto(skb: *mut __sk_buff, proto: __be16) -> i32 {
    let keys: *mut bpf_flow_keys = (*skb).flow_keys;

    match proto as u32 {
        x if x == bpf_htons(ETH_P_IP as __u16) as u32 => {
            bpf_tail_call_static(skb, core::ptr::addr_of_mut!(jmp_table) as *mut _, IP);
        }
        x if x == bpf_htons(ETH_P_IPV6 as __u16) as u32 => {
            bpf_tail_call_static(skb, core::ptr::addr_of_mut!(jmp_table) as *mut _, IPV6);
        }
        x if x == bpf_htons(ETH_P_MPLS_MC as __u16) as u32
            || x == bpf_htons(ETH_P_MPLS_UC as __u16) as u32 =>
        {
            bpf_tail_call_static(skb, core::ptr::addr_of_mut!(jmp_table) as *mut _, MPLS);
        }
        x if x == bpf_htons(ETH_P_8021Q as __u16) as u32
            || x == bpf_htons(ETH_P_8021AD as __u16) as u32 =>
        {
            bpf_tail_call_static(skb, core::ptr::addr_of_mut!(jmp_table) as *mut _, VLAN);
        }
        _ => {
            /* Protocol not supported */
            return export_flow_keys(keys, BPF_DROP);
        }
    }

    export_flow_keys(keys, BPF_DROP)
}

// SEC("flow_dissector")
#[no_mangle]
pub unsafe extern "C" fn _dissect(skb: *mut __sk_buff) -> i32 {
    let keys: *mut bpf_flow_keys = (*skb).flow_keys;

    if (*keys).n_proto == bpf_htons(ETH_P_IP as __u16) {
        /* IP traffic from FLOW_CONTINUE_SADDR falls-back to
         * standard dissector
         */
        let mut _iph: iphdr = core::mem::zeroed();
        let iph = bpf_flow_dissect_get_header(
            skb,
            core::mem::size_of::<iphdr>() as __u16,
            &mut _iph as *mut iphdr as *mut core::ffi::c_void,
        ) as *mut iphdr;
        if !iph.is_null()
            && (*iph).ihl == 5
            && (*iph).saddr == bpf_htonl(FLOW_CONTINUE_SADDR)
        {
            return BPF_FLOW_DISSECTOR_CONTINUE;
        }
    }

    parse_eth_proto(skb, (*keys).n_proto)
}

/* Parses on IPPROTO_* */
#[inline(always)]
unsafe fn parse_ip_proto(skb: *mut __sk_buff, proto: __u8) -> i32 {
    let keys: *mut bpf_flow_keys = (*skb).flow_keys;
    let data_end = (*skb).data_end as usize as *mut core::ffi::c_void;
    let mut _icmp: icmphdr = core::mem::zeroed();
    let mut _gre: gre_hdr = core::mem::zeroed();
    let mut _eth: ethhdr = core::mem::zeroed();
    let mut _tcp: tcphdr = core::mem::zeroed();
    let mut _udp: udphdr = core::mem::zeroed();

    match proto as u32 {
        x if x == IPPROTO_ICMP as u32 => {
            let icmp = bpf_flow_dissect_get_header(
                skb,
                core::mem::size_of::<icmphdr>() as __u16,
                &mut _icmp as *mut icmphdr as *mut core::ffi::c_void,
            ) as *mut icmphdr;
            if icmp.is_null() {
                return export_flow_keys(keys, BPF_DROP);
            }
            return export_flow_keys(keys, BPF_OK);
        }
        x if x == IPPROTO_IPIP as u32 => {
            (*keys).is_encap = true;
            if ((*keys).flags & BPF_FLOW_DISSECTOR_F_STOP_AT_ENCAP) != 0 {
                return export_flow_keys(keys, BPF_OK);
            }

            return parse_eth_proto(skb, bpf_htons(ETH_P_IP as __u16));
        }
        x if x == IPPROTO_IPV6 as u32 => {
            (*keys).is_encap = true;
            if ((*keys).flags & BPF_FLOW_DISSECTOR_F_STOP_AT_ENCAP) != 0 {
                return export_flow_keys(keys, BPF_OK);
            }

            return parse_eth_proto(skb, bpf_htons(ETH_P_IPV6 as __u16));
        }
        x if x == IPPROTO_GRE as u32 => {
            let gre = bpf_flow_dissect_get_header(
                skb,
                core::mem::size_of::<gre_hdr>() as __u16,
                &mut _gre as *mut gre_hdr as *mut core::ffi::c_void,
            ) as *mut gre_hdr;
            if gre.is_null() {
                return export_flow_keys(keys, BPF_DROP);
            }

            if bpf_htons((*gre).flags & GRE_VERSION as __be16) != 0 {
                /* Only inspect standard GRE packets with version 0 */
                return export_flow_keys(keys, BPF_OK);
            }

            (*keys).thoff = (*keys)
                .thoff
                .wrapping_add(core::mem::size_of::<gre_hdr>() as __u16); /* Step over GRE Flags and Proto */
            if GRE_IS_CSUM((*gre).flags) {
                (*keys).thoff = (*keys).thoff.wrapping_add(4); /* Step over chksum and Padding */
            }
            if GRE_IS_KEY((*gre).flags) {
                (*keys).thoff = (*keys).thoff.wrapping_add(4); /* Step over key */
            }
            if GRE_IS_SEQ((*gre).flags) {
                (*keys).thoff = (*keys).thoff.wrapping_add(4); /* Step over sequence number */
            }

            (*keys).is_encap = true;
            if ((*keys).flags & BPF_FLOW_DISSECTOR_F_STOP_AT_ENCAP) != 0 {
                return export_flow_keys(keys, BPF_OK);
            }

            if (*gre).proto == bpf_htons(ETH_P_TEB as __u16) {
                let eth = bpf_flow_dissect_get_header(
                    skb,
                    core::mem::size_of::<ethhdr>() as __u16,
                    &mut _eth as *mut ethhdr as *mut core::ffi::c_void,
                ) as *mut ethhdr;
                if eth.is_null() {
                    return export_flow_keys(keys, BPF_DROP);
                }

                (*keys).thoff = (*keys)
                    .thoff
                    .wrapping_add(core::mem::size_of::<ethhdr>() as __u16);

                return parse_eth_proto(skb, (*eth).h_proto);
            } else {
                return parse_eth_proto(skb, (*gre).proto);
            }
        }
        x if x == IPPROTO_TCP as u32 => {
            let tcp = bpf_flow_dissect_get_header(
                skb,
                core::mem::size_of::<tcphdr>() as __u16,
                &mut _tcp as *mut tcphdr as *mut core::ffi::c_void,
            ) as *mut tcphdr;
            if tcp.is_null() {
                return export_flow_keys(keys, BPF_DROP);
            }

            if (*tcp).doff < 5 {
                return export_flow_keys(keys, BPF_DROP);
            }

            if (tcp as *mut __u8).add(((*tcp).doff << 2) as usize) > data_end as *mut __u8 {
                return export_flow_keys(keys, BPF_DROP);
            }

            (*keys).sport = (*tcp).source;
            (*keys).dport = (*tcp).dest;
            return export_flow_keys(keys, BPF_OK);
        }
        x if x == IPPROTO_UDP as u32 || x == IPPROTO_UDPLITE as u32 => {
            let udp = bpf_flow_dissect_get_header(
                skb,
                core::mem::size_of::<udphdr>() as __u16,
                &mut _udp as *mut udphdr as *mut core::ffi::c_void,
            ) as *mut udphdr;
            if udp.is_null() {
                return export_flow_keys(keys, BPF_DROP);
            }

            (*keys).sport = (*udp).source;
            (*keys).dport = (*udp).dest;
            return export_flow_keys(keys, BPF_OK);
        }
        _ => {
            return export_flow_keys(keys, BPF_DROP);
        }
    }
}

#[inline(always)]
unsafe fn parse_ipv6_proto(skb: *mut __sk_buff, nexthdr: __u8) -> i32 {
    let keys: *mut bpf_flow_keys = (*skb).flow_keys;

    match nexthdr as u32 {
        x if x == IPPROTO_HOPOPTS as u32 || x == IPPROTO_DSTOPTS as u32 => {
            bpf_tail_call_static(skb, core::ptr::addr_of_mut!(jmp_table) as *mut _, IPV6OP);
        }
        x if x == IPPROTO_FRAGMENT as u32 => {
            bpf_tail_call_static(skb, core::ptr::addr_of_mut!(jmp_table) as *mut _, IPV6FR);
        }
        _ => {
            return parse_ip_proto(skb, nexthdr);
        }
    }

    export_flow_keys(keys, BPF_DROP)
}

// SEC("flow_dissector")
#[no_mangle]
pub unsafe extern "C" fn flow_dissector_IP(skb: *mut __sk_buff) -> i32 {
    let data_end = (*skb).data_end as usize as *mut core::ffi::c_void;
    let keys: *mut bpf_flow_keys = (*skb).flow_keys;
    let data = (*skb).data as usize as *mut core::ffi::c_void;
    let mut _iph: iphdr = core::mem::zeroed();
    let mut done = false;

    let iph = bpf_flow_dissect_get_header(
        skb,
        core::mem::size_of::<iphdr>() as __u16,
        &mut _iph as *mut iphdr as *mut core::ffi::c_void,
    ) as *mut iphdr;
    if iph.is_null() {
        return export_flow_keys(keys, BPF_DROP);
    }

    /* IP header cannot be smaller than 20 bytes */
    if (*iph).ihl < 5 {
        return export_flow_keys(keys, BPF_DROP);
    }

    (*keys).addr_proto = ETH_P_IP;
    (*keys).ipv4_src = (*iph).saddr;
    (*keys).ipv4_dst = (*iph).daddr;
    (*keys).ip_proto = (*iph).protocol;

    (*keys).thoff = (*keys).thoff.wrapping_add(((*iph).ihl << 2) as __u16);
    if (data as *mut __u8).add((*keys).thoff as usize) > data_end as *mut __u8 {
        return export_flow_keys(keys, BPF_DROP);
    }

    if ((*iph).frag_off & bpf_htons((IP_MF | IP_OFFSET) as __u16)) != 0 {
        (*keys).is_frag = true;
        if ((*iph).frag_off & bpf_htons(IP_OFFSET as __u16)) != 0 {
            /* From second fragment on, packets do not have headers
             * we can parse.
             */
            done = true;
        } else {
            (*keys).is_first_frag = true;
            /* No need to parse fragmented packet unless
             * explicitly asked for.
             */
            if ((*keys).flags & BPF_FLOW_DISSECTOR_F_PARSE_1ST_FRAG) == 0 {
                done = true;
            }
        }
    }

    if done {
        return export_flow_keys(keys, BPF_OK);
    }

    parse_ip_proto(skb, (*iph).protocol)
}

// SEC("flow_dissector")
#[no_mangle]
pub unsafe extern "C" fn flow_dissector_IPV6(skb: *mut __sk_buff) -> i32 {
    let keys: *mut bpf_flow_keys = (*skb).flow_keys;
    let mut _ip6h: ipv6hdr = core::mem::zeroed();

    let ip6h = bpf_flow_dissect_get_header(
        skb,
        core::mem::size_of::<ipv6hdr>() as __u16,
        &mut _ip6h as *mut ipv6hdr as *mut core::ffi::c_void,
    ) as *mut ipv6hdr;
    if ip6h.is_null() {
        return export_flow_keys(keys, BPF_DROP);
    }

    (*keys).addr_proto = ETH_P_IPV6;
    core::ptr::copy_nonoverlapping(
        core::ptr::addr_of!((*ip6h).saddr) as *const u8,
        core::ptr::addr_of_mut!((*keys).ipv6_src) as *mut u8,
        2 * core::mem::size_of_val(&(*ip6h).saddr),
    );

    (*keys).thoff = (*keys)
        .thoff
        .wrapping_add(core::mem::size_of::<ipv6hdr>() as __u16);
    (*keys).ip_proto = (*ip6h).nexthdr;
    (*keys).flow_label = ip6_flowlabel(ip6h);

    if (*keys).flow_label != 0
        && ((*keys).flags & BPF_FLOW_DISSECTOR_F_STOP_AT_FLOW_LABEL) != 0
    {
        return export_flow_keys(keys, BPF_OK);
    }

    parse_ipv6_proto(skb, (*ip6h).nexthdr)
}

// SEC("flow_dissector")
#[no_mangle]
pub unsafe extern "C" fn flow_dissector_IPV6OP(skb: *mut __sk_buff) -> i32 {
    let keys: *mut bpf_flow_keys = (*skb).flow_keys;
    let mut _ip6h: ipv6_opt_hdr = core::mem::zeroed();

    let ip6h = bpf_flow_dissect_get_header(
        skb,
        core::mem::size_of::<ipv6_opt_hdr>() as __u16,
        &mut _ip6h as *mut ipv6_opt_hdr as *mut core::ffi::c_void,
    ) as *mut ipv6_opt_hdr;
    if ip6h.is_null() {
        return export_flow_keys(keys, BPF_DROP);
    }

    /* hlen is in 8-octets and does not include the first 8 bytes
     * of the header
     */
    (*keys).thoff = (*keys)
        .thoff
        .wrapping_add(((1 + (*ip6h).hdrlen as __u16) << 3) as __u16);
    (*keys).ip_proto = (*ip6h).nexthdr;

    parse_ipv6_proto(skb, (*ip6h).nexthdr)
}

// SEC("flow_dissector")
#[no_mangle]
pub unsafe extern "C" fn flow_dissector_IPV6FR(skb: *mut __sk_buff) -> i32 {
    let keys: *mut bpf_flow_keys = (*skb).flow_keys;
    let mut _fragh: frag_hdr = core::mem::zeroed();

    let fragh = bpf_flow_dissect_get_header(
        skb,
        core::mem::size_of::<frag_hdr>() as __u16,
        &mut _fragh as *mut frag_hdr as *mut core::ffi::c_void,
    ) as *mut frag_hdr;
    if fragh.is_null() {
        return export_flow_keys(keys, BPF_DROP);
    }

    (*keys).thoff = (*keys)
        .thoff
        .wrapping_add(core::mem::size_of::<frag_hdr>() as __u16);
    (*keys).is_frag = true;
    (*keys).ip_proto = (*fragh).nexthdr;

    if ((*fragh).frag_off & bpf_htons(IP6_OFFSET as __u16)) == 0 {
        (*keys).is_first_frag = true;

        /* No need to parse fragmented packet unless
         * explicitly asked for.
         */
        if ((*keys).flags & BPF_FLOW_DISSECTOR_F_PARSE_1ST_FRAG) == 0 {
            return export_flow_keys(keys, BPF_OK);
        }
    } else {
        return export_flow_keys(keys, BPF_OK);
    }

    parse_ipv6_proto(skb, (*fragh).nexthdr)
}

// SEC("flow_dissector")
#[no_mangle]
pub unsafe extern "C" fn flow_dissector_MPLS(skb: *mut __sk_buff) -> i32 {
    let keys: *mut bpf_flow_keys = (*skb).flow_keys;
    let mut _mpls: mpls_label = core::mem::zeroed();

    let mpls = bpf_flow_dissect_get_header(
        skb,
        core::mem::size_of::<mpls_label>() as __u16,
        &mut _mpls as *mut mpls_label as *mut core::ffi::c_void,
    ) as *mut mpls_label;
    if mpls.is_null() {
        return export_flow_keys(keys, BPF_DROP);
    }

    export_flow_keys(keys, BPF_OK)
}

// SEC("flow_dissector")
#[no_mangle]
pub unsafe extern "C" fn flow_dissector_VLAN(skb: *mut __sk_buff) -> i32 {
    let keys: *mut bpf_flow_keys = (*skb).flow_keys;
    let mut _vlan: vlan_hdr = core::mem::zeroed();

    /* Account for double-tagging */
    if (*keys).n_proto == bpf_htons(ETH_P_8021AD as __u16) {
        let vlan = bpf_flow_dissect_get_header(
            skb,
            core::mem::size_of::<vlan_hdr>() as __u16,
            &mut _vlan as *mut vlan_hdr as *mut core::ffi::c_void,
        ) as *mut vlan_hdr;
        if vlan.is_null() {
            return export_flow_keys(keys, BPF_DROP);
        }

        if (*vlan).h_vlan_encapsulated_proto != bpf_htons(ETH_P_8021Q as __u16) {
            return export_flow_keys(keys, BPF_DROP);
        }

        (*keys).nhoff = (*keys)
            .nhoff
            .wrapping_add(core::mem::size_of::<vlan_hdr>() as __u16);
        (*keys).thoff = (*keys)
            .thoff
            .wrapping_add(core::mem::size_of::<vlan_hdr>() as __u16);
    }

    let vlan = bpf_flow_dissect_get_header(
        skb,
        core::mem::size_of::<vlan_hdr>() as __u16,
        &mut _vlan as *mut vlan_hdr as *mut core::ffi::c_void,
    ) as *mut vlan_hdr;
    if vlan.is_null() {
        return export_flow_keys(keys, BPF_DROP);
    }

    (*keys).nhoff = (*keys)
        .nhoff
        .wrapping_add(core::mem::size_of::<vlan_hdr>() as __u16);
    (*keys).thoff = (*keys)
        .thoff
        .wrapping_add(core::mem::size_of::<vlan_hdr>() as __u16);
    /* Only allow 8021AD + 8021Q double tagging and no triple tagging.*/
    if (*vlan).h_vlan_encapsulated_proto == bpf_htons(ETH_P_8021AD as __u16)
        || (*vlan).h_vlan_encapsulated_proto == bpf_htons(ETH_P_8021Q as __u16)
    {
        return export_flow_keys(keys, BPF_DROP);
    }

    (*keys).n_proto = (*vlan).h_vlan_encapsulated_proto;
    parse_eth_proto(skb, (*vlan).h_vlan_encapsulated_proto)
}

// SEC("license")
#[no_mangle]
pub static __license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
