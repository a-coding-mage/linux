// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */
// Translated from profiler.inc.h. C include dependencies are expected to
// provide the referenced BPF, kernel, and profiler symbols.

pub const NULL: usize = 0;

pub const O_WRONLY: u32 = 0o0000001;
pub const O_RDWR: u32 = 0o0000002;
pub const O_DIRECTORY: u32 = 0o0200000;
pub const __O_TMPFILE: u32 = 0o20000000;
pub const O_TMPFILE: u32 = __O_TMPFILE | O_DIRECTORY;
pub const S_IFMT: u32 = 0o0170000;
pub const S_IFSOCK: u32 = 0o140000;
pub const S_IFLNK: u32 = 0o120000;
pub const S_IFREG: u32 = 0o100000;
pub const S_IFBLK: u32 = 0o060000;
pub const S_IFDIR: u32 = 0o040000;
pub const S_IFCHR: u32 = 0o020000;
pub const S_IFIFO: u32 = 0o010000;
pub const S_ISUID: u32 = 0o004000;
pub const S_ISGID: u32 = 0o002000;
pub const S_ISVTX: u32 = 0o001000;

#[inline(always)]
pub const fn S_ISLNK(m: u32) -> bool {
    (m & S_IFMT) == S_IFLNK
}

#[inline(always)]
pub const fn S_ISDIR(m: u32) -> bool {
    (m & S_IFMT) == S_IFDIR
}

#[inline(always)]
pub const fn S_ISCHR(m: u32) -> bool {
    (m & S_IFMT) == S_IFCHR
}

#[inline(always)]
pub const fn S_ISBLK(m: u32) -> bool {
    (m & S_IFMT) == S_IFBLK
}

#[inline(always)]
pub const fn S_ISFIFO(m: u32) -> bool {
    (m & S_IFMT) == S_IFIFO
}

#[inline(always)]
pub const fn S_ISSOCK(m: u32) -> bool {
    (m & S_IFMT) == S_IFSOCK
}

pub const KILL_DATA_ARRAY_SIZE: usize = 8;

#[repr(C)]
pub struct var_kill_data_arr_t {
    pub array: [var_kill_data_t; KILL_DATA_ARRAY_SIZE],
}

#[repr(C)]
pub union any_profiler_data_t {
    pub var_exec: core::mem::ManuallyDrop<var_exec_data_t>,
    pub var_kill: core::mem::ManuallyDrop<var_kill_data_t>,
    pub var_sysctl: core::mem::ManuallyDrop<var_sysctl_data_t>,
    pub var_filemod: core::mem::ManuallyDrop<var_filemod_data_t>,
    pub var_fork: core::mem::ManuallyDrop<var_fork_data_t>,
    pub var_kill_data_arr: core::mem::ManuallyDrop<var_kill_data_arr_t>,
}

#[no_mangle]
pub static mut bpf_config: profiler_config_struct = profiler_config_struct {};

macro_rules! FETCH_CGROUPS_FROM_BPF {
    () => {
        unsafe { bpf_config.fetch_cgroups_from_bpf }
    };
}
macro_rules! CGROUP_FS_INODE {
    () => {
        unsafe { bpf_config.cgroup_fs_inode }
    };
}
macro_rules! CGROUP_LOGIN_SESSION_INODE {
    () => {
        unsafe { bpf_config.cgroup_login_session_inode }
    };
}
macro_rules! KILL_SIGNALS {
    () => {
        unsafe { bpf_config.kill_signals_mask }
    };
}
macro_rules! STALE_INFO {
    () => {
        unsafe { bpf_config.stale_info_secs }
    };
}
macro_rules! INODE_FILTER {
    () => {
        unsafe { bpf_config.inode_filter }
    };
}
macro_rules! READ_ENVIRON_FROM_EXEC {
    () => {
        unsafe { bpf_config.read_environ_from_exec }
    };
}
macro_rules! ENABLE_CGROUP_V1_RESOLVER {
    () => {
        unsafe { bpf_config.enable_cgroup_v1_resolver }
    };
}

#[repr(C)]
pub struct kernfs_iattrs___52 {
    pub ia_iattr: iattr,
}

#[repr(C)]
pub struct kernfs_node___52_id_fields {
    pub ino: u32,
    pub generation: u32,
}

#[repr(C)]
pub union kernfs_node___52_id {
    pub fields: kernfs_node___52_id_fields,
    pub id: u64,
}

#[repr(C)]
pub struct kernfs_node___52 {
    pub id: kernfs_node___52_id,
}

// BPF map definitions translated from anonymous SEC(".maps") C declarations.
// The map descriptor macros (__uint/__type/SEC) are supplied by the BPF build environment.
extern "C" {
    pub static mut data_heap: any_profiler_data_t;
    pub static mut events: core::ffi::c_void;
    pub static mut var_tpid_to_data: core::ffi::c_void;
    pub static mut bpf_func_stats: core::ffi::c_void;
    pub static mut allowed_devices: core::ffi::c_void;
    pub static mut allowed_file_inodes: core::ffi::c_void;
    pub static mut allowed_directory_inodes: core::ffi::c_void;
    pub static mut disallowed_exec_inodes: core::ffi::c_void;
}

#[inline(always)]
pub unsafe fn IS_ERR(ptr: *const core::ffi::c_void) -> bool {
    IS_ERR_VALUE(ptr as usize as core::ffi::c_ulong)
}

#[inline(always)]
pub unsafe fn get_userspace_pid() -> u32 {
    (bpf_get_current_pid_tgid() >> 32) as u32
}

#[inline(always)]
pub fn is_init_process(tgid: u32) -> bool {
    tgid == 1 || tgid == 0
}

#[inline(always)]
pub unsafe fn probe_read_lim(
    dst: *mut core::ffi::c_void,
    src: *mut core::ffi::c_void,
    mut len: core::ffi::c_ulong,
    max: core::ffi::c_ulong,
) -> core::ffi::c_ulong {
    len = if len < max { len } else { max };
    if len > 1 {
        if bpf_probe_read_kernel(dst, len as usize, src) != 0 {
            return 0;
        }
    } else if len == 1 {
        if bpf_probe_read_kernel(dst, 1, src) != 0 {
            return 0;
        }
    }
    len
}

