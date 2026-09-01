// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Jesper Dangaard Brouer */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::c_void;

type __u32 = u32;

/* Dependencies from linux/bpf.h, bpf/bpf_helpers.h, linux/if_ether.h,
 * stddef.h, stdint.h, and errno.h are expected to be supplied by the BPF
 * build environment.
 */

#[repr(C)]
pub struct xdp_md {
    pub data: __u32,
    pub data_end: __u32,
}

#[repr(C)]
pub struct __sk_buff {
    pub len: __u32,
    pub data: __u32,
    pub data_end: __u32,
}

unsafe extern "C" {
    fn bpf_check_mtu(
        ctx: *mut c_void,
        ifindex: __u32,
        mtu_len: *mut __u32,
        len_diff: i32,
        flags: __u32,
    ) -> i32;
}

const XDP_ABORTED: i32 = 0;
const XDP_DROP: i32 = 1;
const XDP_PASS: i32 = 2;

const BPF_OK: i32 = 0;
const BPF_DROP: i32 = 2;
const BPF_REDIRECT: i32 = 7;

const BPF_MTU_CHK_RET_FRAG_NEEDED: i32 = 2;
const BPF_MTU_CHK_SEGS: __u32 = 1;
const ETH_HLEN: __u32 = 14;
const EINVAL: i32 = 22;

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

/* Userspace will update with MTU it can see on device */
#[unsafe(no_mangle)]
pub static GLOBAL_USER_MTU: i32 = 0;
#[unsafe(no_mangle)]
pub static GLOBAL_USER_IFINDEX: __u32 = 0;

/* BPF-prog will update these with MTU values it can see */
#[unsafe(no_mangle)]
pub static mut global_bpf_mtu_xdp: __u32 = 0;
#[unsafe(no_mangle)]
pub static mut global_bpf_mtu_tc: __u32 = 0;

#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xdp_use_helper_basic(ctx: *mut xdp_md) -> i32 {
    let mut mtu_len: __u32 = 0;

    if unsafe { bpf_check_mtu(ctx as *mut c_void, 0, &mut mtu_len, 0, 0) } != 0 {
        return XDP_ABORTED;
    }

    XDP_PASS
}

#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xdp_use_helper(ctx: *mut xdp_md) -> i32 {
    let mut retval: i32 = XDP_PASS; /* Expected retval on successful test */
    let mut mtu_len: __u32 = 0;
    let mut ifindex: __u32 = 0;
    let delta: i32 = 0;

    /* When ifindex is zero, save net_device lookup and use ctx netdev */
    if GLOBAL_USER_IFINDEX > 0 {
        ifindex = GLOBAL_USER_IFINDEX;
    }

    if unsafe { bpf_check_mtu(ctx as *mut c_void, ifindex, &mut mtu_len, delta, 0) } != 0 {
        /* mtu_len is also valid when check fail */
        retval = XDP_ABORTED;
    } else if mtu_len != GLOBAL_USER_MTU as __u32 {
        retval = XDP_DROP;
    }

    unsafe {
        global_bpf_mtu_xdp = mtu_len;
    }
    retval
}

#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xdp_exceed_mtu(ctx: *mut xdp_md) -> i32 {
    let data_end = unsafe { (*ctx).data_end as usize as *mut c_void };
    let data = unsafe { (*ctx).data as usize as *mut c_void };
    let ifindex: __u32 = GLOBAL_USER_IFINDEX;
    let data_len: __u32 = (data_end as usize).wrapping_sub(data as usize) as __u32;
    let mut retval: i32 = XDP_ABORTED; /* Fail */
    let mut mtu_len: __u32 = 0;
    let delta: i32;
    let err: i32;

    /* Exceed MTU with 1 via delta adjust */
    delta = GLOBAL_USER_MTU
        .wrapping_sub(data_len.wrapping_sub(ETH_HLEN) as i32)
        .wrapping_add(1);

    err = unsafe { bpf_check_mtu(ctx as *mut c_void, ifindex, &mut mtu_len, delta, 0) };
    if err != 0 {
        retval = XDP_PASS; /* Success in exceeding MTU check */
        if err != BPF_MTU_CHK_RET_FRAG_NEEDED {
            retval = XDP_DROP;
        }
    }

    unsafe {
        global_bpf_mtu_xdp = mtu_len;
    }
    retval
}

