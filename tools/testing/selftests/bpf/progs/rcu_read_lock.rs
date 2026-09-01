// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

/* Dependencies from the original C source:
 * vmlinux.h, bpf/bpf_helpers.h, bpf/bpf_tracing.h, bpf_tracing_net.h,
 * and bpf_misc.h.
 */

/* clang considers 'sum += 1' as usage but 'sum++' as non-usage.  GCC
 * is more consistent and considers both 'sum += 1' and 'sum++' as
 * non-usage.  This triggers warnings in the functions below.
 *
 * Starting with GCC 16 -Wunused-but-set-variable=2 can be used to
 * mimic clang's behavior.
 *
 * Original C condition:
 * #if !defined(__clang__) && __GNUC__ > 15
 * #pragma GCC diagnostic ignored "-Wunused-but-set-variable"
 * #endif
 */

pub type __u32 = u32;
pub type __s32 = i32;
pub type __u64 = u64;
pub type u64 = u64;

pub const BPF_MAP_TYPE_TASK_STORAGE: u32 = 0;
pub const BPF_MAP_TYPE_ARRAY: u32 = 0;
pub const BPF_F_NO_PREALLOC: u32 = 0;
pub const BPF_LOCAL_STORAGE_GET_F_CREATE: u64 = 0;

#[repr(C)]
pub struct bpf_key {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_attr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_rb_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kernfs_node {
    pub id: __u64,
}

#[repr(C)]
pub struct cgroup {
    pub kn: *mut kernfs_node,
}

#[repr(C)]
pub struct css_set {
    pub dfl_cgrp: *mut cgroup,
}

#[repr(C)]
pub struct task_struct {
    pub pid: __u32,
    pub cgroups: *mut css_set,
    pub real_parent: *mut task_struct,
    pub group_leader: *mut task_struct,
}

#[repr(C)]
pub struct sock {
    pub sk_wq: *mut socket_wq,
}

#[repr(C)]
pub struct socket_wq {
    pub flags: i64,
}

#[repr(C)]
pub struct MapA {
    pub type_: u32,
    pub map_flags: u32,
}

#[repr(C)]
pub struct NodeStashMap {
    pub type_: u32,
    pub max_entries: u32,
}

/* SEC("license") */
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

/* SEC(".maps") */
#[no_mangle]
pub static mut map_a: MapA = MapA {
    type_: BPF_MAP_TYPE_TASK_STORAGE,
    map_flags: BPF_F_NO_PREALLOC,
};

#[no_mangle]
pub static mut user_data: __u32 = 0;
#[no_mangle]
pub static mut target_pid: __u32 = 0;
#[no_mangle]
pub static mut key_serial: __s32 = 0;
#[no_mangle]
pub static mut flags: __u64 = 0;
#[no_mangle]
pub static mut task_storage_val: __u64 = 0;
#[no_mangle]
pub static mut cgroup_id: __u64 = 0;

extern "C" {
    pub fn bpf_lookup_user_key(serial: __s32, flags: __u64) -> *mut bpf_key;
    pub fn bpf_key_put(key: *mut bpf_key);
    pub fn bpf_rcu_read_lock();
    pub fn bpf_rcu_read_unlock();
    pub fn bpf_task_acquire(p: *mut task_struct) -> *mut task_struct;
    pub fn bpf_task_release(p: *mut task_struct);
    pub fn bpf_get_current_task_btf() -> *mut task_struct;
    pub fn bpf_task_storage_get(
        map: *mut MapA,
        task: *mut task_struct,
        value: *const i64,
        flags: u64,
    ) -> *mut i64;
    pub fn bpf_task_pt_regs(task: *mut task_struct) -> *mut pt_regs;
    pub fn PT_REGS_IP(regs: *mut pt_regs) -> usize;
    pub fn bpf_copy_from_user_task(
        dst: *mut core::ffi::c_void,
        size: usize,
        unsafe_ptr: *const core::ffi::c_void,
        task: *mut task_struct,
        flags: u64,
    ) -> i64;
    pub fn bpf_get_prandom_u32() -> u32;
    pub fn bpf_copy_from_user(
        dst: *mut core::ffi::c_void,
        size: usize,
        unsafe_ptr: *const core::ffi::c_void,
    ) -> i64;
    pub fn bpf_copy_from_user_str(
        dst: *mut core::ffi::c_void,
        size: usize,
        unsafe_ptr: *const core::ffi::c_void,
        flags: u64,
    ) -> i64;
    pub fn bpf_map_lookup_elem(map: *mut NodeStashMap, key: *const i32) -> *mut core::ffi::c_void;
}

/* SEC("?fentry.s/" SYS_PREFIX "sys_getpgid") */
#[no_mangle]
pub unsafe extern "C" fn get_cgroup_id(ctx: *mut core::ffi::c_void) -> i32 {
    let task: *mut task_struct;
    let cgroups: *mut css_set;

    task = bpf_get_current_task_btf();
    if (*task).pid != target_pid {
        return 0;
    }

    /* simulate bpf_get_current_cgroup_id() helper */
    bpf_rcu_read_lock();
    cgroups = (*task).cgroups;
    if cgroups.is_null() {
        bpf_rcu_read_unlock();
        return 0;
    }
    cgroup_id = (*(*(*cgroups).dfl_cgrp).kn).id;
    bpf_rcu_read_unlock();
    0
}

/* SEC("?fentry.s/" SYS_PREFIX "sys_getpgid") */
#[no_mangle]
pub unsafe extern "C" fn task_succ(ctx: *mut core::ffi::c_void) -> i32 {
    let task: *mut task_struct;
    let real_parent: *mut task_struct;
    let init_val: i64 = 2;
    let mut ptr: *mut i64;

    task = bpf_get_current_task_btf();
    if (*task).pid != target_pid {
        return 0;
    }

    bpf_rcu_read_lock();
    /* region including helper using rcu ptr real_parent */
    real_parent = (*task).real_parent;
    if !real_parent.is_null() {
        ptr = bpf_task_storage_get(
            &mut map_a,
            real_parent,
            &init_val,
            BPF_LOCAL_STORAGE_GET_F_CREATE,
        );
        if !ptr.is_null() {
            ptr = bpf_task_storage_get(&mut map_a, real_parent, core::ptr::null(), 0);
            if !ptr.is_null() {
                task_storage_val = *ptr as __u64;
            }
        }
    }
    bpf_rcu_read_unlock();
    0
}

/* SEC("?fentry.s/" SYS_PREFIX "sys_nanosleep") */
#[no_mangle]
pub unsafe extern "C" fn no_lock(ctx: *mut core::ffi::c_void) -> i32 {
    let task: *mut task_struct;
    let real_parent: *mut task_struct;

    /* old style ptr_to_btf_id is not allowed in sleepable */
    task = bpf_get_current_task_btf();
    real_parent = (*task).real_parent;
    let _ = bpf_task_storage_get(&mut map_a, real_parent, core::ptr::null(), 0);
    0
}

/* SEC("?fentry.s/" SYS_PREFIX "sys_nanosleep") */
#[no_mangle]
pub unsafe extern "C" fn two_regions(ctx: *mut core::ffi::c_void) -> i32 {
    let task: *mut task_struct;
    let real_parent: *mut task_struct;

    /* two regions */
    task = bpf_get_current_task_btf();
    bpf_rcu_read_lock();
    bpf_rcu_read_unlock();
    bpf_rcu_read_lock();
    real_parent = (*task).real_parent;
    if !real_parent.is_null() {
        let _ = bpf_task_storage_get(&mut map_a, real_parent, core::ptr::null(), 0);
    }
    bpf_rcu_read_unlock();
    0
}

/* SEC("?fentry/" SYS_PREFIX "sys_getpgid") */
#[no_mangle]
pub unsafe extern "C" fn non_sleepable_1(ctx: *mut core::ffi::c_void) -> i32 {
    let task: *mut task_struct;
    let real_parent: *mut task_struct;

    task = bpf_get_current_task_btf();
    bpf_rcu_read_lock();
    real_parent = (*task).real_parent;
    if !real_parent.is_null() {
        let _ = bpf_task_storage_get(&mut map_a, real_parent, core::ptr::null(), 0);
    }
    bpf_rcu_read_unlock();
    0
}

/* SEC("?fentry/" SYS_PREFIX "sys_getpgid") */
#[no_mangle]
pub unsafe extern "C" fn non_sleepable_2(ctx: *mut core::ffi::c_void) -> i32 {
    let task: *mut task_struct;
    let real_parent: *mut task_struct;

    bpf_rcu_read_lock();
    task = bpf_get_current_task_btf();
    bpf_rcu_read_unlock();

    bpf_rcu_read_lock();
    real_parent = (*task).real_parent;
    if !real_parent.is_null() {
        let _ = bpf_task_storage_get(&mut map_a, real_parent, core::ptr::null(), 0);
    }
    bpf_rcu_read_unlock();
    0
}

/* SEC("?fentry.s/" SYS_PREFIX "sys_nanosleep") */
#[no_mangle]
pub unsafe extern "C" fn task_acquire(ctx: *mut core::ffi::c_void) -> i32 {
    let task: *mut task_struct;
    let real_parent: *mut task_struct;
    let mut gparent: *mut task_struct;

    task = bpf_get_current_task_btf();
    bpf_rcu_read_lock();
    real_parent = (*task).real_parent;
    if !real_parent.is_null() {
        /* rcu_ptr->rcu_field */
        gparent = (*real_parent).real_parent;
        if !gparent.is_null() {
            /* acquire a reference which can be used outside rcu read lock region */
            gparent = bpf_task_acquire(gparent);
            if !gparent.is_null() {
                let _ = bpf_task_storage_get(&mut map_a, gparent, core::ptr::null(), 0);
                bpf_task_release(gparent);
            }
        }
    }
    bpf_rcu_read_unlock();
    0
}

/* SEC("?fentry.s/" SYS_PREFIX "sys_getpgid") */
#[no_mangle]
pub unsafe extern "C" fn miss_lock(ctx: *mut core::ffi::c_void) -> i32 {
    let task: *mut task_struct;

    /* missing bpf_rcu_read_lock() */
    task = bpf_get_current_task_btf();
    bpf_rcu_read_lock();
    let _ = bpf_task_storage_get(&mut map_a, task, core::ptr::null(), 0);
    bpf_rcu_read_unlock();
    bpf_rcu_read_unlock();
    0
}

/* SEC("?fentry.s/" SYS_PREFIX "sys_getpgid") */
#[no_mangle]
pub unsafe extern "C" fn miss_unlock(ctx: *mut core::ffi::c_void) -> i32 {
    let task: *mut task_struct;

    /* missing bpf_rcu_read_unlock() */
    task = bpf_get_current_task_btf();
    bpf_rcu_read_lock();
    let _ = bpf_task_storage_get(&mut map_a, task, core::ptr::null(), 0);
    0
}

/* SEC("?fentry/" SYS_PREFIX "sys_getpgid") */
#[no_mangle]
pub unsafe extern "C" fn non_sleepable_rcu_mismatch(ctx: *mut core::ffi::c_void) -> i32 {
    let task: *mut task_struct;
    let real_parent: *mut task_struct;

    task = bpf_get_current_task_btf();
    /* non-sleepable: missing bpf_rcu_read_unlock() in one path */
    bpf_rcu_read_lock();
    real_parent = (*task).real_parent;
    if !real_parent.is_null() {
        let _ = bpf_task_storage_get(&mut map_a, real_parent, core::ptr::null(), 0);
        if !real_parent.is_null() {
            bpf_rcu_read_unlock();
        }
    }
    0
}

/* SEC("?fentry.s/" SYS_PREFIX "sys_getpgid") */
#[no_mangle]
pub unsafe extern "C" fn inproper_sleepable_helper(ctx: *mut core::ffi::c_void) -> i32 {
    let task: *mut task_struct;
    let real_parent: *mut task_struct;
    let regs: *mut pt_regs;
    let mut value: __u32 = 0;
    let ptr: *mut core::ffi::c_void;

    task = bpf_get_current_task_btf();
    /* sleepable helper in rcu read lock region */
    bpf_rcu_read_lock();
    real_parent = (*task).real_parent;
    if !real_parent.is_null() {
        regs = bpf_task_pt_regs(real_parent);
        if !regs.is_null() {
            ptr = PT_REGS_IP(regs) as *mut core::ffi::c_void;
            let _ = bpf_copy_from_user_task(
                &mut value as *mut _ as *mut core::ffi::c_void,
                core::mem::size_of::<u32>(),
                ptr as *const core::ffi::c_void,
                task,
                0,
            );
            user_data = value;
            let _ = bpf_task_storage_get(&mut map_a, real_parent, core::ptr::null(), 0);
        }
    }
    bpf_rcu_read_unlock();
    0
}

/* SEC("?lsm.s/bpf") */
#[no_mangle]
pub unsafe extern "C" fn inproper_sleepable_kfunc(
    cmd: i32,
    attr: *mut bpf_attr,
    size: u32,
    kernel: bool,
) -> i32 {
    let bkey: *mut bpf_key;

    /* sleepable kfunc in rcu read lock region */
    bpf_rcu_read_lock();
    bkey = bpf_lookup_user_key(key_serial, flags);
    bpf_rcu_read_unlock();
    if bkey.is_null() {
        return -1;
    }
    bpf_key_put(bkey);

    0
}

/* SEC("?fentry.s/" SYS_PREFIX "sys_nanosleep") */
#[no_mangle]
pub unsafe extern "C" fn nested_rcu_region(ctx: *mut core::ffi::c_void) -> i32 {
    let task: *mut task_struct;
    let real_parent: *mut task_struct;

    /* nested rcu read lock regions */
    task = bpf_get_current_task_btf();
    bpf_rcu_read_lock();
    bpf_rcu_read_lock();
    real_parent = (*task).real_parent;
    if !real_parent.is_null() {
        let _ = bpf_task_storage_get(&mut map_a, real_parent, core::ptr::null(), 0);
    }
    bpf_rcu_read_unlock();
    bpf_rcu_read_unlock();
    0
}

/* SEC("?fentry.s/" SYS_PREFIX "sys_nanosleep") */
#[no_mangle]
pub unsafe extern "C" fn nested_rcu_region_unbalanced_1(ctx: *mut core::ffi::c_void) -> i32 {
    let task: *mut task_struct;
    let real_parent: *mut task_struct;

    /* nested rcu read lock regions */
    task = bpf_get_current_task_btf();
    bpf_rcu_read_lock();
    bpf_rcu_read_lock();
    real_parent = (*task).real_parent;
    if !real_parent.is_null() {
        let _ = bpf_task_storage_get(&mut map_a, real_parent, core::ptr::null(), 0);
    }
    bpf_rcu_read_unlock();
    bpf_rcu_read_unlock();
    bpf_rcu_read_unlock();
    0
}

/* SEC("?fentry.s/" SYS_PREFIX "sys_nanosleep") */
#[no_mangle]
pub unsafe extern "C" fn nested_rcu_region_unbalanced_2(ctx: *mut core::ffi::c_void) -> i32 {
    let task: *mut task_struct;
    let real_parent: *mut task_struct;

    /* nested rcu read lock regions */
    task = bpf_get_current_task_btf();
    bpf_rcu_read_lock();
    bpf_rcu_read_lock();
    bpf_rcu_read_lock();
    real_parent = (*task).real_parent;
    if !real_parent.is_null() {
        let _ = bpf_task_storage_get(&mut map_a, real_parent, core::ptr::null(), 0);
    }
    bpf_rcu_read_unlock();
    bpf_rcu_read_unlock();
    0
}

/* SEC("?fentry.s/" SYS_PREFIX "sys_getpgid") */
#[no_mangle]
pub unsafe extern "C" fn task_trusted_non_rcuptr(ctx: *mut core::ffi::c_void) -> i32 {
    let task: *mut task_struct;
    let group_leader: *mut task_struct;

    task = bpf_get_current_task_btf();
    bpf_rcu_read_lock();
    /* the pointer group_leader is explicitly marked as trusted */
    group_leader = (*(*task).real_parent).group_leader;
    let _ = bpf_task_storage_get(&mut map_a, group_leader, core::ptr::null(), 0);
    bpf_rcu_read_unlock();
    0
}

/* SEC("?fentry.s/" SYS_PREFIX "sys_getpgid") */
#[no_mangle]
pub unsafe extern "C" fn task_untrusted_rcuptr(ctx: *mut core::ffi::c_void) -> i32 {
    let task: *mut task_struct;
    let real_parent: *mut task_struct;

    task = bpf_get_current_task_btf();
    bpf_rcu_read_lock();
    real_parent = (*task).real_parent;
    bpf_rcu_read_unlock();
    /* helper use of rcu ptr outside the rcu read lock region */
    let _ = bpf_task_storage_get(&mut map_a, real_parent, core::ptr::null(), 0);
    0
}

/* SEC("?fentry.s/" SYS_PREFIX "sys_nanosleep") */
#[no_mangle]
pub unsafe extern "C" fn cross_rcu_region(ctx: *mut core::ffi::c_void) -> i32 {
    let task: *mut task_struct;
    let real_parent: *mut task_struct;

    /* rcu ptr define/use in different regions */
    task = bpf_get_current_task_btf();
    bpf_rcu_read_lock();
    real_parent = (*task).real_parent;
    bpf_rcu_read_unlock();
    bpf_rcu_read_lock();
    let _ = bpf_task_storage_get(&mut map_a, real_parent, core::ptr::null(), 0);
    bpf_rcu_read_unlock();
    0
}

#[inline(never)]
unsafe fn static_subprog(ctx: *mut core::ffi::c_void) -> i32 {
    let mut ret: i32 = 0;

    if bpf_get_prandom_u32() != 0 {
        return core::ptr::read_volatile(&ret).wrapping_add(42);
    }
    core::ptr::read_volatile(&ret).wrapping_add(bpf_get_prandom_u32() as i32)
}

#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn global_subprog(a: u64) -> i32 {
    let mut ret: i32 = a as i32;

