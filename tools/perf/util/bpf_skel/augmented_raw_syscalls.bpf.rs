// SPDX-License-Identifier: GPL-2.0
/*
 * Augment the raw_syscalls tracepoints with the contents of the pointer arguments.
 *
 * This exactly matches what is marshalled into the raw_syscall:sys_enter
 * payload expected by the 'perf trace' beautifiers.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::c_void;
use core::mem::{offset_of, size_of};
use core::ptr;

type __u32 = u32;
type u64 = u64;
type s64 = i64;
type pid_t = i32;
type sa_family_t = u16;

const BPF_MAP_TYPE_PERF_EVENT_ARRAY: __u32 = 4;
const BPF_MAP_TYPE_PROG_ARRAY: __u32 = 3;
const BPF_MAP_TYPE_HASH: __u32 = 1;
const BPF_MAP_TYPE_PERCPU_ARRAY: __u32 = 5;
const BPF_F_CURRENT_CPU: u64 = 0xffff_ffff;
const PATH_MAX: usize = 4096;

const MAX_CPUS: __u32 = 4096;
const TRACE_AUG_MAX_BUF: u64 = 32; /* for buffer augmentation in perf trace */
const SS_MAXSIZE: usize = 128; /* Implementation specific max size */
const PERF_ATTR_SIZE_VER0: __u32 = 64; /* sizeof first published struct */

const fn __PERF_ALIGN_MASK(x: usize, mask: usize) -> usize {
    (x + mask) & !mask
}

const fn PERF_ALIGN(x: usize, a: usize) -> usize {
    __PERF_ALIGN_MASK(x, a - 1)
}

/**
 * is_power_of_2() - check if a value is a power of two
 * @n: the value to check
 *
 * Determine whether some value is a power of two, where zero is *not*
 * considered a power of two.  Return: true if @n is a power of 2, otherwise
 * false.
 */
const fn is_power_of_2(n: usize) -> bool {
    n != 0 && ((n & (n - 1)) == 0)
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
    pub max_entries: __u32,
}

/* bpf-output associated map */
#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut __augmented_syscalls__: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_PERF_EVENT_ARRAY,
    key_size: size_of::<i32>() as __u32,
    value_size: size_of::<__u32>() as __u32,
    max_entries: MAX_CPUS,
};

/*
 * What to augment at entry?
 *
 * Pointer arg payloads (filenames, etc) passed from userspace to the kernel
 */
#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut syscalls_sys_enter: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_PROG_ARRAY,
    key_size: size_of::<__u32>() as __u32,
    value_size: size_of::<__u32>() as __u32,
    max_entries: 1024,
};

/*
 * What to augment at exit?
 *
 * Pointer arg payloads returned from the kernel (struct stat, etc) to userspace.
 */
#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut syscalls_sys_exit: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_PROG_ARRAY,
    key_size: size_of::<__u32>() as __u32,
    value_size: size_of::<__u32>() as __u32,
    max_entries: 1024,
};

#[repr(C)]
pub struct syscall_enter_args {
    pub common_tp_fields: u64,
    pub syscall_nr: isize,
    pub args: [usize; 6],
}

#[repr(C)]
pub struct syscall_exit_args {
    pub common_tp_fields: u64,
    pub syscall_nr: isize,
    pub ret: isize,
}

/*
 * Desired design of maximum size and alignment (see RFC2553)
 */

/*
 * FIXME: Should come from system headers
 *
 * The definition uses anonymous union and struct in order to control the
 * default alignment.
 */
#[repr(C)]
pub struct sockaddr_storage_inner {
    pub ss_family: sa_family_t, /* address family */
    /* Following field(s) are implementation specific */
    pub __data: [i8; SS_MAXSIZE - size_of::<u16>()],
    /* space to achieve desired size, */
    /* _SS_MAXSIZE value minus size of ss_family */
}

#[repr(C)]
pub union sockaddr_storage_union {
    pub inner: core::mem::ManuallyDrop<sockaddr_storage_inner>,
    pub __align: *mut c_void, /* implementation specific desired alignment */
}

#[repr(C)]
pub struct sockaddr_storage {
    pub u: sockaddr_storage_union,
}