#[inline(always)]
pub unsafe fn get_var_spid_index(arr_struct: *mut var_kill_data_arr_t, spid: i32) -> i32 {
    // If UNROLL is defined in C, this loop is pragma-unrolled.
    let mut i = 0usize;
    while i < KILL_DATA_ARRAY_SIZE {
        if (*arr_struct).array[i].meta.pid == spid as u32 {
            return i as i32;
        }
        i += 1;
    }
    -1
}

#[inline(always)]
pub unsafe fn populate_ancestors(task: *mut task_struct, ancestors_data: *mut ancestors_data_t) {
    let mut parent = task;
    let mut num_ancestors: u32 = 0;

    (*ancestors_data).num_ancestors = 0;
    // If UNROLL is defined in C, this loop is pragma-unrolled.
    while num_ancestors < MAX_ANCESTORS {
        parent = BPF_CORE_READ!(parent, real_parent);
        if parent.is_null() {
            break;
        }
        let ppid: u32 = BPF_CORE_READ!(parent, tgid);
        if is_init_process(ppid) {
            break;
        }
        (*ancestors_data).ancestor_pids[num_ancestors as usize] = ppid;
        (*ancestors_data).ancestor_exec_ids[num_ancestors as usize] =
            BPF_CORE_READ!(parent, self_exec_id);
        (*ancestors_data).ancestor_start_times[num_ancestors as usize] =
            BPF_CORE_READ!(parent, start_time);
        (*ancestors_data).num_ancestors = num_ancestors;
        num_ancestors += 1;
    }
}

#[inline(always)]
pub unsafe fn read_full_cgroup_path(
    mut cgroup_node: *mut kernfs_node,
    cgroup_root_node: *mut kernfs_node,
    mut payload: *mut core::ffi::c_void,
    root_pos: *mut i32,
) -> *mut core::ffi::c_void {
    let payload_start = payload;
    let mut filepart_length: usize;

    // If UNROLL is defined in C, this loop is pragma-unrolled.
    let mut i = 0;
    while i < MAX_CGROUPS_PATH_DEPTH {
        filepart_length =
            bpf_probe_read_kernel_str(payload, MAX_PATH as usize, BPF_CORE_READ!(cgroup_node, name))
                as usize;
        if cgroup_node.is_null() {
            return payload;
        }
        if cgroup_node == cgroup_root_node {
            *root_pos = payload.offset_from(payload_start) as i32;
        }
        if bpf_cmp_likely(filepart_length, BPF_CMP_LE, MAX_PATH as usize) {
            payload = payload.add(filepart_length);
        }
        cgroup_node = BPF_CORE_READ!(cgroup_node, __parent);
        i += 1;
    }
    payload
}

pub unsafe fn get_inode_from_kernfs(node: *mut kernfs_node) -> ino_t {
    let node52 = node as *mut kernfs_node___52;

    if bpf_core_field_exists!(node52, id, ino) {
        barrier_var(node52);
        return BPF_CORE_READ!(node52, id, ino) as ino_t;
    } else {
        barrier_var(node);
        return BPF_CORE_READ!(node, id) as u64 as ino_t;
    }
}

extern "C" {
    pub static CONFIG_CGROUP_PIDS: bool;
}

#[repr(C)]
pub enum cgroup_subsys_id___local {
    pids_cgrp_id___local = 123, /* value doesn't matter */
}

#[inline(always)]
pub unsafe fn populate_cgroup_info(
    cgroup_data: *mut cgroup_data_t,
    task: *mut task_struct,
    mut payload: *mut core::ffi::c_void,
) -> *mut core::ffi::c_void {
    let mut root_kernfs: *mut kernfs_node =
        BPF_CORE_READ!(task, nsproxy, cgroup_ns, root_cset, dfl_cgrp, kn);
    let mut proc_kernfs: *mut kernfs_node = BPF_CORE_READ!(task, cgroups, dfl_cgrp, kn);

    // C condition: #if __has_builtin(__builtin_preserve_enum_value)
    if ENABLE_CGROUP_V1_RESOLVER!() && CONFIG_CGROUP_PIDS {
        let cgrp_id: i32 = bpf_core_enum_value(
            cgroup_subsys_id___local::pids_cgrp_id___local as i32,
        );
        // If UNROLL is defined in C, this loop is pragma-unrolled.
        let mut i = 0;
        while i < CGROUP_SUBSYS_COUNT {
            let subsys: *mut cgroup_subsys_state = BPF_CORE_READ!(task, cgroups, subsys[i]);
            if !subsys.is_null() {
                let subsys_id: i32 = BPF_CORE_READ!(subsys, ss, id);
                if subsys_id == cgrp_id {
                    proc_kernfs = BPF_CORE_READ!(subsys, cgroup, kn);
                    root_kernfs = BPF_CORE_READ!(subsys, ss, root, kf_root, kn);
                    break;
                }
            }
            i += 1;
        }
    }

    (*cgroup_data).cgroup_root_inode = get_inode_from_kernfs(root_kernfs);
    (*cgroup_data).cgroup_proc_inode = get_inode_from_kernfs(proc_kernfs);

    if bpf_core_field_exists!(root_kernfs, iattr, ia_mtime) {
        (*cgroup_data).cgroup_root_mtime = BPF_CORE_READ!(root_kernfs, iattr, ia_mtime.tv_nsec);
        (*cgroup_data).cgroup_proc_mtime = BPF_CORE_READ!(proc_kernfs, iattr, ia_mtime.tv_nsec);
    } else {
        let root_iattr =
            BPF_CORE_READ!(root_kernfs, iattr) as *mut kernfs_iattrs___52;
        (*cgroup_data).cgroup_root_mtime =
            BPF_CORE_READ!(root_iattr, ia_iattr, ia_mtime.tv_nsec);

        let proc_iattr =
            BPF_CORE_READ!(proc_kernfs, iattr) as *mut kernfs_iattrs___52;
        (*cgroup_data).cgroup_proc_mtime =
            BPF_CORE_READ!(proc_iattr, ia_iattr, ia_mtime.tv_nsec);
    }

    (*cgroup_data).cgroup_root_length = 0;
    (*cgroup_data).cgroup_proc_length = 0;
    (*cgroup_data).cgroup_full_length = 0;

    let cgroup_root_length =
        bpf_probe_read_kernel_str(payload, MAX_PATH as usize, BPF_CORE_READ!(root_kernfs, name))
            as usize;
    if bpf_cmp_likely(cgroup_root_length, BPF_CMP_LE, MAX_PATH as usize) {
        (*cgroup_data).cgroup_root_length = cgroup_root_length;
        payload = payload.add(cgroup_root_length);
    }

    let cgroup_proc_length =
        bpf_probe_read_kernel_str(payload, MAX_PATH as usize, BPF_CORE_READ!(proc_kernfs, name))
            as usize;
    if bpf_cmp_likely(cgroup_proc_length, BPF_CMP_LE, MAX_PATH as usize) {
        (*cgroup_data).cgroup_proc_length = cgroup_proc_length;
        payload = payload.add(cgroup_proc_length);
    }

    if FETCH_CGROUPS_FROM_BPF!() {
        (*cgroup_data).cgroup_full_path_root_pos = -1;
        let payload_end_pos = read_full_cgroup_path(
            proc_kernfs,
            root_kernfs,
            payload,
            &mut (*cgroup_data).cgroup_full_path_root_pos,
        );
        (*cgroup_data).cgroup_full_length = payload_end_pos.offset_from(payload) as usize;
        payload = payload_end_pos;
    }

    payload
}