    core::ptr::read_volatile(&ret).wrapping_add(static_subprog(core::ptr::null_mut()))
}

#[inline(never)]
unsafe fn static_subprog_lock(ctx: *mut core::ffi::c_void) -> i32 {
    let mut ret: i32 = 0;

    bpf_rcu_read_lock();
    if bpf_get_prandom_u32() != 0 {
        return core::ptr::read_volatile(&ret).wrapping_add(42);
    }
    core::ptr::read_volatile(&ret).wrapping_add(bpf_get_prandom_u32() as i32)
}

#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn global_subprog_lock(a: u64) -> i32 {
    let mut ret: i32 = a as i32;

    core::ptr::read_volatile(&ret).wrapping_add(static_subprog_lock(core::ptr::null_mut()))
}

#[inline(never)]
unsafe fn static_subprog_unlock(ctx: *mut core::ffi::c_void) -> i32 {
    let mut ret: i32 = 0;

    bpf_rcu_read_unlock();
    if bpf_get_prandom_u32() != 0 {
        return core::ptr::read_volatile(&ret).wrapping_add(42);
    }
    core::ptr::read_volatile(&ret).wrapping_add(bpf_get_prandom_u32() as i32)
}

#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn global_subprog_unlock(a: u64) -> i32 {
    let mut ret: i32 = a as i32;

    core::ptr::read_volatile(&ret).wrapping_add(static_subprog_unlock(core::ptr::null_mut()))
}