#[repr(C)]
pub union augmented_arg_union {
    pub value: [i8; PATH_MAX],
    pub saddr: core::mem::ManuallyDrop<sockaddr_storage>,
}

#[repr(C)]
pub struct augmented_arg {
    pub size: u32,
    pub err: i32,
    pub u: augmented_arg_union,
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut pids_filtered: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_HASH,
    key_size: size_of::<pid_t>() as __u32,
    value_size: size_of::<bool>() as __u32,
    max_entries: 64,
};

#[repr(C)]
pub struct augmented_args_payload {
    pub args: syscall_enter_args,
    pub arg: augmented_arg,
    pub arg2: augmented_arg, // We have to reserve space for two arguments (rename, etc)
}

// We need more tmp space than the BPF stack can give us
#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut augmented_args_tmp: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_PERCPU_ARRAY,
    key_size: size_of::<i32>() as __u32,
    value_size: size_of::<augmented_args_payload>() as __u32,
    max_entries: 1,
};

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut beauty_map_enter: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_HASH,
    key_size: size_of::<i32>() as __u32,
    value_size: size_of::<[__u32; 6]>() as __u32,
    max_entries: 512,
};

#[repr(C)]
pub struct beauty_payload_enter {
    pub args: syscall_enter_args,
    pub aug_args: [augmented_arg; 6],
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut beauty_payload_enter_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_PERCPU_ARRAY,
    key_size: size_of::<i32>() as __u32,
    value_size: size_of::<beauty_payload_enter>() as __u32,
    max_entries: 1,
};

#[repr(C)]
pub struct timespec64 {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

unsafe extern "C" {
    fn bpf_map_lookup_elem(map: *mut c_void, key: *const c_void) -> *mut c_void;
    fn bpf_perf_event_output(
        ctx: *mut c_void,
        map: *mut c_void,
        flags: u64,
        data: *const c_void,
        size: i32,
    ) -> i32;
    fn bpf_probe_read_user(dst: *mut c_void, size: u32, unsafe_ptr: *const c_void) -> i32;
    fn bpf_probe_read_user_str(dst: *mut c_void, size: u32, unsafe_ptr: *const c_void) -> i32;
    fn bpf_probe_read_kernel(dst: *mut c_void, size: usize, unsafe_ptr: *const c_void) -> i32;
    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_tail_call(ctx: *mut c_void, prog_array_map: *mut c_void, index: isize);
    fn bpf_ksym_exists(sym: *const c_void) -> bool;
    static bpf_iter_num_new: c_void;
}

#[inline(always)]
unsafe fn barrier_var<T>(_var: T) {}

#[inline(always)]
unsafe fn augmented_args_payload_ptr() -> *mut augmented_args_payload {
    let key: i32 = 0;
    unsafe {
        bpf_map_lookup_elem(
            &raw mut augmented_args_tmp as *mut c_void,
            &key as *const _ as *const c_void,
        ) as *mut augmented_args_payload
    }
}

#[inline(always)]
unsafe fn augmented__output(
    ctx: *mut c_void,
    args: *mut augmented_args_payload,
    len: i32,
) -> i32 {
    /* If perf_event_output fails, return non-zero so that it gets recorded unaugmented */
    unsafe {
        bpf_perf_event_output(
            ctx,
            &raw mut __augmented_syscalls__ as *mut c_void,
            BPF_F_CURRENT_CPU,
            args as *const c_void,
            len,
        )
    }
}

#[inline(always)]
unsafe fn augmented__beauty_output(ctx: *mut c_void, data: *mut c_void, len: i32) -> i32 {
    unsafe {
        bpf_perf_event_output(
            ctx,
            &raw mut __augmented_syscalls__ as *mut c_void,
            BPF_F_CURRENT_CPU,
            data as *const c_void,
            len,
        )
    }
}

#[inline(always)]
unsafe fn augmented_arg__read_str(
    augmented_arg: *mut augmented_arg,
    arg: *const c_void,
    arg_len: u32,
) -> u32 {
    let mut augmented_len: u32 = size_of::<augmented_arg>() as u32;
    let string_len: i32 =
        unsafe { bpf_probe_read_user_str((*augmented_arg).u.value.as_mut_ptr() as *mut c_void, arg_len, arg) };

    unsafe {
        (*augmented_arg).size = 0;
        (*augmented_arg).err = 0;
    }
    /*
     * probe_read_str may return < 0, e.g. -EFAULT
     * So we leave that in the augmented_arg->size that userspace will
     */
    if string_len > 0 {
        augmented_len -= (size_of::<[i8; PATH_MAX]>() as i32 - string_len) as u32;
        const _: () = assert!(is_power_of_2(size_of::<[i8; PATH_MAX]>()));
        augmented_len &= (size_of::<[i8; PATH_MAX]>() - 1) as u32;
        unsafe {
            (*augmented_arg).size = string_len as u32;
        }
    } else {
        /*
         * So that username notice the error while still being able
         * to skip this augmented arg record
         */
        unsafe {
            (*augmented_arg).err = string_len;
        }
        augmented_len = offset_of!(augmented_arg, u) as u32;
    }

    augmented_len
}

#[unsafe(link_section = "tp/raw_syscalls/sys_enter")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syscall_unaugmented(_args: *mut syscall_enter_args) -> i32 {
    1
}