#[inline(always)]
pub unsafe fn populate_var_metadata(
    metadata: *mut var_metadata_t,
    task: *mut task_struct,
    pid: u32,
    mut payload: *mut core::ffi::c_void,
) -> *mut core::ffi::c_void {
    let uid_gid: u64 = bpf_get_current_uid_gid();

    (*metadata).uid = uid_gid as u32;
    (*metadata).gid = (uid_gid >> 32) as u32;
    (*metadata).pid = pid;
    (*metadata).exec_id = BPF_CORE_READ!(task, self_exec_id);
    (*metadata).start_time = BPF_CORE_READ!(task, start_time);
    (*metadata).comm_length = 0;

    let comm_length = bpf_core_read_str(payload, TASK_COMM_LEN as usize, &mut (*task).comm) as usize;
    if bpf_cmp_likely(comm_length, BPF_CMP_LE, TASK_COMM_LEN as usize) {
        (*metadata).comm_length = comm_length;
        payload = payload.add(comm_length);
    }

    payload
}

#[inline(always)]
pub unsafe fn get_var_kill_data(
    ctx: *mut pt_regs,
    spid: i32,
    tpid: i32,
    sig: i32,
) -> *mut var_kill_data_t {
    let mut zero: i32 = 0;
    let kill_data =
        bpf_map_lookup_elem(&mut data_heap as *mut _ as *mut core::ffi::c_void, &mut zero as *mut _ as *mut core::ffi::c_void)
            as *mut var_kill_data_t;

    if kill_data.is_null() {
        return core::ptr::null_mut();
    }
    let task = bpf_get_current_task() as *mut task_struct;

    let mut payload = populate_var_metadata(&mut (*kill_data).meta, task, spid as u32, (*kill_data).payload.as_mut_ptr() as *mut _);
    payload = populate_cgroup_info(&mut (*kill_data).cgroup_data, task, payload);
    let payload_length = payload.offset_from((*kill_data).payload.as_mut_ptr() as *mut _) as usize;
    (*kill_data).payload_length = payload_length;
    populate_ancestors(task, &mut (*kill_data).ancestors_info);
    (*kill_data).meta.r#type = KILL_EVENT;
    (*kill_data).kill_target_pid = tpid;
    (*kill_data).kill_sig = sig;
    (*kill_data).kill_count = 1;
    (*kill_data).last_kill_time = bpf_ktime_get_ns();
    kill_data
}

#[inline(always)]
pub unsafe fn trace_var_sys_kill(ctx: *mut core::ffi::c_void, tpid: i32, sig: i32) -> i32 {
    if (KILL_SIGNALS!() & (1u64 << sig)) == 0 {
        return 0;
    }

    let spid = get_userspace_pid();
    let arr_struct = bpf_map_lookup_elem(
        &mut var_tpid_to_data as *mut _ as *mut core::ffi::c_void,
        &tpid as *const _ as *mut core::ffi::c_void,
    ) as *mut var_kill_data_arr_t;

    if arr_struct.is_null() {
        let kill_data = get_var_kill_data(ctx as *mut pt_regs, spid as i32, tpid, sig);
        let mut zero: i32 = 0;

        if kill_data.is_null() {
            return 0;
        }
        let arr_struct = bpf_map_lookup_elem(
            &mut data_heap as *mut _ as *mut core::ffi::c_void,
            &mut zero as *mut _ as *mut core::ffi::c_void,
        ) as *mut var_kill_data_arr_t;
        if arr_struct.is_null() {
            return 0;
        }
        bpf_probe_read_kernel(
            &mut (*arr_struct).array[0] as *mut _ as *mut core::ffi::c_void,
            core::mem::size_of_val(&(*arr_struct).array[0]),
            kill_data as *mut core::ffi::c_void,
        );
        bpf_map_update_elem(
            &mut var_tpid_to_data as *mut _ as *mut core::ffi::c_void,
            &tpid as *const _ as *mut core::ffi::c_void,
            arr_struct as *mut core::ffi::c_void,
            0,
        );
    } else {
        let index = get_var_spid_index(arr_struct, spid as i32);

        if index == -1 {
            let kill_data = get_var_kill_data(ctx as *mut pt_regs, spid as i32, tpid, sig);
            if kill_data.is_null() {
                return 0;
            }
            // If UNROLL is defined in C, this loop is pragma-unrolled.
            let mut i = 0usize;
            while i < KILL_DATA_ARRAY_SIZE {
                if (*arr_struct).array[i].meta.pid == 0 {
                    bpf_probe_read_kernel(
                        &mut (*arr_struct).array[i] as *mut _ as *mut core::ffi::c_void,
                        core::mem::size_of_val(&(*arr_struct).array[i]),
                        kill_data as *mut core::ffi::c_void,
                    );
                    bpf_map_update_elem(
                        &mut var_tpid_to_data as *mut _ as *mut core::ffi::c_void,
                        &tpid as *const _ as *mut core::ffi::c_void,
                        arr_struct as *mut core::ffi::c_void,
                        0,
                    );

                    return 0;
                }
                i += 1;
            }
            return 0;
        }

        let kill_data = &mut (*arr_struct).array[index as usize] as *mut var_kill_data_t;

        let delta_sec = (bpf_ktime_get_ns() - (*kill_data).last_kill_time) / 1_000_000_000;

        if delta_sec < STALE_INFO!() {
            (*kill_data).kill_count += 1;
            (*kill_data).last_kill_time = bpf_ktime_get_ns();
            bpf_probe_read_kernel(
                &mut (*arr_struct).array[index as usize] as *mut _ as *mut core::ffi::c_void,
                core::mem::size_of_val(&(*arr_struct).array[index as usize]),
                kill_data as *mut core::ffi::c_void,
            );
        } else {
            let kill_data = get_var_kill_data(ctx as *mut pt_regs, spid as i32, tpid, sig);
            if kill_data.is_null() {
                return 0;
            }
            bpf_probe_read_kernel(
                &mut (*arr_struct).array[index as usize] as *mut _ as *mut core::ffi::c_void,
                core::mem::size_of_val(&(*arr_struct).array[index as usize]),
                kill_data as *mut core::ffi::c_void,
            );
        }
        bpf_map_update_elem(
            &mut var_tpid_to_data as *mut _ as *mut core::ffi::c_void,
            &tpid as *const _ as *mut core::ffi::c_void,
            arr_struct as *mut core::ffi::c_void,
            0,
        );
    }
    0
}

