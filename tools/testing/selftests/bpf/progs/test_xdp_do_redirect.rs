// SPDX-License-Identifier: GPL-2.0
// Translated from C. External BPF/kernel types, constants, and helpers are
// expected to be provided by the surrounding build environment.

const ETH_ALEN: usize = 6;
const HDR_SZ: usize = core::mem::size_of::<ethhdr>()
    + core::mem::size_of::<ipv6hdr>()
    + core::mem::size_of::<udphdr>();

/**
 * enum frame_mark - magics to distinguish page/packet paths
 * @MARK_XMIT: page was recycled due to the frame being "xmitted" by the NIC.
 * @MARK_IN: frame is being processed by the input XDP prog.
 * @MARK_SKB: frame did hit the TC ingress hook as an skb.
 */
#[repr(u32)]
enum frame_mark {
    MARK_XMIT = 0u32,
    MARK_IN = 0x42,
    MARK_SKB = 0x45,
}

unsafe extern "C" {
    fn bpf_xdp_adjust_meta(xdp: *mut xdp_md, delta: __s32) -> c_long;
    fn bpf_redirect(ifindex: __u32, flags: __u64) -> c_long;
}

#[no_mangle]
pub static mut ifindex_out: core::ffi::c_int = 0;
#[no_mangle]
pub static mut ifindex_in: core::ffi::c_int = 0;
#[no_mangle]
pub static mut expect_dst: [__u8; ETH_ALEN] = [0; ETH_ALEN];
#[no_mangle]
pub static mut pkts_seen_xdp: core::ffi::c_int = 0;
#[no_mangle]
pub static mut pkts_seen_zero: core::ffi::c_int = 0;
#[no_mangle]
pub static mut pkts_seen_tc: core::ffi::c_int = 0;
#[no_mangle]
pub static mut retcode: core::ffi::c_int = XDP_REDIRECT;

#[no_mangle]
#[link_section = "xdp"]
pub unsafe extern "C" fn xdp_redirect(xdp: *mut xdp_md) -> core::ffi::c_int {
    let metadata = (*xdp).data_meta as usize as *mut __u32;
    let data_end = (*xdp).data_end as usize as *mut core::ffi::c_void;
    let data = (*xdp).data as usize as *mut core::ffi::c_void;

    let payload = (data as *mut __u8).add(HDR_SZ);
    let ret = core::ptr::read_volatile(core::ptr::addr_of!(retcode));

    if payload.add(1) as *mut core::ffi::c_void > data_end {
        return XDP_ABORTED;
    }

    if (*xdp).ingress_ifindex != core::ptr::read_volatile(core::ptr::addr_of!(ifindex_in)) as __u32 {
        return XDP_ABORTED;
    }

    if metadata.add(1) as *mut core::ffi::c_void > data {
        return XDP_ABORTED;
    }

    if *metadata != 0x42 {
        return XDP_ABORTED;
    }

    if *payload == frame_mark::MARK_XMIT as __u8 {
        let seen = core::ptr::read_volatile(core::ptr::addr_of!(pkts_seen_zero));
        core::ptr::write_volatile(core::ptr::addr_of_mut!(pkts_seen_zero), seen + 1);
    }

    *payload = frame_mark::MARK_IN as __u8;

    if bpf_xdp_adjust_meta(xdp, core::mem::size_of::<__u64>() as __s32) != 0 {
        return XDP_ABORTED;
    }

    if core::ptr::read_volatile(core::ptr::addr_of!(retcode)) > XDP_PASS {
        let code = core::ptr::read_volatile(core::ptr::addr_of!(retcode));
        core::ptr::write_volatile(core::ptr::addr_of_mut!(retcode), code - 1);
    }

    if ret == XDP_REDIRECT {
        return bpf_redirect(core::ptr::read_volatile(core::ptr::addr_of!(ifindex_out)) as __u32, 0)
            as core::ffi::c_int;
    }

    ret
}

unsafe fn check_pkt(
    data: *mut core::ffi::c_void,
    data_end: *mut core::ffi::c_void,
    mark: __u32,
) -> bool {
    let iph = (data as *mut __u8).add(core::mem::size_of::<ethhdr>()) as *mut ipv6hdr;
    let payload = (data as *mut __u8).add(HDR_SZ);

    if payload.add(1) as *mut core::ffi::c_void > data_end {
        return false;
    }

    if (*iph).nexthdr != IPPROTO_UDP || *payload != frame_mark::MARK_IN as __u8 {
        return false;
    }

    /* reset the payload so the same packet doesn't get counted twice when
     * it cycles back through the kernel path and out the dst veth
     */
    *payload = mark as __u8;
    true
}

#[no_mangle]
#[link_section = "xdp"]
pub unsafe extern "C" fn xdp_count_pkts(xdp: *mut xdp_md) -> core::ffi::c_int {
    let data = (*xdp).data as usize as *mut core::ffi::c_void;
    let data_end = (*xdp).data_end as usize as *mut core::ffi::c_void;

    if check_pkt(data, data_end, frame_mark::MARK_XMIT as __u32) {
        let seen = core::ptr::read_volatile(core::ptr::addr_of!(pkts_seen_xdp));
        core::ptr::write_volatile(core::ptr::addr_of_mut!(pkts_seen_xdp), seen + 1);
    }

    /* Return %XDP_DROP to recycle the data page with %MARK_XMIT, like
     * it exited a physical NIC. Those pages will be counted in the
     * pkts_seen_zero counter above.
     */
    XDP_DROP
}

#[no_mangle]
#[link_section = "xdp"]
pub unsafe extern "C" fn xdp_redirect_to_111(_xdp: *mut xdp_md) -> core::ffi::c_int {
    bpf_redirect(111, 0) as core::ffi::c_int
}

#[no_mangle]
#[link_section = "xdp"]
pub unsafe extern "C" fn xdp_redirect_to_222(_xdp: *mut xdp_md) -> core::ffi::c_int {
    bpf_redirect(222, 0) as core::ffi::c_int
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn tc_count_pkts(skb: *mut __sk_buff) -> core::ffi::c_int {
    let data = (*skb).data as usize as *mut core::ffi::c_void;
    let data_end = (*skb).data_end as usize as *mut core::ffi::c_void;

    if check_pkt(data, data_end, frame_mark::MARK_SKB as __u32) {
        let seen = core::ptr::read_volatile(core::ptr::addr_of!(pkts_seen_tc));
        core::ptr::write_volatile(core::ptr::addr_of_mut!(pkts_seen_tc), seen + 1);
    }

    /* Will be either recycled or freed, %MARK_SKB makes sure it won't
     * hit any of the counters above.
     */
    0
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [core::ffi::c_char; 4] = [
    b'G' as core::ffi::c_char,
    b'P' as core::ffi::c_char,
    b'L' as core::ffi::c_char,
    0,
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