/*
 * These will be tail_called from SEC("raw_syscalls:sys_enter"), so will find in
 * augmented_args_tmp what was read by that raw_syscalls:sys_enter and go
 * on from there, reading the first syscall arg as a string, i.e. open's
 * filename.
 */
#[unsafe(link_section = "tp/syscalls/sys_enter_connect")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_enter_connect(args: *mut syscall_enter_args) -> i32 {
    let augmented_args = unsafe { augmented_args_payload_ptr() };
    let sockaddr_arg = unsafe { (*args).args[1] as *const c_void };
    let mut socklen: u32 = unsafe { (*args).args[2] as u32 };
    let len: u32 = (size_of::<u64>() + size_of::<syscall_enter_args>()) as u32; // the size + err in all 'augmented_arg' structs

    if augmented_args.is_null() {
        return 1; /* Failure: don't filter */
    }

    const _: () = assert!(is_power_of_2(size_of::<sockaddr_storage>()));
    socklen &= (size_of::<sockaddr_storage>() - 1) as u32;

    unsafe {
        bpf_probe_read_user(
            &mut (*augmented_args).arg.u.saddr as *mut _ as *mut c_void,
            socklen,
            sockaddr_arg,
        );
        (*augmented_args).arg.size = socklen;
        (*augmented_args).arg.err = 0;

        augmented__output(args as *mut c_void, augmented_args, (len + socklen) as i32)
    }
}

#[unsafe(link_section = "tp/syscalls/sys_enter_sendto")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_enter_sendto(args: *mut syscall_enter_args) -> i32 {
    let augmented_args = unsafe { augmented_args_payload_ptr() };
    let sockaddr_arg = unsafe { (*args).args[4] as *const c_void };
    let mut socklen: u32 = unsafe { (*args).args[5] as u32 };
    let len: u32 = (size_of::<u64>() + size_of::<syscall_enter_args>()) as u32; // the size + err in all 'augmented_arg' structs

    if augmented_args.is_null() {
        return 1; /* Failure: don't filter */
    }

    socklen &= (size_of::<sockaddr_storage>() - 1) as u32;

    unsafe {
        bpf_probe_read_user(
            &mut (*augmented_args).arg.u.saddr as *mut _ as *mut c_void,
            socklen,
            sockaddr_arg,
        );

        augmented__output(args as *mut c_void, augmented_args, (len + socklen) as i32)
    }
}

#[unsafe(link_section = "tp/syscalls/sys_enter_open")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_enter_open(args: *mut syscall_enter_args) -> i32 {
    let augmented_args = unsafe { augmented_args_payload_ptr() };
    let filename_arg = unsafe { (*args).args[0] as *const c_void };
    let mut len: u32 = size_of::<syscall_enter_args>() as u32;

    if augmented_args.is_null() {
        return 1; /* Failure: don't filter */
    }

    unsafe {
        len += augmented_arg__read_str(
            &mut (*augmented_args).arg,
            filename_arg,
            size_of::<[i8; PATH_MAX]>() as u32,
        );

        augmented__output(args as *mut c_void, augmented_args, len as i32)
    }
}