/* SEC("?fentry.s/" SYS_PREFIX "sys_getpgid") */
#[no_mangle]
pub unsafe extern "C" fn rcu_read_lock_subprog(ctx: *mut core::ffi::c_void) -> i32 {
    let mut ret: i32 = 0;

    bpf_rcu_read_lock();
    if bpf_get_prandom_u32() != 0 {
        let tmp = core::ptr::read_volatile(&ret).wrapping_add(static_subprog(ctx));
        core::ptr::write_volatile(&mut ret, tmp);
    }
    bpf_rcu_read_unlock();
    0
}

/* SEC("?fentry.s/" SYS_PREFIX "sys_getpgid") */
#[no_mangle]
pub unsafe extern "C" fn rcu_read_lock_global_subprog(ctx: *mut core::ffi::c_void) -> i32 {
    let mut ret: i32 = 0;

    bpf_rcu_read_lock();
    if bpf_get_prandom_u32() != 0 {
        let tmp = core::ptr::read_volatile(&ret).wrapping_add(global_subprog(core::ptr::read_volatile(&ret) as u64));
        core::ptr::write_volatile(&mut ret, tmp);
    }
    bpf_rcu_read_unlock();
    0
}

/* SEC("?fentry.s/" SYS_PREFIX "sys_getpgid") */
#[no_mangle]
pub unsafe extern "C" fn rcu_read_lock_subprog_lock(ctx: *mut core::ffi::c_void) -> i32 {
    let mut ret: i32 = 0;

    let tmp = core::ptr::read_volatile(&ret).wrapping_add(static_subprog_lock(ctx));
    core::ptr::write_volatile(&mut ret, tmp);
    bpf_rcu_read_unlock();
    0
}

