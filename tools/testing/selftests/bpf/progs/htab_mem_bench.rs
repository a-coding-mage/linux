// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2023. Huawei Technologies Co., Ltd */

// Dependencies from the original C source:
// <stdbool.h>, <errno.h>, <linux/types.h>, <linux/bpf.h>,
// <bpf/bpf_helpers.h>, and <bpf/bpf_tracing.h>.

const OP_BATCH: u32 = 64;
const BPF_MAP_TYPE_HASH: u32 = 1;
const BPF_F_NO_PREALLOC: u32 = 1;
const BPF_NOEXIST: u64 = 1;

#[repr(C)]
struct update_ctx {
    from: u32,
    step: u32,
}

#[repr(C)]
struct bpf_map_def {
    type_: u32,
    key_size: u32,
    map_flags: u32,
}

#[link_section = ".maps"]
#[no_mangle]
static mut htab: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_HASH,
    key_size: 4,
    map_flags: BPF_F_NO_PREALLOC,
};

#[link_section = "license"]
#[no_mangle]
static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
static mut zeroed_value: [u8; 4096] = [0; 4096];
#[no_mangle]
static mut nr_thread: u32 = 0;
#[no_mangle]
static mut op_cnt: i64 = 0;

unsafe extern "C" {
    fn bpf_map_update_elem(
        map: *mut bpf_map_def,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: u64,
    ) -> i64;
    fn bpf_map_delete_elem(map: *mut bpf_map_def, key: *const core::ffi::c_void) -> i64;
    fn bpf_get_smp_processor_id() -> u64;
    fn bpf_loop(
        nr_loops: u32,
        callback_fn: unsafe extern "C" fn(u32, *mut update_ctx) -> i32,
        callback_ctx: *mut core::ffi::c_void,
        flags: u64,
    ) -> i64;
}

unsafe fn sync_fetch_and_add_op_cnt(value: i64) -> i64 {
    let atomic = core::ptr::addr_of_mut!(op_cnt).cast::<core::sync::atomic::AtomicI64>();

    (*atomic).fetch_add(value, core::sync::atomic::Ordering::SeqCst)
}

unsafe fn write_htab(_i: u32, ctx: *mut update_ctx, flags: u32) -> i32 {
    bpf_map_update_elem(
        core::ptr::addr_of_mut!(htab),
        core::ptr::addr_of!((*ctx).from).cast::<core::ffi::c_void>(),
        core::ptr::addr_of!(zeroed_value).cast::<core::ffi::c_void>(),
        flags as u64,
    );
    (*ctx).from = (*ctx).from.wrapping_add((*ctx).step);

    0
}

unsafe extern "C" fn overwrite_htab(i: u32, ctx: *mut update_ctx) -> i32 {
    write_htab(i, ctx, 0)
}

unsafe extern "C" fn newwrite_htab(i: u32, ctx: *mut update_ctx) -> i32 {
    write_htab(i, ctx, BPF_NOEXIST as u32)
}

unsafe extern "C" fn del_htab(_i: u32, ctx: *mut update_ctx) -> i32 {
    bpf_map_delete_elem(
        core::ptr::addr_of_mut!(htab),
        core::ptr::addr_of!((*ctx).from).cast::<core::ffi::c_void>(),
    );
    (*ctx).from = (*ctx).from.wrapping_add((*ctx).step);

    0
}

#[link_section = "?tp/syscalls/sys_enter_getpgid"]
#[no_mangle]
pub unsafe extern "C" fn overwrite(ctx: *mut core::ffi::c_void) -> i32 {
    let mut update: update_ctx = core::mem::zeroed();

    update.from = bpf_get_smp_processor_id() as u32;
    update.step = nr_thread;
    bpf_loop(
        OP_BATCH,
        overwrite_htab,
        core::ptr::addr_of_mut!(update).cast::<core::ffi::c_void>(),
        0,
    );
    sync_fetch_and_add_op_cnt(1);
    let _ = ctx;
    0
}

#[link_section = "?tp/syscalls/sys_enter_getpgid"]
#[no_mangle]
pub unsafe extern "C" fn batch_add_batch_del(ctx: *mut core::ffi::c_void) -> i32 {
    let mut update: update_ctx = core::mem::zeroed();

    update.from = bpf_get_smp_processor_id() as u32;
    update.step = nr_thread;
    bpf_loop(
        OP_BATCH,
        overwrite_htab,
        core::ptr::addr_of_mut!(update).cast::<core::ffi::c_void>(),
        0,
    );

    update.from = bpf_get_smp_processor_id() as u32;
    bpf_loop(
        OP_BATCH,
        del_htab,
        core::ptr::addr_of_mut!(update).cast::<core::ffi::c_void>(),
        0,
    );

    sync_fetch_and_add_op_cnt(2);
    let _ = ctx;
    0
}

#[link_section = "?tp/syscalls/sys_enter_getpgid"]
#[no_mangle]
pub unsafe extern "C" fn add_only(ctx: *mut core::ffi::c_void) -> i32 {
    let mut update: update_ctx = core::mem::zeroed();

    update.from = (bpf_get_smp_processor_id() as u32) / 2;
    update.step = nr_thread / 2;
    bpf_loop(
        OP_BATCH,
        newwrite_htab,
        core::ptr::addr_of_mut!(update).cast::<core::ffi::c_void>(),
        0,
    );
    sync_fetch_and_add_op_cnt(1);
    let _ = ctx;
    0
}

#[link_section = "?tp/syscalls/sys_enter_getppid"]
#[no_mangle]
pub unsafe extern "C" fn del_only(ctx: *mut core::ffi::c_void) -> i32 {
    let mut update: update_ctx = core::mem::zeroed();

    update.from = (bpf_get_smp_processor_id() as u32) / 2;
    update.step = nr_thread / 2;
    bpf_loop(
        OP_BATCH,
        del_htab,
        core::ptr::addr_of_mut!(update).cast::<core::ffi::c_void>(),
        0,
    );
    sync_fetch_and_add_op_cnt(1);
    let _ = ctx;
    0
}
