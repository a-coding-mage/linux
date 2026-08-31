// SPDX-License-Identifier: GPL-2.0
// Dependencies from the original C source:
// <stdint.h>, <linux/bpf.h>, <linux/if_ether.h>, <linux/stddef.h>,
// <linux/in.h>, <linux/ip.h>, <linux/pkt_cls.h>, <linux/tcp.h>,
// <bpf/bpf_helpers.h>, and <bpf/bpf_endian.h>.

/* the maximum delay we are willing to add (drop packets beyond that) */
const TIME_HORIZON_NS: u64 = 2000 * 1000 * 1000;
const NS_PER_SEC: u64 = 1000000000;
const ECN_HORIZON_NS: u64 = 5000000;

const BPF_MAP_TYPE_HASH: u32 = 1;
const BPF_ANY: u64 = 0;
const BPF_EXIST: u64 = 2;
const TC_ACT_OK: i32 = 0;
const TC_ACT_SHOT: i32 = 2;
const ETH_P_IP: u16 = 0x0800;
const IPPROTO_TCP: u8 = 6;

#[repr(C)]
pub struct __sk_buff {
    pub len: u32,
    pub pkt_type: u32,
    pub mark: u32,
    pub queue_mapping: u32,
    pub protocol: u32,
    pub vlan_present: u32,
    pub vlan_tci: u32,
    pub vlan_proto: u32,
    pub priority: u32,
    pub ingress_ifindex: u32,
    pub ifindex: u32,
    pub tc_index: u32,
    pub cb: [u32; 5],
    pub hash: u32,
    pub tc_classid: u32,
    pub data: u32,
    pub data_end: u32,
    pub napi_id: u32,
    pub family: u32,
    pub remote_ip4: u32,
    pub local_ip4: u32,
    pub remote_ip6: [u32; 4],
    pub local_ip6: [u32; 4],
    pub remote_port: u32,
    pub local_port: u32,
    pub data_meta: u32,
    pub tstamp: u64,
    pub wire_len: u32,
    pub gso_segs: u32,
    pub sk: *mut core::ffi::c_void,
    pub gso_size: u32,
    pub tstamp_type: u8,
    pub hwtstamp: u64,
}

#[repr(C)]
pub struct ethhdr {
    pub h_dest: [u8; 6],
    pub h_source: [u8; 6],
    pub h_proto: u16,
}

#[repr(C)]
pub struct iphdr {
    pub _bitfield_1: u8,
    pub tos: u8,
    pub tot_len: u16,
    pub id: u16,
    pub frag_off: u16,
    pub ttl: u8,
    pub protocol: u8,
    pub check: u16,
    pub saddr: u32,
    pub daddr: u32,
}

impl iphdr {
    #[inline(always)]
    unsafe fn ihl(&self) -> u8 {
        self._bitfield_1 & 0x0f
    }
}

#[repr(C)]
pub struct tcphdr {
    pub source: u16,
    pub dest: u16,
    pub seq: u32,
    pub ack_seq: u32,
    pub _bitfield_1: u16,
    pub window: u16,
    pub check: u16,
    pub urg_ptr: u16,
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: u32,
    pub key_size: u32,
    pub value_size: u32,
    pub max_entries: u32,
    pub map_flags: u32,
}

/* flow_key => last_tstamp timestamp used */
#[no_mangle]
#[link_section = ".maps"]
pub static mut flow_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_HASH,
    key_size: core::mem::size_of::<u32>() as u32,
    value_size: core::mem::size_of::<u64>() as u32,
    max_entries: 1,
    map_flags: 0,
};

#[no_mangle]
pub static mut target_rate: u64 = 0;

extern "C" {
    fn bpf_map_lookup_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    fn bpf_map_update_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: u64,
    ) -> i64;
    fn bpf_ktime_get_ns() -> u64;
    fn bpf_skb_ecn_set_ce(skb: *mut __sk_buff) -> i64;
    fn bpf_htons(hostshort: u16) -> u16;
}

