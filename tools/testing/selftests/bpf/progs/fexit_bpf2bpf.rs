// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2019 Facebook */

/*
 * C dependencies translated as external declarations:
 * <linux/stddef.h>, <linux/if_ether.h>, <linux/ipv6.h>, <linux/bpf.h>,
 * <linux/tcp.h>, <bpf/bpf_helpers.h>, <bpf/bpf_endian.h>,
 * <bpf/bpf_tracing.h>
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

pub type __u32 = u32;
pub type __u64 = u64;

const ETH_HLEN: usize = 14;
const IPPROTO_TCP: u8 = 6;

const fn __bpf_constant_htons(x: u16) -> u16 {
    x.to_be()
}

#[repr(C)]
pub struct sk_buff {
    pub len: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct __sk_buff {
    pub len: __u32,
    pub data: __u32,
    pub data_end: __u32,
    pub ifindex: __u32,
}

#[repr(C)]
pub struct ethhdr {
    pub h_dest: [u8; 6],
    pub h_source: [u8; 6],
    pub h_proto: u16,
}

#[repr(C)]
pub struct ipv6hdr {
    pub priority_version: u8,
    pub flow_lbl: [u8; 3],
    pub payload_len: u16,
    pub nexthdr: u8,
    pub hop_limit: u8,
    pub saddr: [u8; 16],
    pub daddr: [u8; 16],
}

#[repr(C)]
pub struct tcphdr {
    pub source: u16,
    pub dest: u16,
    pub seq: u32,
    pub ack_seq: u32,
    pub doff_res_flags: u16,
    pub window: u16,
    pub check: u16,
    pub urg_ptr: u16,
}

impl tcphdr {
    unsafe fn syn(&self) -> u16 {
        (self.doff_res_flags >> 1) & 1
    }

    unsafe fn set_syn(&mut self, val: u16) {
        self.doff_res_flags = (self.doff_res_flags & !(1 << 1)) | ((val & 1) << 1);
    }
}

unsafe extern "C" {
    fn bpf_probe_read_kernel(
        dst: *mut ::core::ffi::c_void,
        size: usize,
        unsafe_ptr: *const ::core::ffi::c_void,
    ) -> i64;
    fn bpf_skb_load_bytes(
        skb: *const __sk_buff,
        offset: __u32,
        to: *mut ::core::ffi::c_void,
        len: __u32,
    ) -> i64;
}

#[unsafe(no_mangle)]
pub static mut test_result: __u64 = 0;

// SEC("fexit/test_pkt_access")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_main(skb: *mut sk_buff, ret: i32) -> i32 {
    let len: i32;

    len = unsafe { (*skb).len as i32 };
    if len != 74 || ret != 0 {
        return 0;
    }
    unsafe {
        test_result = 1;
    }
    0
}

#[unsafe(no_mangle)]
pub static mut test_result_subprog1: __u64 = 0;

// SEC("fexit/test_pkt_access_subprog1")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_subprog1(skb: *mut sk_buff, ret: i32) -> i32 {
    let len: i32;

    len = unsafe { (*skb).len as i32 };
    if len != 74 || ret != 148 {
        return 0;
    }
    unsafe {
        test_result_subprog1 = 1;
    }
    0
}

/* Though test_pkt_access_subprog2() is defined in C as:
 * static __attribute__ ((noinline))
 * int test_pkt_access_subprog2(int val, volatile struct __sk_buff *skb)
 * {
 *     return skb->len * val;
 * }
 * llvm optimizations remove 'int val' argument and generate BPF assembly:
 *   r0 = *(u32 *)(r1 + 0)
 *   w0 <<= 1
 *   exit
 * Before llvm23, in such case the verifier falls back to conservative and
 * tracing program can access arguments and return value as u64
 * instead of accurate types. With llvm23, the true signature
 *   int test_pkt_access_subprog2(volatile struct __sk_buff *skb)
 * is available in btf.
 */
// C condition preserved: #if __clang_major__ >= 23
#[cfg(clang_major_ge_23)]
#[repr(C)]
pub struct args_subprog2 {
    pub args: [__u64; 1],
    pub ret: __u64,
}

// C condition preserved: #else of #if __clang_major__ >= 23
#[cfg(not(clang_major_ge_23))]
#[repr(C)]
pub struct args_subprog2 {
    pub args: [__u64; 5],
    pub ret: __u64,
}

#[unsafe(no_mangle)]
pub static mut test_result_subprog2: __u64 = 0;

// SEC("fexit/test_pkt_access_subprog2")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_subprog2(ctx: *mut args_subprog2) -> i32 {
    let skb: *mut sk_buff = unsafe { (*ctx).args[0] as *mut ::core::ffi::c_void as *mut sk_buff };
    let mut ret: __u64;
    let mut len: i32 = 0;

    unsafe {
        bpf_probe_read_kernel(
            &mut len as *mut _ as *mut ::core::ffi::c_void,
            ::core::mem::size_of_val(&len),
            &(*skb).len as *const _ as *const ::core::ffi::c_void,
        );
    }

    ret = unsafe { (*ctx).ret };
    /* bpf_prog_test_load() loads "test_pkt_access.bpf.o" with
     * BPF_F_TEST_RND_HI32 which randomizes upper 32 bits after BPF_ALU32
     * insns. Hence after 'w0 <<= 1' upper bits of $rax are random. That is
     * expected and correct. Trim them.
     */
    ret = ret as __u32 as __u64;
    if len != 74 || ret != 148 {
        return 0;
    }
    unsafe {
        test_result_subprog2 = 1;
    }
    0
}

