/* SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause) */

/*
 * Common BPF ELF operations.
 *
 * Copyright (C) 2013-2015 Alexei Starovoitov <ast@kernel.org>
 * Copyright (C) 2015 Wang Nan <wangnan0@huawei.com>
 * Copyright (C) 2015 Huawei Inc.
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation;
 * version 2.1 of the License (not later!)
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this program; if not,  see <http://www.gnu.org/licenses>
 */

/* C includes translated as Rust dependency intent:
 * <linux/bpf.h>, <stdbool.h>, <stddef.h>, <stdint.h>,
 * "libbpf_common.h", and "libbpf_legacy.h".
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub type __u32 = u32;
pub type __s32 = i32;
pub type __u64 = u64;
pub type size_t = usize;

/* Enum and struct definitions are supplied by <linux/bpf.h>. */
pub type bpf_map_type = c_uint;
pub type bpf_prog_type = c_uint;
pub type bpf_attach_type = c_uint;
pub type bpf_stats_type = c_uint;

#[repr(C)]
pub struct bpf_insn {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_prog_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_btf_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link_info {
    _private: [u8; 0],
}

#[repr(C)]
pub union bpf_iter_link_info {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn libbpf_set_memlock_rlim(memlock_bytes: size_t) -> c_int;
}

#[repr(C)]
pub struct bpf_log_opts {
    pub sz: size_t,
    pub buf: *mut c_char,
    pub size: __u32,
    pub level: __u32,
    pub true_size: __u32,
}

pub const bpf_log_opts__last_field: &str = "true_size";

#[repr(C)]
pub struct bpf_map_create_opts {
    pub sz: size_t,
    pub btf_fd: __u32,
    pub btf_key_type_id: __u32,
    pub btf_value_type_id: __u32,
    pub btf_vmlinux_value_type_id: __u32,
    pub inner_map_fd: __u32,
    pub map_flags: __u32,
    pub map_extra: __u64,
    pub numa_node: __u32,
    pub map_ifindex: __u32,
    pub value_type_btf_obj_fd: __s32,
    pub token_fd: __u32,
    pub excl_prog_hash: *const c_void,
    pub excl_prog_hash_size: __u32,
    pub log_opts: *mut bpf_log_opts,
}

pub const bpf_map_create_opts__last_field: &str = "log_opts";

unsafe extern "C" {
    pub fn bpf_map_create(
        map_type: bpf_map_type,
        map_name: *const c_char,
        key_size: __u32,
        value_size: __u32,
        max_entries: __u32,
        opts: *const bpf_map_create_opts,
    ) -> c_int;
}

#[repr(C)]
pub struct bpf_prog_load_opts {
    pub sz: size_t,
    pub attempts: c_int,
    pub expected_attach_type: bpf_attach_type,
    pub prog_btf_fd: __u32,
    pub prog_flags: __u32,
    pub prog_ifindex: __u32,
    pub kern_version: __u32,
    pub attach_btf_id: __u32,
    pub attach_prog_fd: __u32,
    pub attach_btf_obj_fd: __u32,
    pub fd_array: *const c_int,
    pub func_info: *const c_void,
    pub func_info_cnt: __u32,
    pub func_info_rec_size: __u32,
    pub line_info: *const c_void,
    pub line_info_cnt: __u32,
    pub line_info_rec_size: __u32,
    pub log_level: __u32,
    pub log_size: __u32,
    pub log_buf: *mut c_char,
    pub log_true_size: __u32,
    pub token_fd: __u32,
    pub fd_array_cnt: __u32,
}

pub const bpf_prog_load_opts__last_field: &str = "fd_array_cnt";

unsafe extern "C" {
    pub fn bpf_prog_load(
        prog_type: bpf_prog_type,
        prog_name: *const c_char,
        license: *const c_char,
        insns: *const bpf_insn,
        insn_cnt: size_t,
        opts: *mut bpf_prog_load_opts,
    ) -> c_int;
}

pub const MAPS_RELAX_COMPAT: c_int = 0x01;
pub const BPF_LOG_BUF_SIZE: __u32 = u32::MAX >> 8;

#[repr(C)]
pub struct bpf_btf_load_opts {
    pub sz: size_t,
    pub log_buf: *mut c_char,
    pub log_level: __u32,
    pub log_size: __u32,
    pub log_true_size: __u32,
    pub btf_flags: __u32,
    pub token_fd: __u32,
}

pub const bpf_btf_load_opts__last_field: &str = "token_fd";

unsafe extern "C" {
    pub fn bpf_btf_load(
        btf_data: *const c_void,
        btf_size: size_t,
        opts: *mut bpf_btf_load_opts,
    ) -> c_int;
    pub fn bpf_map_update_elem(
        fd: c_int,
        key: *const c_void,
        value: *const c_void,
        flags: __u64,
    ) -> c_int;
    pub fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    pub fn bpf_map_lookup_elem_flags(
        fd: c_int,
        key: *const c_void,
        value: *mut c_void,
        flags: __u64,
    ) -> c_int;
    pub fn bpf_map_lookup_and_delete_elem(
        fd: c_int,
        key: *const c_void,
        value: *mut c_void,
    ) -> c_int;
    pub fn bpf_map_lookup_and_delete_elem_flags(
        fd: c_int,
        key: *const c_void,
        value: *mut c_void,
        flags: __u64,
    ) -> c_int;
    pub fn bpf_map_delete_elem(fd: c_int, key: *const c_void) -> c_int;
    pub fn bpf_map_delete_elem_flags(fd: c_int, key: *const c_void, flags: __u64) -> c_int;
    pub fn bpf_map_get_next_key(fd: c_int, key: *const c_void, next_key: *mut c_void) -> c_int;
    pub fn bpf_map_freeze(fd: c_int) -> c_int;
}

#[repr(C)]
pub struct bpf_map_batch_opts {
    pub sz: size_t,
    pub elem_flags: __u64,
    pub flags: __u64,
}

pub const bpf_map_batch_opts__last_field: &str = "flags";

unsafe extern "C" {
    pub fn bpf_map_delete_batch(
        fd: c_int,
        keys: *const c_void,
        count: *mut __u32,
        opts: *const bpf_map_batch_opts,
    ) -> c_int;
    pub fn bpf_map_lookup_batch(
        fd: c_int,
        in_batch: *mut c_void,
        out_batch: *mut c_void,
        keys: *mut c_void,
        values: *mut c_void,
        count: *mut __u32,
        opts: *const bpf_map_batch_opts,
    ) -> c_int;
    pub fn bpf_map_lookup_and_delete_batch(
        fd: c_int,
        in_batch: *mut c_void,
        out_batch: *mut c_void,
        keys: *mut c_void,
        values: *mut c_void,
        count: *mut __u32,
        opts: *const bpf_map_batch_opts,
    ) -> c_int;
    pub fn bpf_map_update_batch(
        fd: c_int,
        keys: *const c_void,
        values: *const c_void,
        count: *mut __u32,
        opts: *const bpf_map_batch_opts,
    ) -> c_int;
}

#[repr(C)]
pub struct bpf_obj_pin_opts {
    pub sz: size_t,
    pub file_flags: __u32,
    pub path_fd: c_int,
}

pub const bpf_obj_pin_opts__last_field: &str = "path_fd";

unsafe extern "C" {
    pub fn bpf_obj_pin(fd: c_int, pathname: *const c_char) -> c_int;
    pub fn bpf_obj_pin_opts(
        fd: c_int,
        pathname: *const c_char,
        opts: *const bpf_obj_pin_opts,
    ) -> c_int;
}

#[repr(C)]
pub struct bpf_obj_get_opts {
    pub sz: size_t,
    pub file_flags: __u32,
    pub path_fd: c_int,
}

pub const bpf_obj_get_opts__last_field: &str = "path_fd";

unsafe extern "C" {
    pub fn bpf_obj_get(pathname: *const c_char) -> c_int;
    pub fn bpf_obj_get_opts(pathname: *const c_char, opts: *const bpf_obj_get_opts) -> c_int;
    pub fn bpf_prog_attach(
        prog_fd: c_int,
        attachable_fd: c_int,
        type_: bpf_attach_type,
        flags: c_uint,
    ) -> c_int;
    pub fn bpf_prog_detach(attachable_fd: c_int, type_: bpf_attach_type) -> c_int;
    pub fn bpf_prog_detach2(prog_fd: c_int, attachable_fd: c_int, type_: bpf_attach_type) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union bpf_prog_attach_opts_replace {
    pub replace_prog_fd: c_int,
    pub replace_fd: c_int,
}

#[repr(C)]
pub struct bpf_prog_attach_opts {
    pub sz: size_t,
    pub flags: __u32,
    pub replace: bpf_prog_attach_opts_replace,
    pub relative_fd: c_int,
    pub relative_id: __u32,
    pub expected_revision: __u64,
}

pub const bpf_prog_attach_opts__last_field: &str = "expected_revision";

#[repr(C)]
pub struct bpf_prog_detach_opts {
    pub sz: size_t,
    pub flags: __u32,
    pub relative_fd: c_int,
    pub relative_id: __u32,
    pub expected_revision: __u64,
}

pub const bpf_prog_detach_opts__last_field: &str = "expected_revision";

unsafe extern "C" {
    pub fn bpf_prog_attach_opts(
        prog_fd: c_int,
        target: c_int,
        type_: bpf_attach_type,
        opts: *const bpf_prog_attach_opts,
    ) -> c_int;
    pub fn bpf_prog_detach_opts(
        prog_fd: c_int,
        target: c_int,
        type_: bpf_attach_type,
        opts: *const bpf_prog_detach_opts,
    ) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_link_create_opts_perf_event {
    pub bpf_cookie: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_link_create_opts_kprobe_multi {
    pub flags: __u32,
    pub cnt: __u32,
    pub syms: *const *const c_char,
    pub addrs: *const c_ulong,
    pub cookies: *const __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_link_create_opts_uprobe_multi {
    pub flags: __u32,
    pub cnt: __u32,
    pub path: *const c_char,
    pub offsets: *const c_ulong,
    pub ref_ctr_offsets: *const c_ulong,
    pub cookies: *const __u64,
    pub pid: __u32,
    pub path_fd: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_link_create_opts_tracing {
    pub cookie: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_link_create_opts_netfilter {
    pub pf: __u32,
    pub hooknum: __u32,
    pub priority: __s32,
    pub flags: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_link_create_opts_relative {
    pub relative_fd: __u32,
    pub relative_id: __u32,
    pub expected_revision: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_link_create_opts_tracing_multi {
    pub ids: *const __u32,
    pub cookies: *const __u64,
    pub cnt: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union bpf_link_create_opts_union {
    pub perf_event: bpf_link_create_opts_perf_event,
    pub kprobe_multi: bpf_link_create_opts_kprobe_multi,
    pub uprobe_multi: bpf_link_create_opts_uprobe_multi,
    pub tracing: bpf_link_create_opts_tracing,
    pub netfilter: bpf_link_create_opts_netfilter,
    pub tcx: bpf_link_create_opts_relative,
    pub netkit: bpf_link_create_opts_relative,
    pub cgroup: bpf_link_create_opts_relative,
    pub tracing_multi: bpf_link_create_opts_tracing_multi,
}

#[repr(C)]
pub struct bpf_link_create_opts {
    pub sz: size_t,
    pub flags: __u32,
    pub iter_info: *mut bpf_iter_link_info,
    pub iter_info_len: __u32,
    pub target_btf_id: __u32,
    pub anon: bpf_link_create_opts_union,
}

pub const bpf_link_create_opts__last_field: &str = "uprobe_multi.path_fd";

unsafe extern "C" {
    pub fn bpf_link_create(
        prog_fd: c_int,
        target_fd: c_int,
        attach_type: bpf_attach_type,
        opts: *const bpf_link_create_opts,
    ) -> c_int;
    pub fn bpf_link_detach(link_fd: c_int) -> c_int;
}

#[repr(C)]
pub struct bpf_link_update_opts {
    pub sz: size_t,
    pub flags: __u32,
    pub old_prog_fd: __u32,
    pub old_map_fd: __u32,
}

pub const bpf_link_update_opts__last_field: &str = "old_map_fd";

unsafe extern "C" {
    pub fn bpf_link_update(
        link_fd: c_int,
        new_prog_fd: c_int,
        opts: *const bpf_link_update_opts,
    ) -> c_int;
    pub fn bpf_iter_create(link_fd: c_int) -> c_int;
}

#[repr(C)]
pub struct bpf_prog_test_run_attr {
    pub prog_fd: c_int,
    pub repeat: c_int,
    pub data_in: *const c_void,
    pub data_size_in: __u32,
    pub data_out: *mut c_void,
    pub data_size_out: __u32,
    pub retval: __u32,
    pub duration: __u32,
    pub ctx_in: *const c_void,
    pub ctx_size_in: __u32,
    pub ctx_out: *mut c_void,
    pub ctx_size_out: __u32,
}

unsafe extern "C" {
    pub fn bpf_prog_get_next_id(start_id: __u32, next_id: *mut __u32) -> c_int;
    pub fn bpf_map_get_next_id(start_id: __u32, next_id: *mut __u32) -> c_int;
    pub fn bpf_btf_get_next_id(start_id: __u32, next_id: *mut __u32) -> c_int;
    pub fn bpf_link_get_next_id(start_id: __u32, next_id: *mut __u32) -> c_int;
}

#[repr(C)]
pub struct bpf_get_fd_by_id_opts {
    pub sz: size_t,
    pub open_flags: __u32,
    pub token_fd: __u32,
}

pub const bpf_get_fd_by_id_opts__last_field: &str = "token_fd";

unsafe extern "C" {
    pub fn bpf_prog_get_fd_by_id(id: __u32) -> c_int;
    pub fn bpf_prog_get_fd_by_id_opts(id: __u32, opts: *const bpf_get_fd_by_id_opts) -> c_int;
    pub fn bpf_map_get_fd_by_id(id: __u32) -> c_int;
    pub fn bpf_map_get_fd_by_id_opts(id: __u32, opts: *const bpf_get_fd_by_id_opts) -> c_int;
    pub fn bpf_btf_get_fd_by_id(id: __u32) -> c_int;
    pub fn bpf_btf_get_fd_by_id_opts(id: __u32, opts: *const bpf_get_fd_by_id_opts) -> c_int;
    pub fn bpf_link_get_fd_by_id(id: __u32) -> c_int;
    pub fn bpf_link_get_fd_by_id_opts(id: __u32, opts: *const bpf_get_fd_by_id_opts) -> c_int;
    pub fn bpf_obj_get_info_by_fd(bpf_fd: c_int, info: *mut c_void, info_len: *mut __u32)
        -> c_int;
    pub fn bpf_prog_get_info_by_fd(
        prog_fd: c_int,
        info: *mut bpf_prog_info,
        info_len: *mut __u32,
    ) -> c_int;
    pub fn bpf_map_get_info_by_fd(
        map_fd: c_int,
        info: *mut bpf_map_info,
        info_len: *mut __u32,
    ) -> c_int;
    pub fn bpf_btf_get_info_by_fd(
        btf_fd: c_int,
        info: *mut bpf_btf_info,
        info_len: *mut __u32,
    ) -> c_int;
    pub fn bpf_link_get_info_by_fd(
        link_fd: c_int,
        info: *mut bpf_link_info,
        info_len: *mut __u32,
    ) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union bpf_prog_query_opts_count {
    pub prog_cnt: __u32,
    pub count: __u32,
}

#[repr(C)]
pub struct bpf_prog_query_opts {
    pub sz: size_t,
    pub query_flags: __u32,
    pub attach_flags: __u32,
    pub prog_ids: *mut __u32,
    pub cnt: bpf_prog_query_opts_count,
    pub prog_attach_flags: *mut __u32,
    pub link_ids: *mut __u32,
    pub link_attach_flags: *mut __u32,
    pub revision: __u64,
}

pub const bpf_prog_query_opts__last_field: &str = "revision";

unsafe extern "C" {
    pub fn bpf_prog_query_opts(
        target: c_int,
        type_: bpf_attach_type,
        opts: *mut bpf_prog_query_opts,
    ) -> c_int;
    pub fn bpf_prog_query(
        target_fd: c_int,
        type_: bpf_attach_type,
        query_flags: __u32,
        attach_flags: *mut __u32,
        prog_ids: *mut __u32,
        prog_cnt: *mut __u32,
    ) -> c_int;
}

#[repr(C)]
pub struct bpf_raw_tp_opts {
    pub sz: size_t,
    pub tp_name: *const c_char,
    pub cookie: __u64,
}

pub const bpf_raw_tp_opts__last_field: &str = "cookie";

unsafe extern "C" {
    pub fn bpf_raw_tracepoint_open_opts(prog_fd: c_int, opts: *mut bpf_raw_tp_opts) -> c_int;
    pub fn bpf_raw_tracepoint_open(name: *const c_char, prog_fd: c_int) -> c_int;
    pub fn bpf_task_fd_query(
        pid: c_int,
        fd: c_int,
        flags: __u32,
        buf: *mut c_char,
        buf_len: *mut __u32,
        prog_id: *mut __u32,
        fd_type: *mut __u32,
        probe_offset: *mut __u64,
        probe_addr: *mut __u64,
    ) -> c_int;
    pub fn bpf_enable_stats(type_: bpf_stats_type) -> c_int;
}

#[repr(C)]
pub struct bpf_prog_bind_opts {
    pub sz: size_t,
    pub flags: __u32,
}

pub const bpf_prog_bind_opts__last_field: &str = "flags";

unsafe extern "C" {
    pub fn bpf_prog_bind_map(
        prog_fd: c_int,
        map_fd: c_int,
        opts: *const bpf_prog_bind_opts,
    ) -> c_int;
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub sz: size_t,
    pub data_in: *const c_void,
    pub data_out: *mut c_void,
    pub data_size_in: __u32,
    pub data_size_out: __u32,
    pub ctx_in: *const c_void,
    pub ctx_out: *mut c_void,
    pub ctx_size_in: __u32,
    pub ctx_size_out: __u32,
    pub retval: __u32,
    pub repeat: c_int,
    pub duration: __u32,
    pub flags: __u32,
    pub cpu: __u32,
    pub batch_size: __u32,
}

pub const bpf_test_run_opts__last_field: &str = "batch_size";

unsafe extern "C" {
    pub fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
}

#[repr(C)]
pub struct bpf_token_create_opts {
    pub sz: size_t,
    pub flags: __u32,
}

pub const bpf_token_create_opts__last_field: &str = "flags";

unsafe extern "C" {
    pub fn bpf_token_create(bpffs_fd: c_int, opts: *mut bpf_token_create_opts) -> c_int;
}

#[repr(C)]
pub struct bpf_prog_stream_read_opts {
    pub sz: size_t,
}

pub const bpf_prog_stream_read_opts__last_field: &str = "sz";

unsafe extern "C" {
    pub fn bpf_prog_stream_read(
        prog_fd: c_int,
        stream_id: __u32,
        buf: *mut c_void,
        buf_len: __u32,
        opts: *mut bpf_prog_stream_read_opts,
    ) -> c_int;
}

#[repr(C)]
pub struct bpf_prog_assoc_struct_ops_opts {
    pub sz: size_t,
    pub flags: __u32,
}

pub const bpf_prog_assoc_struct_ops_opts__last_field: &str = "flags";

unsafe extern "C" {
    pub fn bpf_prog_assoc_struct_ops(
        prog_fd: c_int,
        map_fd: c_int,
        opts: *mut bpf_prog_assoc_struct_ops_opts,
    ) -> c_int;
}