/* SEC("?fentry.s/" SYS_PREFIX "sys_getpgid") */
#[no_mangle]
pub unsafe extern "C" fn rcu_read_lock_global_subprog_lock(ctx: *mut core::ffi::c_void) -> i32 {
    let mut ret: i32 = 0;

    let tmp = core::ptr::read_volatile(&ret).wrapping_add(global_subprog_lock(core::ptr::read_volatile(&ret) as u64));
    core::ptr::write_volatile(&mut ret, tmp);
    bpf_rcu_read_unlock();
    0
}

/* SEC("?fentry.s/" SYS_PREFIX "sys_getpgid") */
#[no_mangle]
pub unsafe extern "C" fn rcu_read_lock_subprog_unlock(ctx: *mut core::ffi::c_void) -> i32 {
    let mut ret: i32 = 0;

    bpf_rcu_read_lock();
    let tmp = core::ptr::read_volatile(&ret).wrapping_add(static_subprog_unlock(ctx));
    core::ptr::write_volatile(&mut ret, tmp);
    0
}

/* SEC("?fentry.s/" SYS_PREFIX "sys_getpgid") */
#[no_mangle]
pub unsafe extern "C" fn rcu_read_lock_global_subprog_unlock(ctx: *mut core::ffi::c_void) -> i32 {
    let mut ret: i32 = 0;

    bpf_rcu_read_lock();
    let tmp = core::ptr::read_volatile(&ret).wrapping_add(global_subprog_unlock(core::ptr::read_volatile(&ret) as u64));
    core::ptr::write_volatile(&mut ret, tmp);
    0
}

