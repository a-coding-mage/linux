// SPDX-License-Identifier: GPL-2.0
// C source included <vmlinux.h>, <bpf/bpf_helpers.h>, and <bpf/bpf_endian.h>.
// The translated BPF helper/types/section attributes are expected from the
// surrounding build/bindings.

pub const BPF_NO_KFUNC_PROTOTYPES: bool = true;

pub const ETH_P_IP: u16 = 0x0800;
pub const ETH_P_IPV6: u16 = 0x86dd;
pub const IP_MF: u16 = 0x2000; /* "More Fragments" */
pub const IP_OFFSET: u16 = 0x1fff; /* "Fragment Offset" */
pub const AF_INET: u32 = 2;
pub const AF_INET6: u32 = 10;

#[repr(C)]
pub struct bpf_flowtable_opts___local {
    pub error: s32,
}

#[repr(C)]
pub struct flow_offload_tuple_rhash___local {}

extern "C" {
    #[link_name = "bpf_xdp_flow_lookup"]
    pub fn bpf_xdp_flow_lookup(
        arg1: *mut xdp_md,
        arg2: *mut bpf_fib_lookup,
        arg3: *mut bpf_flowtable_opts___local,
        arg4: u32,
    ) -> *mut flow_offload_tuple_rhash___local;
}

#[repr(C)]
pub struct stats_map {
    // Original C declaration:
    // __uint(type, BPF_MAP_TYPE_ARRAY);
    // __type(key, __u32);
    // __type(value, __u32);
    // __uint(max_entries, 1);
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut stats: stats_map = stats_map {};

#[inline]
unsafe fn xdp_flowtable_offload_check_iphdr(iph: *mut iphdr) -> bool {
    /* ip fragmented traffic */
    if ((*iph).frag_off & bpf_htons((IP_MF | IP_OFFSET) as u16)) != 0 {
        return false;
    }

    /* ip options */
    if ((*iph).ihl * 4) as usize != core::mem::size_of::<iphdr>() {
        return false;
    }

    if (*iph).ttl <= 1 {
        return false;
    }

    true
}

#[inline]
unsafe fn xdp_flowtable_offload_check_tcp_state(
    ports: *mut core::ffi::c_void,
    data_end: *mut core::ffi::c_void,
    proto: u8,
) -> bool {
    if proto == IPPROTO_TCP as u8 {
        let tcph: *mut tcphdr = ports as *mut tcphdr;

        if tcph.add(1) as *mut core::ffi::c_void > data_end {
            return false;
        }

        if (*tcph).fin() != 0 || (*tcph).rst() != 0 {
            return false;
        }
    }

    true
}

#[repr(C)]
pub struct flow_ports___local {
    pub source: __be16,
    pub dest: __be16,
}
// Original C used __attribute__((preserve_access_index)).

#[no_mangle]
#[link_section = "xdp.frags"]
pub unsafe extern "C" fn xdp_flowtable_do_lookup(ctx: *mut xdp_md) -> i32 {
    let data_end: *mut core::ffi::c_void = (*ctx).data_end as usize as *mut core::ffi::c_void;
    let mut opts: bpf_flowtable_opts___local = core::mem::zeroed();
    let mut tuplehash: *mut flow_offload_tuple_rhash___local;
    let mut tuple: bpf_fib_lookup = core::mem::zeroed();
    tuple.ifindex = (*ctx).ingress_ifindex;
    let data: *mut core::ffi::c_void = (*ctx).data as usize as *mut core::ffi::c_void;
    let eth: *mut ethhdr = data as *mut ethhdr;
    let mut ports: *mut flow_ports___local;
    let mut key: __u32 = 0;
    let mut val: *mut __u32;

    if eth.add(1) as *mut core::ffi::c_void > data_end {
        return XDP_DROP;
    }

    match (*eth).h_proto {
        x if x == bpf_htons(ETH_P_IP) => {
            let iph: *mut iphdr = (data as *mut u8).add(core::mem::size_of::<ethhdr>()) as *mut iphdr;

            ports = iph.add(1) as *mut flow_ports___local;
            if ports.add(1) as *mut core::ffi::c_void > data_end {
                return XDP_PASS;
            }

            /* sanity check on ip header */
            if !xdp_flowtable_offload_check_iphdr(iph) {
                return XDP_PASS;
            }

            if !xdp_flowtable_offload_check_tcp_state(
                ports as *mut core::ffi::c_void,
                data_end,
                (*iph).protocol,
            ) {
                return XDP_PASS;
            }

            tuple.family = AF_INET;
            tuple.tos = (*iph).tos;
            tuple.l4_protocol = (*iph).protocol;
            tuple.tot_len = bpf_ntohs((*iph).tot_len);
            tuple.ipv4_src = (*iph).saddr;
            tuple.ipv4_dst = (*iph).daddr;
            tuple.sport = (*ports).source;
            tuple.dport = (*ports).dest;
        }
        x if x == bpf_htons(ETH_P_IPV6) => {
            let src: *mut in6_addr = tuple.ipv6_src.as_mut_ptr() as *mut in6_addr;
            let dst: *mut in6_addr = tuple.ipv6_dst.as_mut_ptr() as *mut in6_addr;
            let ip6h: *mut ipv6hdr =
                (data as *mut u8).add(core::mem::size_of::<ethhdr>()) as *mut ipv6hdr;

            ports = ip6h.add(1) as *mut flow_ports___local;
            if ports.add(1) as *mut core::ffi::c_void > data_end {
                return XDP_PASS;
            }

            if (*ip6h).hop_limit <= 1 {
                return XDP_PASS;
            }

            if !xdp_flowtable_offload_check_tcp_state(
                ports as *mut core::ffi::c_void,
                data_end,
                (*ip6h).nexthdr,
            ) {
                return XDP_PASS;
            }

            tuple.family = AF_INET6;
            tuple.l4_protocol = (*ip6h).nexthdr;
            tuple.tot_len = bpf_ntohs((*ip6h).payload_len);
            *src = (*ip6h).saddr;
            *dst = (*ip6h).daddr;
            tuple.sport = (*ports).source;
            tuple.dport = (*ports).dest;
        }
        _ => {
            return XDP_PASS;
        }
    }

    tuplehash = bpf_xdp_flow_lookup(
        ctx,
        &mut tuple,
        &mut opts,
        core::mem::size_of::<bpf_flowtable_opts___local>() as u32,
    );
    if tuplehash.is_null() {
        return XDP_PASS;
    }

    val = bpf_map_lookup_elem(
        core::ptr::addr_of_mut!(stats) as *mut core::ffi::c_void,
        &mut key as *mut __u32 as *mut core::ffi::c_void,
    ) as *mut __u32;
    if !val.is_null() {
        __sync_add_and_fetch(val, 1);
    }

    XDP_PASS
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";
