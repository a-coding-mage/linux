// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */

// C header guard / #pragma once omitted in Rust.
// External C typedefs used by this header are expected from surrounding bindings:
// pid_t, uid_t, gid_t, ino_t.

pub const TASK_COMM_LEN: usize = 16;
pub const MAX_ANCESTORS: usize = 4;
pub const MAX_PATH: usize = 256;
pub const KILL_TARGET_LEN: usize = 64;
pub const CTL_MAXNAME: usize = 10;
pub const MAX_ARGS_LEN: usize = 4096;
pub const MAX_FILENAME_LEN: usize = 512;
pub const MAX_ENVIRON_LEN: usize = 8192;
pub const MAX_PATH_DEPTH: usize = 32;
pub const MAX_FILEPATH_LENGTH: usize = MAX_PATH_DEPTH * MAX_PATH;
pub const MAX_CGROUPS_PATH_DEPTH: usize = 8;

pub const MAX_METADATA_PAYLOAD_LEN: usize = TASK_COMM_LEN;

pub const MAX_CGROUP_PAYLOAD_LEN: usize =
    MAX_PATH * 2 + (MAX_PATH * MAX_CGROUPS_PATH_DEPTH);

pub const MAX_CAP_PAYLOAD_LEN: usize = MAX_METADATA_PAYLOAD_LEN + MAX_CGROUP_PAYLOAD_LEN;

pub const MAX_SYSCTL_PAYLOAD_LEN: usize =
    MAX_METADATA_PAYLOAD_LEN + MAX_CGROUP_PAYLOAD_LEN + CTL_MAXNAME + MAX_PATH;

pub const MAX_KILL_PAYLOAD_LEN: usize =
    MAX_METADATA_PAYLOAD_LEN + MAX_CGROUP_PAYLOAD_LEN + TASK_COMM_LEN + KILL_TARGET_LEN;

pub const MAX_EXEC_PAYLOAD_LEN: usize =
    MAX_METADATA_PAYLOAD_LEN + MAX_CGROUP_PAYLOAD_LEN + MAX_FILENAME_LEN + MAX_ARGS_LEN
        + MAX_ENVIRON_LEN;

pub const MAX_FILEMOD_PAYLOAD_LEN: usize =
    MAX_METADATA_PAYLOAD_LEN + MAX_CGROUP_PAYLOAD_LEN + MAX_FILEPATH_LENGTH
        + MAX_FILEPATH_LENGTH;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum data_type {
    INVALID_EVENT,
    EXEC_EVENT,
    FORK_EVENT,
    KILL_EVENT,
    SYSCTL_EVENT,
    FILEMOD_EVENT,
    MAX_DATA_TYPE_EVENT,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum filemod_type {
    FMOD_OPEN,
    FMOD_LINK,
    FMOD_SYMLINK,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ancestors_data_t {
    pub ancestor_pids: [pid_t; MAX_ANCESTORS],
    pub ancestor_exec_ids: [u32; MAX_ANCESTORS],
    pub ancestor_start_times: [u64; MAX_ANCESTORS],
    pub num_ancestors: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct var_metadata_t {
    pub type_: data_type,
    pub pid: pid_t,
    pub exec_id: u32,
    pub uid: uid_t,
    pub gid: gid_t,
    pub start_time: u64,
    pub cpu_id: u32,
    pub bpf_stats_num_perf_events: u64,
    pub bpf_stats_start_ktime_ns: u64,
    pub comm_length: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgroup_data_t {
    pub cgroup_root_inode: ino_t,
    pub cgroup_proc_inode: ino_t,
    pub cgroup_root_mtime: u64,
    pub cgroup_proc_mtime: u64,
    pub cgroup_root_length: u16,
    pub cgroup_proc_length: u16,
    pub cgroup_full_length: u16,
    pub cgroup_full_path_root_pos: ::core::ffi::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct var_sysctl_data_t {
    pub meta: var_metadata_t,
    pub cgroup_data: cgroup_data_t,
    pub ancestors_info: ancestors_data_t,
    pub sysctl_val_length: u8,
    pub sysctl_path_length: u16,
    pub payload: [::core::ffi::c_char; MAX_SYSCTL_PAYLOAD_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct var_kill_data_t {
    pub meta: var_metadata_t,
    pub cgroup_data: cgroup_data_t,
    pub ancestors_info: ancestors_data_t,
    pub kill_target_pid: pid_t,
    pub kill_sig: ::core::ffi::c_int,
    pub kill_count: u32,
    pub last_kill_time: u64,
    pub kill_target_name_length: u8,
    pub kill_target_cgroup_proc_length: u8,
    pub payload: [::core::ffi::c_char; MAX_KILL_PAYLOAD_LEN],
    pub payload_length: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct var_exec_data_t {
    pub meta: var_metadata_t,
    pub cgroup_data: cgroup_data_t,
    pub parent_pid: pid_t,
    pub parent_exec_id: u32,
    pub parent_uid: uid_t,
    pub parent_start_time: u64,
    pub bin_path_length: u16,
    pub cmdline_length: u16,
    pub environment_length: u16,
    pub payload: [::core::ffi::c_char; MAX_EXEC_PAYLOAD_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct var_fork_data_t {
    pub meta: var_metadata_t,
    pub parent_pid: pid_t,
    pub parent_exec_id: u32,
    pub parent_start_time: u64,
    pub payload: [::core::ffi::c_char; MAX_METADATA_PAYLOAD_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct var_filemod_data_t {
    pub meta: var_metadata_t,
    pub cgroup_data: cgroup_data_t,
    pub fmod_type: filemod_type,
    pub dst_flags: ::core::ffi::c_uint,
    pub src_device_id: u32,
    pub dst_device_id: u32,
    pub src_inode: ino_t,
    pub dst_inode: ino_t,
    pub src_filepath_length: u16,
    pub dst_filepath_length: u16,
    pub payload: [::core::ffi::c_char; MAX_FILEMOD_PAYLOAD_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct profiler_config_struct {
    pub fetch_cgroups_from_bpf: bool,
    pub cgroup_fs_inode: ino_t,
    pub cgroup_login_session_inode: ino_t,
    pub kill_signals_mask: u64,
    pub inode_filter: ino_t,
    pub stale_info_secs: u32,
    pub use_variable_buffers: bool,
    pub read_environ_from_exec: bool,
    pub enable_cgroup_v1_resolver: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_func_stats_data {
    pub time_elapsed_ns: u64,
    pub num_executions: u64,
    pub num_perf_events: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_func_stats_ctx {
    pub start_time_ns: u64,
    pub bpf_func_stats_data_val: *mut bpf_func_stats_data,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum bpf_function_id {
    profiler_bpf_proc_sys_write,
    profiler_bpf_sched_process_exec,
    profiler_bpf_sched_process_exit,
    profiler_bpf_sys_enter_kill,
    profiler_bpf_do_file_open_ret,
    profiler_bpf_sched_process_fork,
    profiler_bpf_vfs_link,
    profiler_bpf_vfs_symlink,
    profiler_bpf_max_function_id,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
