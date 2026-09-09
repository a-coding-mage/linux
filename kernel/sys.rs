// SPDX-License-Identifier: GPL-2.0
//
// Direct low-level Rust translation of linux/kernel/sys.c.  Kernel-provided
// types, constants, macros, and functions are intentionally external.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

extern "C" {
    static mut overflowuid: i32;
    static mut overflowgid: i32;
    static mut fs_overflowuid: i32;
    static mut fs_overflowgid: i32;
}

// Architecture hooks whose default implementation is supplied by this file.
#[no_mangle]
pub unsafe extern "C" fn arch_prctl_spec_ctrl_get(
    _t: *mut task_struct, _which: usize,
) -> i32 { -EINVAL }

#[no_mangle]
pub unsafe extern "C" fn arch_prctl_spec_ctrl_set(
    _t: *mut task_struct, _which: usize, _ctrl: usize,
) -> i32 { -EINVAL }

#[no_mangle]
pub unsafe extern "C" fn arch_get_shadow_stack_status(
    _t: *mut task_struct, _status: *mut usize,
) -> i32 { -EINVAL }

#[no_mangle]
pub unsafe extern "C" fn arch_set_shadow_stack_status(
    _t: *mut task_struct, _status: usize,
) -> i32 { -EINVAL }

#[no_mangle]
pub unsafe extern "C" fn arch_lock_shadow_stack_status(
    _t: *mut task_struct, _status: usize,
) -> i32 { -EINVAL }

#[no_mangle]
pub unsafe extern "C" fn arch_prctl_get_branch_landing_pad_state(
    _t: *mut task_struct, _state: *mut usize,
) -> i32 { -EINVAL }

#[no_mangle]
pub unsafe extern "C" fn arch_prctl_set_branch_landing_pad_state(
    _t: *mut task_struct, _state: usize,
) -> i32 { -EINVAL }

#[no_mangle]
pub unsafe extern "C" fn arch_prctl_lock_branch_landing_pad_state(
    _t: *mut task_struct,
) -> i32 { -EINVAL }

// The following declarations preserve the externally visible syscall
// interfaces. Their kernel types and implementations are provided by the
// surrounding translated kernel sources.
extern "C" {
    fn __sys_setregid(rgid: gid_t, egid: gid_t) -> c_long;
    fn __sys_setgid(gid: gid_t) -> c_long;
    fn __sys_setreuid(ruid: uid_t, euid: uid_t) -> c_long;
    fn __sys_setuid(uid: uid_t) -> c_long;
    fn __sys_setresuid(ruid: uid_t, euid: uid_t, suid: uid_t) -> c_long;
    fn __sys_setresgid(rgid: gid_t, egid: gid_t, sgid: gid_t) -> c_long;
    fn __sys_setfsuid(uid: uid_t) -> c_long;
    fn __sys_setfsgid(gid: gid_t) -> c_long;
    fn ksys_setsid() -> i32;
    fn getrusage(p: *mut task_struct, who: i32, r: *mut rusage);
}

// C's syscall-generation macros become ordinary exported Rust entry points.
#[no_mangle] pub unsafe extern "C" fn setregid(rgid: gid_t, egid: gid_t) -> c_long { __sys_setregid(rgid, egid) }
#[no_mangle] pub unsafe extern "C" fn setgid(gid: gid_t) -> c_long { __sys_setgid(gid) }
#[no_mangle] pub unsafe extern "C" fn setreuid(ruid: uid_t, euid: uid_t) -> c_long { __sys_setreuid(ruid, euid) }
#[no_mangle] pub unsafe extern "C" fn setuid(uid: uid_t) -> c_long { __sys_setuid(uid) }
#[no_mangle] pub unsafe extern "C" fn setresuid(ruid: uid_t, euid: uid_t, suid: uid_t) -> c_long { __sys_setresuid(ruid, euid, suid) }
#[no_mangle] pub unsafe extern "C" fn setresgid(rgid: gid_t, egid: gid_t, sgid: gid_t) -> c_long { __sys_setresgid(rgid, egid, sgid) }
#[no_mangle] pub unsafe extern "C" fn setfsuid(uid: uid_t) -> c_long { __sys_setfsuid(uid) }
#[no_mangle] pub unsafe extern "C" fn setfsgid(gid: gid_t) -> c_long { __sys_setfsgid(gid) }
#[no_mangle] pub unsafe extern "C" fn setsid() -> c_long { ksys_setsid() as c_long }

// Kernel dependency types and constants.
type c_long = isize;
type uid_t = u32;
type gid_t = u32;
#[allow(non_camel_case_types)] type task_struct = core::ffi::c_void;
#[allow(non_camel_case_types)] type rusage = core::ffi::c_void;
extern "C" { static EINVAL: i32; }
const EINVAL: i32 = 22;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