#[unsafe(link_section = "tp/syscalls/sys_enter_openat")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_enter_openat(args: *mut syscall_enter_args) -> i32 {
    let augmented_args = unsafe { augmented_args_payload_ptr() };
    let filename_arg = unsafe { (*args).args[1] as *const c_void };
    let mut len: u32 = size_of::<syscall_enter_args>() as u32;

    if augmented_args.is_null() {
        return 1; /* Failure: don't filter */
    }

    unsafe {
        len += augmented_arg__read_str(
            &mut (*augmented_args).arg,
            filename_arg,
            size_of::<[i8; PATH_MAX]>() as u32,
        );

        augmented__output(args as *mut c_void, augmented_args, len as i32)
    }
}

#[unsafe(link_section = "tp/syscalls/sys_enter_rename")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_enter_rename(args: *mut syscall_enter_args) -> i32 {
    let augmented_args = unsafe { augmented_args_payload_ptr() };
    let oldpath_arg = unsafe { (*args).args[0] as *const c_void };
    let newpath_arg = unsafe { (*args).args[1] as *const c_void };
    let mut len: u32 = size_of::<syscall_enter_args>() as u32;
    let oldpath_len: u32;
    let newpath_len: u32;

    if augmented_args.is_null() {
        return 1; /* Failure: don't filter */
    }

    len += 2 * size_of::<u64>() as u32; // The overhead of size and err, just before the payload...

    unsafe {
        oldpath_len = augmented_arg__read_str(
            &mut (*augmented_args).arg,
            oldpath_arg,
            size_of::<[i8; PATH_MAX]>() as u32,
        );
        (*augmented_args).arg.size = PERF_ALIGN((oldpath_len + 1) as usize, size_of::<u64>()) as u32;
        len += (*augmented_args).arg.size;

        /* Every read from userspace is limited to value size */
        if (*augmented_args).arg.size > size_of::<[i8; PATH_MAX]>() as u32 {
            return 1; /* Failure: don't filter */
        }

        let arg2 = ((*augmented_args).arg.u.value.as_mut_ptr() as *mut c_void)
            .add((*augmented_args).arg.size as usize) as *mut augmented_arg;

        newpath_len =
            augmented_arg__read_str(arg2, newpath_arg, size_of::<[i8; PATH_MAX]>() as u32);
        (*arg2).size = newpath_len;

        len += newpath_len;

        augmented__output(args as *mut c_void, augmented_args, len as i32)
    }
}

#[unsafe(link_section = "tp/syscalls/sys_enter_renameat2")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_enter_renameat2(args: *mut syscall_enter_args) -> i32 {
    let augmented_args = unsafe { augmented_args_payload_ptr() };
    let oldpath_arg = unsafe { (*args).args[1] as *const c_void };
    let newpath_arg = unsafe { (*args).args[3] as *const c_void };
    let mut len: u32 = size_of::<syscall_enter_args>() as u32;
    let oldpath_len: u32;
    let newpath_len: u32;

    if augmented_args.is_null() {
        return 1; /* Failure: don't filter */
    }

    len += 2 * size_of::<u64>() as u32; // The overhead of size and err, just before the payload...

    unsafe {
        oldpath_len = augmented_arg__read_str(
            &mut (*augmented_args).arg,
            oldpath_arg,
            size_of::<[i8; PATH_MAX]>() as u32,
        );
        (*augmented_args).arg.size = PERF_ALIGN((oldpath_len + 1) as usize, size_of::<u64>()) as u32;
        len += (*augmented_args).arg.size;

        /* Every read from userspace is limited to value size */
        if (*augmented_args).arg.size > size_of::<[i8; PATH_MAX]>() as u32 {
            return 1; /* Failure: don't filter */
        }

        let arg2 = ((*augmented_args).arg.u.value.as_mut_ptr() as *mut c_void)
            .add((*augmented_args).arg.size as usize) as *mut augmented_arg;

        newpath_len =
            augmented_arg__read_str(arg2, newpath_arg, size_of::<[i8; PATH_MAX]>() as u32);
        (*arg2).size = newpath_len;

        len += newpath_len;

        augmented__output(args as *mut c_void, augmented_args, len as i32)
    }
}

// we need just the start, get the size to then copy it
#[repr(C)]
pub struct perf_event_attr_size {
    pub type_: __u32,
    /*
     * Size of the attr structure, for fwd/bwd compat.
     */
    pub size: __u32,
}