#[inline(always)]
pub unsafe fn bpf_stats_enter(bpf_stat_ctx: *mut bpf_func_stats_ctx, func_id: bpf_function_id) {
    let mut func_id_key: i32 = func_id as i32;

    (*bpf_stat_ctx).start_time_ns = bpf_ktime_get_ns();
    (*bpf_stat_ctx).bpf_func_stats_data_val = bpf_map_lookup_elem(
        &mut bpf_func_stats as *mut _ as *mut core::ffi::c_void,
        &mut func_id_key as *mut _ as *mut core::ffi::c_void,
    ) as *mut bpf_func_stats_data;
    if !(*bpf_stat_ctx).bpf_func_stats_data_val.is_null() {
        (*(*bpf_stat_ctx).bpf_func_stats_data_val).num_executions += 1;
    }
}

#[inline(always)]
pub unsafe fn bpf_stats_exit(bpf_stat_ctx: *mut bpf_func_stats_ctx) {
    if !(*bpf_stat_ctx).bpf_func_stats_data_val.is_null() {
        (*(*bpf_stat_ctx).bpf_func_stats_data_val).time_elapsed_ns +=
            bpf_ktime_get_ns() - (*bpf_stat_ctx).start_time_ns;
    }
}

#[inline(always)]
pub unsafe fn bpf_stats_pre_submit_var_perf_event(
    bpf_stat_ctx: *mut bpf_func_stats_ctx,
    meta: *mut var_metadata_t,
) {
    if !(*bpf_stat_ctx).bpf_func_stats_data_val.is_null() {
        (*(*bpf_stat_ctx).bpf_func_stats_data_val).num_perf_events += 1;
        (*meta).bpf_stats_num_perf_events =
            (*(*bpf_stat_ctx).bpf_func_stats_data_val).num_perf_events;
    }
    (*meta).bpf_stats_start_ktime_ns = (*bpf_stat_ctx).start_time_ns;
    (*meta).cpu_id = bpf_get_smp_processor_id();
}

#[inline(always)]
pub unsafe fn read_absolute_file_path_from_dentry(
    mut filp_dentry: *mut dentry,
    mut payload: *mut core::ffi::c_void,
) -> usize {
    let mut length: usize = 0;
    let mut filepart_length: usize;
    let mut parent_dentry: *mut dentry;

    // If UNROLL is defined in C, this loop is pragma-unrolled.
    let mut i = 0;
    while i < MAX_PATH_DEPTH {
        filepart_length = bpf_probe_read_kernel_str(
            payload,
            MAX_PATH as usize,
            BPF_CORE_READ!(filp_dentry, d_name.name),
        ) as usize;
        bpf_nop_mov(filepart_length);
        if bpf_cmp_unlikely(filepart_length, BPF_CMP_GT, MAX_PATH as usize) {
            break;
        }
        payload = payload.add(filepart_length);
        length += filepart_length;

        parent_dentry = BPF_CORE_READ!(filp_dentry, d_parent);
        if filp_dentry == parent_dentry {
            break;
        }
        filp_dentry = parent_dentry;
        i += 1;
    }

    length
}

#[inline(always)]
pub unsafe fn is_ancestor_in_allowed_inodes(mut filp_dentry: *mut dentry) -> bool {
    let mut parent_dentry: *mut dentry;
    // If UNROLL is defined in C, this loop is pragma-unrolled.
    let mut i = 0;
    while i < MAX_PATH_DEPTH {
        let dir_ino: u64 = BPF_CORE_READ!(filp_dentry, d_inode, i_ino);
        let allowed_dir = bpf_map_lookup_elem(
            &mut allowed_directory_inodes as *mut _ as *mut core::ffi::c_void,
            &dir_ino as *const _ as *mut core::ffi::c_void,
        ) as *mut bool;

        if !allowed_dir.is_null() {
            return true;
        }
        parent_dentry = BPF_CORE_READ!(filp_dentry, d_parent);
        if filp_dentry == parent_dentry {
            break;
        }
        filp_dentry = parent_dentry;
        i += 1;
    }
    false
}

#[inline(always)]
pub unsafe fn is_dentry_allowed_for_filemod(
    file_dentry: *mut dentry,
    device_id: *mut u32,
    file_ino: *mut u64,
) -> bool {
    let dev_id: u32 = BPF_CORE_READ!(file_dentry, d_sb, s_dev);
    *device_id = dev_id;
    let allowed_device = bpf_map_lookup_elem(
        &mut allowed_devices as *mut _ as *mut core::ffi::c_void,
        &dev_id as *const _ as *mut core::ffi::c_void,
    ) as *mut bool;

    if allowed_device.is_null() {
        return false;
    }

    let ino: u64 = BPF_CORE_READ!(file_dentry, d_inode, i_ino);
    *file_ino = ino;
    let allowed_file = bpf_map_lookup_elem(
        &mut allowed_file_inodes as *mut _ as *mut core::ffi::c_void,
        &ino as *const _ as *mut core::ffi::c_void,
    ) as *mut bool;

    if allowed_file.is_null() {
        if !is_ancestor_in_allowed_inodes(BPF_CORE_READ!(file_dentry, d_parent)) {
            return false;
        }
    }
    true
}

