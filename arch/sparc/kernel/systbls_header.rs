/* SPDX-License-Identifier: GPL-2.0 */

// Translated from systbls.h.
// The original Linux header dependencies are supplied by other translation units.

extern "C" {
    pub fn sys_getpagesize() -> core::ffi::c_long;
    pub fn sys_sparc_pipe() -> core::ffi::c_long;
    pub fn sys_nis_syscall() -> core::ffi::c_long;
    pub fn sys_getdomainname(
        name: *mut core::ffi::c_char,
        len: core::ffi::c_int,
    ) -> core::ffi::c_long;
    pub fn do_rt_sigreturn(regs: *mut pt_regs);
    pub fn sys_mmap(
        addr: core::ffi::c_ulong,
        len: core::ffi::c_ulong,
        prot: core::ffi::c_ulong,
        flags: core::ffi::c_ulong,
        fd: core::ffi::c_ulong,
        off: core::ffi::c_ulong,
    ) -> core::ffi::c_long;
    pub fn sparc_breakpoint(regs: *mut pt_regs);

    // CONFIG_SPARC32
    #[cfg(CONFIG_SPARC32)]
    pub fn sys_mmap2(
        addr: core::ffi::c_ulong,
        len: core::ffi::c_ulong,
        prot: core::ffi::c_ulong,
        flags: core::ffi::c_ulong,
        fd: core::ffi::c_ulong,
        pgoff: core::ffi::c_ulong,
    ) -> core::ffi::c_long;
    #[cfg(CONFIG_SPARC32)]
    pub fn sys_sparc_remap_file_pages(
        start: core::ffi::c_ulong,
        size: core::ffi::c_ulong,
        prot: core::ffi::c_ulong,
        pgoff: core::ffi::c_ulong,
        flags: core::ffi::c_ulong,
    ) -> core::ffi::c_long;

    // CONFIG_SPARC64
    #[cfg(CONFIG_SPARC64)]
    pub fn sys_sparc_ipc(
        call: core::ffi::c_uint,
        first: core::ffi::c_int,
        second: core::ffi::c_ulong,
        third: core::ffi::c_ulong,
        ptr: *mut core::ffi::c_void,
        fifth: core::ffi::c_long,
    ) -> core::ffi::c_long;
    #[cfg(CONFIG_SPARC64)]
    pub fn sparc64_personality(personality: core::ffi::c_ulong) -> core::ffi::c_long;
    #[cfg(CONFIG_SPARC64)]
    pub fn sys64_munmap(addr: core::ffi::c_ulong, len: size_t) -> core::ffi::c_long;
    #[cfg(CONFIG_SPARC64)]
    pub fn sys64_mremap(
        addr: core::ffi::c_ulong,
        old_len: core::ffi::c_ulong,
        new_len: core::ffi::c_ulong,
        flags: core::ffi::c_ulong,
        new_addr: core::ffi::c_ulong,
    ) -> core::ffi::c_ulong;
    #[cfg(CONFIG_SPARC64)]
    pub fn sys_utrap_install(
        r#type: utrap_entry_t,
        new_p: utrap_handler_t,
        new_d: utrap_handler_t,
        old_p: *mut utrap_handler_t,
        old_d: *mut utrap_handler_t,
    ) -> core::ffi::c_long;
    #[cfg(CONFIG_SPARC64)]
    pub fn sys_memory_ordering(model: core::ffi::c_ulong) -> core::ffi::c_long;
    #[cfg(CONFIG_SPARC64)]
    pub fn sparc64_set_context(regs: *mut pt_regs);
    #[cfg(CONFIG_SPARC64)]
    pub fn sparc64_get_context(regs: *mut pt_regs);
    #[cfg(CONFIG_SPARC64)]
    pub fn compat_sys_truncate64(
        path: *const core::ffi::c_char,
        high: u32,
        low: u32,
    ) -> core::ffi::c_long;
    #[cfg(CONFIG_SPARC64)]
    pub fn compat_sys_ftruncate64(
        fd: core::ffi::c_uint,
        high: u32,
        low: u32,
    ) -> core::ffi::c_long;
    #[cfg(CONFIG_SPARC64)]
    pub fn compat_sys_stat64(
        filename: *const core::ffi::c_char,
        statbuf: *mut compat_stat64,
    ) -> core::ffi::c_long;
    #[cfg(CONFIG_SPARC64)]
    pub fn compat_sys_lstat64(
        filename: *const core::ffi::c_char,
        statbuf: *mut compat_stat64,
    ) -> core::ffi::c_long;
    #[cfg(CONFIG_SPARC64)]
    pub fn compat_sys_fstat64(
        fd: core::ffi::c_uint,
        statbuf: *mut compat_stat64,
    ) -> core::ffi::c_long;
    #[cfg(CONFIG_SPARC64)]
    pub fn compat_sys_fstatat64(
        dfd: core::ffi::c_uint,
        filename: *const core::ffi::c_char,
        statbuf: *mut compat_stat64,
        flag: core::ffi::c_int,
    ) -> core::ffi::c_long;
    #[cfg(CONFIG_SPARC64)]
    pub fn compat_sys_pread64(
        fd: core::ffi::c_uint,
        ubuf: *mut core::ffi::c_char,
        count: compat_size_t,
        poshi: u32,
        poslo: u32,
    ) -> core::ffi::c_long;
    #[cfg(CONFIG_SPARC64)]
    pub fn compat_sys_pwrite64(
        fd: core::ffi::c_uint,
        ubuf: *mut core::ffi::c_char,
        count: compat_size_t,
        poshi: u32,
        poslo: u32,
    ) -> core::ffi::c_long;
    #[cfg(CONFIG_SPARC64)]
    pub fn compat_sys_readahead(
        fd: core::ffi::c_int,
        offhi: core::ffi::c_uint,
        offlo: core::ffi::c_uint,
        count: compat_size_t,
    ) -> core::ffi::c_long;
    #[cfg(CONFIG_SPARC64)]
    pub fn compat_sys_fadvise64(
        fd: core::ffi::c_int,
        offhi: core::ffi::c_uint,
        offlo: core::ffi::c_uint,
        len: compat_size_t,
        advice: core::ffi::c_int,
    ) -> core::ffi::c_long;
    #[cfg(CONFIG_SPARC64)]
    pub fn compat_sys_fadvise64_64(
        fd: core::ffi::c_int,
        offhi: core::ffi::c_uint,
        offlo: core::ffi::c_uint,
        lenhi: core::ffi::c_uint,
        lenlo: core::ffi::c_uint,
        advice: core::ffi::c_int,
    ) -> core::ffi::c_long;
    #[cfg(CONFIG_SPARC64)]
    pub fn compat_sys_sync_file_range(
        fd: core::ffi::c_uint,
        off_high: core::ffi::c_uint,
        off_low: core::ffi::c_uint,
        nb_high: core::ffi::c_uint,
        nb_low: core::ffi::c_uint,
        flags: core::ffi::c_uint,
    ) -> core::ffi::c_long;
    #[cfg(CONFIG_SPARC64)]
    pub fn compat_sys_fallocate(
        fd: core::ffi::c_int,
        mode: core::ffi::c_int,
        offhi: u32,
        offlo: u32,
        lenhi: u32,
        lenlo: u32,
    ) -> core::ffi::c_long;
}

// Opaque types and aliases are supplied by the translated dependency headers:
// pt_regs, size_t, utrap_entry_t, utrap_handler_t, compat_stat64, compat_size_t.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
