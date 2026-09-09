/* SPDX-License-Identifier: GPL-2.0 */

/* C header guard: _ASM_X86_IA32_H */

/* 32 bit structures for IA32 support. */

#[cfg(CONFIG_IA32_EMULATION)]
#[repr(C)]
pub struct UcontextIa32 {
    pub uc_flags: ::core::ffi::c_uint,
    pub uc_link: ::core::ffi::c_uint,
    pub uc_stack: compat_stack_t,
    pub uc_mcontext: sigcontext_32,
    /* mask last for extensibility */
    pub uc_sigmask: compat_sigset_t,
}

/* This matches struct stat64 in glibc2.2, hence the absolutely
 * insane amounts of padding around dev_t's.
 */
#[cfg(CONFIG_IA32_EMULATION)]
#[repr(C, packed)]
pub struct Stat64 {
    pub st_dev: u64,
    pub __pad0: [u8; 4],
    pub __st_ino: u32,
    pub st_mode: u32,
    pub st_nlink: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub st_rdev: u64,
    pub __pad3: [u8; 4],
    pub st_size: i64,
    pub st_blksize: u32,
    /* Number 512-byte blocks allocated */
    pub st_blocks: i64,
    pub st_atime: u32,
    pub st_atime_nsec: u32,
    pub st_mtime: u32,
    pub st_mtime_nsec: u32,
    pub st_ctime: u32,
    pub st_ctime_nsec: u32,
    pub st_ino: u64,
}

#[cfg(CONFIG_IA32_EMULATION)]
pub const STAT64_HAS_BROKEN_ST_INO: u32 = 1;

#[cfg(CONFIG_IA32_EMULATION)]
unsafe extern "C" {
    pub static mut __ia32_enabled: bool;
}

#[cfg(CONFIG_IA32_EMULATION)]
#[inline(always)]
pub unsafe fn ia32_enabled() -> bool {
    __ia32_enabled
}

#[cfg(CONFIG_IA32_EMULATION)]
#[inline]
pub unsafe fn ia32_disable() {
    __ia32_enabled = false;
}

#[cfg(not(CONFIG_IA32_EMULATION))]
#[inline(always)]
pub const fn ia32_enabled() -> bool {
    /* Equivalent to IS_ENABLED(CONFIG_X86_32). */
    cfg!(CONFIG_X86_32)
}

#[cfg(not(CONFIG_IA32_EMULATION))]
#[inline]
pub const fn ia32_disable() {}

#[inline]
pub unsafe fn ia32_enabled_verbose() -> bool {
    let enabled = ia32_enabled();

    if cfg!(CONFIG_IA32_EMULATION) && !enabled {
        /* Equivalent to pr_notice_once(...). */
        pr_notice_once("32-bit emulation disabled. You can reenable with ia32_emulation=on\n");
    }

    enabled
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