// SEC("kprobe/proc_sys_write")
pub unsafe extern "C" fn kprobe__proc_sys_write(
    ctx: *mut pt_regs,
    filp: *mut file,
    buf: *const core::ffi::c_char,
    count: usize,
    ppos: *mut loff_t,
) -> ssize_t {
    let mut stats_ctx: bpf_func_stats_ctx = core::mem::zeroed();
    bpf_stats_enter(&mut stats_ctx, profiler_bpf_proc_sys_write);

    let pid = get_userspace_pid();
    let mut zero: i32 = 0;
    let sysctl_data = bpf_map_lookup_elem(
        &mut data_heap as *mut _ as *mut core::ffi::c_void,
        &mut zero as *mut _ as *mut core::ffi::c_void,
    ) as *mut var_sysctl_data_t;
    if sysctl_data.is_null() {
        bpf_stats_exit(&mut stats_ctx);
        return 0;
    }

    let task = bpf_get_current_task() as *mut task_struct;
    (*sysctl_data).meta.r#type = SYSCTL_EVENT;
    let mut payload =
        populate_var_metadata(&mut (*sysctl_data).meta, task, pid, (*sysctl_data).payload.as_mut_ptr() as *mut _);
    payload = populate_cgroup_info(&mut (*sysctl_data).cgroup_data, task, payload);

    populate_ancestors(task, &mut (*sysctl_data).ancestors_info);

    (*sysctl_data).sysctl_val_length = 0;
    (*sysctl_data).sysctl_path_length = 0;

    let sysctl_val_length =
        bpf_probe_read_kernel_str(payload, CTL_MAXNAME as usize, buf as *const core::ffi::c_void)
            as usize;
    if bpf_cmp_likely(sysctl_val_length, BPF_CMP_LE, CTL_MAXNAME as usize) {
        (*sysctl_data).sysctl_val_length = sysctl_val_length;
        payload = payload.add(sysctl_val_length);
    }

    let sysctl_path_length = bpf_probe_read_kernel_str(
        payload,
        MAX_PATH as usize,
        BPF_CORE_READ!(filp, f_path.dentry, d_name.name),
    ) as usize;
    if bpf_cmp_likely(sysctl_path_length, BPF_CMP_LE, MAX_PATH as usize) {
        (*sysctl_data).sysctl_path_length = sysctl_path_length;
        payload = payload.add(sysctl_path_length);
    }

    bpf_stats_pre_submit_var_perf_event(&mut stats_ctx, &mut (*sysctl_data).meta);
    let mut data_len = payload.offset_from(sysctl_data as *mut core::ffi::c_void) as usize;
    data_len = if data_len > core::mem::size_of::<var_sysctl_data_t>() {
        core::mem::size_of::<var_sysctl_data_t>()
    } else {
        data_len
    };
    bpf_perf_event_output(
        ctx as *mut core::ffi::c_void,
        &mut events as *mut _ as *mut core::ffi::c_void,
        BPF_F_CURRENT_CPU,
        sysctl_data as *mut core::ffi::c_void,
        data_len,
    );
    bpf_stats_exit(&mut stats_ctx);
    0
}

// SEC("tracepoint/syscalls/sys_enter_kill")
pub unsafe extern "C" fn tracepoint__syscalls__sys_enter_kill(
    ctx: *mut syscall_trace_enter,
) -> i32 {
    let mut stats_ctx: bpf_func_stats_ctx = core::mem::zeroed();

    bpf_stats_enter(&mut stats_ctx, profiler_bpf_sys_enter_kill);
    let pid = (*ctx).args[0] as i32;
    let sig = (*ctx).args[1] as i32;
    let ret = trace_var_sys_kill(ctx as *mut core::ffi::c_void, pid, sig);
    bpf_stats_exit(&mut stats_ctx);
    ret
}

// SEC("raw_tracepoint/sched_process_exit")
pub unsafe extern "C" fn raw_tracepoint__sched_process_exit(ctx: *mut core::ffi::c_void) -> i32 {
    let mut zero: i32 = 0;
    let mut stats_ctx: bpf_func_stats_ctx = core::mem::zeroed();
    bpf_stats_enter(&mut stats_ctx, profiler_bpf_sched_process_exit);

    let tpid = get_userspace_pid();

    let arr_struct = bpf_map_lookup_elem(
        &mut var_tpid_to_data as *mut _ as *mut core::ffi::c_void,
        &tpid as *const _ as *mut core::ffi::c_void,
    ) as *mut var_kill_data_arr_t;
    let kill_data = bpf_map_lookup_elem(
        &mut data_heap as *mut _ as *mut core::ffi::c_void,
        &mut zero as *mut _ as *mut core::ffi::c_void,
    ) as *mut var_kill_data_t;

    if arr_struct.is_null() || kill_data.is_null() {
        bpf_stats_exit(&mut stats_ctx);
        return 0;
    }

    let task = bpf_get_current_task() as *mut task_struct;
    let proc_kernfs: *mut kernfs_node = BPF_CORE_READ!(task, cgroups, dfl_cgrp, kn);

    // If UNROLL is defined in C, this loop is pragma-unrolled.
    let mut i = 0usize;
    while i < KILL_DATA_ARRAY_SIZE {
        let past_kill_data = &mut (*arr_struct).array[i] as *mut var_kill_data_t;

        if !past_kill_data.is_null() && (*past_kill_data).kill_target_pid == tpid as pid_t {
            bpf_probe_read_kernel(
                kill_data as *mut core::ffi::c_void,
                core::mem::size_of_val(&*past_kill_data),
                past_kill_data as *mut core::ffi::c_void,
            );
            let mut payload = (*kill_data).payload.as_mut_ptr() as *mut core::ffi::c_void;
            let offset = (*kill_data).payload_length;
            if offset >= MAX_METADATA_PAYLOAD_LEN + MAX_CGROUP_PAYLOAD_LEN {
                return 0;
            }
            payload = payload.add(offset);

            (*kill_data).kill_target_name_length = 0;
            (*kill_data).kill_target_cgroup_proc_length = 0;

            let comm_length =
                bpf_core_read_str(payload, TASK_COMM_LEN as usize, &mut (*task).comm) as usize;
            if bpf_cmp_likely(comm_length, BPF_CMP_LE, TASK_COMM_LEN as usize) {
                (*kill_data).kill_target_name_length = comm_length;
                payload = payload.add(comm_length);
            }

            let cgroup_proc_length = bpf_probe_read_kernel_str(
                payload,
                KILL_TARGET_LEN as usize,
                BPF_CORE_READ!(proc_kernfs, name),
            ) as usize;
            if bpf_cmp_likely(cgroup_proc_length, BPF_CMP_LE, KILL_TARGET_LEN as usize) {
                (*kill_data).kill_target_cgroup_proc_length = cgroup_proc_length;
                payload = payload.add(cgroup_proc_length);
            }

            bpf_stats_pre_submit_var_perf_event(&mut stats_ctx, &mut (*kill_data).meta);
            let mut data_len = payload.offset_from(kill_data as *mut core::ffi::c_void) as usize;
            data_len = if data_len > core::mem::size_of::<var_kill_data_t>() {
                core::mem::size_of::<var_kill_data_t>()
            } else {
                data_len
            };
            bpf_perf_event_output(
                ctx,
                &mut events as *mut _ as *mut core::ffi::c_void,
                BPF_F_CURRENT_CPU,
                kill_data as *mut core::ffi::c_void,
                data_len,
            );
        }
        i += 1;
    }
    bpf_map_delete_elem(
        &mut var_tpid_to_data as *mut _ as *mut core::ffi::c_void,
        &tpid as *const _ as *mut core::ffi::c_void,
    );
    bpf_stats_exit(&mut stats_ctx);
    0
}

