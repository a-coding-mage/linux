// SPDX-License-Identifier: GPL-2.0

/*
 * Copyright 2020 Google LLC.
 */

// C includes translated as external dependency intent:
// "vmlinux.h", <errno.h>, <bpf/bpf_core_read.h>, <bpf/bpf_helpers.h>,
// <bpf/bpf_tracing.h>, and "bpf_misc.h".

#[repr(C)]
pub struct vm_area_struct {
    pub vm_mm: *mut mm_struct,
    pub vm_start: c_ulong,
    pub vm_end: c_ulong,
}

#[repr(C)]
pub struct mm_struct {
    pub start_stack: c_ulong,
    pub arg_start: c_ulong,
}

#[repr(C)]
pub struct linux_binprm {
    pub vma: *mut vm_area_struct,
    pub mm: *mut mm_struct,
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

type __u32 = u32;
type __s32 = i32;
type __u64 = u64;
type c_ulong = u64;
type c_long = i64;
type c_int = i32;
type c_void = core::ffi::c_void;

const BPF_MAP_TYPE_ARRAY: __u32 = 2;
const BPF_MAP_TYPE_HASH: __u32 = 1;
const BPF_MAP_TYPE_LRU_HASH: __u32 = 9;
const BPF_MAP_TYPE_PERCPU_ARRAY: __u32 = 6;
const BPF_MAP_TYPE_PERCPU_HASH: __u32 = 5;
const BPF_MAP_TYPE_LRU_PERCPU_HASH: __u32 = 10;
const BPF_MAP_TYPE_ARRAY_OF_MAPS: __u32 = 12;
const BPF_MAP_TYPE_HASH_OF_MAPS: __u32 = 13;
const EPERM: c_int = 1;
const EFAULT: c_int = 14;

extern "C" {
    fn bpf_get_current_pid_tgid() -> __u64;
    fn bpf_copy_from_user(dst: *mut c_void, size: __u32, unsafe_ptr: *const c_void) -> c_long;
    fn bpf_map_lookup_elem(map: *const c_void, key: *const c_void) -> *mut c_void;
    fn PT_REGS_PARM1_SYSCALL(regs: *mut pt_regs) -> c_ulong;
    fn PT_REGS_PARM2_SYSCALL(regs: *mut pt_regs) -> c_int;
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: __u32,
    pub max_entries: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
}

#[repr(C)]
pub struct inner_map {
    pub type_: __u32,
    pub max_entries: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
}

#[repr(C)]
pub struct outer_arr {
    pub type_: __u32,
    pub max_entries: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
    pub values: [*const inner_map; 1],
}

#[repr(C)]
pub struct outer_hash {
    pub type_: __u32,
    pub max_entries: __u32,
    pub key_size: __u32,
    pub values: [*const inner_map; 1],
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut array: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__u64>() as __u32,
};

#[link_section = ".maps"]
#[no_mangle]
pub static mut hash: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: 1,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__u64>() as __u32,
};

#[link_section = ".maps"]
#[no_mangle]
pub static mut lru_hash: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_LRU_HASH,
    max_entries: 1,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__u64>() as __u32,
};

#[link_section = ".maps"]
#[no_mangle]
pub static mut percpu_array: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_PERCPU_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__u64>() as __u32,
};

#[link_section = ".maps"]
#[no_mangle]
pub static mut percpu_hash: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_PERCPU_HASH,
    max_entries: 1,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__u64>() as __u32,
};

#[link_section = ".maps"]
#[no_mangle]
pub static mut lru_percpu_hash: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_LRU_PERCPU_HASH,
    max_entries: 1,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__u64>() as __u32,
};

#[link_section = ".maps"]
#[no_mangle]
pub static mut inner_map: inner_map = inner_map {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<c_int>() as __u32,
    value_size: core::mem::size_of::<__u64>() as __u32,
};

#[link_section = ".maps"]
#[no_mangle]
pub static mut outer_arr: outer_arr = outer_arr {
    type_: BPF_MAP_TYPE_ARRAY_OF_MAPS,
    max_entries: 1,
    key_size: core::mem::size_of::<c_int>() as __u32,
    value_size: core::mem::size_of::<c_int>() as __u32,
    values: [unsafe { &inner_map as *const inner_map }],
};

#[link_section = ".maps"]
#[no_mangle]
pub static mut outer_hash: outer_hash = outer_hash {
    type_: BPF_MAP_TYPE_HASH_OF_MAPS,
    max_entries: 1,
    key_size: core::mem::size_of::<c_int>() as __u32,
    values: [unsafe { &inner_map as *const inner_map }],
};

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut monitored_pid: c_int = 0;
#[no_mangle]
pub static mut mprotect_count: c_int = 0;
#[no_mangle]
pub static mut bprm_count: c_int = 0;