#[inline(always)]
unsafe fn throttle_flow(skb: *mut __sk_buff) -> i32 {
    let key: i32 = 0;
    let last_tstamp: *mut u64 = bpf_map_lookup_elem(
        core::ptr::addr_of_mut!(flow_map) as *mut core::ffi::c_void,
        (&key as *const i32).cast::<core::ffi::c_void>(),
    )
    .cast::<u64>();
    let delay_ns: u64 = ((*skb).len as u64)
        .wrapping_mul(NS_PER_SEC)
        .wrapping_div(target_rate);
    let now: u64 = bpf_ktime_get_ns();
    let mut tstamp: u64;
    let mut next_tstamp: u64 = 0;

    if !last_tstamp.is_null() {
        next_tstamp = (*last_tstamp).wrapping_add(delay_ns);
    }

    tstamp = (*skb).tstamp;
    if tstamp < now {
        tstamp = now;
    }

    /* should we throttle? */
    if next_tstamp <= tstamp {
        if bpf_map_update_elem(
            core::ptr::addr_of_mut!(flow_map) as *mut core::ffi::c_void,
            (&key as *const i32).cast::<core::ffi::c_void>(),
            (&tstamp as *const u64).cast::<core::ffi::c_void>(),
            BPF_ANY,
        ) != 0
        {
            return TC_ACT_SHOT;
        }
        return TC_ACT_OK;
    }

    /* do not queue past the time horizon */
    if next_tstamp.wrapping_sub(now) >= TIME_HORIZON_NS {
        return TC_ACT_SHOT;
    }

    /* set ecn bit, if needed */
    if next_tstamp.wrapping_sub(now) >= ECN_HORIZON_NS {
        bpf_skb_ecn_set_ce(skb);
    }

    if bpf_map_update_elem(
        core::ptr::addr_of_mut!(flow_map) as *mut core::ffi::c_void,
        (&key as *const i32).cast::<core::ffi::c_void>(),
        (&next_tstamp as *const u64).cast::<core::ffi::c_void>(),
        BPF_EXIST,
    ) != 0
    {
        return TC_ACT_SHOT;
    }
    (*skb).tstamp = next_tstamp;

    TC_ACT_OK
}

#[inline(always)]
unsafe fn handle_tcp(skb: *mut __sk_buff, tcp: *mut tcphdr) -> i32 {
    let data_end: *mut core::ffi::c_void = ((*skb).data_end as usize) as *mut core::ffi::c_void;

    /* drop malformed packets */
    if tcp.add(1) as *mut core::ffi::c_void > data_end {
        return TC_ACT_SHOT;
    }

    if (*tcp).source == bpf_htons(9000) {
        return throttle_flow(skb);
    }

    TC_ACT_OK
}

#[inline(always)]
unsafe fn handle_ipv4(skb: *mut __sk_buff) -> i32 {
    let data_end: *mut core::ffi::c_void = ((*skb).data_end as usize) as *mut core::ffi::c_void;
    let data: *mut core::ffi::c_void = ((*skb).data as usize) as *mut core::ffi::c_void;
    let mut iph: *mut iphdr;
    let ihl: u32;

    /* drop malformed packets */
    if (data as *mut u8).add(core::mem::size_of::<ethhdr>()) as *mut core::ffi::c_void > data_end {
        return TC_ACT_SHOT;
    }
    iph = (data as *mut u8).add(core::mem::size_of::<ethhdr>()) as *mut iphdr;
    if iph.add(1) as *mut core::ffi::c_void > data_end {
        return TC_ACT_SHOT;
    }
    ihl = ((*iph).ihl() as u32).wrapping_mul(4);
    if (iph as *mut core::ffi::c_void as *mut u8).add(ihl as usize) as *mut core::ffi::c_void
        > data_end
    {
        return TC_ACT_SHOT;
    }

    if (*iph).protocol == IPPROTO_TCP {
        return handle_tcp(
            skb,
            (iph as *mut core::ffi::c_void as *mut u8).add(ihl as usize) as *mut tcphdr,
        );
    }

    TC_ACT_OK
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn tc_prog(skb: *mut __sk_buff) -> i32 {
    if (*skb).protocol == bpf_htons(ETH_P_IP) as u32 {
        return handle_ipv4(skb);
    }

    TC_ACT_OK
}

#[no_mangle]
#[link_section = "license"]
pub static __license: [u8; 4] = *b"GPL\0";
