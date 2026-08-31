// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Isovalent */

// Dependencies from the original C includes:
// "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>, "bpf_misc.h"

#[repr(C)]
pub struct __sk_buff {
    pub ifindex: i32,
    pub mark: u32,
}

#[repr(C)]
pub struct linux_binprm {
    _private: [u8; 0],
}

pub type __u32 = u32;

pub const TCX_PASS: i32 = 0;

unsafe extern "C" {
    fn bpf_strtol(buf: *const i8, buf_len: usize, flags: u64, res: *mut i64) -> i64;
    fn bpf_check_mtu(
        ctx: *mut __sk_buff,
        ifindex: i32,
        mtu_len: *mut __u32,
        len_diff: i32,
        flags: u64,
    ) -> i64;
    fn bpf_copy_from_user(dst: *mut core::ffi::c_void, len: i32, unsafe_ptr: *const core::ffi::c_void)
        -> i64;
    fn bpf_get_prandom_u32() -> __u32;
}

#[no_mangle]
pub static foo: i64 = 42;

#[no_mangle]
pub static mut bar: i64 = 0;

#[no_mangle]
pub static mut bart: i64 = 96;

// SEC("tc/ingress")
// __description("rodata/strtol: write rejected")
// __failure __msg("write into map forbidden")
#[no_mangle]
pub unsafe extern "C" fn tcx1(skb: *mut __sk_buff) -> i32 {
    let mut buff: [i8; 3] = [b'8' as i8, b'4' as i8, b'\0' as i8];
    unsafe {
        bpf_strtol(
            buff.as_mut_ptr(),
            core::mem::size_of_val(&buff),
            0,
            &foo as *const i64 as *mut i64,
        );
    }
    TCX_PASS
}

// SEC("tc/ingress")
// __description("bss/strtol: write accepted")
// __success
#[no_mangle]
pub unsafe extern "C" fn tcx2(skb: *mut __sk_buff) -> i32 {
    let mut buff: [i8; 3] = [b'8' as i8, b'4' as i8, b'\0' as i8];
    unsafe {
        bpf_strtol(
            buff.as_mut_ptr(),
            core::mem::size_of_val(&buff),
            0,
            &raw mut bar,
        );
    }
    TCX_PASS
}

// SEC("tc/ingress")
// __description("data/strtol: write accepted")
// __success
#[no_mangle]
pub unsafe extern "C" fn tcx3(skb: *mut __sk_buff) -> i32 {
    let mut buff: [i8; 3] = [b'8' as i8, b'4' as i8, b'\0' as i8];
    unsafe {
        bpf_strtol(
            buff.as_mut_ptr(),
            core::mem::size_of_val(&buff),
            0,
            &raw mut bart,
        );
    }
    TCX_PASS
}

// SEC("tc/ingress")
// __description("rodata/mtu: write rejected")
// __failure __msg("write into map forbidden")
#[no_mangle]
pub unsafe extern "C" fn tcx4(skb: *mut __sk_buff) -> i32 {
    unsafe {
        bpf_check_mtu(
            skb,
            (*skb).ifindex,
            &foo as *const i64 as *mut __u32,
            0,
            0,
        );
    }
    TCX_PASS
}

// SEC("tc/ingress")
// __description("bss/mtu: write accepted")
// __success
#[no_mangle]
pub unsafe extern "C" fn tcx5(skb: *mut __sk_buff) -> i32 {
    unsafe {
        bpf_check_mtu(skb, (*skb).ifindex, &raw mut bar as *mut __u32, 0, 0);
    }
    TCX_PASS
}

// SEC("tc/ingress")
// __description("data/mtu: write accepted")
// __success
#[no_mangle]
pub unsafe extern "C" fn tcx6(skb: *mut __sk_buff) -> i32 {
    unsafe {
        bpf_check_mtu(skb, (*skb).ifindex, &raw mut bart as *mut __u32, 0, 0);
    }
    TCX_PASS
}

#[inline]
unsafe fn write_fixed(p: *mut core::ffi::c_void, val: __u32) {
    unsafe {
        core::ptr::write_volatile(p as *mut __u32, val);
    }
}

#[inline]
unsafe fn write_dyn(p: *mut core::ffi::c_void, val: *mut core::ffi::c_void, len: i32) {
    unsafe {
        bpf_copy_from_user(p, len, val);
    }
}

// SEC("tc/ingress")
// __description("rodata/mark: write with unknown reg rejected")
// __failure __msg("write into map forbidden")
#[no_mangle]
pub unsafe extern "C" fn tcx7(skb: *mut __sk_buff) -> i32 {
    unsafe {
        write_fixed(&foo as *const i64 as *mut core::ffi::c_void, (*skb).mark);
    }
    TCX_PASS
}

// SEC("lsm.s/bprm_committed_creds")
// __description("rodata/mark: write with unknown reg rejected")
// __failure __msg("write into map forbidden")
// Original C used BPF_PROG(bprm, struct linux_binprm *bprm).
#[no_mangle]
pub unsafe extern "C" fn bprm(bprm: *mut linux_binprm) -> i32 {
    unsafe {
        write_dyn(
            &foo as *const i64 as *mut core::ffi::c_void,
            &raw mut bart as *mut core::ffi::c_void,
            (bpf_get_prandom_u32() & 3) as i32,
        );
    }
    0
}

// SEC("license")
#[no_mangle]
pub static mut LICENSE: [u8; 4] = *b"GPL\0";
