// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

// C dependencies translated as external Rust dependencies:
// "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>,
// <bpf/bpf_core_read.h>, and "bpf_kfuncs.h".

type u64 = u64;

const IFNAMSIZ: usize = 16;
const XDP_PASS: i32 = 2;

// Original BPF map declaration:
// struct {
//     __uint(type, BPF_MAP_TYPE_TASK_STORAGE);
//     __uint(map_flags, BPF_F_NO_PREALLOC);
//     __type(key, int);
//     __type(value, long);
// } enter_id SEC(".maps");
#[link_section = ".maps"]
#[no_mangle]
pub static mut enter_id: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_TASK_STORAGE,
    map_flags: BPF_F_NO_PREALLOC,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<core::ffi::c_long>() as u32,
    max_entries: 0,
};

#[no_mangle]
pub static mut ifindex: i32 = 0;
#[no_mangle]
pub static mut ingress_ifindex: i32 = 0;
#[no_mangle]
pub static mut name: [core::ffi::c_char; IFNAMSIZ] = [0; IFNAMSIZ];
#[no_mangle]
pub static mut inum: u32 = 0;
#[no_mangle]
pub static mut meta_len: u32 = 0;
#[no_mangle]
pub static mut frag0_len: u32 = 0;
#[no_mangle]
pub static mut kskb_len: u32 = 0;
#[no_mangle]
pub static mut kskb2_len: u32 = 0;

extern "C" {
    fn bpf_get_current_task_btf() -> *mut task_struct;
    fn bpf_task_storage_get(
        map: *mut bpf_map_def,
        task: *mut task_struct,
        value: *mut core::ffi::c_void,
        flags: u64,
    ) -> *mut core::ffi::c_void;
}

#[link_section = "?xdp"]
#[no_mangle]
pub unsafe extern "C" fn md_xdp(ctx: *mut xdp_md) -> i32 {
    let kctx: *mut xdp_buff = bpf_cast_to_kern_ctx(ctx);
    let dev: *mut net_device;

    dev = (*(*kctx).rxq).dev;
    ifindex = (*dev).ifindex;
    inum = (*(*(*dev).nd_net.net).ns).inum;
    core::ptr::copy_nonoverlapping((*dev).name.as_ptr(), name.as_mut_ptr(), IFNAMSIZ);
    ingress_ifindex = (*ctx).ingress_ifindex;
    XDP_PASS
}

#[link_section = "?tc"]
#[no_mangle]
pub unsafe extern "C" fn md_skb(skb: *mut __sk_buff) -> i32 {
    let kskb: *mut sk_buff = bpf_cast_to_kern_ctx(skb);
    let mut shared_info: *mut skb_shared_info;
    let mut kskb2: *mut sk_buff;

    kskb_len = (*kskb).len;

    /* Simulate the following kernel macro:
     *   #define skb_shinfo(SKB) ((struct skb_shared_info *)(skb_end_pointer(SKB)))
     */
    shared_info = ((*kskb).head.add((*kskb).end as usize)) as *mut skb_shared_info;
    meta_len = (*shared_info).meta_len;
    frag0_len = (*(*shared_info).frag_list).len;

    /* kskb2 should be equal to kskb */
    kskb2 = kskb as *mut sk_buff;
    kskb2_len = (*kskb2).len;
    0
}

#[link_section = "?tp_btf/sys_enter"]
#[no_mangle]
pub unsafe extern "C" fn untrusted_ptr(regs: *mut pt_regs, id: core::ffi::c_long) -> i32 {
    let mut task: *mut task_struct;
    let mut task_dup: *mut task_struct;

    let _ = regs;
    let _ = id;

    task = bpf_get_current_task_btf();
    task_dup = task as *mut task_struct;
    let _ = bpf_task_storage_get(&mut enter_id, task_dup, core::ptr::null_mut(), 0);
    0
}

#[link_section = "?tracepoint/syscalls/sys_enter_nanosleep"]
#[no_mangle]
pub unsafe extern "C" fn kctx_u64(ctx: *mut core::ffi::c_void) -> i32 {
    let kctx: *mut u64 = ctx as *mut u64;

    let _ = kctx;
    0
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [core::ffi::c_char; 4] = [
    b'G' as core::ffi::c_char,
    b'P' as core::ffi::c_char,
    b'L' as core::ffi::c_char,
    0,
];
