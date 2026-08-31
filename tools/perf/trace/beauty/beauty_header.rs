/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

pub type size_t = usize;
pub type pid_t = c_int;
pub type u8_t = u8;
pub type u16_t = u16;
pub type u64_t = u64;

#[repr(C)]
pub struct strarray {
    pub offset: u64,
    pub nr_entries: c_int,
    pub prefix: *const c_char,
    pub entries: *const *const c_char,
}

macro_rules! DEFINE_STRARRAY {
    ($array:ident, $_prefix:expr) => {
        strarray {
            offset: 0,
            nr_entries: $array.len() as c_int,
            entries: $array.as_ptr(),
            prefix: $_prefix,
        }
    };
}

macro_rules! DEFINE_STRARRAY_OFFSET {
    ($array:ident, $_prefix:expr, $off:expr) => {
        strarray {
            offset: $off,
            nr_entries: $array.len() as c_int,
            entries: $array.as_ptr(),
            prefix: $_prefix,
        }
    };
}

extern "C" {
    pub fn strarray__scnprintf(
        sa: *mut strarray,
        bf: *mut c_char,
        size: size_t,
        intfmt: *const c_char,
        show_prefix: bool,
        val: c_int,
    ) -> size_t;
    pub fn strarray__scnprintf_suffix(
        sa: *mut strarray,
        bf: *mut c_char,
        size: size_t,
        intfmt: *const c_char,
        show_suffix: bool,
        val: c_int,
    ) -> size_t;
    pub fn strarray__scnprintf_flags(
        sa: *mut strarray,
        bf: *mut c_char,
        size: size_t,
        show_prefix: bool,
        flags: c_ulong,
    ) -> size_t;

    pub fn strarray__strtoul(sa: *mut strarray, bf: *mut c_char, size: size_t, ret: *mut u64) -> bool;
    pub fn strarray__strtoul_flags(
        sa: *mut strarray,
        bf: *mut c_char,
        size: size_t,
        ret: *mut u64,
    ) -> bool;
}

#[repr(C)]
pub struct trace {
    _private: [u8; 0],
}

extern "C" {
    pub fn trace__show_zeros(trace: *const trace) -> bool;
}

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

extern "C" {
    pub fn trace__host(trace: *const trace) -> *mut machine;
}

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file {
    pub pathname: *mut c_char,
    pub dev_maj: c_int,
}

extern "C" {
    pub fn thread__files_entry(thread: *mut thread, fd: c_int) -> *mut file;
}

#[repr(C)]
pub struct strarrays {
    pub nr_entries: c_int,
    pub entries: *mut *mut strarray,
}

macro_rules! DEFINE_STRARRAYS {
    ($array:ident) => {
        strarrays {
            nr_entries: $array.len() as c_int,
            entries: $array.as_ptr() as *mut *mut strarray,
        }
    };
}

extern "C" {
    pub fn strarrays__scnprintf(
        sas: *mut strarrays,
        bf: *mut c_char,
        size: size_t,
        intfmt: *const c_char,
        show_prefix: bool,
        val: c_int,
    ) -> size_t;

    pub fn strarrays__strtoul(
        sas: *mut strarrays,
        bf: *mut c_char,
        size: size_t,
        ret: *mut u64,
    ) -> bool;

    pub fn pid__scnprintf_fd(
        trace: *mut trace,
        pid: pid_t,
        fd: c_int,
        bf: *mut c_char,
        size: size_t,
    ) -> size_t;

    pub static mut strarray__socket_families: strarray;
    pub static mut strarray__socket_level: strarray;
}

