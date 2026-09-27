/* SPDX-License-Identifier: GPL-2.0 */

// Translated from systbls.h.
// The original Linux header dependencies are supplied by other translation units.

extern "C" {
    pub fn sys_getpagesize() -> kernel::ffi::c_long;
    pub fn sys_sparc_pipe() -> kernel::ffi::c_long;
    pub fn sys_nis_syscall() -> kernel::ffi::c_long;
    pub fn sys_getdomainname(
        name: *mut kernel::ffi::c_char,
        len: kernel::ffi::c_int,
    ) -> kernel::ffi::c_long;
    pub fn do_rt_sigreturn(regs: *mut pt_regs);
    pub fn sys_mmap(
        addr: kernel::ffi::c_ulong,
        len: kernel::ffi::c_ulong,
        prot: kernel::ffi::c_ulong,
        flags: kernel::ffi::c_ulong,
        fd: kernel::ffi::c_ulong,
        off: kernel::ffi::c_ulong,
    ) -> kernel::ffi::c_long;
    pub fn sparc_breakpoint(regs: *mut pt_regs);

    // CONFIG_SPARC32
    #[cfg(CONFIG_SPARC32)]
    pub fn sys_mmap2(
        addr: kernel::ffi::c_ulong,
        len: kernel::ffi::c_ulong,
        prot: kernel::ffi::c_ulong,
        flags: kernel::ffi::c_ulong,
        fd: kernel::ffi::c_ulong,
        pgoff: kernel::ffi::c_ulong,
    ) -> kernel::ffi::c_long;
    #[cfg(CONFIG_SPARC32)]
    pub fn sys_sparc_remap_file_pages(
        start: kernel::ffi::c_ulong,
        size: kernel::ffi::c_ulong,
        prot: kernel::ffi::c_ulong,
        pgoff: kernel::ffi::c_ulong,
        flags: kernel::ffi::c_ulong,
    ) -> kernel::ffi::c_long;

    // CONFIG_SPARC64
    #[cfg(CONFIG_SPARC64)]
    pub fn sys_sparc_ipc(
        call: kernel::ffi::c_uint,
        first: kernel::ffi::c_int,
        second: kernel::ffi::c_ulong,
        third: kernel::ffi::c_ulong,
        ptr: *mut kernel::ffi::c_void,
        fifth: kernel::ffi::c_long,
    ) -> kernel::ffi::c_long;
    #[cfg(CONFIG_SPARC64)]
    pub fn sparc64_personality(personality: kernel::ffi::c_ulong) -> kernel::ffi::c_long;
    #[cfg(CONFIG_SPARC64)]
    pub fn sys64_munmap(addr: kernel::ffi::c_ulong, len: size_t) -> kernel::ffi::c_long;
    #[cfg(CONFIG_SPARC64)]
    pub fn sys64_mremap(
        addr: kernel::ffi::c_ulong,
        old_len: kernel::ffi::c_ulong,
        new_len: kernel::ffi::c_ulong,
        flags: kernel::ffi::c_ulong,
        new_addr: kernel::ffi::c_ulong,
    ) -> kernel::ffi::c_ulong;
    #[cfg(CONFIG_SPARC64)]
    pub fn sys_utrap_install(
        r#type: utrap_entry_t,
        new_p: utrap_handler_t,
        new_d: utrap_handler_t,
        old_p: *mut utrap_handler_t,
        old_d: *mut utrap_handler_t,
    ) -> kernel::ffi::c_long;
    #[cfg(CONFIG_SPARC64)]
    pub fn sys_memory_ordering(model: kernel::ffi::c_ulong) -> kernel::ffi::c_long;
    #[cfg(CONFIG_SPARC64)]
    pub fn sparc64_set_context(regs: *mut pt_regs);
    #[cfg(CONFIG_SPARC64)]
    pub fn sparc64_get_context(regs: *mut pt_regs);
    #[cfg(CONFIG_SPARC64)]
    pub fn compat_sys_truncate64(
        path: *const kernel::ffi::c_char,
        high: u32,
        low: u32,
    ) -> kernel::ffi::c_long;
    #[cfg(CONFIG_SPARC64)]
    pub fn compat_sys_ftruncate64(
        fd: kernel::ffi::c_uint,
        high: u32,
        low: u32,
    ) -> kernel::ffi::c_long;
    #[cfg(CONFIG_SPARC64)]
    pub fn compat_sys_stat64(
        filename: *const kernel::ffi::c_char,
        statbuf: *mut compat_stat64,
    ) -> kernel::ffi::c_long;
    #[cfg(CONFIG_SPARC64)]
    pub fn compat_sys_lstat64(
        filename: *const kernel::ffi::c_char,
        statbuf: *mut compat_stat64,
    ) -> kernel::ffi::c_long;
    #[cfg(CONFIG_SPARC64)]
    pub fn compat_sys_fstat64(
        fd: kernel::ffi::c_uint,
        statbuf: *mut compat_stat64,
    ) -> kernel::ffi::c_long;
    #[cfg(CONFIG_SPARC64)]
    pub fn compat_sys_fstatat64(
        dfd: kernel::ffi::c_uint,
        filename: *const kernel::ffi::c_char,
        statbuf: *mut compat_stat64,
        flag: kernel::ffi::c_int,
    ) -> kernel::ffi::c_long;
    #[cfg(CONFIG_SPARC64)]
    pub fn compat_sys_pread64(
        fd: kernel::ffi::c_uint,
        ubuf: *mut kernel::ffi::c_char,
        count: compat_size_t,
        poshi: u32,
        poslo: u32,
    ) -> kernel::ffi::c_long;
    #[cfg(CONFIG_SPARC64)]
    pub fn compat_sys_pwrite64(
        fd: kernel::ffi::c_uint,
        ubuf: *mut kernel::ffi::c_char,
        count: compat_size_t,
        poshi: u32,
        poslo: u32,
    ) -> kernel::ffi::c_long;
    #[cfg(CONFIG_SPARC64)]
    pub fn compat_sys_readahead(
        fd: kernel::ffi::c_int,
        offhi: kernel::ffi::c_uint,
        offlo: kernel::ffi::c_uint,
        count: compat_size_t,
    ) -> kernel::ffi::c_long;
    #[cfg(CONFIG_SPARC64)]
    pub fn compat_sys_fadvise64(
        fd: kernel::ffi::c_int,
        offhi: kernel::ffi::c_uint,
        offlo: kernel::ffi::c_uint,
        len: compat_size_t,
        advice: kernel::ffi::c_int,
    ) -> kernel::ffi::c_long;
    #[cfg(CONFIG_SPARC64)]
    pub fn compat_sys_fadvise64_64(
        fd: kernel::ffi::c_int,
        offhi: kernel::ffi::c_uint,
        offlo: kernel::ffi::c_uint,
        lenhi: kernel::ffi::c_uint,
        lenlo: kernel::ffi::c_uint,
        advice: kernel::ffi::c_int,
    ) -> kernel::ffi::c_long;
    #[cfg(CONFIG_SPARC64)]
    pub fn compat_sys_sync_file_range(
        fd: kernel::ffi::c_uint,
        off_high: kernel::ffi::c_uint,
        off_low: kernel::ffi::c_uint,
        nb_high: kernel::ffi::c_uint,
        nb_low: kernel::ffi::c_uint,
        flags: kernel::ffi::c_uint,
    ) -> kernel::ffi::c_long;
    #[cfg(CONFIG_SPARC64)]
    pub fn compat_sys_fallocate(
        fd: kernel::ffi::c_int,
        mode: kernel::ffi::c_int,
        offhi: u32,
        offlo: u32,
        lenhi: u32,
        lenlo: u32,
    ) -> kernel::ffi::c_long;
}

// Opaque types and aliases are supplied by the translated dependency headers:
// pt_regs, size_t, utrap_entry_t, utrap_handler_t, compat_stat64, compat_size_t.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
