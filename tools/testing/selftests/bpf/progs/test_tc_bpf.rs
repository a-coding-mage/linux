// SPDX-License-Identifier: GPL-2.0

// C includes translated as external dependencies expected from the BPF build
// environment:
// - <linux/bpf.h>
// - <bpf/bpf_helpers.h>
// - <linux/if_ether.h>
// - <linux/ip.h>

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
}

#[repr(C, packed)]
struct ethhdr {
    h_dest: [u8; 6],
    h_source: [u8; 6],
    h_proto: u16,
}

#[repr(C, packed)]
struct iphdr {
    ihl_version: u8,
    tos: u8,
    tot_len: u16,
    id: u16,
    frag_off: u16,
    ttl: u8,
    protocol: u8,
    check: u16,
    saddr: u32,
    daddr: u32,
}

/* Dummy prog to test TC-BPF API */

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn cls(skb: *mut __sk_buff) -> i32 {
    return 0;
}

/* Prog to verify tc-bpf without cap_sys_admin and cap_perfmon */
#[link_section = "tcx/ingress"]
#[no_mangle]
pub unsafe extern "C" fn pkt_ptr(skb: *mut __sk_buff) -> i32 {
    let iph: *mut iphdr = ((*skb).data as *mut ::core::ffi::c_void)
        .offset(::core::mem::size_of::<ethhdr>() as isize) as *mut iphdr;

    if (iph.offset(1) as i64) > ((*skb).data_end as i64) {
        return 1;
    }
    return 0;
}
