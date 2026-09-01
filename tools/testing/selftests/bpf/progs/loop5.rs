// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook
// C dependencies: <linux/bpf.h>, <bpf/bpf_helpers.h>

// SEC("license")
#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [u8; 4] = *b"GPL\0";

unsafe extern "C" {
    fn barrier();
}

#[repr(C)]
pub struct __sk_buff {
    pub len: u32,
}

// SEC("socket")
#[unsafe(no_mangle)]
#[unsafe(link_section = "socket")]
pub unsafe extern "C" fn while_true(skb: *mut __sk_buff) -> i32 {
    let mut i: i32 = 0;

    loop {
        if unsafe { (*skb).len } != 0 {
            i += 3;
        } else {
            i += 7;
        }
        if i == 9 {
            break;
        }
        unsafe {
            barrier();
        }
        if i == 10 {
            break;
        }
        unsafe {
            barrier();
        }
        if i == 13 {
            break;
        }
        unsafe {
            barrier();
        }
        if i == 14 {
            break;
        }
    }
    i
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