#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn global_sleepable_helper_subprog(mut i: i32) -> i32 {
    if i != 0 {
        let _ = bpf_copy_from_user(
            &mut i as *mut _ as *mut core::ffi::c_void,
            core::mem::size_of_val(&i),
            core::ptr::null(),
        );
    }
    i
}

#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn global_sleepable_kfunc_subprog(mut i: i32) -> i32 {
    if i != 0 {
        let _ = bpf_copy_from_user_str(
            &mut i as *mut _ as *mut core::ffi::c_void,
            core::mem::size_of_val(&i),
            core::ptr::null(),
            0,
        );
    }
    global_subprog(i as u64);
    i
}

#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn global_subprog_calling_sleepable_global(i: i32) -> i32 {
    if i == 0 {
        global_sleepable_kfunc_subprog(i);
    }
    i
}

/* SEC("?fentry.s/" SYS_PREFIX "sys_getpgid") */
#[no_mangle]
pub unsafe extern "C" fn rcu_read_lock_sleepable_helper_global_subprog(
    ctx: *mut core::ffi::c_void,
) -> i32 {
    let mut ret: i32 = 0;

    bpf_rcu_read_lock();
    let tmp = core::ptr::read_volatile(&ret)
        .wrapping_add(global_sleepable_helper_subprog(core::ptr::read_volatile(&ret)));
    core::ptr::write_volatile(&mut ret, tmp);
    bpf_rcu_read_unlock();
    0
}