// SEC("raw_tracepoint/sched_process_exec")
pub unsafe extern "C" fn raw_tracepoint__sched_process_exec(
    ctx: *mut bpf_raw_tracepoint_args,
) -> i32 {
    let mut stats_ctx: bpf_func_stats_ctx = core::mem::zeroed();
    bpf_stats_enter(&mut stats_ctx, profiler_bpf_sched_process_exec);

    let bprm = (*ctx).args[2] as *mut linux_binprm;
    let inode: u64 = BPF_CORE_READ!(bprm, file, f_inode, i_ino);

    let should_filter_binprm = bpf_map_lookup_elem(
        &mut disallowed_exec_inodes as *mut _ as *mut core::ffi::c_void,
        &inode as *const _ as *mut core::ffi::c_void,
    ) as *mut bool;
    if !should_filter_binprm.is_null() {
        bpf_stats_exit(&mut stats_ctx);
        return 0;
    }

    let mut zero: i32 = 0;
    let proc_exec_data = bpf_map_lookup_elem(
        &mut data_heap as *mut _ as *mut core::ffi::c_void,
        &mut zero as *mut _ as *mut core::ffi::c_void,
    ) as *mut var_exec_data_t;
    if proc_exec_data.is_null() {
        bpf_stats_exit(&mut stats_ctx);
        return 0;
    }

    if INODE_FILTER!() != 0 && inode != INODE_FILTER!() {
        return 0;
    }

    let pid = get_userspace_pid();
    let task = bpf_get_current_task() as *mut task_struct;

    (*proc_exec_data).meta.r#type = EXEC_EVENT;
    (*proc_exec_data).bin_path_length = 0;
    (*proc_exec_data).cmdline_length = 0;
    (*proc_exec_data).environment_length = 0;
    let mut payload = populate_var_metadata(
        &mut (*proc_exec_data).meta,
        task,
        pid,
        (*proc_exec_data).payload.as_mut_ptr() as *mut _,
    );
    payload = populate_cgroup_info(&mut (*proc_exec_data).cgroup_data, task, payload);

    let parent_task: *mut task_struct = BPF_CORE_READ!(task, real_parent);
    (*proc_exec_data).parent_pid = BPF_CORE_READ!(parent_task, tgid);
    (*proc_exec_data).parent_uid = BPF_CORE_READ!(parent_task, real_cred, uid.val);
    (*proc_exec_data).parent_exec_id = BPF_CORE_READ!(parent_task, self_exec_id);
    (*proc_exec_data).parent_start_time = BPF_CORE_READ!(parent_task, start_time);

    let filename: *const core::ffi::c_char = BPF_CORE_READ!(bprm, filename);
    let bin_path_length =
        bpf_probe_read_kernel_str(payload, MAX_FILENAME_LEN as usize, filename as *const _) as usize;
    if bpf_cmp_likely(bin_path_length, BPF_CMP_LE, MAX_FILENAME_LEN as usize) {
        (*proc_exec_data).bin_path_length = bin_path_length;
        payload = payload.add(bin_path_length);
    }

    let arg_start = BPF_CORE_READ!(task, mm, arg_start) as *mut core::ffi::c_void;
    let arg_end = BPF_CORE_READ!(task, mm, arg_end) as *mut core::ffi::c_void;
    let cmdline_length = probe_read_lim(
        payload,
        arg_start,
        arg_end.offset_from(arg_start) as core::ffi::c_ulong,
        MAX_ARGS_LEN as core::ffi::c_ulong,
    ) as u32;

    if bpf_cmp_likely(cmdline_length as usize, BPF_CMP_LE, MAX_ARGS_LEN as usize) {
        (*proc_exec_data).cmdline_length = cmdline_length;
        payload = payload.add(cmdline_length as usize);
    }

    if READ_ENVIRON_FROM_EXEC!() {
        let env_start = BPF_CORE_READ!(task, mm, env_start) as *mut core::ffi::c_void;
        let env_end = BPF_CORE_READ!(task, mm, env_end) as *mut core::ffi::c_void;
        let env_len = probe_read_lim(
            payload,
            env_start,
            env_end.offset_from(env_start) as core::ffi::c_ulong,
            MAX_ENVIRON_LEN as core::ffi::c_ulong,
        );
        if cmdline_length as usize <= MAX_ENVIRON_LEN {
            (*proc_exec_data).environment_length = env_len as usize;
            payload = payload.add(env_len as usize);
        }
    }

    bpf_stats_pre_submit_var_perf_event(&mut stats_ctx, &mut (*proc_exec_data).meta);
    let mut data_len = payload.offset_from(proc_exec_data as *mut core::ffi::c_void) as usize;
    data_len = if data_len > core::mem::size_of::<var_exec_data_t>() {
        core::mem::size_of::<var_exec_data_t>()
    } else {
        data_len
    };
    bpf_perf_event_output(
        ctx as *mut core::ffi::c_void,
        &mut events as *mut _ as *mut core::ffi::c_void,
        BPF_F_CURRENT_CPU,
        proc_exec_data as *mut core::ffi::c_void,
        data_len,
    );
    bpf_stats_exit(&mut stats_ctx);
    0
}