#[unsafe(link_section = "tp/syscalls/sys_enter_perf_event_open")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_enter_perf_event_open(args: *mut syscall_enter_args) -> i32 {
    let augmented_args = unsafe { augmented_args_payload_ptr() };
    let attr = unsafe { (*args).args[0] as *const perf_event_attr_size };
    let attr_read: *const perf_event_attr_size;
    let len: u32 = (size_of::<u64>() + size_of::<syscall_enter_args>()) as u32; // the size + err in all 'augmented_arg' structs

    if augmented_args.is_null() {
        return 1; /* Failure: don't filter */
    }

    unsafe {
        if bpf_probe_read_user(
            (*augmented_args).arg.u.value.as_mut_ptr() as *mut c_void,
            size_of::<perf_event_attr_size>() as u32,
            attr as *const c_void,
        ) < 0
        {
            return 1; /* Failure: don't filter */
        }

        attr_read = (*augmented_args).arg.u.value.as_ptr() as *const perf_event_attr_size;

        let mut size: __u32 = (*attr_read).size;

        if size == 0 {
            size = PERF_ATTR_SIZE_VER0;
        }

        if size > size_of::<[i8; PATH_MAX]>() as __u32 {
            return 1; /* Failure: don't filter */
        }

        // Now that we read attr->size and tested it against the size limits, read it completely
        if bpf_probe_read_user(
            (*augmented_args).arg.u.value.as_mut_ptr() as *mut c_void,
            size,
            attr as *const c_void,
        ) < 0
        {
            return 1; /* Failure: don't filter */
        }

        augmented__output(args as *mut c_void, augmented_args, (len + size) as i32)
    }
}

#[unsafe(link_section = "tp/syscalls/sys_enter_clock_nanosleep")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_enter_clock_nanosleep(args: *mut syscall_enter_args) -> i32 {
    let augmented_args = unsafe { augmented_args_payload_ptr() };
    let rqtp_arg = unsafe { (*args).args[2] as *const c_void };
    let len: u32 = (size_of::<u64>() + size_of::<syscall_enter_args>()) as u32; // the size + err in all 'augmented_arg' structs
    let size: __u32 = size_of::<timespec64>() as __u32;

    if augmented_args.is_null() {
        return 1; /* Failure: don't filter */
    }

    if size > size_of::<[i8; PATH_MAX]>() as __u32 {
        return 1; /* Failure: don't filter */
    }

    unsafe {
        bpf_probe_read_user(
            (*augmented_args).arg.u.value.as_mut_ptr() as *mut c_void,
            size,
            rqtp_arg,
        );

        augmented__output(args as *mut c_void, augmented_args, (len + size) as i32)
    }
}

#[unsafe(link_section = "tp/syscalls/sys_enter_nanosleep")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_enter_nanosleep(args: *mut syscall_enter_args) -> i32 {
    let augmented_args = unsafe { augmented_args_payload_ptr() };
    let req_arg = unsafe { (*args).args[0] as *const c_void };
    let len: u32 = size_of::<syscall_enter_args>() as u32;
    let size: __u32 = size_of::<timespec64>() as __u32;

    if augmented_args.is_null() {
        return 1; /* Failure: don't filter */
    }

    if size > size_of::<[i8; PATH_MAX]>() as __u32 {
        return 1; /* Failure: don't filter */
    }

    unsafe {
        bpf_probe_read_user(
            (*augmented_args).arg.u.value.as_mut_ptr() as *mut c_void,
            size,
            req_arg,
        );

        augmented__output(args as *mut c_void, augmented_args, (len + size) as i32)
    }
}

unsafe fn getpid() -> pid_t {
    unsafe { bpf_get_current_pid_tgid() as pid_t }
}

unsafe fn pid_filter__has(pids: *mut bpf_map_def, pid: pid_t) -> bool {
    unsafe {
        !bpf_map_lookup_elem(
            pids as *mut c_void,
            &pid as *const _ as *const c_void,
        )
        .is_null()
    }
}

#[unsafe(no_mangle)]
pub static mut ZERO: u64 = 0;