/* SEC("?fentry.s/" SYS_PREFIX "sys_getpgid") */
#[no_mangle]
pub unsafe extern "C" fn rcu_read_lock_sleepable_kfunc_global_subprog(
    ctx: *mut core::ffi::c_void,
) -> i32 {
    let mut ret: i32 = 0;

    bpf_rcu_read_lock();
    let tmp = core::ptr::read_volatile(&ret)
        .wrapping_add(global_sleepable_kfunc_subprog(core::ptr::read_volatile(&ret)));
    core::ptr::write_volatile(&mut ret, tmp);
    bpf_rcu_read_unlock();
    0
}

/* SEC("?fentry.s/" SYS_PREFIX "sys_getpgid") */
#[no_mangle]
pub unsafe extern "C" fn rcu_read_lock_sleepable_global_subprog_indirect(
    ctx: *mut core::ffi::c_void,
) -> i32 {
    let mut ret: i32 = 0;

    bpf_rcu_read_lock();
    let tmp = core::ptr::read_volatile(&ret)
        .wrapping_add(global_subprog_calling_sleepable_global(core::ptr::read_volatile(&ret)));
    core::ptr::write_volatile(&mut ret, tmp);
    bpf_rcu_read_unlock();
    0
}

#[repr(C)]
pub struct rcu_node_data {
    pub key: i64,
    pub node: bpf_rb_node,
}