// SEC("kretprobe/do_file_open")
pub unsafe extern "C" fn kprobe_ret__do_file_open(ctx: *mut pt_regs) -> i32 {
    let mut stats_ctx: bpf_func_stats_ctx = core::mem::zeroed();
    bpf_stats_enter(&mut stats_ctx, profiler_bpf_do_file_open_ret);

    let filp = PT_REGS_RC_CORE(ctx) as *mut file;

    if filp.is_null() || IS_ERR(filp as *const core::ffi::c_void) {
        bpf_stats_exit(&mut stats_ctx);
        return 0;
    }
    let flags: u32 = BPF_CORE_READ!(filp, f_flags);
    if (flags & (O_RDWR | O_WRONLY)) == 0 {
        bpf_stats_exit(&mut stats_ctx);
        return 0;
    }
    if (flags & O_TMPFILE) > 0 {
        bpf_stats_exit(&mut stats_ctx);
        return 0;
    }
    let file_inode: *mut inode = BPF_CORE_READ!(filp, f_inode);
    let mode: umode_t = BPF_CORE_READ!(file_inode, i_mode);
    if S_ISDIR(mode as u32)
        || S_ISCHR(mode as u32)
        || S_ISBLK(mode as u32)
        || S_ISFIFO(mode as u32)
        || S_ISSOCK(mode as u32)
    {
        bpf_stats_exit(&mut stats_ctx);
        return 0;
    }

    let filp_dentry: *mut dentry = BPF_CORE_READ!(filp, f_path.dentry);
    let mut device_id: u32 = 0;
    let mut file_ino: u64 = 0;
    if !is_dentry_allowed_for_filemod(filp_dentry, &mut device_id, &mut file_ino) {
        bpf_stats_exit(&mut stats_ctx);
        return 0;
    }

    let mut zero: i32 = 0;
    let filemod_data = bpf_map_lookup_elem(
        &mut data_heap as *mut _ as *mut core::ffi::c_void,
        &mut zero as *mut _ as *mut core::ffi::c_void,
    ) as *mut var_filemod_data_t;
    if filemod_data.is_null() {
        bpf_stats_exit(&mut stats_ctx);
        return 0;
    }

    let pid = get_userspace_pid();
    let task = bpf_get_current_task() as *mut task_struct;

    (*filemod_data).meta.r#type = FILEMOD_EVENT;
    (*filemod_data).fmod_type = FMOD_OPEN;
    (*filemod_data).dst_flags = flags;
    (*filemod_data).src_inode = 0;
    (*filemod_data).dst_inode = file_ino;
    (*filemod_data).src_device_id = 0;
    (*filemod_data).dst_device_id = device_id;
    (*filemod_data).src_filepath_length = 0;
    (*filemod_data).dst_filepath_length = 0;

    let mut payload = populate_var_metadata(
        &mut (*filemod_data).meta,
        task,
        pid,
        (*filemod_data).payload.as_mut_ptr() as *mut _,
    );
    payload = populate_cgroup_info(&mut (*filemod_data).cgroup_data, task, payload);

    let len = read_absolute_file_path_from_dentry(filp_dentry, payload);
    if bpf_cmp_likely(len, BPF_CMP_LE, MAX_FILEPATH_LENGTH as usize) {
        payload = payload.add(len);
        (*filemod_data).dst_filepath_length = len;
    }
    bpf_stats_pre_submit_var_perf_event(&mut stats_ctx, &mut (*filemod_data).meta);
    let mut data_len = payload.offset_from(filemod_data as *mut core::ffi::c_void) as usize;
    data_len = if data_len > core::mem::size_of::<var_filemod_data_t>() {
        core::mem::size_of::<var_filemod_data_t>()
    } else {
        data_len
    };
    bpf_perf_event_output(
        ctx as *mut core::ffi::c_void,
        &mut events as *mut _ as *mut core::ffi::c_void,
        BPF_F_CURRENT_CPU,
        filemod_data as *mut core::ffi::c_void,
        data_len,
    );
    bpf_stats_exit(&mut stats_ctx);
    0
}

// SEC("kprobe/vfs_link")
pub unsafe extern "C" fn kprobe__vfs_link(
    ctx: *mut pt_regs,
    old_dentry: *mut dentry,
    idmap: *mut mnt_idmap,
    dir: *mut inode,
    new_dentry: *mut dentry,
    delegated_inode: *mut *mut inode,
) -> i32 {
    let mut stats_ctx: bpf_func_stats_ctx = core::mem::zeroed();
    bpf_stats_enter(&mut stats_ctx, profiler_bpf_vfs_link);

    let mut src_device_id: u32 = 0;
    let mut src_file_ino: u64 = 0;
    let mut dst_device_id: u32 = 0;
    let mut dst_file_ino: u64 = 0;
    if !is_dentry_allowed_for_filemod(old_dentry, &mut src_device_id, &mut src_file_ino)
        && !is_dentry_allowed_for_filemod(new_dentry, &mut dst_device_id, &mut dst_file_ino)
    {
        bpf_stats_exit(&mut stats_ctx);
        return 0;
    }

    let mut zero: i32 = 0;
    let filemod_data = bpf_map_lookup_elem(
        &mut data_heap as *mut _ as *mut core::ffi::c_void,
        &mut zero as *mut _ as *mut core::ffi::c_void,
    ) as *mut var_filemod_data_t;
    if filemod_data.is_null() {
        bpf_stats_exit(&mut stats_ctx);
        return 0;
    }

    let pid = get_userspace_pid();
    let task = bpf_get_current_task() as *mut task_struct;

    (*filemod_data).meta.r#type = FILEMOD_EVENT;
    (*filemod_data).fmod_type = FMOD_LINK;
    (*filemod_data).dst_flags = 0;
    (*filemod_data).src_inode = src_file_ino;
    (*filemod_data).dst_inode = dst_file_ino;
    (*filemod_data).src_device_id = src_device_id;
    (*filemod_data).dst_device_id = dst_device_id;
    (*filemod_data).src_filepath_length = 0;
    (*filemod_data).dst_filepath_length = 0;

    let mut payload = populate_var_metadata(
        &mut (*filemod_data).meta,
        task,
        pid,
        (*filemod_data).payload.as_mut_ptr() as *mut _,
    );
    payload = populate_cgroup_info(&mut (*filemod_data).cgroup_data, task, payload);

    let mut len = read_absolute_file_path_from_dentry(old_dentry, payload);
    if bpf_cmp_likely(len, BPF_CMP_LE, MAX_FILEPATH_LENGTH as usize) {
        payload = payload.add(len);
        (*filemod_data).src_filepath_length = len;
    }

    len = read_absolute_file_path_from_dentry(new_dentry, payload);
    if bpf_cmp_likely(len, BPF_CMP_LE, MAX_FILEPATH_LENGTH as usize) {
        payload = payload.add(len);
        (*filemod_data).dst_filepath_length = len;
    }

    bpf_stats_pre_submit_var_perf_event(&mut stats_ctx, &mut (*filemod_data).meta);
    let mut data_len = payload.offset_from(filemod_data as *mut core::ffi::c_void) as usize;
    data_len = if data_len > core::mem::size_of::<var_filemod_data_t>() {
        core::mem::size_of::<var_filemod_data_t>()
    } else {
        data_len
    };
    bpf_perf_event_output(
        ctx as *mut core::ffi::c_void,
        &mut events as *mut _ as *mut core::ffi::c_void,
        BPF_F_CURRENT_CPU,
        filemod_data as *mut core::ffi::c_void,
        data_len,
    );
    bpf_stats_exit(&mut stats_ctx);
    0
}

