/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Architecture specific compatibility types.
 *
 * The C header includes Linux and x86 headers; their symbols are expected to
 * be supplied by the surrounding translation unit.
 */

pub type compat_mode_t = u16;
pub type __compat_uid_t = u16;
pub type __compat_gid_t = u16;
pub type compat_dev_t = u16;
pub type compat_ipc_pid_t = u16;

pub const COMPAT_UTS_MACHINE: &[u8] = b"i686\0\0";

pub type compat_nlink_t = u16;

#[repr(C)]
pub struct compat_stat {
    pub st_dev: u32,
    pub st_ino: compat_ino_t,
    pub st_mode: compat_mode_t,
    pub st_nlink: compat_nlink_t,
    pub st_uid: __compat_uid_t,
    pub st_gid: __compat_gid_t,
    pub st_rdev: u32,
    pub st_size: u32,
    pub st_blksize: u32,
    pub st_blocks: u32,
    pub st_atime: u32,
    pub st_atime_nsec: u32,
    pub st_mtime: u32,
    pub st_mtime_nsec: u32,
    pub st_ctime: u32,
    pub st_ctime_nsec: u32,
    pub __unused4: u32,
    pub __unused5: u32,
}

/* IA32 uses 4 byte alignment for 64 bit quantities; compat flock64 is packed. */
/* C: #define __ARCH_NEED_COMPAT_FLOCK64_PACKED */

#[repr(C)]
pub struct compat_statfs {
    pub f_type: core::ffi::c_int,
    pub f_bsize: core::ffi::c_int,
    pub f_blocks: core::ffi::c_int,
    pub f_bfree: core::ffi::c_int,
    pub f_bavail: core::ffi::c_int,
    pub f_files: core::ffi::c_int,
    pub f_ffree: core::ffi::c_int,
    pub f_fsid: compat_fsid_t,
    pub f_namelen: core::ffi::c_int, /* SunOS ignores this field. */
    pub f_frsize: core::ffi::c_int,
    pub f_flags: core::ffi::c_int,
    pub f_spare: [core::ffi::c_int; 4],
}

/* CONFIG_X86_X32_ABI controls this build-time definition in the C header. */
#[cfg(feature = "CONFIG_X86_X32_ABI")]
#[macro_export]
macro_rules! COMPAT_USE_64BIT_TIME {
    () => {{
        !!(task_pt_regs(current).orig_ax & __X32_SYSCALL_BIT)
    }};
}

pub unsafe fn in_x32_syscall() -> bool {
    #[cfg(feature = "CONFIG_X86_X32_ABI")]
    {
        if task_pt_regs(current).orig_ax & __X32_SYSCALL_BIT != 0 {
            return true;
        }
    }
    false
}

pub unsafe fn in_32bit_syscall() -> bool {
    in_ia32_syscall() || in_x32_syscall()
}

#[cfg(feature = "CONFIG_COMPAT")]
pub unsafe fn in_compat_syscall() -> bool {
    in_32bit_syscall()
}

/* C macro aliases: in_compat_syscall and compat_need_64bit_alignment_fixup. */

#[repr(C)]
pub struct compat_siginfo {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_X86_X32_ABI")]
unsafe extern "C" {
    pub fn copy_siginfo_to_user32(
        to: *mut compat_siginfo,
        from: *const kernel_siginfo_t,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
