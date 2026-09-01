// SPDX-License-Identifier: GPL-2.0-only
// C dependencies: <vmlinux.h>, <bpf/bpf_helpers.h>, "bpf_misc.h"

#[repr(C)]
pub struct __sk_buff {
    _unused: [u8; 0],
}

unsafe extern "C" {
    fn barrier_var(p: *mut i32);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub unsafe extern "C" fn foo(p: *mut i32) -> i32 {
    unsafe {
        barrier_var(p);
        if !p.is_null() {
            *p = 42;
            *p
        } else {
            0
        }
    }
}

#[unsafe(no_mangle)]
pub static i: i32 = 0;

// SEC("tc")
// __failure __msg("Caller passes invalid args into func#1")
#[unsafe(no_mangle)]
#[unsafe(link_section = "tc")]
pub unsafe extern "C" fn global_func17(skb: *mut __sk_buff) -> i32 {
    unsafe { foo((&raw const i) as *mut i32) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
