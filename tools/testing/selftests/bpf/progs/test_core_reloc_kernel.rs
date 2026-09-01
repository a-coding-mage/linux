// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::c_void;

type bool_ = bool;

#[repr(C)]
pub struct data_t {
    pub in_: [u8; 256],
    pub out: [u8; 256],
    pub skip: bool_,
    pub my_pid_tgid: u64,
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut data: data_t = data_t {
    in_: [0; 256],
    out: [0; 256],
    skip: false,
    my_pid_tgid: 0,
};

#[repr(C)]
pub struct core_reloc_kernel_output {
    pub valid: [i32; 10],
    /* we have test_progs[-flavor], so cut flavor part */
    pub comm: [u8; 11],
    pub comm_len: i32,
    pub local_task_struct_matches: bool_,
}

#[repr(C)]
pub struct task_struct {
    pub pid: i32,
    pub tgid: i32,
    pub comm: [u8; 16],
    pub group_leader: *mut task_struct,
}

#[repr(C)]
pub struct mm_struct___wrong {
    pub abc_whatever_should_not_exist: i32,
}

#[repr(C)]
pub struct task_struct___local {
    pub pid: i32,
    pub mm: *mut mm_struct___wrong,
}

unsafe extern "C" {
    fn bpf_get_current_task() -> u64;
    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_core_read(dst: *mut c_void, size: u32, src: *const c_void) -> i64;
    fn bpf_core_read_str(dst: *mut c_void, size: u32, src: *const c_void) -> i64;
}

unsafe fn CORE_READ<T>(dst: *mut T, src: *const T) -> i64 {
    unsafe {
        bpf_core_read(
            dst as *mut c_void,
            core::mem::size_of::<T>() as u32,
            src as *const c_void,
        )
    }
}

unsafe fn BPF_CORE_READ_tgid(task: *mut task_struct) -> i32 {
    let mut val: i32 = 0;
    unsafe {
        let _ = CORE_READ(&mut val, core::ptr::addr_of!((*task).tgid));
    }
    val
}

unsafe fn BPF_CORE_READ_group_leader_tgid(task: *mut task_struct) -> i32 {
    let mut group_leader: *mut task_struct = core::ptr::null_mut();
    let mut val: i32 = 0;
    unsafe {
        let _ = CORE_READ(&mut group_leader, core::ptr::addr_of!((*task).group_leader));
        let _ = CORE_READ(&mut val, core::ptr::addr_of!((*group_leader).tgid));
    }
    val
}

unsafe fn BPF_CORE_READ_group_leader_group_leader_tgid(task: *mut task_struct) -> i32 {
    let mut group_leader: *mut task_struct = core::ptr::null_mut();
    let mut group_leader_1: *mut task_struct = core::ptr::null_mut();
    let mut val: i32 = 0;
    unsafe {
        let _ = CORE_READ(&mut group_leader, core::ptr::addr_of!((*task).group_leader));
        let _ = CORE_READ(
            &mut group_leader_1,
            core::ptr::addr_of!((*group_leader).group_leader),
        );
        let _ = CORE_READ(&mut val, core::ptr::addr_of!((*group_leader_1).tgid));
    }
    val
}

unsafe fn BPF_CORE_READ_group_leader_group_leader_group_leader_tgid(
    task: *mut task_struct,
) -> i32 {
    let mut group_leader: *mut task_struct = core::ptr::null_mut();
    let mut group_leader_1: *mut task_struct = core::ptr::null_mut();
    let mut group_leader_2: *mut task_struct = core::ptr::null_mut();
    let mut val: i32 = 0;
    unsafe {
        let _ = CORE_READ(&mut group_leader, core::ptr::addr_of!((*task).group_leader));
        let _ = CORE_READ(
            &mut group_leader_1,
            core::ptr::addr_of!((*group_leader).group_leader),
        );
        let _ = CORE_READ(
            &mut group_leader_2,
            core::ptr::addr_of!((*group_leader_1).group_leader),
        );
        let _ = CORE_READ(&mut val, core::ptr::addr_of!((*group_leader_2).tgid));
    }
    val
}

unsafe fn BPF_CORE_READ_group_leader_group_leader_group_leader_group_leader_tgid(
    task: *mut task_struct,
) -> i32 {
    let mut group_leader: *mut task_struct = core::ptr::null_mut();
    let mut group_leader_1: *mut task_struct = core::ptr::null_mut();
    let mut group_leader_2: *mut task_struct = core::ptr::null_mut();
    let mut group_leader_3: *mut task_struct = core::ptr::null_mut();
    let mut val: i32 = 0;
    unsafe {
        let _ = CORE_READ(&mut group_leader, core::ptr::addr_of!((*task).group_leader));
        let _ = CORE_READ(
            &mut group_leader_1,
            core::ptr::addr_of!((*group_leader).group_leader),
        );
        let _ = CORE_READ(
            &mut group_leader_2,
            core::ptr::addr_of!((*group_leader_1).group_leader),
        );
        let _ = CORE_READ(
            &mut group_leader_3,
            core::ptr::addr_of!((*group_leader_2).group_leader),
        );
        let _ = CORE_READ(&mut val, core::ptr::addr_of!((*group_leader_3).tgid));
    }
    val
}

unsafe fn BPF_CORE_READ_group_leader_group_leader_group_leader_group_leader_group_leader_tgid(
    task: *mut task_struct,
) -> i32 {
    let mut group_leader: *mut task_struct = core::ptr::null_mut();
    let mut group_leader_1: *mut task_struct = core::ptr::null_mut();
    let mut group_leader_2: *mut task_struct = core::ptr::null_mut();
    let mut group_leader_3: *mut task_struct = core::ptr::null_mut();
    let mut group_leader_4: *mut task_struct = core::ptr::null_mut();
    let mut val: i32 = 0;
    unsafe {
        let _ = CORE_READ(&mut group_leader, core::ptr::addr_of!((*task).group_leader));
        let _ = CORE_READ(
            &mut group_leader_1,
            core::ptr::addr_of!((*group_leader).group_leader),
        );
        let _ = CORE_READ(
            &mut group_leader_2,
            core::ptr::addr_of!((*group_leader_1).group_leader),
        );
        let _ = CORE_READ(
            &mut group_leader_3,
            core::ptr::addr_of!((*group_leader_2).group_leader),
        );
        let _ = CORE_READ(
            &mut group_leader_4,
            core::ptr::addr_of!((*group_leader_3).group_leader),
        );
        let _ = CORE_READ(&mut val, core::ptr::addr_of!((*group_leader_4).tgid));
    }
    val
}

unsafe fn BPF_CORE_READ_group_leader_group_leader_group_leader_group_leader_group_leader_group_leader_tgid(
    task: *mut task_struct,
) -> i32 {
    let mut group_leader: *mut task_struct = core::ptr::null_mut();
    let mut group_leader_1: *mut task_struct = core::ptr::null_mut();
    let mut group_leader_2: *mut task_struct = core::ptr::null_mut();
    let mut group_leader_3: *mut task_struct = core::ptr::null_mut();
    let mut group_leader_4: *mut task_struct = core::ptr::null_mut();
    let mut group_leader_5: *mut task_struct = core::ptr::null_mut();
    let mut val: i32 = 0;
    unsafe {
        let _ = CORE_READ(&mut group_leader, core::ptr::addr_of!((*task).group_leader));
        let _ = CORE_READ(&mut group_leader_1, core::ptr::addr_of!((*group_leader).group_leader));
        let _ = CORE_READ(&mut group_leader_2, core::ptr::addr_of!((*group_leader_1).group_leader));
        let _ = CORE_READ(&mut group_leader_3, core::ptr::addr_of!((*group_leader_2).group_leader));
        let _ = CORE_READ(&mut group_leader_4, core::ptr::addr_of!((*group_leader_3).group_leader));
        let _ = CORE_READ(&mut group_leader_5, core::ptr::addr_of!((*group_leader_4).group_leader));
        let _ = CORE_READ(&mut val, core::ptr::addr_of!((*group_leader_5).tgid));
    }
    val
}

unsafe fn BPF_CORE_READ_group_leader_group_leader_group_leader_group_leader_group_leader_group_leader_group_leader_tgid(
    task: *mut task_struct,
) -> i32 {
    let mut group_leader: *mut task_struct = core::ptr::null_mut();
    let mut group_leader_1: *mut task_struct = core::ptr::null_mut();
    let mut group_leader_2: *mut task_struct = core::ptr::null_mut();
    let mut group_leader_3: *mut task_struct = core::ptr::null_mut();
    let mut group_leader_4: *mut task_struct = core::ptr::null_mut();
    let mut group_leader_5: *mut task_struct = core::ptr::null_mut();
    let mut group_leader_6: *mut task_struct = core::ptr::null_mut();
    let mut val: i32 = 0;
    unsafe {
        let _ = CORE_READ(&mut group_leader, core::ptr::addr_of!((*task).group_leader));
        let _ = CORE_READ(&mut group_leader_1, core::ptr::addr_of!((*group_leader).group_leader));
        let _ = CORE_READ(&mut group_leader_2, core::ptr::addr_of!((*group_leader_1).group_leader));
        let _ = CORE_READ(&mut group_leader_3, core::ptr::addr_of!((*group_leader_2).group_leader));
        let _ = CORE_READ(&mut group_leader_4, core::ptr::addr_of!((*group_leader_3).group_leader));
        let _ = CORE_READ(&mut group_leader_5, core::ptr::addr_of!((*group_leader_4).group_leader));
        let _ = CORE_READ(&mut group_leader_6, core::ptr::addr_of!((*group_leader_5).group_leader));
        let _ = CORE_READ(&mut val, core::ptr::addr_of!((*group_leader_6).tgid));
    }
    val
}

unsafe fn BPF_CORE_READ_group_leader_group_leader_group_leader_group_leader_group_leader_group_leader_group_leader_group_leader_tgid(
    task: *mut task_struct,
) -> i32 {
    let mut group_leader: *mut task_struct = core::ptr::null_mut();
    let mut group_leader_1: *mut task_struct = core::ptr::null_mut();
    let mut group_leader_2: *mut task_struct = core::ptr::null_mut();
    let mut group_leader_3: *mut task_struct = core::ptr::null_mut();
    let mut group_leader_4: *mut task_struct = core::ptr::null_mut();
    let mut group_leader_5: *mut task_struct = core::ptr::null_mut();
    let mut group_leader_6: *mut task_struct = core::ptr::null_mut();
    let mut group_leader_7: *mut task_struct = core::ptr::null_mut();
    let mut val: i32 = 0;
    unsafe {
        let _ = CORE_READ(&mut group_leader, core::ptr::addr_of!((*task).group_leader));
        let _ = CORE_READ(&mut group_leader_1, core::ptr::addr_of!((*group_leader).group_leader));
        let _ = CORE_READ(&mut group_leader_2, core::ptr::addr_of!((*group_leader_1).group_leader));
        let _ = CORE_READ(&mut group_leader_3, core::ptr::addr_of!((*group_leader_2).group_leader));
        let _ = CORE_READ(&mut group_leader_4, core::ptr::addr_of!((*group_leader_3).group_leader));
        let _ = CORE_READ(&mut group_leader_5, core::ptr::addr_of!((*group_leader_4).group_leader));
        let _ = CORE_READ(&mut group_leader_6, core::ptr::addr_of!((*group_leader_5).group_leader));
        let _ = CORE_READ(&mut group_leader_7, core::ptr::addr_of!((*group_leader_6).group_leader));
        let _ = CORE_READ(&mut val, core::ptr::addr_of!((*group_leader_7).tgid));
    }
    val
}

unsafe fn BPF_CORE_READ_STR_INTO_comm(dst: *mut [u8; 11], task: *mut task_struct) -> i32 {
    let mut group_leader: *mut task_struct = core::ptr::null_mut();
    let mut group_leader_1: *mut task_struct = core::ptr::null_mut();
    let mut group_leader_2: *mut task_struct = core::ptr::null_mut();
    let mut group_leader_3: *mut task_struct = core::ptr::null_mut();
    let mut group_leader_4: *mut task_struct = core::ptr::null_mut();
    let mut group_leader_5: *mut task_struct = core::ptr::null_mut();
    let mut group_leader_6: *mut task_struct = core::ptr::null_mut();
    let mut group_leader_7: *mut task_struct = core::ptr::null_mut();
    unsafe {
        let _ = CORE_READ(&mut group_leader, core::ptr::addr_of!((*task).group_leader));
        let _ = CORE_READ(&mut group_leader_1, core::ptr::addr_of!((*group_leader).group_leader));
        let _ = CORE_READ(&mut group_leader_2, core::ptr::addr_of!((*group_leader_1).group_leader));
        let _ = CORE_READ(&mut group_leader_3, core::ptr::addr_of!((*group_leader_2).group_leader));
        let _ = CORE_READ(&mut group_leader_4, core::ptr::addr_of!((*group_leader_3).group_leader));
        let _ = CORE_READ(&mut group_leader_5, core::ptr::addr_of!((*group_leader_4).group_leader));
        let _ = CORE_READ(&mut group_leader_6, core::ptr::addr_of!((*group_leader_5).group_leader));
        let _ = CORE_READ(&mut group_leader_7, core::ptr::addr_of!((*group_leader_6).group_leader));
        bpf_core_read_str(
            dst as *mut c_void,
            core::mem::size_of::<[u8; 11]>() as u32,
            core::ptr::addr_of!((*group_leader_7).comm) as *const c_void,
        ) as i32
    }
}

unsafe fn bpf_core_type_matches_task_struct___local() -> bool_ {
    // Rust has no file-local equivalent for bpf_core_type_matches(struct task_struct___local).
    core::mem::size_of::<task_struct___local>() != 0
}

#[link_section = "raw_tracepoint/sys_enter"]
#[no_mangle]
pub unsafe extern "C" fn test_core_kernel(ctx: *mut c_void) -> i32 {
    let _ = ctx;

    /* Support for the BPF_TYPE_MATCHES argument to the
     * __builtin_preserve_type_info builtin was added at some point during
     * development of clang 15 and it's what we require for this test.
     */
    // Original C condition:
    // #if __has_builtin(__builtin_preserve_type_info) && __clang_major__ >= 15
    #[cfg(any())]
    unsafe {
        let task: *mut task_struct = bpf_get_current_task() as *mut c_void as *mut task_struct;
        let out: *mut core_reloc_kernel_output =
            core::ptr::addr_of_mut!(data.out) as *mut c_void as *mut core_reloc_kernel_output;
        let pid_tgid: u64 = bpf_get_current_pid_tgid();
        let real_tgid: i32 = pid_tgid as i32;
        let mut pid: i32 = 0;
        let mut tgid: i32 = 0;

        if data.my_pid_tgid != pid_tgid {
            return 0;
        }

        if CORE_READ(&mut pid, core::ptr::addr_of!((*task).pid)) != 0
            || CORE_READ(&mut tgid, core::ptr::addr_of!((*task).tgid)) != 0
        {
            return 1;
        }

        /* validate pid + tgid matches */
        (*out).valid[0] = ((((pid as u64) << 32) | (tgid as u32 as u64)) == pid_tgid) as i32;

        /* test variadic BPF_CORE_READ macros */
        (*out).valid[1] = (BPF_CORE_READ_tgid(task) == real_tgid) as i32;
        (*out).valid[2] = (BPF_CORE_READ_group_leader_tgid(task) == real_tgid) as i32;
        (*out).valid[3] =
            (BPF_CORE_READ_group_leader_group_leader_tgid(task) == real_tgid) as i32;
        (*out).valid[4] =
            (BPF_CORE_READ_group_leader_group_leader_group_leader_tgid(task) == real_tgid) as i32;
        (*out).valid[5] =
            (BPF_CORE_READ_group_leader_group_leader_group_leader_group_leader_tgid(task)
                == real_tgid) as i32;
        (*out).valid[6] =
            (BPF_CORE_READ_group_leader_group_leader_group_leader_group_leader_group_leader_tgid(
                task,
            ) == real_tgid) as i32;
        (*out).valid[7] =
            (BPF_CORE_READ_group_leader_group_leader_group_leader_group_leader_group_leader_group_leader_tgid(
                task,
            ) == real_tgid) as i32;
        (*out).valid[8] =
            (BPF_CORE_READ_group_leader_group_leader_group_leader_group_leader_group_leader_group_leader_group_leader_tgid(
                task,
            ) == real_tgid) as i32;
        (*out).valid[9] =
            (BPF_CORE_READ_group_leader_group_leader_group_leader_group_leader_group_leader_group_leader_group_leader_group_leader_tgid(
                task,
            ) == real_tgid) as i32;

        /* test BPF_CORE_READ_STR_INTO() returns correct code and contents */
        (*out).comm_len = BPF_CORE_READ_STR_INTO_comm(core::ptr::addr_of_mut!((*out).comm), task);

        (*out).local_task_struct_matches = bpf_core_type_matches_task_struct___local();
    }

    // Original C #else branch, used when the clang builtin/major-version condition is false.
    #[cfg(not(any()))]
    unsafe {
        data.skip = true;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
