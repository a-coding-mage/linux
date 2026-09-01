// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook
// C dependencies: <linux/bpf.h>, <bpf/bpf_helpers.h>, and "test_jhash.h".
// ATTR was defined in C as __attribute__((noinline)) for test_jhash.h.

extern "C" {
    fn jhash(key: *const core::ffi::c_void, length: u32, initval: u32) -> u32;
}

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn balancer_ingress(ctx: *mut __sk_buff) -> i32 {
    let data_end = (*ctx).data_end as usize as *mut core::ffi::c_void;
    let data = (*ctx).data as usize as *mut core::ffi::c_void;
    let mut ptr: *mut core::ffi::c_void;
    let nh_off: i32;
    let mut i: i32 = 0;

    nh_off = 14;

    /* pragma unroll doesn't work on large loops */

    macro_rules! C {
        () => {{
            ptr = (data as *mut u8).offset(i as isize) as *mut core::ffi::c_void;
            if (ptr as *mut u8).offset(nh_off as isize) > data_end as *mut u8 {
                return 0;
            }
            (*ctx).tc_index = jhash(
                ptr as *const core::ffi::c_void,
                nh_off as u32,
                (*ctx).cb[0].wrapping_add(i as u32),
            );
            i = i.wrapping_add(1);
        }};
    }

    macro_rules! C30 {
        () => {{
            C!(); C!(); C!(); C!(); C!(); C!(); C!(); C!(); C!(); C!();
            C!(); C!(); C!(); C!(); C!(); C!(); C!(); C!(); C!(); C!();
            C!(); C!(); C!(); C!(); C!(); C!(); C!(); C!(); C!(); C!();
        }};
    }

    C30!();
    C30!();
    C30!(); /* 90 calls */
    0
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
