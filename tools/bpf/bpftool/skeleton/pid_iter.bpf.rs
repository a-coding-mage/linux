// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
/* Copyright (c) 2020 Facebook */

/* Rust translation of dependencies originally included from:
 * <vmlinux.h>, <bpf/bpf_helpers.h>, <bpf/bpf_core_read.h>,
 * <bpf/bpf_tracing.h>, and "pid_iter.h".
 */

pub type __u32 = u32;
pub type __u64 = u64;
pub type u64 = u64;

/* keep in sync with the definition in main.h */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_obj_type {
    BPF_OBJ_UNKNOWN,
    BPF_OBJ_PROG,
    BPF_OBJ_MAP,
    BPF_OBJ_LINK,
    BPF_OBJ_BTF,
}

#[repr(C)]
pub struct bpf_perf_link___local {
    pub link: bpf_link,
    pub perf_file: *mut file,
} /* preserve_access_index */;

#[repr(C)]
pub struct perf_event___local {
    pub bpf_cookie: u64,
} /* preserve_access_index */;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_link_type___local {
    BPF_LINK_TYPE_PERF_EVENT___local = 7,
}

unsafe extern "C" {
    pub static bpf_link_fops: core::ffi::c_void;
    pub static bpf_link_fops_poll: core::ffi::c_void;
    pub static bpf_map_fops: core::ffi::c_void;
    pub static bpf_prog_fops: core::ffi::c_void;
    pub static btf_fops: core::ffi::c_void;

    pub fn bpf_probe_read_kernel_str(
        dst: *mut core::ffi::c_void,
        size: u32,
        unsafe_ptr: *const core::ffi::c_void,
    ) -> i64;
    pub fn bpf_seq_write(seq: *mut core::ffi::c_void, data: *const core::ffi::c_void, len: u32) -> i64;
}

#[unsafe(no_mangle)]
pub static mut obj_type: bpf_obj_type = bpf_obj_type::BPF_OBJ_UNKNOWN;

#[repr(C)]
pub struct bpf_prog_aux {
    pub id: __u32,
}

#[repr(C)]
pub struct bpf_prog {
    pub aux: *mut bpf_prog_aux,
}

#[repr(C)]
pub struct bpf_map {
    pub id: __u32,
}

#[repr(C)]
pub struct btf {
    pub id: __u32,
}

#[repr(C)]
pub struct bpf_link {
    pub id: __u32,
    pub type_: i32,
}

#[repr(C)]
pub struct file {
    pub f_op: *const core::ffi::c_void,
    pub private_data: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct task_struct {
    pub tgid: __u32,
    pub group_leader: *mut task_struct,
    pub comm: [u8; 16],
}

#[repr(C)]
pub struct bpf_iter_meta {
    pub seq: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct bpf_iter__task_file {
    pub meta: *mut bpf_iter_meta,
    pub task: *mut task_struct,
    pub file: *mut file,
}

#[repr(C)]
pub struct pid_iter_entry {
    pub pid: __u32,
    pub id: __u32,
    pub has_bpf_cookie: bool,
    pub bpf_cookie: __u64,
    pub comm: [u8; 16],
}

#[inline(always)]
unsafe fn get_obj_id(ent: *mut core::ffi::c_void, type_: bpf_obj_type) -> __u32 {
    match type_ {
        bpf_obj_type::BPF_OBJ_PROG => {
            let prog = ent as *mut bpf_prog;
            let aux = unsafe { (*prog).aux };
            unsafe { (*aux).id }
        }
        bpf_obj_type::BPF_OBJ_MAP => {
            let map = ent as *mut bpf_map;
            unsafe { (*map).id }
        }
        bpf_obj_type::BPF_OBJ_BTF => {
            let btf = ent as *mut btf;
            unsafe { (*btf).id }
        }
        bpf_obj_type::BPF_OBJ_LINK => {
            let link = ent as *mut bpf_link;
            unsafe { (*link).id }
        }
        _ => 0,
    }
}

/* could be used only with BPF_LINK_TYPE_PERF_EVENT links */
unsafe fn get_bpf_cookie(link: *mut bpf_link) -> __u64 {
    let perf_link = link as *mut bpf_perf_link___local;
    let event = unsafe { (*(*perf_link).perf_file).private_data as *mut perf_event___local };

    unsafe { (*event).bpf_cookie }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "iter/task_file")]
pub unsafe extern "C" fn iter(ctx: *mut bpf_iter__task_file) -> i32 {
    let file = unsafe { (*ctx).file };
    let task = unsafe { (*ctx).task };
    let mut e: pid_iter_entry;
    let fops: *const core::ffi::c_void;

    if file.is_null() || task.is_null() {
        return 0;
    }

    match unsafe { obj_type } {
        bpf_obj_type::BPF_OBJ_PROG => {
            fops = unsafe { &bpf_prog_fops as *const _ as *const core::ffi::c_void };
        }
        bpf_obj_type::BPF_OBJ_MAP => {
            fops = unsafe { &bpf_map_fops as *const _ as *const core::ffi::c_void };
        }
        bpf_obj_type::BPF_OBJ_BTF => {
            fops = unsafe { &btf_fops as *const _ as *const core::ffi::c_void };
        }
        bpf_obj_type::BPF_OBJ_LINK => {
            if unsafe { (&bpf_link_fops_poll as *const _ as *const core::ffi::c_void) != core::ptr::null() }
                && unsafe { (*file).f_op == (&bpf_link_fops_poll as *const _ as *const core::ffi::c_void) }
            {
                fops = unsafe { &bpf_link_fops_poll as *const _ as *const core::ffi::c_void };
            } else {
                fops = unsafe { &bpf_link_fops as *const _ as *const core::ffi::c_void };
            }
        }
        _ => {
            return 0;
        }
    }

    if unsafe { (*file).f_op != fops } {
        return 0;
    }

    e = unsafe { core::mem::zeroed() };
    e.pid = unsafe { (*task).tgid };
    e.id = unsafe { get_obj_id((*file).private_data, obj_type) };

    if unsafe { obj_type == bpf_obj_type::BPF_OBJ_LINK } {
        /* Original C additionally checks:
         * bpf_core_enum_value_exists(enum bpf_link_type___local,
         *                            BPF_LINK_TYPE_PERF_EVENT___local)
         */
        let link = unsafe { (*file).private_data as *mut bpf_link };

        if unsafe { (*link).type_ }
            == bpf_link_type___local::BPF_LINK_TYPE_PERF_EVENT___local as i32
        {
            e.has_bpf_cookie = true;
            e.bpf_cookie = unsafe { get_bpf_cookie(link) };
        }
    }

    unsafe {
        bpf_probe_read_kernel_str(
            e.comm.as_mut_ptr() as *mut core::ffi::c_void,
            core::mem::size_of_val(&e.comm) as u32,
            (*(*task).group_leader).comm.as_ptr() as *const core::ffi::c_void,
        );
        bpf_seq_write(
            (*(*ctx).meta).seq,
            &e as *const _ as *const core::ffi::c_void,
            core::mem::size_of_val(&e) as u32,
        );
    }

    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static LICENSE: &[u8; 13] = b"Dual BSD/GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
