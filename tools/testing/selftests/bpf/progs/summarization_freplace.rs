// SPDX-License-Identifier: GPL-2.0

// Dependencies from <vmlinux.h> and <bpf/bpf_helpers.h> are expected to be
// provided by the surrounding BPF build environment.

#[repr(C)]
pub struct __sk_buff {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct pt_regs {
    _unused: [u8; 0],
}

unsafe extern "C" {
    fn bpf_skb_pull_data(sk: *mut __sk_buff, len: u32) -> i64;
    fn bpf_copy_from_user(dst: *mut core::ffi::c_void, size: u32, unsafe_ptr: *const core::ffi::c_void)
        -> i64;
}

#[unsafe(link_section = "?freplace")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn changes_pkt_data(sk: *mut __sk_buff) -> i64 {
    unsafe { bpf_skb_pull_data(sk, 0) }
}

#[unsafe(link_section = "?freplace")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn does_not_change_pkt_data(_sk: *mut __sk_buff) -> i64 {
    0
}

#[unsafe(link_section = "?freplace")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn might_sleep(_ctx: *mut pt_regs) -> i64 {
    let mut i: i32 = 0;

    unsafe {
        bpf_copy_from_user(
            (&mut i as *mut i32).cast::<core::ffi::c_void>(),
            core::mem::size_of_val(&i) as u32,
            core::ptr::null(),
        );
    }
    i as i64
}

#[unsafe(link_section = "?freplace")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn does_not_sleep(_ctx: *mut pt_regs) -> i64 {
    0
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";