#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xdp_minus_delta(ctx: *mut xdp_md) -> i32 {
    let mut retval: i32 = XDP_PASS; /* Expected retval on successful test */
    let data_end = unsafe { (*ctx).data_end as usize as *mut c_void };
    let data = unsafe { (*ctx).data as usize as *mut c_void };
    let ifindex: __u32 = GLOBAL_USER_IFINDEX;
    let data_len: __u32 = (data_end as usize).wrapping_sub(data as usize) as __u32;
    let mut mtu_len: __u32 = 0;
    let delta: i32;

    /* Borderline test case: Minus delta exceeding packet length allowed */
    delta = -((data_len.wrapping_sub(ETH_HLEN)).wrapping_add(1) as i32);

    /* Minus length (adjusted via delta) still pass MTU check, other helpers
     * are responsible for catching this, when doing actual size adjust
     */
    if unsafe { bpf_check_mtu(ctx as *mut c_void, ifindex, &mut mtu_len, delta, 0) } != 0 {
        retval = XDP_ABORTED;
    }

    unsafe {
        global_bpf_mtu_xdp = mtu_len;
    }
    retval
}

#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xdp_input_len(ctx: *mut xdp_md) -> i32 {
    let mut retval: i32 = XDP_PASS; /* Expected retval on successful test */
    let data_end = unsafe { (*ctx).data_end as usize as *mut c_void };
    let data = unsafe { (*ctx).data as usize as *mut c_void };
    let ifindex: __u32 = GLOBAL_USER_IFINDEX;
    let data_len: __u32 = (data_end as usize).wrapping_sub(data as usize) as __u32;

    /* API allow user give length to check as input via mtu_len param,
     * resulting MTU value is still output in mtu_len param after call.
     *
     * Input len is L3, like MTU and iph->tot_len.
     * Remember XDP data_len is L2.
     */
    let mut mtu_len: __u32 = data_len.wrapping_sub(ETH_HLEN);

    if unsafe { bpf_check_mtu(ctx as *mut c_void, ifindex, &mut mtu_len, 0, 0) } != 0 {
        retval = XDP_ABORTED;
    }

    unsafe {
        global_bpf_mtu_xdp = mtu_len;
    }
    retval
}

#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xdp_input_len_exceed(ctx: *mut xdp_md) -> i32 {
    let mut retval: i32 = XDP_ABORTED; /* Fail */
    let ifindex: __u32 = GLOBAL_USER_IFINDEX;
    let err: i32;

    /* API allow user give length to check as input via mtu_len param,
     * resulting MTU value is still output in mtu_len param after call.
     *
     * Input length value is L3 size like MTU.
     */
    let mut mtu_len: __u32 = GLOBAL_USER_MTU as __u32;

    mtu_len = mtu_len.wrapping_add(1); /* Exceed with 1 */

    err = unsafe { bpf_check_mtu(ctx as *mut c_void, ifindex, &mut mtu_len, 0, 0) };
    if err == BPF_MTU_CHK_RET_FRAG_NEEDED {
        retval = XDP_PASS; /* Success in exceeding MTU check */
    }

    unsafe {
        global_bpf_mtu_xdp = mtu_len;
    }
    retval
}

#[unsafe(link_section = "tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tc_use_helper(ctx: *mut __sk_buff) -> i32 {
    let mut retval: i32 = BPF_OK; /* Expected retval on successful test */
    let mut mtu_len: __u32 = 0;
    let delta: i32 = 0;

    if unsafe { bpf_check_mtu(ctx as *mut c_void, 0, &mut mtu_len, delta, 0) } != 0 {
        retval = BPF_DROP;
    } else if mtu_len != GLOBAL_USER_MTU as __u32 {
        retval = BPF_REDIRECT;
    }

    unsafe {
        global_bpf_mtu_tc = mtu_len;
    }
    retval
}

#[unsafe(link_section = "tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tc_exceed_mtu(ctx: *mut __sk_buff) -> i32 {
    let ifindex: __u32 = GLOBAL_USER_IFINDEX;
    let mut retval: i32 = BPF_DROP; /* Fail */
    let skb_len: __u32 = unsafe { (*ctx).len };
    let mut mtu_len: __u32 = 0;
    let delta: i32;
    let err: i32;

    /* Exceed MTU with 1 via delta adjust */
    delta = GLOBAL_USER_MTU
        .wrapping_sub(skb_len.wrapping_sub(ETH_HLEN) as i32)
        .wrapping_add(1);

    err = unsafe { bpf_check_mtu(ctx as *mut c_void, ifindex, &mut mtu_len, delta, 0) };
    if err != 0 {
        retval = BPF_OK; /* Success in exceeding MTU check */
        if err != BPF_MTU_CHK_RET_FRAG_NEEDED {
            retval = BPF_DROP;
        }
    }

    unsafe {
        global_bpf_mtu_tc = mtu_len;
    }
    retval
}