#[unsafe(no_mangle)]
pub static mut test_result_subprog3: __u64 = 0;

// SEC("fexit/test_pkt_access_subprog3")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_subprog3(val: i32, skb: *mut sk_buff, ret: i32) -> i32 {
    let len: i32;

    len = unsafe { (*skb).len as i32 };
    if len != 74 || ret != 74 * val || val != 3 {
        return 0;
    }
    unsafe {
        test_result_subprog3 = 1;
    }
    0
}

#[unsafe(no_mangle)]
pub static mut test_get_skb_len: __u64 = 0;

// SEC("freplace/get_skb_len")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn new_get_skb_len(skb: *mut __sk_buff) -> i32 {
    let len: i32 = unsafe { (*skb).len as i32 };

    if len != 74 {
        return 0;
    }
    unsafe {
        test_get_skb_len = 1;
    }
    74 /* original get_skb_len() returns skb->len */
}

#[unsafe(no_mangle)]
pub static mut test_get_skb_ifindex: __u64 = 0;

// SEC("freplace/get_skb_ifindex")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn new_get_skb_ifindex(
    val: i32,
    skb: *mut __sk_buff,
    var: i32,
) -> i32 {
    let data_end: *mut ::core::ffi::c_void =
        unsafe { (*skb).data_end as isize as *mut ::core::ffi::c_void };
    let data: *mut ::core::ffi::c_void =
        unsafe { (*skb).data as isize as *mut ::core::ffi::c_void };
    let mut ip6: ipv6hdr = unsafe { ::core::mem::zeroed() };
    let mut ip6p: *mut ipv6hdr;
    let ifindex: i32 = unsafe { (*skb).ifindex as i32 };

    /* check that BPF extension can read packet via direct packet access */
    if unsafe { (data as *mut u8).add(ETH_HLEN + ::core::mem::size_of::<ipv6hdr>()) }
        > data_end as *mut u8
    {
        return 0;
    }
    ip6p = unsafe { (data as *mut u8).add(ETH_HLEN) as *mut ipv6hdr };

    if unsafe { (*ip6p).nexthdr } != IPPROTO_TCP
        || unsafe { (*ip6p).payload_len } != __bpf_constant_htons(123)
    {
        return 0;
    }

    /* check that legacy packet access helper works too */
    if unsafe {
        bpf_skb_load_bytes(
            skb,
            ETH_HLEN as __u32,
            &mut ip6 as *mut _ as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<ipv6hdr>() as __u32,
        )
    } < 0
    {
        return 0;
    }
    ip6p = &mut ip6;
    if unsafe { (*ip6p).nexthdr } != IPPROTO_TCP
        || unsafe { (*ip6p).payload_len } != __bpf_constant_htons(123)
    {
        return 0;
    }

    if ifindex != 1 || val != 3 || var != 1 {
        return 0;
    }
    unsafe {
        test_get_skb_ifindex = 1;
    }
    3 /* original get_skb_ifindex() returns val * ifindex * var */
}

#[unsafe(no_mangle)]
pub static mut test_get_constant: __u64 = 0;

// SEC("freplace/get_constant")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn new_get_constant(val: ::core::ffi::c_long) -> i32 {
    if val != 123 {
        return 0;
    }
    unsafe {
        ::core::ptr::write_volatile(&raw mut test_get_constant, 1);
    }
    unsafe {
        ::core::ptr::read_volatile(&raw const test_get_constant) as i32
    } /* original get_constant() returns val - 122 */
}

#[unsafe(no_mangle)]
pub static mut test_pkt_write_access_subprog: __u64 = 0;

// SEC("freplace/test_pkt_write_access_subprog")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn new_test_pkt_write_access_subprog(
    skb: *mut __sk_buff,
    off: __u32,
) -> i32 {
    let data: *mut ::core::ffi::c_void =
        unsafe { (*skb).data as isize as *mut ::core::ffi::c_void };
    let data_end: *mut ::core::ffi::c_void =
        unsafe { (*skb).data_end as isize as *mut ::core::ffi::c_void };
    let tcp: *mut tcphdr;

    if off as usize > ::core::mem::size_of::<ethhdr>() + ::core::mem::size_of::<ipv6hdr>() {
        return -1;
    }

    tcp = unsafe { (data as *mut u8).add(off as usize) as *mut tcphdr };
    if unsafe { tcp.add(1) as *mut u8 } > data_end as *mut u8 {
        return -1;
    }

    /* make modifications to the packet data */
    unsafe {
        (*tcp).check = (*tcp).check.wrapping_add(1);
        (*tcp).set_syn(0);
    }

    unsafe {
        test_pkt_write_access_subprog = 1;
    }
    0
}

// SEC("license")
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