/**
 * augmented_arg: extra payload for syscall pointer arguments
 *
 * If perf_sample->raw_size is more than what a syscall sys_enter_FOO puts, then
 * its the arguments contents, so that we can show more than just a
 * pointer. This will be done initially with eBPF, the start of that is at the
 * tools/perf/util/bpf_skel/augmented_syscalls.bpf.c that will eventually be
 * done automagically caching the running kernel tracefs events data into an
 * eBPF C script, that then gets compiled and its .o file cached for subsequent
 * use. For char pointers like the ones for 'open' like syscalls its easy, for
 * the rest we should use DWARF or better, BTF, much more compact.
 *
 * @size: 8 if all we need is an integer, otherwise all of the augmented arg.
 * @int_arg: will be used for integer like pointer contents, like 'accept's 'upeer_addrlen'
 * @value: u64 aligned, for structs, pathnames
 */
#[repr(C)]
pub struct augmented_arg {
    pub size: c_int,
    pub int_arg: c_int,
    pub value: [u64; 0],
}

#[repr(C)]
pub struct syscall_arg_fmt {
    _private: [u8; 0],
}

/**
 * @val: value of syscall argument being formatted
 * @len: for tracepoint dynamic arrays, if fmt->nr_entries == 0, then its not a fixed array, look at arg->len
 * @args: All the args, use syscall_args__val(arg, nth) to access one
 * @augmented_args: Extra data that can be collected, for instance, with eBPF for expanding the pathname for open, etc
 * @augmented_args_size: augmented_args total payload size
 * @thread: tid state (maps, pid, tid, etc)
 * @trace: 'perf trace' internals: all threads, etc
 * @parm: private area, may be an strarray, for instance
 * @idx: syscall arg idx (is this the first?)
 * @mask: a syscall arg may mask another arg, see syscall_arg__scnprintf_futex_op
 * @show_string_prefix: When there is a common prefix in a string table, show it or not
 */
#[repr(C)]
pub struct syscall_arg_augmented {
    pub args: *mut augmented_arg,
    pub size: c_int,
}

#[repr(C)]
pub struct syscall_arg {
    pub val: c_ulong,
    pub args: *mut u8,
    pub fmt: *mut syscall_arg_fmt,
    pub augmented: syscall_arg_augmented,
    pub thread: *mut thread,
    pub trace: *mut trace,
    pub parm: *mut c_void,
    pub type_name: *mut c_char,
    pub len: u16,
    pub idx: u8,
    pub mask: u8,
    pub show_string_prefix: bool,
}

pub type syscall_arg_scnprintf_t =
    Option<unsafe extern "C" fn(bf: *mut c_char, size: size_t, arg: *mut syscall_arg) -> size_t>;

