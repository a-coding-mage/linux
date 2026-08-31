// SPDX-License-Identifier: GPL-2.0-only

/*
 * Copyright 2020 Google LLC.
 */

// C dependencies removed from executable Rust:
// <errno.h>, <linux/bpf.h>, <linux/if_ether.h>, <linux/ip.h>,
// and <bpf/bpf_helpers.h> provide the external constants, types, map
// declaration helpers, section annotations, and BPF helper declarations used
// below.

extern "C" {
    static mut test_result: core::ffi::c_void;

    fn bpf_skb_load_bytes_relative(
        skb: *mut __sk_buff,
        offset: u32,
        to: *mut core::ffi::c_void,
        len: u32,
        start_header: u32,
    ) -> i64;

    fn bpf_map_update_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: u64,
    ) -> i64;
}

// Map definition translated from:
// struct {
//     __uint(type, BPF_MAP_TYPE_ARRAY);
//     __uint(max_entries, 1);
//     __type(key, __u32);
//     __type(value, __u32);
// } test_result SEC(".maps");
//
// The concrete BPF map declaration macros and SEC(".maps") section placement
// are supplied by the surrounding eBPF build environment.

extern "C" {
    type __sk_buff;
    type ethhdr;
    type iphdr;
}

extern "C" {
    static EFAULT: i32;
    static BPF_HDR_START_MAC: u32;
    static BPF_HDR_START_NET: u32;
    static BPF_ANY: u64;
}

#[no_mangle]
#[link_section = "cgroup_skb/egress"]
pub unsafe extern "C" fn load_bytes_relative(skb: *mut __sk_buff) -> i32 {
    let mut eth: ethhdr = core::mem::zeroed();
    let mut iph: iphdr = core::mem::zeroed();

    let map_key: u32 = 0;
    let mut test_passed: u32 = 0;

    /* MAC header is not set by the time cgroup_skb/egress triggers */
    if bpf_skb_load_bytes_relative(
        skb,
        0,
        &mut eth as *mut ethhdr as *mut core::ffi::c_void,
        core::mem::size_of::<ethhdr>() as u32,
        BPF_HDR_START_MAC,
    ) != -(EFAULT as i64)
    {
        goto_fail(&map_key, &test_passed);
        return 1;
    }

    if bpf_skb_load_bytes_relative(
        skb,
        0,
        &mut iph as *mut iphdr as *mut core::ffi::c_void,
        core::mem::size_of::<iphdr>() as u32,
        BPF_HDR_START_NET,
    ) != 0
    {
        goto_fail(&map_key, &test_passed);
        return 1;
    }

    if bpf_skb_load_bytes_relative(
        skb,
        0xffff,
        &mut iph as *mut iphdr as *mut core::ffi::c_void,
        core::mem::size_of::<iphdr>() as u32,
        BPF_HDR_START_NET,
    ) != -(EFAULT as i64)
    {
        goto_fail(&map_key, &test_passed);
        return 1;
    }

    test_passed = 1;

    goto_fail(&map_key, &test_passed);

    1
}

unsafe fn goto_fail(map_key: &u32, test_passed: &u32) {
    bpf_map_update_elem(
        &mut test_result as *mut core::ffi::c_void,
        map_key as *const u32 as *const core::ffi::c_void,
        test_passed as *const u32 as *const core::ffi::c_void,
        BPF_ANY,
    );
}
