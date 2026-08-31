// SPDX-License-Identifier: GPL-2.0
// C dependencies translated as external Rust dependencies:
// "vmlinux.h", <string.h>, <bpf/bpf_helpers.h>, <bpf/bpf_endian.h>,
// and <bpf/bpf_tracing.h>.

#[repr(C)]
pub struct grehdr {
    pub flags: __be16,
    pub protocol: __be16,
}

unsafe extern "C" {
    fn bpf_htons(x: __u16) -> __be16;
    fn bpf_htonl(x: __u32) -> __be32;
    fn bpf_lwt_push_encap(
        skb: *mut __sk_buff,
        r#type: __u32,
        hdr: *mut core::ffi::c_void,
        len: __u32,
    ) -> core::ffi::c_int;
}

// SEC("encap_gre")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bpf_lwt_encap_gre(skb: *mut __sk_buff) -> core::ffi::c_int {
    #[repr(C)]
    struct encap_hdr {
        iph: iphdr,
        greh: grehdr,
    }

    let mut hdr: encap_hdr = unsafe { core::mem::zeroed() };
    let err: core::ffi::c_int;

    hdr.iph.ihl = 5;
    hdr.iph.version = 4;
    hdr.iph.ttl = 0x40;
    hdr.iph.protocol = 47; /* IPPROTO_GRE */
    #[cfg(target_endian = "little")]
    {
        hdr.iph.saddr = 0x640110ac; /* 172.16.1.100 */
        hdr.iph.daddr = 0x641010ac; /* 172.16.16.100 */
    }
    #[cfg(target_endian = "big")]
    {
        hdr.iph.saddr = 0xac100164; /* 172.16.1.100 */
        hdr.iph.daddr = 0xac101064; /* 172.16.16.100 */
    }
    hdr.iph.tot_len = unsafe {
        bpf_htons(
            ((*skb).len as usize + core::mem::size_of::<encap_hdr>()) as __u16,
        )
    };

    hdr.greh.protocol = unsafe { (*skb).protocol };

    err = unsafe {
        bpf_lwt_push_encap(
            skb,
            BPF_LWT_ENCAP_IP,
            &mut hdr as *mut _ as *mut core::ffi::c_void,
            core::mem::size_of::<encap_hdr>() as __u32,
        )
    };
    if err != 0 {
        return BPF_DROP;
    }

    BPF_LWT_REROUTE
}

// SEC("encap_gre6")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bpf_lwt_encap_gre6(skb: *mut __sk_buff) -> core::ffi::c_int {
    #[repr(C)]
    struct encap_hdr {
        ip6hdr: ipv6hdr,
        greh: grehdr,
    }

    let mut hdr: encap_hdr = unsafe { core::mem::zeroed() };
    let err: core::ffi::c_int;

    hdr.ip6hdr.version = 6;
    hdr.ip6hdr.payload_len = unsafe {
        bpf_htons(((*skb).len as usize + core::mem::size_of::<grehdr>()) as __u16)
    };
    hdr.ip6hdr.nexthdr = 47; /* IPPROTO_GRE */
    hdr.ip6hdr.hop_limit = 0x40;
    /* fb01::1 */
    hdr.ip6hdr.saddr.in6_u.u6_addr8[0] = 0xfb;
    hdr.ip6hdr.saddr.in6_u.u6_addr8[1] = 1;
    hdr.ip6hdr.saddr.in6_u.u6_addr8[15] = 1;
    /* fb10::1 */
    hdr.ip6hdr.daddr.in6_u.u6_addr8[0] = 0xfb;
    hdr.ip6hdr.daddr.in6_u.u6_addr8[1] = 0x10;
    hdr.ip6hdr.daddr.in6_u.u6_addr8[15] = 1;

    hdr.greh.protocol = unsafe { (*skb).protocol };

    err = unsafe {
        bpf_lwt_push_encap(
            skb,
            BPF_LWT_ENCAP_IP,
            &mut hdr as *mut _ as *mut core::ffi::c_void,
            core::mem::size_of::<encap_hdr>() as __u32,
        )
    };
    if err != 0 {
        return BPF_DROP;
    }

    BPF_LWT_REROUTE
}