/*
 * Determine what type of argument and how many bytes to read from user space, using the
 * value in the beauty_map. This is the relation of parameter type and its corresponding
 * value in the beauty map, and how many bytes we read eventually:
 *
 * string: 1                         -> size of string
 * struct: size of struct            -> size of struct
 * buffer: -1 * (index of paired len) -> value of paired len (maximum: TRACE_AUG_MAX_BUF)
 */
#[inline(always)]
unsafe fn augment_arg(
    args: *mut syscall_enter_args,
    i: i32,
    beauty_map: *mut u32,
    payload: *mut beauty_payload_enter,
    offset: u64,
) -> i32 {
    let mut index: i32;
    let value_size: i32 =
        (size_of::<augmented_arg>() - offset_of!(augmented_arg, u)) as i32;
    let payload_offset: *mut augmented_arg;
    let mut aug_size: s64;
    let size: s64;
    let mut augmented: bool;
    let arg: *mut c_void;

    unsafe {
        arg = (*args).args[i as usize] as *mut c_void;
        augmented = false;
        size = *beauty_map.add(i as usize) as s64;
        aug_size = size; /* size of the augmented data read from user space */

        if size == 0 || arg.is_null() {
            return 0;
        }

        /* bounds check for the verifier */
        if offset
            > (size_of::<[augmented_arg; 6]>() - size_of::<augmented_arg>()) as u64
        {
            return -1;
        }
        barrier_var(offset);
        payload_offset =
            ((&mut (*payload).aug_args as *mut _ as *mut c_void).add(offset as usize))
                as *mut augmented_arg;

        if size == 1 {
            /* string */
            aug_size = bpf_probe_read_user_str(
                (*payload_offset).u.value.as_mut_ptr() as *mut c_void,
                value_size as u32,
                arg,
            ) as s64;
            /* minimum of 0 to pass the verifier */
            if aug_size < 0 {
                aug_size = 0;
            }

            augmented = true;
        } else if size > 0 && size <= value_size as s64 {
            /* struct */
            if bpf_probe_read_user(
                (*payload_offset).u.value.as_mut_ptr() as *mut c_void,
                size as u32,
                arg,
            ) == 0
            {
                augmented = true;
            }
        } else if (size as i32) < 0 && size >= -6 {
            /* buffer */
            index = -((size + 1) as i32);
            barrier_var(index); // Prevent clang (noticed with v18) from removing the &= 7 trick.
            index &= 7; // Satisfy the bounds checking with the verifier in some kernels.
            aug_size = if (*args).args[index as usize] as u64 > TRACE_AUG_MAX_BUF {
                TRACE_AUG_MAX_BUF as s64
            } else {
                (*args).args[index as usize] as s64
            };

            if aug_size > 0 {
                if bpf_probe_read_user(
                    (*payload_offset).u.value.as_mut_ptr() as *mut c_void,
                    aug_size as u32,
                    arg,
                ) == 0
                {
                    augmented = true;
                }
            }
        }

        /* Augmented data size is limited to sizeof(augmented_arg->unnamed union with value field) */
        if aug_size > value_size as s64 {
            aug_size = value_size as s64;
        }

        /* write data to payload */
        if augmented {
            let written: i32 = offset_of!(augmented_arg, u) as i32 + aug_size as i32;

            if written < 0 || written > size_of::<augmented_arg>() as i32 {
                return -1;
            }

            (*payload_offset).size = aug_size as u32;
            return written;
        }
    }

    0
}

