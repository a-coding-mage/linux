// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */
// C dependencies: <vmlinux.h>, <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>,
// and <bpf/usdt.bpf.h>.

pub type __u64 = u64;

const BPF_F_USER_STACK: __u64 = 256;

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut entry_stack1: [__u64; 32] = [0; 32];
#[no_mangle]
pub static mut exit_stack1: [__u64; 32] = [0; 32];
#[no_mangle]
pub static mut entry_stack1_recur: [__u64; 32] = [0; 32];
#[no_mangle]
pub static mut exit_stack1_recur: [__u64; 32] = [0; 32];
#[no_mangle]
pub static mut entry_stack2: [__u64; 32] = [0; 32];
#[no_mangle]
pub static mut entry_stack3: [__u64; 32] = [0; 32];
#[no_mangle]
pub static mut entry_stack4: [__u64; 32] = [0; 32];
#[no_mangle]
pub static mut exit_stack4: [__u64; 32] = [0; 32];
#[no_mangle]
pub static mut usdt_stack: [__u64; 32] = [0; 32];

#[no_mangle]
pub static mut entry1_len: i32 = 0;
#[no_mangle]
pub static mut exit1_len: i32 = 0;
#[no_mangle]
pub static mut entry1_recur_len: i32 = 0;
#[no_mangle]
pub static mut exit1_recur_len: i32 = 0;
#[no_mangle]
pub static mut entry2_len: i32 = 0;
#[no_mangle]
pub static mut exit2_len: i32 = 0;
#[no_mangle]
pub static mut entry3_len: i32 = 0;
#[no_mangle]
pub static mut exit3_len: i32 = 0;
#[no_mangle]
pub static mut entry4_len: i32 = 0;
#[no_mangle]
pub static mut exit4_len: i32 = 0;
#[no_mangle]
pub static mut usdt_len: i32 = 0;

const SZ: usize = core::mem::size_of::<[__u64; 32]>();

extern "C" {
    fn bpf_get_stack(ctx: *mut core::ffi::c_void, buf: *mut core::ffi::c_void, size: __u64, flags: __u64) -> i64;
}

#[no_mangle]
static mut uprobe_1_recur: bool = false;

#[no_mangle]
#[link_section = "uprobe//proc/self/exe:target_1"]
pub unsafe extern "C" fn uprobe_1(ctx: *mut core::ffi::c_void) -> i32 {
    /* target_1 is recursive with depth of 2, so we capture two separate
     * stack traces, depending on which occurrence it is
     */
    if !uprobe_1_recur {
        entry1_len = bpf_get_stack(
            ctx,
            &mut entry_stack1 as *mut _ as *mut core::ffi::c_void,
            SZ as __u64,
            BPF_F_USER_STACK,
        ) as i32;
    } else {
        entry1_recur_len = bpf_get_stack(
            ctx,
            &mut entry_stack1_recur as *mut _ as *mut core::ffi::c_void,
            SZ as __u64,
            BPF_F_USER_STACK,
        ) as i32;
    }

    uprobe_1_recur = true;
    0
}

#[no_mangle]
static mut uretprobe_1_recur: bool = false;

#[no_mangle]
#[link_section = "uretprobe//proc/self/exe:target_1"]
pub unsafe extern "C" fn uretprobe_1(ctx: *mut core::ffi::c_void) -> i32 {
    /* see above, target_1 is recursive */

    /* NOTE: order of returns is reversed to order of entries */
    if !uretprobe_1_recur {
        exit1_recur_len = bpf_get_stack(
            ctx,
            &mut exit_stack1_recur as *mut _ as *mut core::ffi::c_void,
            SZ as __u64,
            BPF_F_USER_STACK,
        ) as i32;
    } else {
        exit1_len = bpf_get_stack(
            ctx,
            &mut exit_stack1 as *mut _ as *mut core::ffi::c_void,
            SZ as __u64,
            BPF_F_USER_STACK,
        ) as i32;
    }

    uretprobe_1_recur = true;
    0
}

#[no_mangle]
#[link_section = "uprobe//proc/self/exe:target_2"]
pub unsafe extern "C" fn uprobe_2(ctx: *mut core::ffi::c_void) -> i32 {
    entry2_len = bpf_get_stack(
        ctx,
        &mut entry_stack2 as *mut _ as *mut core::ffi::c_void,
        SZ as __u64,
        BPF_F_USER_STACK,
    ) as i32;
    0
}

/* no uretprobe for target_2 */

#[no_mangle]
#[link_section = "uprobe//proc/self/exe:target_3"]
pub unsafe extern "C" fn uprobe_3(ctx: *mut core::ffi::c_void) -> i32 {
    entry3_len = bpf_get_stack(
        ctx,
        &mut entry_stack3 as *mut _ as *mut core::ffi::c_void,
        SZ as __u64,
        BPF_F_USER_STACK,
    ) as i32;
    0
}

/* no uretprobe for target_3 */

#[no_mangle]
#[link_section = "uprobe//proc/self/exe:target_4"]
pub unsafe extern "C" fn uprobe_4(ctx: *mut core::ffi::c_void) -> i32 {
    entry4_len = bpf_get_stack(
        ctx,
        &mut entry_stack4 as *mut _ as *mut core::ffi::c_void,
        SZ as __u64,
        BPF_F_USER_STACK,
    ) as i32;
    0
}

#[no_mangle]
#[link_section = "uretprobe//proc/self/exe:target_4"]
pub unsafe extern "C" fn uretprobe_4(ctx: *mut core::ffi::c_void) -> i32 {
    exit4_len = bpf_get_stack(
        ctx,
        &mut exit_stack4 as *mut _ as *mut core::ffi::c_void,
        SZ as __u64,
        BPF_F_USER_STACK,
    ) as i32;
    0
}

#[no_mangle]
#[link_section = "usdt//proc/self/exe:uretprobe_stack:target"]
pub unsafe extern "C" fn usdt_probe(ctx: *mut core::ffi::c_void) -> i32 {
    usdt_len = bpf_get_stack(
        ctx,
        &mut usdt_stack as *mut _ as *mut core::ffi::c_void,
        SZ as __u64,
        BPF_F_USER_STACK,
    ) as i32;
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