#[repr(C)]
pub struct rcu_node_stash {
    /* struct rcu_node_data __kptr *node; */
    pub node: *mut rcu_node_data,
}

/*
 * Necessary so that LLVM emits BTF for rcu_node_data rather than just a
 * fwd reference to it, same as in progs/local_kptr_stash.c.
 */
#[no_mangle]
pub static mut just_here_because_btf_bug: *mut rcu_node_data = core::ptr::null_mut();

/* SEC(".maps") */
#[no_mangle]
pub static mut node_stash: NodeStashMap = NodeStashMap {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
};

#[no_mangle]
pub static mut non_own_ref_key: i64 = 0;

/* SEC("?fentry.s/" SYS_PREFIX "sys_getpgid") */
#[no_mangle]
pub unsafe extern "C" fn non_own_ref_untrusted_ld(ctx: *mut core::ffi::c_void) -> i32 {
    let stash: *mut rcu_node_stash;
    let node: *mut rcu_node_data;
    let key: i32 = 0;

    stash = bpf_map_lookup_elem(&mut node_stash, &key) as *mut rcu_node_stash;
    if stash.is_null() {
        return 0;
    }
    bpf_rcu_read_lock();
    node = (*stash).node;
    if node.is_null() {
        bpf_rcu_read_unlock();
        return 0;
    }
    bpf_rcu_read_unlock();
    /*
     * The unlock leaves node as PTR_TO_BTF_ID | MEM_ALLOC | PTR_UNTRUSTED
     * | NON_OWN_REF, and the load below has to get the BPF_PROBE_MEM
     * rewrite for it, otherwise a bad address panics the kernel.
     */
    non_own_ref_key = (*node).key;
    0
}

#[no_mangle]
pub static mut rcu_untrusted_wq_flags: i64 = 0;

/* SEC("?tp_btf/tcp_probe") */
#[no_mangle]
pub unsafe extern "C" fn rcu_untrusted_union_ld(sk: *mut sock) -> i32 {
    let wq: *mut socket_wq;

    /*
     * sk_wq sits in a two member union, so btf_struct_walk() marks the
     * pointer PTR_UNTRUSTED, and the __rcu tag on the member adds MEM_RCU
     * on top of it. struct sock is not on the __safe_rcu_or_null allow
     * list, hence the two stay combined and the load below has to get the
     * BPF_PROBE_MEM rewrite for PTR_TO_BTF_ID | PTR_UNTRUSTED | MEM_RCU,
     * otherwise a bad address panics the kernel.
     *
     * The __rcu tag only reaches BTF on a clang built kernel, that is, one
     * with CONFIG_PAHOLE_HAS_BTF_TAG. On a gcc built kernel the walk yields
     * a plain untrusted pointer, which is rewritten either way.
     */
    wq = (*sk).sk_wq;
    if wq.is_null() {
        return 0;
    }
    rcu_untrusted_wq_flags = (*wq).flags;
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