unsafe fn augment_sys_enter(ctx: *mut c_void, args: *mut syscall_enter_args) -> i32 {
    let mut do_output: bool = false;
    let mut i: i32;
    let zero: i32 = 0;
    let mut written: i32;
    let mut output: u64 = 0; /* has to be u64, otherwise it won't pass the verifier */
    let nr: u32;
    let beauty_map: *mut u32;
    let payload: *mut beauty_payload_enter;

    /* fall back to do predefined tail call */
    if args.is_null() {
        return 1;
    }

    unsafe {
        /* use syscall number to get beauty_map entry */
        nr = (*args).syscall_nr as __u32;
        beauty_map = bpf_map_lookup_elem(
            &raw mut beauty_map_enter as *mut c_void,
            &nr as *const _ as *const c_void,
        ) as *mut u32;

        /* set up payload for output */
        payload = bpf_map_lookup_elem(
            &raw mut beauty_payload_enter_map as *mut c_void,
            &zero as *const _ as *const c_void,
        ) as *mut beauty_payload_enter;

        if beauty_map.is_null() || payload.is_null() {
            return 1;
        }

        /* copy the sys_enter header, which has the syscall_nr */
        ptr::copy_nonoverlapping(args, &mut (*payload).args, 1);

        if bpf_ksym_exists(&raw const bpf_iter_num_new as *const c_void) {
            /* Original C uses bpf_for(i, 0, 6) when bpf_iter_num_new exists. */
            i = 0;
            while i < 6 {
                written = augment_arg(args, i, beauty_map, payload, output);
                if written < 0 {
                    return 1;
                }
                if written > 0 {
                    output += written as u64;
                    /*
                     * guide the verifier to forget range of `output`, which
                     * helps to prove convergence of the loop
                     */
                    output += ZERO;
                    do_output = true;
                }
                i += 1;
            }
        } else {
            i = 0;
            while i < 6 {
                written = augment_arg(args, i, beauty_map, payload, output);
                if written < 0 {
                    return 1;
                }
                if written > 0 {
                    output += written as u64;
                    do_output = true;
                }
                i += 1;
            }
        }

        if !do_output
            || (size_of::<syscall_enter_args>() as u64 + output)
                > size_of::<beauty_payload_enter>() as u64
        {
            return 1;
        }

        augmented__beauty_output(
            ctx,
            payload as *mut c_void,
            (size_of::<syscall_enter_args>() as u64 + output) as i32,
        )
    }
}

#[unsafe(link_section = "tp/raw_syscalls/sys_enter")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_enter(args: *mut syscall_enter_args) -> i32 {
    let augmented_args: *mut augmented_args_payload;
    /*
     * We start len, the amount of data that will be in the perf ring
     * buffer, if this is not filtered out by one of pid_filter__has(),
     * syscall->enabled, etc, with the non-augmented raw syscall payload,
     * i.e. sizeof(augmented_args->args).
     *
     * We'll add to this as we add augmented syscalls right after that
     * initial, non-augmented raw_syscalls:sys_enter payload.
     */

    unsafe {
        if pid_filter__has(&raw mut pids_filtered, getpid()) {
            return 0;
        }

        augmented_args = augmented_args_payload_ptr();
        if augmented_args.is_null() {
            return 1;
        }

        bpf_probe_read_kernel(
            &mut (*augmented_args).args as *mut _ as *mut c_void,
            size_of::<syscall_enter_args>(),
            args as *const c_void,
        );

        /*
         * Jump to syscall specific augmenter, even if the default one,
         * "!raw_syscalls:unaugmented" that will just return 1 to return the
         * unaugmented tracepoint payload.
         */
        if augment_sys_enter(args as *mut c_void, &mut (*augmented_args).args) != 0 {
            bpf_tail_call(
                args as *mut c_void,
                &raw mut syscalls_sys_enter as *mut c_void,
                (*augmented_args).args.syscall_nr,
            );
        }

        // If not found on the PROG_ARRAY syscalls map, then we're filtering it:
        0
    }
}

#[unsafe(link_section = "tp/raw_syscalls/sys_exit")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_exit(args: *mut syscall_exit_args) -> i32 {
    let mut exit_args: syscall_exit_args = syscall_exit_args {
        common_tp_fields: 0,
        syscall_nr: 0,
        ret: 0,
    };

    unsafe {
        if pid_filter__has(&raw mut pids_filtered, getpid()) {
            return 0;
        }

        bpf_probe_read_kernel(
            &mut exit_args as *mut _ as *mut c_void,
            size_of::<syscall_exit_args>(),
            args as *const c_void,
        );
        /*
         * Jump to syscall specific return augmenter, even if the default one,
         * "!raw_syscalls:unaugmented" that will just return 1 to return the
         * unaugmented tracepoint payload.
         */
        bpf_tail_call(
            args as *mut c_void,
            &raw mut syscalls_sys_exit as *mut c_void,
            exit_args.syscall_nr,
        );
        /*
         * If not found on the PROG_ARRAY syscalls map, then we're filtering it:
         */
        0
    }
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
