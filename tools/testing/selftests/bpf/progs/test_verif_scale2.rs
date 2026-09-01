// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook
// C dependencies: "vmlinux.h", <bpf/bpf_helpers.h>
// C defines ATTR as __always_inline before including "test_jhash.h".

extern "C" {
    fn jhash(ptr: *mut core::ffi::c_void, length: i32, initval: u32) -> i32;
}

#[repr(C)]
pub struct __sk_buff {
    pub data_end: u32,
    pub data: u32,
    pub tc_index: i32,
    pub cb: [u32; 5],
}

macro_rules! SEC {
    ($name:expr) => {};
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn balancer_ingress(ctx: *mut __sk_buff) -> i32 {
    let data_end: *mut core::ffi::c_void = (*ctx).data_end as i64 as *mut core::ffi::c_void;
    let data: *mut core::ffi::c_void = (*ctx).data as i64 as *mut core::ffi::c_void;
    let mut ptr: *mut core::ffi::c_void;
    let mut nh_off: i32;
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
                ptr,
                nh_off,
                (*ctx).cb[0usize].wrapping_add({
                    let old = i;
                    i = i.wrapping_add(1);
                    old as u32
                }),
            );
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
    return 0;
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