#[link_section = "lsm/file_mprotect"]
#[no_mangle]
pub unsafe extern "C" fn test_int_hook(
    vma: *mut vm_area_struct,
    reqprot: c_ulong,
    prot: c_ulong,
    mut ret: c_int,
) -> c_int {
    let mm: *mut mm_struct = (*vma).vm_mm;

    if ret != 0 || mm.is_null() {
        return ret;
    }

    let pid: __s32 = (bpf_get_current_pid_tgid() >> 32) as __s32;
    let mut is_stack: c_int = 0;

    is_stack = (((*vma).vm_start <= (*mm).start_stack) && ((*vma).vm_end >= (*mm).start_stack)) as c_int;

    if is_stack != 0 && monitored_pid == pid {
        mprotect_count += 1;
        ret = -EPERM;
    }

    ret
}

#[link_section = "lsm.s/bprm_committed_creds"]
#[no_mangle]
pub unsafe extern "C" fn test_void_hook(bprm: *mut linux_binprm) -> c_int {
    let pid: __u32 = (bpf_get_current_pid_tgid() >> 32) as __u32;
    let mut inner_map: *mut inner_map;
    let mut args: [u8; 64] = [0; 64];
    let key: __u32 = 0;
    let mut value: *mut __u64;

    if monitored_pid as __u32 == pid {
        bprm_count += 1;
    }

    bpf_copy_from_user(
        args.as_mut_ptr() as *mut c_void,
        core::mem::size_of_val(&args) as __u32,
        (*(*(*bprm).vma).vm_mm).arg_start as *const c_void,
    );
    bpf_copy_from_user(
        args.as_mut_ptr() as *mut c_void,
        core::mem::size_of_val(&args) as __u32,
        (*(*bprm).mm).arg_start as *const c_void,
    );

    value = bpf_map_lookup_elem(&array as *const _ as *const c_void, &key as *const _ as *const c_void) as *mut __u64;
    if !value.is_null() {
        *value = 0;
    }
    value = bpf_map_lookup_elem(&hash as *const _ as *const c_void, &key as *const _ as *const c_void) as *mut __u64;
    if !value.is_null() {
        *value = 0;
    }
    value = bpf_map_lookup_elem(&lru_hash as *const _ as *const c_void, &key as *const _ as *const c_void) as *mut __u64;
    if !value.is_null() {
        *value = 0;
    }
    value = bpf_map_lookup_elem(&percpu_array as *const _ as *const c_void, &key as *const _ as *const c_void) as *mut __u64;
    if !value.is_null() {
        *value = 0;
    }
    value = bpf_map_lookup_elem(&percpu_hash as *const _ as *const c_void, &key as *const _ as *const c_void) as *mut __u64;
    if !value.is_null() {
        *value = 0;
    }
    value = bpf_map_lookup_elem(&lru_percpu_hash as *const _ as *const c_void, &key as *const _ as *const c_void) as *mut __u64;
    if !value.is_null() {
        *value = 0;
    }
    inner_map = bpf_map_lookup_elem(&outer_arr as *const _ as *const c_void, &key as *const _ as *const c_void) as *mut inner_map;
    if !inner_map.is_null() {
        value = bpf_map_lookup_elem(inner_map as *const c_void, &key as *const _ as *const c_void) as *mut __u64;
        if !value.is_null() {
            *value = 0;
        }
    }
    inner_map = bpf_map_lookup_elem(&outer_hash as *const _ as *const c_void, &key as *const _ as *const c_void) as *mut inner_map;
    if !inner_map.is_null() {
        value = bpf_map_lookup_elem(inner_map as *const c_void, &key as *const _ as *const c_void) as *mut __u64;
        if !value.is_null() {
            *value = 0;
        }
    }

    0
}

#[link_section = "lsm/task_free"] /* lsm/ is ok, lsm.s/ fails */
#[no_mangle]
pub unsafe extern "C" fn test_task_free(task: *mut task_struct) -> c_int {
    0
}

#[no_mangle]
pub static mut copy_test: c_int = 0;

// Original C section used: SEC("fentry.s/" SYS_PREFIX "sys_setdomainname").
#[link_section = "fentry.s/sys_setdomainname"]
#[no_mangle]
pub unsafe extern "C" fn test_sys_setdomainname(regs: *mut pt_regs) -> c_int {
    let ptr: *mut c_void = PT_REGS_PARM1_SYSCALL(regs) as *mut c_void;
    let len: c_int = PT_REGS_PARM2_SYSCALL(regs);
    let mut buf: c_int = 0;
    let mut ret: c_long;

    ret = bpf_copy_from_user(
        &mut buf as *mut _ as *mut c_void,
        core::mem::size_of_val(&buf) as __u32,
        ptr as *const c_void,
    );
    if len == -2 && ret == 0 && buf == 1234 {
        copy_test += 1;
    }
    if len == -3 && ret == -(EFAULT as c_long) {
        copy_test += 1;
    }
    if len == -4 && ret == -(EFAULT as c_long) {
        copy_test += 1;
    }
    0
}