#[unsafe(link_section = "tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tc_exceed_mtu_da(ctx: *mut __sk_buff) -> i32 {
    /* SKB Direct-Access variant */
    let data_end = unsafe { (*ctx).data_end as usize as *mut c_void };
    let data = unsafe { (*ctx).data as usize as *mut c_void };
    let ifindex: __u32 = GLOBAL_USER_IFINDEX;
    let data_len: __u32 = (data_end as usize).wrapping_sub(data as usize) as __u32;
    let mut retval: i32 = BPF_DROP; /* Fail */
    let mut mtu_len: __u32 = 0;
    let delta: i32;
    let err: i32;

    /* Exceed MTU with 1 via delta adjust */
    delta = GLOBAL_USER_MTU
        .wrapping_sub(data_len.wrapping_sub(ETH_HLEN) as i32)
        .wrapping_add(1);

    err = unsafe { bpf_check_mtu(ctx as *mut c_void, ifindex, &mut mtu_len, delta, 0) };
    if err != 0 {
        retval = BPF_OK; /* Success in exceeding MTU check */
        if err != BPF_MTU_CHK_RET_FRAG_NEEDED {
            retval = BPF_DROP;
        }
    }

    unsafe {
        global_bpf_mtu_tc = mtu_len;
    }
    retval
}

#[unsafe(link_section = "tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tc_minus_delta(ctx: *mut __sk_buff) -> i32 {
    let mut retval: i32 = BPF_OK; /* Expected retval on successful test */
    let ifindex: __u32 = GLOBAL_USER_IFINDEX;
    let skb_len: __u32 = unsafe { (*ctx).len };
    let mut mtu_len: __u32 = 0;
    let delta: i32;

    /* Borderline test case: Minus delta exceeding packet length allowed */
    delta = -((skb_len.wrapping_sub(ETH_HLEN)).wrapping_add(1) as i32);

    /* Minus length (adjusted via delta) still pass MTU check, other helpers
     * are responsible for catching this, when doing actual size adjust
     */
    if unsafe { bpf_check_mtu(ctx as *mut c_void, ifindex, &mut mtu_len, delta, 0) } != 0 {
        retval = BPF_DROP;
    }

    unsafe {
        global_bpf_mtu_xdp = mtu_len;
    }
    retval
}

#[unsafe(link_section = "tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tc_input_len(ctx: *mut __sk_buff) -> i32 {
    let mut retval: i32 = BPF_OK; /* Expected retval on successful test */
    let ifindex: __u32 = GLOBAL_USER_IFINDEX;

    /* API allow user give length to check as input via mtu_len param,
     * resulting MTU value is still output in mtu_len param after call.
     *
     * Input length value is L3 size.
     */
    let mut mtu_len: __u32 = GLOBAL_USER_MTU as __u32;

    if unsafe { bpf_check_mtu(ctx as *mut c_void, ifindex, &mut mtu_len, 0, 0) } != 0 {
        retval = BPF_DROP;
    }

    unsafe {
        global_bpf_mtu_xdp = mtu_len;
    }
    retval
}

#[unsafe(link_section = "tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tc_input_len_exceed(ctx: *mut __sk_buff) -> i32 {
    let mut retval: i32 = BPF_DROP; /* Fail */
    let ifindex: __u32 = GLOBAL_USER_IFINDEX;
    let err: i32;

    /* API allow user give length to check as input via mtu_len param,
     * resulting MTU value is still output in mtu_len param after call.
     *
     * Input length value is L3 size like MTU.
     */
    let mut mtu_len: __u32 = GLOBAL_USER_MTU as __u32;

    mtu_len = mtu_len.wrapping_add(1); /* Exceed with 1 */

    err = unsafe { bpf_check_mtu(ctx as *mut c_void, ifindex, &mut mtu_len, 0, 0) };
    if err == BPF_MTU_CHK_RET_FRAG_NEEDED {
        retval = BPF_OK; /* Success in exceeding MTU check */
    }

    unsafe {
        global_bpf_mtu_xdp = mtu_len;
    }
    retval
}

#[unsafe(link_section = "tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tc_chk_segs_flag(ctx: *mut __sk_buff) -> i32 {
    let mut mtu_len: __u32 = 0;
    let err: i32;

    err = unsafe {
        bpf_check_mtu(
            ctx as *mut c_void,
            GLOBAL_USER_IFINDEX,
            &mut mtu_len,
            0,
            BPF_MTU_CHK_SEGS,
        )
    };

    if err == -EINVAL {
        BPF_OK
    } else {
        BPF_DROP
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
