// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */

// Dependencies from the original C source:
// "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_core_read.h>, <bpf/bpf_tracing.h>

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct bpf_testmod_test_read_ctx {
    /* field order is mixed up */
    pub len: size_t,
    pub buf: *mut ::core::ffi::c_char,
    pub off: loff_t,
}
// Original C used __attribute__((preserve_access_index)) on this type.

#[repr(C)]
pub struct data_t {
    pub in_: [::core::ffi::c_char; 256],
    pub out: [::core::ffi::c_char; 256],
    pub skip: bool,
    pub my_pid_tgid: u64,
}

#[no_mangle]
pub static mut data: data_t = data_t {
    in_: [0; 256],
    out: [0; 256],
    skip: false,
    my_pid_tgid: 0,
};

#[repr(C)]
pub struct core_reloc_module_output {
    pub len: ::core::ffi::c_longlong,
    pub off: ::core::ffi::c_longlong,
    pub read_ctx_sz: ::core::ffi::c_int,
    pub read_ctx_exists: bool,
    pub buf_exists: bool,
    pub len_exists: bool,
    pub off_exists: bool,
    /* we have test_progs[-flavor], so cut flavor part */
    pub comm: [::core::ffi::c_char; 11],
    pub comm_len: ::core::ffi::c_int,
}

extern "C" {
    fn bpf_get_current_pid_tgid() -> u64;

    static __has_builtin___builtin_preserve_enum_value: bool;

    fn BPF_CORE_READ_task_pid(task: *mut task_struct) -> i32;
    fn BPF_CORE_READ_task_tgid(task: *mut task_struct) -> i32;
    fn BPF_CORE_READ_read_ctx_len(read_ctx: *mut bpf_testmod_test_read_ctx) -> size_t;
    fn BPF_CORE_READ_read_ctx_off(read_ctx: *mut bpf_testmod_test_read_ctx) -> loff_t;
    fn BPF_CORE_READ_STR_INTO_comm(
        dst: *mut [::core::ffi::c_char; 11],
        task: *mut task_struct,
    ) -> ::core::ffi::c_int;

    fn bpf_core_type_size_bpf_testmod_test_read_ctx() -> ::core::ffi::c_int;
    fn bpf_core_type_exists_bpf_testmod_test_read_ctx() -> bool;
    fn bpf_core_field_exists_read_ctx_buf() -> bool;
    fn bpf_core_field_exists_read_ctx_off() -> bool;
    fn bpf_core_field_exists_read_ctx_len() -> bool;
}

// Types supplied by the original include dependencies.
pub type size_t = usize;
pub type loff_t = i64;

#[repr(C)]
pub struct task_struct {
    pub pid: i32,
    pub tgid: i32,
    pub comm: [::core::ffi::c_char; 0],
}

#[link_section = "raw_tp/bpf_testmod_test_read"]
#[no_mangle]
pub unsafe extern "C" fn test_core_module_probed(
    task: *mut task_struct,
    read_ctx: *mut bpf_testmod_test_read_ctx,
) -> ::core::ffi::c_int {
    // Original condition: #if __has_builtin(__builtin_preserve_enum_value)
    if __has_builtin___builtin_preserve_enum_value {
        let out: *mut core_reloc_module_output =
            (&raw mut data.out as *mut [::core::ffi::c_char; 256]).cast();
        let pid_tgid: u64 = bpf_get_current_pid_tgid();
        let real_tgid: i32 = (pid_tgid >> 32) as i32;
        let real_pid: i32 = pid_tgid as i32;

        if data.my_pid_tgid != pid_tgid {
            return 0;
        }

        if BPF_CORE_READ_task_pid(task) != real_pid || BPF_CORE_READ_task_tgid(task) != real_tgid {
            return 0;
        }

        (*out).len = BPF_CORE_READ_read_ctx_len(read_ctx) as ::core::ffi::c_longlong;
        (*out).off = BPF_CORE_READ_read_ctx_off(read_ctx) as ::core::ffi::c_longlong;

        (*out).read_ctx_sz = bpf_core_type_size_bpf_testmod_test_read_ctx();
        (*out).read_ctx_exists = bpf_core_type_exists_bpf_testmod_test_read_ctx();
        (*out).buf_exists = bpf_core_field_exists_read_ctx_buf();
        (*out).off_exists = bpf_core_field_exists_read_ctx_off();
        (*out).len_exists = bpf_core_field_exists_read_ctx_len();

        (*out).comm_len = BPF_CORE_READ_STR_INTO_comm(&raw mut (*out).comm, task);
    } else {
        data.skip = true;
    }

    0
}

#[link_section = "tp_btf/bpf_testmod_test_read"]
#[no_mangle]
pub unsafe extern "C" fn test_core_module_direct(
    task: *mut task_struct,
    read_ctx: *mut bpf_testmod_test_read_ctx,
) -> ::core::ffi::c_int {
    // Original condition: #if __has_builtin(__builtin_preserve_enum_value)
    if __has_builtin___builtin_preserve_enum_value {
        let out: *mut core_reloc_module_output =
            (&raw mut data.out as *mut [::core::ffi::c_char; 256]).cast();
        let pid_tgid: u64 = bpf_get_current_pid_tgid();
        let real_tgid: i32 = (pid_tgid >> 32) as i32;
        let real_pid: i32 = pid_tgid as i32;

        if data.my_pid_tgid != pid_tgid {
            return 0;
        }

        if (*task).pid != real_pid || (*task).tgid != real_tgid {
            return 0;
        }

        (*out).len = (*read_ctx).len as ::core::ffi::c_longlong;
        (*out).off = (*read_ctx).off as ::core::ffi::c_longlong;

        (*out).read_ctx_sz = bpf_core_type_size_bpf_testmod_test_read_ctx();
        (*out).read_ctx_exists = bpf_core_type_exists_bpf_testmod_test_read_ctx();
        (*out).buf_exists = bpf_core_field_exists_read_ctx_buf();
        (*out).off_exists = bpf_core_field_exists_read_ctx_off();
        (*out).len_exists = bpf_core_field_exists_read_ctx_len();

        (*out).comm_len = BPF_CORE_READ_STR_INTO_comm(&raw mut (*out).comm, task);
    } else {
        data.skip = true;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
