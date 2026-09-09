/* SPDX-License-Identifier: GPL-2.0
 * Copyright (c) 2018 Jesper Dangaard Brouer, Red Hat Inc.
 *
 * Example howto transfer info from XDP to SKB, e.g. skb->mark
 * -----------------------------------------------------------
 * This uses the XDP data_meta infrastructure, and is a cooperation
 * between two bpf-programs (1) XDP and (2) clsact at TC-ingress hook.
 *
 * Notice: This example does not use the BPF C-loader,
 * but instead rely on the iproute2 TC tool for loading BPF-objects.
 */

// C dependencies: <uapi/linux/bpf.h>, <uapi/linux/pkt_cls.h>,
// and <bpf/bpf_helpers.h> supply the kernel types, constants, and helpers.

const XDP_ABORTED: i32 = 0;
const XDP_PASS: i32 = 2;
const TC_ACT_OK: i32 = 0;

#[repr(C)]
#[repr(align(4))]
pub struct meta_info {
    pub mark: u32,
}

#[repr(C)]
pub struct xdp_md {
    pub data: u32,
    pub data_end: u32,
    pub data_meta: u32,
    pub ingress_ifindex: u32,
    pub rx_queue_index: u32,
    pub egress_ifindex: u32,
}

#[repr(C)]
pub struct __sk_buff {
    pub data: u32,
    pub data_end: u32,
    pub data_meta: u32,
    pub mark: u32,
}

unsafe extern "C" {
    fn bpf_xdp_adjust_meta(ctx: *mut xdp_md, delta: i32) -> i32;
}

#[unsafe(link_section = "xdp_mark")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _xdp_mark(ctx: *mut xdp_md) -> i32 {
    let meta: *mut meta_info;
    let data: *mut core::ffi::c_void;
    let ret: i32;

    /* Reserve space in-front of data pointer for our meta info.
     * (Notice drivers not supporting data_meta will fail here!)
     */
    ret = bpf_xdp_adjust_meta(ctx, -(core::mem::size_of::<meta_info>() as i32));
    if ret < 0 {
        return XDP_ABORTED;
    }

    /* Notice: Kernel-side verifier requires that loading of
     * ctx->data MUST happen _after_ helper bpf_xdp_adjust_meta(),
     * as pkt-data pointers are invalidated.  Helpers that require
     * this are determined/marked by bpf_helper_changes_pkt_data()
     */
    data = (*ctx).data as usize as *mut core::ffi::c_void;

    /* Check data_meta have room for meta_info struct */
    meta = (*ctx).data_meta as usize as *mut meta_info;
    if meta.add(1) as *mut core::ffi::c_void > data {
        return XDP_ABORTED;
    }

    (*meta).mark = 42;

    XDP_PASS
}

#[unsafe(link_section = "tc_mark")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _tc_mark(ctx: *mut __sk_buff) -> i32 {
    let data: *mut core::ffi::c_void = (*ctx).data as usize as *mut core::ffi::c_void;
    let data_meta: *mut core::ffi::c_void = (*ctx).data_meta as usize as *mut core::ffi::c_void;
    let meta: *mut meta_info = data_meta as *mut meta_info;

    /* Check XDP gave us some data_meta */
    if meta.add(1) as *mut core::ffi::c_void > data {
        (*ctx).mark = 41;
        /* Skip "accept" if no data_meta is avail */
        return TC_ACT_OK;
    }

    /* Hint: See func tc_cls_act_is_valid_access() for BPF_WRITE access */
    (*ctx).mark = (*meta).mark; /* Transfer XDP-mark to SKB-mark */

    TC_ACT_OK
}

/* Manually attaching these programs:
export DEV=ixgbe2
export FILE=xdp2skb_meta_kern.o

# via TC command
tc qdisc del dev $DEV clsact 2> /dev/null
tc qdisc add dev $DEV clsact
tc filter  add dev $DEV ingress prio 1 handle 1 bpf da obj $FILE sec tc_mark
tc filter show dev $DEV ingress

# XDP via IP command:
ip link set dev $DEV xdp off
ip link set dev $DEV xdp obj $FILE sec xdp_mark

# Use iptable to "see" if SKBs are marked
iptables -I INPUT -p icmp -m mark --mark 41  # == 0x29
iptables -I INPUT -p icmp -m mark --mark 42  # == 0x2a

# Hint: catch XDP_ABORTED errors via
perf record -e xdp:*
perf script

*/

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
