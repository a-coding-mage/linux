// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of testing/selftests/filesystems/wrappers.h.
// C header guard, _GNU_SOURCE, and include directives are C-only; the included
// headers provide Linux syscall numbers, mount constants, and C types.

use core::ffi::{c_char, c_long, c_void};

unsafe extern "C" {
    fn syscall(num: c_long, ...) -> c_long;
}

pub const STATX_MNT_ID_UNIQUE: u32 = 0x00004000; /* Want/got extended stx_mount_id */

pub unsafe fn sys_fsopen(fsname: *const c_char, flags: u32) -> i32 {
    unsafe { syscall(__NR_fsopen as c_long, fsname, flags) as i32 }
}

pub unsafe fn sys_fsconfig(
    fd: i32,
    cmd: u32,
    key: *const c_char,
    value: *const c_char,
    aux: i32,
) -> i32 {
    unsafe { syscall(__NR_fsconfig as c_long, fd, cmd, key, value, aux) as i32 }
}

pub unsafe fn sys_fsmount(fd: i32, flags: u32, attr_flags: u32) -> i32 {
    unsafe { syscall(__NR_fsmount as c_long, fd, flags, attr_flags) as i32 }
}

pub unsafe fn sys_mount(
    src: *const c_char,
    tgt: *const c_char,
    fst: *const c_char,
    flags: u64,
    data: *const c_void,
) -> i32 {
    unsafe { syscall(__NR_mount as c_long, src, tgt, fst, flags, data) as i32 }
}

pub const MOVE_MOUNT_F_EMPTY_PATH: u32 = 0x00000004; /* Empty from path permitted */
pub const MOVE_MOUNT_T_EMPTY_PATH: u32 = 0x00000040; /* Empty to path permitted */

/*
 * C fallback for missing __NR_move_mount:
 *   alpha: 539
 *   MIPS o32: 4429
 *   MIPS n32: 6429
 *   MIPS n64: 5429
 *   all other architectures: 429
 */
#[cfg(target_arch = "alpha")]
pub const __NR_move_mount: c_long = 539;
#[cfg(target_arch = "mips")]
pub const __NR_move_mount: c_long = 4429;
#[cfg(all(target_arch = "mips64", target_pointer_width = "32"))]
pub const __NR_move_mount: c_long = 6429;
#[cfg(all(target_arch = "mips64", target_pointer_width = "64"))]
pub const __NR_move_mount: c_long = 5429;
#[cfg(not(any(target_arch = "alpha", target_arch = "mips", target_arch = "mips64")))]
pub const __NR_move_mount: c_long = 429;

pub unsafe fn sys_move_mount(
    from_dfd: i32,
    from_pathname: *const c_char,
    to_dfd: i32,
    to_pathname: *const c_char,
    flags: u32,
) -> i32 {
    unsafe {
        syscall(
            __NR_move_mount as c_long,
            from_dfd,
            from_pathname,
            to_dfd,
            to_pathname,
            flags,
        ) as i32
    }
}

pub const OPEN_TREE_CLONE: u32 = 1;

// C fallback maps OPEN_TREE_CLOEXEC to O_CLOEXEC from system headers.
pub const OPEN_TREE_CLOEXEC: u32 = O_CLOEXEC as u32;

pub const AT_RECURSIVE: u32 = 0x8000; /* Apply to the entire subtree */

/*
 * C fallback for missing __NR_open_tree:
 *   alpha: 538
 *   MIPS o32: 4428
 *   MIPS n32: 6428
 *   MIPS n64: 5428
 *   all other architectures: 428
 */
#[cfg(target_arch = "alpha")]
pub const __NR_open_tree: c_long = 538;
#[cfg(target_arch = "mips")]
pub const __NR_open_tree: c_long = 4428;
#[cfg(all(target_arch = "mips64", target_pointer_width = "32"))]
pub const __NR_open_tree: c_long = 6428;
#[cfg(all(target_arch = "mips64", target_pointer_width = "64"))]
pub const __NR_open_tree: c_long = 5428;
#[cfg(not(any(target_arch = "alpha", target_arch = "mips", target_arch = "mips64")))]
pub const __NR_open_tree: c_long = 428;

pub unsafe fn sys_open_tree(dfd: i32, filename: *const c_char, flags: u32) -> i32 {
    unsafe { syscall(__NR_open_tree as c_long, dfd, filename, flags) as i32 }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