// SEC("kprobe/vfs_symlink")
pub unsafe extern "C" fn kprobe__vfs_symlink(
    ctx: *mut pt_regs,
    dir: *mut inode,
    dentry: *mut dentry,
    oldname: *const core::ffi::c_char,
) -> i32 {
    let mut stats_ctx: bpf_func_stats_ctx = core::mem::zeroed();
    bpf_stats_enter(&mut stats_ctx, profiler_bpf_vfs_symlink);

    let mut dst_device_id: u32 = 0;
    let mut dst_file_ino: u64 = 0;
    if !is_dentry_allowed_for_filemod(dentry, &mut dst_device_id, &mut dst_file_ino) {
        bpf_stats_exit(&mut stats_ctx);
        return 0;
    }

    let mut zero: i32 = 0;
    let filemod_data = bpf_map_lookup_elem(
        &mut data_heap as *mut _ as *mut core::ffi::c_void,
        &mut zero as *mut _ as *mut core::ffi::c_void,
    ) as *mut var_filemod_data_t;
    if filemod_data.is_null() {
        bpf_stats_exit(&mut stats_ctx);
        return 0;
    }

    let pid = get_userspace_pid();
    let task = bpf_get_current_task() as *mut task_struct;

    (*filemod_data).meta.r#type = FILEMOD_EVENT;
    (*filemod_data).fmod_type = FMOD_SYMLINK;
    (*filemod_data).dst_flags = 0;
    (*filemod_data).src_inode = 0;
    (*filemod_data).dst_inode = dst_file_ino;
    (*filemod_data).src_device_id = 0;
    (*filemod_data).dst_device_id = dst_device_id;
    (*filemod_data).src_filepath_length = 0;
    (*filemod_data).dst_filepath_length = 0;

    let mut payload = populate_var_metadata(
        &mut (*filemod_data).meta,
        task,
        pid,
        (*filemod_data).payload.as_mut_ptr() as *mut _,
    );
    payload = populate_cgroup_info(&mut (*filemod_data).cgroup_data, task, payload);

    let mut len =
        bpf_probe_read_kernel_str(payload, MAX_FILEPATH_LENGTH as usize, oldname as *const _) as usize;
    if bpf_cmp_likely(len, BPF_CMP_LE, MAX_FILEPATH_LENGTH as usize) {
        payload = payload.add(len);
        (*filemod_data).src_filepath_length = len;
    }
    len = read_absolute_file_path_from_dentry(dentry, payload);
    if bpf_cmp_likely(len, BPF_CMP_LE, MAX_FILEPATH_LENGTH as usize) {
        payload = payload.add(len);
        (*filemod_data).dst_filepath_length = len;
    }
    bpf_stats_pre_submit_var_perf_event(&mut stats_ctx, &mut (*filemod_data).meta);
    let mut data_len = payload.offset_from(filemod_data as *mut core::ffi::c_void) as usize;
    data_len = if data_len > core::mem::size_of::<var_filemod_data_t>() {
        core::mem::size_of::<var_filemod_data_t>()
    } else {
        data_len
    };
    bpf_perf_event_output(
        ctx as *mut core::ffi::c_void,
        &mut events as *mut _ as *mut core::ffi::c_void,
        BPF_F_CURRENT_CPU,
        filemod_data as *mut core::ffi::c_void,
        data_len,
    );
    bpf_stats_exit(&mut stats_ctx);
    0
}

// SEC("raw_tracepoint/sched_process_fork")
pub unsafe extern "C" fn raw_tracepoint__sched_process_fork(
    ctx: *mut bpf_raw_tracepoint_args,
) -> i32 {
    let mut stats_ctx: bpf_func_stats_ctx = core::mem::zeroed();
    bpf_stats_enter(&mut stats_ctx, profiler_bpf_sched_process_fork);

    let mut zero: i32 = 0;
    let fork_data = bpf_map_lookup_elem(
        &mut data_heap as *mut _ as *mut core::ffi::c_void,
        &mut zero as *mut _ as *mut core::ffi::c_void,
    ) as *mut var_fork_data_t;
    if fork_data.is_null() {
        bpf_stats_exit(&mut stats_ctx);
        return 0;
    }

    let parent = (*ctx).args[0] as *mut task_struct;
    let child = (*ctx).args[1] as *mut task_struct;
    (*fork_data).meta.r#type = FORK_EVENT;

    let payload = populate_var_metadata(
        &mut (*fork_data).meta,
        child,
        BPF_CORE_READ!(child, pid),
        (*fork_data).payload.as_mut_ptr() as *mut _,
    );
    (*fork_data).parent_pid = BPF_CORE_READ!(parent, pid);
    (*fork_data).parent_exec_id = BPF_CORE_READ!(parent, self_exec_id);
    (*fork_data).parent_start_time = BPF_CORE_READ!(parent, start_time);
    bpf_stats_pre_submit_var_perf_event(&mut stats_ctx, &mut (*fork_data).meta);

    let mut data_len = payload.offset_from(fork_data as *mut core::ffi::c_void) as usize;
    data_len = if data_len > core::mem::size_of::<var_fork_data_t>() {
        core::mem::size_of::<var_fork_data_t>()
    } else {
        data_len
    };
    bpf_perf_event_output(
        ctx as *mut core::ffi::c_void,
        &mut events as *mut _ as *mut core::ffi::c_void,
        BPF_F_CURRENT_CPU,
        fork_data as *mut core::ffi::c_void,
        data_len,
    );
    bpf_stats_exit(&mut stats_ctx);
    0
}

// char _license[] SEC("license") = "GPL";
#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
