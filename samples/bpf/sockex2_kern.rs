// Translated from sockex2_kern.c. The included kernel and BPF definitions are
// supplied by the surrounding build environment.

const IP_MF: u16 = 0x2000;
const IP_OFFSET: u16 = 0x1FFF;

#[repr(C)]
pub struct vlan_hdr {
    pub h_vlan_TCI: u16,
    pub h_vlan_encapsulated_proto: u16,
}

#[repr(C)]
pub union flow_key_record_ports {
    pub ports: u32,
    pub port16: [u16; 2],
}

#[repr(C)]
pub struct flow_key_record {
    pub src: u32,
    pub dst: u32,
    pub ports_union: flow_key_record_ports,
    pub thoff: u16,
    pub ip_proto: u8,
}

#[inline]
unsafe fn proto_ports_offset(proto: u64) -> i32 {
    match proto {
        IPPROTO_TCP | IPPROTO_UDP | IPPROTO_ESP | IPPROTO_SCTP | IPPROTO_UDPLITE => 0,
        IPPROTO_AH => 4,
        _ => 0,
    }
}

#[inline]
unsafe fn ip_is_fragment(ctx: *mut __sk_buff, nhoff: u64) -> i32 {
    load_half(ctx, nhoff + core::mem::offset_of!(iphdr, frag_off)) as i32
        & (IP_MF as i32 | IP_OFFSET as i32)
}

#[inline]
unsafe fn ipv6_addr_hash(ctx: *mut __sk_buff, off: u64) -> u32 {
    let w0 = load_word(ctx, off);
    let w1 = load_word(ctx, off + 4);
    let w2 = load_word(ctx, off + 8);
    let w3 = load_word(ctx, off + 12);
    (w0 ^ w1 ^ w2 ^ w3) as u32
}

#[inline]
unsafe fn parse_ip(
    skb: *mut __sk_buff,
    mut nhoff: u64,
    ip_proto: *mut u64,
    flow: *mut flow_key_record,
) -> u64 {
    if unlikely(ip_is_fragment(skb, nhoff) != 0) {
        *ip_proto = 0;
    } else {
        *ip_proto = load_byte(skb, nhoff + core::mem::offset_of!(iphdr, protocol));
    }

    if *ip_proto != IPPROTO_GRE as u64 {
        (*flow).src = load_word(skb, nhoff + core::mem::offset_of!(iphdr, saddr));
        (*flow).dst = load_word(skb, nhoff + core::mem::offset_of!(iphdr, daddr));
    }

    let verlen = load_byte(skb, nhoff + 0);
    if likely(verlen == 0x45) {
        nhoff += 20;
    } else {
        nhoff += ((verlen & 0xF) << 2) as u64;
    }
    nhoff
}

#[inline]
unsafe fn parse_ipv6(
    skb: *mut __sk_buff,
    mut nhoff: u64,
    ip_proto: *mut u64,
    flow: *mut flow_key_record,
) -> u64 {
    *ip_proto = load_byte(skb, nhoff + core::mem::offset_of!(ipv6hdr, nexthdr));
    (*flow).src = ipv6_addr_hash(skb, nhoff + core::mem::offset_of!(ipv6hdr, saddr));
    (*flow).dst = ipv6_addr_hash(skb, nhoff + core::mem::offset_of!(ipv6hdr, daddr));
    nhoff += core::mem::size_of::<ipv6hdr>() as u64;
    nhoff
}

#[inline]
unsafe fn flow_dissector(skb: *mut __sk_buff, flow: *mut flow_key_record) -> bool {
    let mut nhoff = ETH_HLEN as u64;
    let mut ip_proto: u64;
    let mut proto = load_half(skb, 12);
    let poff: i32;

    if proto == ETH_P_8021AD as u64 {
        proto = load_half(skb, nhoff + core::mem::offset_of!(vlan_hdr, h_vlan_encapsulated_proto));
        nhoff += core::mem::size_of::<vlan_hdr>() as u64;
    }
    if proto == ETH_P_8021Q as u64 {
        proto = load_half(skb, nhoff + core::mem::offset_of!(vlan_hdr, h_vlan_encapsulated_proto));
        nhoff += core::mem::size_of::<vlan_hdr>() as u64;
    }

    if likely(proto == ETH_P_IP as u64) {
        nhoff = parse_ip(skb, nhoff, &mut ip_proto, flow);
    } else if proto == ETH_P_IPV6 as u64 {
        nhoff = parse_ipv6(skb, nhoff, &mut ip_proto, flow);
    } else {
        return false;
    }

    match ip_proto {
        x if x == IPPROTO_GRE as u64 => {
            #[repr(C)] struct gre_hdr { flags: u16, proto: u16 }
            let gre_flags = load_half(skb, nhoff + core::mem::offset_of!(gre_hdr, flags));
            let gre_proto = load_half(skb, nhoff + core::mem::offset_of!(gre_hdr, proto));
            if gre_flags & (GRE_VERSION | GRE_ROUTING) != 0 { }
            else {
                proto = gre_proto; nhoff += 4;
                if gre_flags & GRE_CSUM != 0 { nhoff += 4; }
                if gre_flags & GRE_KEY != 0 { nhoff += 4; }
                if gre_flags & GRE_SEQ != 0 { nhoff += 4; }
                if proto == ETH_P_8021Q as u64 {
                    proto = load_half(skb, nhoff + core::mem::offset_of!(vlan_hdr, h_vlan_encapsulated_proto));
                    nhoff += core::mem::size_of::<vlan_hdr>() as u64;
                }
                if proto == ETH_P_IP as u64 { nhoff = parse_ip(skb, nhoff, &mut ip_proto, flow); }
                else if proto == ETH_P_IPV6 as u64 { nhoff = parse_ipv6(skb, nhoff, &mut ip_proto, flow); }
                else { return false; }
            }
        }
        x if x == IPPROTO_IPIP as u64 => { nhoff = parse_ip(skb, nhoff, &mut ip_proto, flow); }
        x if x == IPPROTO_IPV6 as u64 => { nhoff = parse_ipv6(skb, nhoff, &mut ip_proto, flow); }
        _ => {}
    }

    (*flow).ip_proto = ip_proto as u8;
    poff = proto_ports_offset(ip_proto);
    if poff >= 0 {
        nhoff += poff as u64;
        (*flow).ports_union.ports = load_word(skb, nhoff);
    }
    (*flow).thoff = nhoff as u16;
    true
}

#[repr(C)]
pub struct pair { pub packets: i64, pub bytes: i64 }

// BPF map declaration corresponding to: hash_map SEC(".maps").
extern "C" { static mut hash_map: bpf_map_def; }

#[no_mangle]
pub unsafe extern "C" fn bpf_prog2(skb: *mut __sk_buff) -> i32 {
    let mut flow: flow_key_record = core::mem::zeroed();
    if !flow_dissector(skb, &mut flow) { return 0; }
    let key = flow.dst;
    let value = bpf_map_lookup_elem(&mut hash_map, &key);
    if !value.is_null() {
        __sync_fetch_and_add(&mut (*value).packets, 1);
        __sync_fetch_and_add(&mut (*value).bytes, (*skb).len as i64);
    } else {
        let val = pair { packets: 1, bytes: (*skb).len as i64 };
        bpf_map_update_elem(&mut hash_map, &key, &val, BPF_ANY);
    }
    0
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