extern "C" {
    pub fn syscall_arg__val(arg: *mut syscall_arg, idx: u8) -> c_ulong;

    pub fn syscall_arg__scnprintf_strarray_flags(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__strtoul_strarray(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
        ret: *mut u64,
    ) -> bool;
    pub fn syscall_arg__strtoul_strarray_flags(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
        ret: *mut u64,
    ) -> bool;
    pub fn syscall_arg__strtoul_strarrays(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
        ret: *mut u64,
    ) -> bool;
    pub fn syscall_arg__scnprintf_x86_irq_vectors(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__strtoul_x86_irq_vectors(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
        ret: *mut u64,
    ) -> bool;
    pub fn syscall_arg__scnprintf_x86_MSR(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__strtoul_x86_MSR(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
        ret: *mut u64,
    ) -> bool;
    pub fn syscall_arg__scnprintf_strarrays(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_fd(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_hex(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_ptr(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_int(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_long(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_pid(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_clone_flags(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_fcntl_cmd(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_fcntl_arg(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_flock(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_fsmount_flags(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_fsmount_attr_flags(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_fspick_flags(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_ioctl_cmd(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_kcmp_type(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_kcmp_idx(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__mask_val_mount_flags(arg: *mut syscall_arg, flags: c_ulong) -> c_ulong;
    pub fn syscall_arg__scnprintf_mount_flags(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_move_mount_flags(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_pkey_alloc_access_rights(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_open_flags(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_x86_arch_prctl_code(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_prctl_option(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;

    pub static mut strarray__prctl_options: strarray;

    pub fn syscall_arg__scnprintf_prctl_arg2(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_prctl_arg3(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_renameat2_flags(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_sockaddr(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_socket_protocol(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_socket_level(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_fs_at_flags(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_faccessat2_flags(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_statx_mask(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_sync_file_range_flags(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_timespec(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;

    pub fn open__scnprintf_flags(
        flags: c_ulong,
        bf: *mut c_char,
        size: size_t,
        show_prefix: bool,
    ) -> size_t;

    pub fn syscall_arg__set_ret_scnprintf(
        arg: *mut syscall_arg,
        ret_scnprintf: syscall_arg_scnprintf_t,
    );

    pub static mut strarray__fsconfig_cmds: strarray;

    pub fn syscall_arg__scnprintf_eventfd_flags(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_futex_op(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_futex_val3(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_mmap_prot(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;

    pub static mut strarray__mmap_flags: strarray;

    pub fn syscall_arg__scnprintf_mmap_flags(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_mremap_flags(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_madvise_behavior(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_mode_t(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_msg_flags(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_perf_flags(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_perf_event_attr(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_sched_policy(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_seccomp_op(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_seccomp_flags(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_signum(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_socket_type(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    pub fn syscall_arg__scnprintf_waitid_options(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
}

pub use syscall_arg__scnprintf_strarray_flags as SCA_STRARRAY_FLAGS;
pub use syscall_arg__strtoul_strarray as STUL_STRARRAY;
pub use syscall_arg__strtoul_strarray_flags as STUL_STRARRAY_FLAGS;
pub use syscall_arg__strtoul_strarrays as STUL_STRARRAYS;
pub use syscall_arg__scnprintf_x86_irq_vectors as SCA_X86_IRQ_VECTORS;
pub use syscall_arg__strtoul_x86_irq_vectors as STUL_X86_IRQ_VECTORS;
pub use syscall_arg__scnprintf_x86_MSR as SCA_X86_MSR;
pub use syscall_arg__strtoul_x86_MSR as STUL_X86_MSR;
pub use syscall_arg__scnprintf_strarrays as SCA_STRARRAYS;
pub use syscall_arg__scnprintf_fd as SCA_FD;
pub use syscall_arg__scnprintf_hex as SCA_HEX;
pub use syscall_arg__scnprintf_ptr as SCA_PTR;
pub use syscall_arg__scnprintf_int as SCA_INT;
pub use syscall_arg__scnprintf_long as SCA_LONG;
pub use syscall_arg__scnprintf_pid as SCA_PID;
pub use syscall_arg__scnprintf_clone_flags as SCA_CLONE_FLAGS;
pub use syscall_arg__scnprintf_fcntl_cmd as SCA_FCNTL_CMD;
pub use syscall_arg__scnprintf_fcntl_arg as SCA_FCNTL_ARG;
pub use syscall_arg__scnprintf_flock as SCA_FLOCK;
pub use syscall_arg__scnprintf_fsmount_flags as SCA_FSMOUNT_FLAGS;
pub use syscall_arg__scnprintf_fsmount_attr_flags as SCA_FSMOUNT_ATTR_FLAGS;
pub use syscall_arg__scnprintf_fspick_flags as SCA_FSPICK_FLAGS;
pub use syscall_arg__scnprintf_ioctl_cmd as SCA_IOCTL_CMD;
pub use syscall_arg__scnprintf_kcmp_type as SCA_KCMP_TYPE;
pub use syscall_arg__scnprintf_kcmp_idx as SCA_KCMP_IDX;
pub use syscall_arg__mask_val_mount_flags as SCAMV_MOUNT_FLAGS;
pub use syscall_arg__scnprintf_mount_flags as SCA_MOUNT_FLAGS;
pub use syscall_arg__scnprintf_move_mount_flags as SCA_MOVE_MOUNT_FLAGS;
pub use syscall_arg__scnprintf_pkey_alloc_access_rights as SCA_PKEY_ALLOC_ACCESS_RIGHTS;
pub use syscall_arg__scnprintf_open_flags as SCA_OPEN_FLAGS;
pub use syscall_arg__scnprintf_x86_arch_prctl_code as SCA_X86_ARCH_PRCTL_CODE;
pub use syscall_arg__scnprintf_prctl_option as SCA_PRCTL_OPTION;
pub use syscall_arg__scnprintf_prctl_arg2 as SCA_PRCTL_ARG2;
pub use syscall_arg__scnprintf_prctl_arg3 as SCA_PRCTL_ARG3;
pub use syscall_arg__scnprintf_renameat2_flags as SCA_RENAMEAT2_FLAGS;
pub use syscall_arg__scnprintf_sockaddr as SCA_SOCKADDR;

// 'argname' is just documentational at this point, to remove the previous comment with that info.
// C macro intent: { .scnprintf = SCA_SOCKADDR, .from_user = true, }
macro_rules! SCA_SOCKADDR_FROM_USER {
    ($argname:ident) => {
        /* requires the external syscall_arg_fmt field layout */
    };
}

pub use syscall_arg__scnprintf_socket_protocol as SCA_SK_PROTO;
pub use syscall_arg__scnprintf_socket_level as SCA_SK_LEVEL;
pub use syscall_arg__scnprintf_fs_at_flags as SCA_FS_AT_FLAGS;
pub use syscall_arg__scnprintf_faccessat2_flags as SCA_FACCESSAT2_FLAGS;
pub use syscall_arg__scnprintf_statx_mask as SCA_STATX_MASK;
pub use syscall_arg__scnprintf_sync_file_range_flags as SCA_SYNC_FILE_RANGE_FLAGS;
pub use syscall_arg__scnprintf_timespec as SCA_TIMESPEC;

// 'argname' is just documentational at this point, to remove the previous comment with that info.
// C macro intent: { .scnprintf = SCA_TIMESPEC, .from_user = true, }
macro_rules! SCA_TIMESPEC_FROM_USER {
    ($argname:ident) => {
        /* requires the external syscall_arg_fmt field layout */
    };
}

pub use syscall_arg__scnprintf_eventfd_flags as SCA_EFD_FLAGS;
pub use syscall_arg__scnprintf_futex_op as SCA_FUTEX_OP;
pub use syscall_arg__scnprintf_futex_val3 as SCA_FUTEX_VAL3;
pub use syscall_arg__scnprintf_mmap_prot as SCA_MMAP_PROT;
pub use syscall_arg__scnprintf_mmap_flags as SCA_MMAP_FLAGS;
pub use syscall_arg__scnprintf_mremap_flags as SCA_MREMAP_FLAGS;
pub use syscall_arg__scnprintf_madvise_behavior as SCA_MADV_BHV;
pub use syscall_arg__scnprintf_mode_t as SCA_MODE_T;
pub use syscall_arg__scnprintf_msg_flags as SCA_MSG_FLAGS;
pub use syscall_arg__scnprintf_perf_flags as SCA_PERF_FLAGS;
pub use syscall_arg__scnprintf_perf_event_attr as SCA_PERF_ATTR;

// C macro intent: { .scnprintf = SCA_PERF_ATTR, .from_user = true, }
macro_rules! SCA_PERF_ATTR_FROM_USER {
    ($argname:ident) => {
        /* requires the external syscall_arg_fmt field layout */
    };
}

pub use syscall_arg__scnprintf_sched_policy as SCA_SCHED_POLICY;
pub use syscall_arg__scnprintf_seccomp_op as SCA_SECCOMP_OP;
pub use syscall_arg__scnprintf_seccomp_flags as SCA_SECCOMP_FLAGS;
pub use syscall_arg__scnprintf_signum as SCA_SIGNUM;
pub use syscall_arg__scnprintf_socket_type as SCA_SK_TYPE;
pub use syscall_arg__scnprintf_waitid_options as SCA_WAITID_OPTIONS;