pub const VXLAN_PORT: __u16 = 4789;
pub const VXLAN_FLAGS: __u32 = 0x08000000;
pub const VXLAN_VNI: __u32 = 1;

pub const ETH_ALEN: usize = 6; /* Octets in one ethernet addr	 */
pub const ETH_P_IP: __u16 = 0x0800; /* Internet Protocol packet	*/
pub const ETH_P_IPV6: __u16 = 0x86DD; /* IPv6 over bluebook		*/

static bcast: [__u8; ETH_ALEN] = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff];

static srcmac: [__u8; ETH_ALEN] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x01];

// SEC("encap_vxlan")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bpf_lwt_encap_vxlan(skb: *mut __sk_buff) -> core::ffi::c_int {
    #[repr(C, packed)]
    struct encap_hdr {
        iph: iphdr,
        udph: udphdr,
        vxh: vxlanhdr,
        eth: ethhdr,
    }

    let mut hdr: encap_hdr = unsafe { core::mem::zeroed() };
    let err: core::ffi::c_int;

    hdr.iph.ihl = 5;
    hdr.iph.version = 4;
    hdr.iph.ttl = 0x40;
    hdr.iph.protocol = 17; /* IPPROTO_UDP */
    hdr.iph.tot_len = unsafe {
        bpf_htons(((*skb).len as usize + core::mem::size_of::<encap_hdr>()) as __u16)
    };
    #[cfg(target_endian = "little")]
    {
        hdr.iph.saddr = 0x640510ac; /* 172.16.5.100  */
        hdr.iph.daddr = 0x641110ac; /* 172.16.17.100 */
    }
    #[cfg(target_endian = "big")]
    {
        hdr.iph.saddr = 0xac100564; /* 172.16.5.100 */
        hdr.iph.daddr = 0xac101164; /* 172.16.17.100 */
    }

    hdr.udph.source = unsafe { bpf_htons(VXLAN_PORT) };
    hdr.udph.dest = unsafe { bpf_htons(VXLAN_PORT) };
    hdr.udph.len = unsafe {
        bpf_htons(
            ((*skb).len as usize
                + core::mem::size_of::<udphdr>()
                + core::mem::size_of::<vxlanhdr>()
                + core::mem::size_of::<ethhdr>()) as __u16,
        )
    };

    hdr.vxh.vx_flags = unsafe { bpf_htonl(VXLAN_FLAGS) };
    hdr.vxh.vx_vni = unsafe { bpf_htonl(VXLAN_VNI << 8) };

    unsafe {
        core::ptr::copy_nonoverlapping(bcast.as_ptr(), hdr.eth.h_dest.as_mut_ptr(), ETH_ALEN);
        core::ptr::copy_nonoverlapping(srcmac.as_ptr(), hdr.eth.h_source.as_mut_ptr(), ETH_ALEN);
    }
    hdr.eth.h_proto = unsafe { bpf_htons(ETH_P_IP) };

    err = unsafe {
        bpf_lwt_push_encap(
            skb,
            BPF_LWT_ENCAP_IP,
            &mut hdr as *mut _ as *mut core::ffi::c_void,
            core::mem::size_of::<encap_hdr>() as __u32,
        )
    };
    if err != 0 {
        return BPF_DROP;
    }

    BPF_LWT_REROUTE
}

