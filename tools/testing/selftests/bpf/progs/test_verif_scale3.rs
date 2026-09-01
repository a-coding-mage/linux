// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook
// C dependencies: <linux/bpf.h>, <bpf/bpf_helpers.h>, "test_jhash.h"

use core::ffi::c_void;

// Original C: #define ATTR __attribute__((noinline))
macro_rules! ATTR {
    ($item:item) => {
        #[inline(never)]
        $item
    };
}

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
}

extern "C" {
    fn jhash(key: *const c_void, length: u32, initval: u32) -> u32;
}

// Original C section attribute: SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn balancer_ingress(ctx: *mut __sk_buff) -> i32 {
    let data_end: *mut c_void = (*ctx).data_end as usize as *mut c_void;
    let data: *mut c_void = (*ctx).data as usize as *mut c_void;
    let mut ptr: *mut c_void;
    let nh_off: i32;
    let mut i: i32 = 0;

    nh_off = 32;

    /* pragma unroll doesn't work on large loops */

    macro_rules! C {
        () => {{
            ptr = (data as *mut u8).offset(i as isize) as *mut c_void;
            if (ptr as *mut u8).offset(nh_off as isize) > data_end as *mut u8 {
                return 0;
            }
            (*ctx).tc_index = jhash(
                ptr as *const c_void,
                nh_off as u32,
                (*ctx).cb[0].wrapping_add(i as u32),
            );
            i += 1;
        }};
    }
    macro_rules! C30 {
        () => {
            C!();
            C!();
            C!();
            C!();
            C!();
            C!();
            C!();
            C!();
            C!();
            C!();
            C!();
            C!();
            C!();
            C!();
            C!();
            C!();
            C!();
            C!();
            C!();
            C!();
            C!();
            C!();
            C!();
            C!();
            C!();
            C!();
            C!();
            C!();
            C!();
            C!();
        };
    }
    C30!();
    C30!();
    C30!(); /* 90 calls */
    return 0;
}

// Original C section attribute: SEC("license")
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