// SEC("encap_vxlan6")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bpf_lwt_encap_vxlan6(skb: *mut __sk_buff) -> core::ffi::c_int {
    #[repr(C, packed)]
    struct encap_hdr {
        ip6hdr: ipv6hdr,
        udph: udphdr,
        vxh: vxlanhdr,
        eth: ethhdr,
    }

    let mut hdr: encap_hdr = unsafe { core::mem::zeroed() };
    let err: core::ffi::c_int;

    hdr.ip6hdr.version = 6;
    hdr.ip6hdr.nexthdr = 17; /* IPPROTO_UDP */
    hdr.ip6hdr.hop_limit = 0x40;
    hdr.ip6hdr.payload_len = unsafe {
        bpf_htons(
            ((*skb).len as usize
                + core::mem::size_of::<udphdr>()
                + core::mem::size_of::<vxlanhdr>()
                + core::mem::size_of::<ethhdr>()) as __u16,
        )
    };
    /* fb05::1 */
    hdr.ip6hdr.saddr.in6_u.u6_addr8[0] = 0xfb;
    hdr.ip6hdr.saddr.in6_u.u6_addr8[1] = 0x05;
    hdr.ip6hdr.saddr.in6_u.u6_addr8[15] = 1;
    /* fb11::1 */
    hdr.ip6hdr.daddr.in6_u.u6_addr8[0] = 0xfb;
    hdr.ip6hdr.daddr.in6_u.u6_addr8[1] = 0x11;
    hdr.ip6hdr.daddr.in6_u.u6_addr8[15] = 1;

    hdr.udph.source = unsafe { bpf_htons(VXLAN_PORT) };
    hdr.udph.dest = unsafe { bpf_htons(VXLAN_PORT) };
    hdr.udph.len = unsafe {
        bpf_htons(
            ((*skb).len as usize
                + core::mem::size_of::<udphdr>()
                + core::mem::size_of::<vxlanhdr>()
                + core::mem::size_of::<ethhdr>()) as __u16,
        )
    };

    hdr.vxh.vx_flags = unsafe { bpf_htonl(VXLAN_FLAGS) };
    hdr.vxh.vx_vni = unsafe { bpf_htonl(VXLAN_VNI << 8) };

    unsafe {
        core::ptr::copy_nonoverlapping(bcast.as_ptr(), hdr.eth.h_dest.as_mut_ptr(), ETH_ALEN);
        core::ptr::copy_nonoverlapping(srcmac.as_ptr(), hdr.eth.h_source.as_mut_ptr(), ETH_ALEN);
    }
    hdr.eth.h_proto = unsafe { bpf_htons(ETH_P_IPV6) };

    err = unsafe {
        bpf_lwt_push_encap(
            skb,
            BPF_LWT_ENCAP_IP,
            &mut hdr as *mut _ as *mut core::ffi::c_void,
            core::mem::size_of::<encap_hdr>() as __u32,
        )
    };
    if err != 0 {
        return BPF_DROP;
    }

    BPF_LWT_REROUTE
}

pub static tgt_ip_version: core::ffi::c_int = 0;

#[unsafe(no_mangle)]
pub static mut transport_hdr: __u16 = 0;
#[unsafe(no_mangle)]
pub static mut network_hdr: __u16 = 0;
#[unsafe(no_mangle)]
pub static mut fexit_triggered: bool = false;

// SEC("?fexit/bpf_lwt_push_ip_encap")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fexit_lwt_push_ip_encap(
    skb: *mut sk_buff,
    hdr: *mut core::ffi::c_void,
    len: u32,
    ingress: bool,
    retval: core::ffi::c_int,
) -> core::ffi::c_int {
    let iph: *mut iphdr;

    let _ = hdr;
    let _ = len;
    let _ = ingress;

    if retval != 0 || unsafe { fexit_triggered } {
        return 0;
    }

    iph = unsafe { ((*skb).head).add((*skb).network_header as usize) as *mut iphdr };
    if unsafe { (*iph).version } != tgt_ip_version {
        return 0;
    }

    if (unsafe { (*iph).version } == 4 && unsafe { (*iph).protocol } == 17 /* IPPROTO_UDP */)
        || (unsafe { (*iph).version } == 6
            && unsafe { (*(iph as *mut ipv6hdr)).nexthdr } == 17 /* IPPROTO_UDP */)
    {
        unsafe {
            fexit_triggered = true;
            transport_hdr = (*skb).transport_header;
            network_hdr = (*skb).network_header;
        }
    }
    0
}

// char _license[] SEC("license") = "GPL";
#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";
